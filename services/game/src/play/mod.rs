mod builder;
mod player;
mod playermgr;
mod proto;
mod worker;
use self::{
    player::{DBplayer, Player},
    playermgr::PlayerLoader,
    proto::WProto,
    worker::WorkerHandle,
};
use crate::{
    hub::{ChanCtx, GProto, Hub, ModuleName, PMSG},
    nats::MQProtoReq,
    timer::{TMProtoReq, TimerKind},
};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
pub use builder::Builder;
use gsfw::{chanrpc::broker::Broker, component, util::unwrap_as, RegistryExt};
use hashbrown::HashSet;
pub use proto::{PLProtoAck, PLProtoReq};
use std::{error::Error as StdError, sync::Arc};
use tokio::sync::mpsc;
use tracing::info;

pub struct PlayComponent {
    // workers: Vec<(worker::Worker, crossbeam_channel::Sender<WProto>)>,
    workers: Vec<WorkerHandle>,
    pmsg_rx: crossbeam_channel::Receiver<PMSG>,
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

    async fn init(
        mut self: Box<Self>,
    ) -> Result<Box<dyn component::Component<Hub>>, Box<dyn StdError + Send>> {
        // register player message to mpsc::Receiver
        self.init_sub_pmsg().await?;
        // self.init_timers().await?;
        Ok(self)
    }
}

impl PlayComponent {
    async fn _single_worker_run(&mut self) -> anyhow::Result<()> {
        let WorkerHandle {
            worker,
            wtx,
            close_tx: wkr_close_tx,
        } = self.workers.remove(0);
        let (psc_close_tx, psc_close_rx) = crossbeam_channel::bounded::<()>(1);
        // let (wkr_close_tx, wkr_close_rx) = crossbeam_channel::bounded::<()>(1);
        let (pld_close_tx, pld_close_rx) = crossbeam_channel::bounded::<()>(1);
        // spawn running worker in a new thread
        let worker_handle = std::thread::spawn(move || worker.run());
        let psc_casttx = self.broker.cast_tx(ModuleName::Nats);
        // spawn thread to proxy PSC to nats
        let prx = self.pmsg_rx.clone();
        let psc_span = tracing::info_span!("psc thread").or_current();
        let psc_handle = std::thread::spawn(move || {
            let _p = psc_span.enter();
            loop {
                crossbeam_channel::select! {
                    recv(prx) -> msg => match msg {
                        Ok(pmsg) => psc_casttx.blocking_cast(GProto::PMSG(pmsg)),
                        Err(err) => {
                            tracing::error!("recv psc error. {}", err);
                            break;
                        }
                    },
                    recv(psc_close_rx) -> _ => {
                        tracing::debug!("psc thread recv close signal");
                        return
                    }
                }
            }
        });
        let (pltx, plrx) = crossbeam_channel::bounded::<PMSG>(1024);
        let pload = PlayerLoader::new(
            Arc::new(self.broker.call_tx(ModuleName::DB)),
            Arc::new(self.broker.call_tx(ModuleName::Play)),
            Arc::new(self.broker.cast_tx(ModuleName::Play)),
            plrx,
            pld_close_rx,
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
                        tracing::info!("add player to worker[0]");
                        self._pset.insert(player.pid);
                        // self.send_player_to_worker(player, 0);
                        wtx.send(WProto::AddPlayer(player)).unwrap();
                        req.ok(GProto::Ok);
                    }
                },
                GProto::TMProtoNtf(snapshot) => {
                    tracing::info!("[Game] timer trigger: {:#?}", snapshot);
                }
                GProto::CtrlShutdown => {
                    info!("[play] recv shutdown");

                    pld_close_tx.send(()).unwrap();
                    pload_handle.join().expect("pload thread join fail")?;
                    tracing::debug!("pload_handle join success");

                    psc_close_tx.send(()).unwrap();
                    psc_handle.join().expect("psc thread join fail.");
                    tracing::debug!("psc_handle join success");

                    wkr_close_tx.send(()).unwrap();
                    worker_handle.join().expect("worker thread join fail.")?;
                    tracing::debug!("worker_handle join success");

                    return Ok(());
                }
                _unexpected => tracing::error!(
                    "recv unpexted ChanProto: {}",
                    Into::<&'static str>::into(_unexpected)
                ),
            }
        }
        Err(anyhow!("no ProtoSender left. exit"))
    }

    async fn _multi_worker_run(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    async fn init_sub_pmsg(&mut self) -> anyhow::Result<()> {
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
            return Err(anyhow!(
                "fail to register player message to self broker. {}",
                err
            ));
        }
        Ok(())
    }

    async fn init_timers(&mut self) -> anyhow::Result<()> {
        self.dispatch_interval(
            std::time::Duration::from_secs(5),
            TimerKind::GameDataLanding,
        )
        .await?;
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

    async fn dispatch_timeout(
        &mut self,
        duration: std::time::Duration,
        kind: TimerKind,
    ) -> anyhow::Result<()> {
        let snapshot = unwrap_as!(
            self.broker
                .call(
                    ModuleName::Timer,
                    TMProtoReq::NewTimeout { duration, kind }.into(),
                )
                .await?,
            GProto::TMProtoAck
        );
        tracing::debug!("dispatch new timeout timer: {:?}", snapshot);
        Ok(())
    }

    async fn dispatch_interval(
        &mut self,
        duration: std::time::Duration,
        kind: TimerKind,
    ) -> anyhow::Result<()> {
        let snapshot = unwrap_as!(
            self.broker
                .call(
                    ModuleName::Timer,
                    TMProtoReq::NewInterval { duration, kind }.into(),
                )
                .await?,
            GProto::TMProtoAck
        );
        tracing::debug!("dispatch new interval timer: {:?}", snapshot);
        Ok(())
    }

    async fn dispatch_deadline(
        &mut self,
        deadline: std::time::Instant,
        kind: TimerKind,
    ) -> anyhow::Result<()> {
        let snapshot = unwrap_as!(
            self.broker
                .call(
                    ModuleName::Timer,
                    TMProtoReq::NewDeadline { deadline, kind }.into(),
                )
                .await?,
            GProto::TMProtoAck
        );
        tracing::debug!("dispatch new deadline timer: {:?}", snapshot);
        Ok(())
    }
}
