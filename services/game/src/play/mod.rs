mod builder;
mod player;
mod playermgr;
mod proto;
mod worker;
use self::{
    player::{DBplayer, Player},
    playermgr::PlayerLoader,
    proto::WProto,
};
use crate::{
    hub::{ChanCtx, GProto, Hub, ModuleName, PMSG},
    nats::MQProtoReq,
};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
pub use builder::Builder;
use gsfw::{chanrpc::broker::Broker, component, RegistryExt};
use hashbrown::HashSet;
pub use proto::{PLProtoAck, PLProtoReq};
use std::{error::Error as StdError, sync::Arc};
use tokio::sync::mpsc;
use tracing::info;

pub struct PlayComponent {
    workers: Vec<(
        worker::Worker,
        crossbeam_channel::Sender<WProto>,
        crossbeam_channel::Receiver<PMSG>,
    )>,
    rx: mpsc::Receiver<ChanCtx>,
    broker: Hub,

    _pset: HashSet<i64>,
}

#[async_trait]
impl component::Component<Hub> for PlayComponent {
    fn name(&self) -> ModuleName {
        ModuleName::Play
    }

    async fn run(mut self: Box<Self>) -> Result<(), Box<dyn StdError + Send>> {
        if self.workers.len() == 1 {
            self._single_worker_run().await
        } else {
            self._multi_worker_run().await
        }
        .map_err(Into::into)
    }

    async fn init(&mut self) -> Result<(), Box<dyn StdError + Send>> {
        // register player message to mpsc::Receiver
        self.init_sub_pmsg().await.unwrap();
        Ok(())
    }
}

impl PlayComponent {
    async fn _single_worker_run(&mut self) -> anyhow::Result<()> {
        let (mut worker, wtx, prx) = self.workers.remove(0);
        let (close_tx, close_rx) = crossbeam_channel::bounded::<()>(1);
        // spawn running worker in a new thread
        let close_rx1 = close_rx.clone();
        let worker_handle = std::thread::spawn(move || worker.run(close_rx1));
        let psc_casttx = self.broker.cast_tx(ModuleName::Nats);
        // spawn thread to proxy PSC to nats
        let close_rx1 = close_rx.clone();
        let psc_handle = std::thread::spawn(move || loop {
            crossbeam_channel::select! {
                recv(prx) -> msg => match msg {
                    Ok(pmsg) => psc_casttx.blocking_cast(GProto::PMSG(pmsg)),
                    Err(err) => {
                        tracing::error!("[PSC thread]. {}", err);
                        break;
                    }
                },
                recv(close_rx1) -> _ => return
            }
        });
        let (pltx, plrx) = crossbeam_channel::bounded::<PMSG>(1024);
        let pload = PlayerLoader::new(
            Arc::new(self.broker.call_tx(ModuleName::DB)),
            Arc::new(self.broker.call_tx(ModuleName::Play)),
            Arc::new(self.broker.cast_tx(ModuleName::Play)),
            plrx,
            close_rx.clone(),
        );
        let pload_handle = std::thread::spawn(move || pload.run());

        // handle message
        while let Some(req) = self.rx.recv().await {
            match req.payload() {
                GProto::PMSG(msg) => {
                    // check player loaded
                    if self._pset.contains(&msg.player_id) {
                        wtx.send(WProto::PMSG(msg))?;
                    } else {
                        // send pcs to PlayerLoader
                        pltx.send(msg)?;
                    }
                }
                GProto::PLProtoReq(inner) => match inner {
                    PLProtoReq::AddPlayer(player) => {
                        tracing::debug!("add player to worker[0]");
                        self._pset.insert(player.pid);
                        // self.send_player_to_worker(player, 0);
                        wtx.send(WProto::AddPlayer(player)).unwrap();
                        req.ok(GProto::Ok);
                    }
                },
                GProto::CtrlShutdown => {
                    info!("[play] recv shutdown");
                    close_tx.send(()).unwrap();
                    worker_handle.join().expect("worker thread join fail.")?;
                    pload_handle.join().expect("pload thread join fail")?;
                    psc_handle.join().expect("psc thread join fail.");
                    return Ok(());
                }
                _unexpected => tracing::error!(
                    "[play] recv unpexted ChanProto: {}",
                    Into::<&'static str>::into(_unexpected)
                ),
            }
        }
        Err(anyhow!("[play] no ProtoSender left"))
    }

    async fn _multi_worker_run(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    async fn init_sub_pmsg(
        &mut self,
    ) -> std::result::Result<(), Box<(dyn StdError + std::marker::Send + 'static)>> {
        if let Err(err) = Broker::call(
            &self.broker,
            ModuleName::Nats,
            GProto::MQProtoReq(MQProtoReq::Sub2HubReq {
                topic: "csp.*".to_string(),
                decode_fn: Self::pmsg_decode_fn,
            }),
        )
        .await
        {
            return Err(anyhow!("fail to register player message to self broker. {}", err).into());
        }
        Ok(())
    }

    fn pmsg_decode_fn(msg: async_nats::Message) -> anyhow::Result<GProto> {
        if let Some(strpid) = msg.subject.split('.').skip(1).last() {
            if let Ok(num) = strpid.parse() {
                let proto = cspb::Registry::decode_frame(msg.payload)?;
                Ok(GProto::PMSG(PMSG {
                    player_id: num,
                    message: proto,
                }))
            } else {
                bail!("invalid MQ message: {:?}", msg);
            }
        } else {
            bail!("invalid MQ message: {:?}", msg);
        }
    }
}
