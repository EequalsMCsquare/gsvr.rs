mod builder;
mod handler;
mod item;
mod item_mgr;
mod player;
mod player_mgr;
use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
use anyhow::{anyhow, bail};
pub use builder::Builder;
use game_core::{broker::Broker, component::Component};
use pb::Message;
use player_mgr::PlayerMgr;
use std::{cell::RefCell, error::Error as StdError};
use tokio::sync::mpsc;

pub struct PlayComponent {
    players: RefCell<PlayerMgr>,
    rx: mpsc::Receiver<ChanCtx>,
    broker: Hub,
}

#[async_trait::async_trait]
impl Component<ModuleName, ChanProto> for PlayComponent {
    type BrkrError = Error;
    fn name(&self) -> ModuleName {
        ModuleName::Play
    }

    async fn run(mut self: Box<Self>) -> anyhow::Result<()> {
        std::thread::spawn(move || self._run())
            .join()
            .map_err(|err| anyhow!("JoinError: {:?}", err))?
    }

    async fn init(
        &mut self,
    ) -> std::result::Result<(), Box<(dyn StdError + std::marker::Send + 'static)>> {
        // register player message to mpsc::Receiver
        self.init_sub_pmsg().await?;
        Ok(())
    }
}

impl PlayComponent {
    fn _run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Some(req) = self.rx.blocking_recv() {
                match req.payload {
                    ChanProto::CsPMsgNtf { player_id, message } => {
                        tracing::debug!("recv player-{}: {:?}", player_id, message);
                        self.handle_pmsg(player_id, message);
                    }
                    ChanProto::CtrlShutdown => {
                        tracing::info!("[{:?}]recv shutdown", ModuleName::Play);
                        return Ok(());
                    }
                    _um => tracing::error!("[play] receive unhandled ChanProto: {:?}", _um),
                }
            } else {
                return Err(anyhow!("[play] no ProtoSender left"));
            }
        }
    }

    async fn init_sub_pmsg(
        &mut self,
    ) -> std::result::Result<(), Box<(dyn StdError + std::marker::Send + 'static)>> {
        if let Err(err) = self
            .broker
            .call(
                ModuleName::Nats,
                ChanProto::Sub2HubReq {
                    topic: "cspmsg.*".to_string(),
                    decode_fn: Self::pmsg_decode_fn,
                },
            )
            .await
        {
            return Err(anyhow!("fail to register player message to self broker. {}", err).into());
        }
        Ok(())
    }

    fn sendp(&self, player_id: u64, msg: pb::ScMsg) {
        tracing::debug!("send player-{} {:?}", player_id, msg);
        self.broker.blocking_cast(
            ModuleName::Nats,
            ChanProto::ScPMsgNtf {
                player_id,
                message: msg,
            },
        )
    }

    fn pmsg_decode_fn(msg: async_nats::Message) -> anyhow::Result<ChanProto> {
        if let Some(strpid) = msg.subject.split('.').skip(1).last() {
            if let Ok(num) = strpid.parse() {
                let proto = match pb::CsProto::decode(msg.payload) {
                    Ok(csproto) => csproto,
                    Err(err) => return Err(anyhow!("{:?}", err)),
                };
                if let Some(payload) = proto.payload {
                    return Ok(ChanProto::CsPMsgNtf {
                        player_id: num,
                        message: payload,
                    });
                } else {
                    bail!("no payload")
                }
            } else {
                bail!("invalid MQ message: {:?}", msg);
            }
        } else {
            bail!("invalid MQ message: {:?}", msg);
        }
    }
}