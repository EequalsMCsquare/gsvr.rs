mod builder;
mod handler;
mod player;
mod player_mgr;
use crate::hub::{ChanCtx, ChanProto, Hub, ModuleName};
use anyhow::{anyhow, bail};
pub use builder::Builder;
use game_core::{
    broker::Broker,
    plugin::{Plugin, PluginJoinHandle},
};
use pb::Message;
use pin_project::pin_project;
use player_mgr::PlayerMgr;
use std::cell::RefCell;
use tokio::sync::mpsc;

pub struct PlayPlugin {
    players: RefCell<PlayerMgr>,

    rx: mpsc::Receiver<ChanCtx>,
    hub: Hub,
}

impl Plugin<ModuleName, ChanProto> for PlayPlugin {
    fn name(&self) -> ModuleName {
        ModuleName::Play
    }

    fn channel(&self) -> mpsc::Sender<game_core::broker::ChanCtx<ChanProto, ModuleName>> {
        self.hub.get_tx(self.name()).clone()
    }

    fn run(mut self: Box<Self>) -> PluginJoinHandle<anyhow::Error> {
        PluginJoinHandle::ThreadHandle(std::thread::spawn(move || loop {
            match self.rx.blocking_recv() {
                Some(req) => self.handle_chanproto(req),
                None => return Err(anyhow!("[play] no ProtoSender left")),
            };
        }))
    }

    fn init(&mut self) -> anyhow::Result<()> {
        // register player message to mpsc::Receiver
        let hub = self.hub.clone();
        std::thread::spawn(move || -> anyhow::Result<()> {
            if let Err(err) = hub.blocking_call(
                ModuleName::Nats,
                ChanProto::Sub2HubReq {
                    topic: "cspmsg.*".to_string(),
                    decode_fn: |msg: async_nats::Message| {
                        if let Some(strpid) = msg.subject.split('.').skip(1).last() {
                            if let Ok(num) = strpid.parse() {
                                let proto = match pb::CsProto::decode(msg.payload) {
                                    Ok(csproto) => csproto,
                                    Err(err) => return Err(anyhow!("{:?}", err)),
                                };
                                if let Some(payload) = proto.payload {
                                    return Ok(ChanProto::CsPMsg {
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
                    },
                },
            ) {
                panic!("fail to register player message to self broker. {}", err);
            }
            Ok(())
        })
        .join()
        .unwrap()
        .map_err(Into::into)
    }
}

impl PlayPlugin {
    fn handle_chanproto(&self, req: ChanCtx) {
        match req.payload {
            ChanProto::CsPMsg { player_id, message } => {
                tracing::debug!("recv player-{}: {:?}", player_id, message);
                self.handle_pmsg(player_id, message);
            }
            _um => tracing::error!("[play] receive unhandled ChanProto: {:?}", _um),
        }
    }

    fn sendp(&self, player_id: u64, msg: pb::ScMsg) {
        tracing::debug!("send player-{} {:?}", player_id, msg);
        self.hub.blocking_cast(
            ModuleName::Nats,
            ChanProto::ScPMsg {
                player_id,
                message: msg,
            },
        )
    }
}

#[pin_project]
pub struct Module {
    players: RefCell<PlayerMgr>,

    rx: mpsc::Receiver<ChanCtx>,
    tx: mpsc::Sender<ChanCtx>,
    hub: Option<Hub>,
}

// impl Module {
//     pub fn new() -> Module {
//         let (tx, rx) = mpsc::channel(1024);
//         Self {
//             players: RefCell::new(PlayerMgr::new()),
//             rx,
//             tx,
//             hub: None,
//         }
//     }

//     pub fn chanrpc(&self) -> mpsc::Sender<ChanCtx> {
//         self.tx.clone()
//     }

//     pub fn with_hub(&mut self, mut hub: Hub) {
//         hub.name = ModuleName::Play;
//         self.hub = Some(hub);
//     }

//     pub async fn init(&mut self) -> anyhow::Result<()> {
//         // subscribe play message topic
//         self.hub
//             .as_ref()
//             .unwrap()
//             .call(
//                 ModuleName::Nats,
//                 ChanProto::Sub2HubReq {
//                     topic: "cspmsg.*".to_string(),
//                     decode_fn: |msg: async_nats::Message| {
//                         if let Some(strpid) = msg.subject.split('.').skip(1).last() {
//                             if let Ok(num) = strpid.parse() {
//                                 let proto = match pb::CsProto::decode(msg.payload) {
//                                     Ok(csproto) => csproto,
//                                     Err(err) => return Err(anyhow!("{:?}", err)),
//                                 };
//                                 if let Some(payload) = proto.payload {
//                                     return Ok(ChanProto::CsPMsg {
//                                         player_id: num,
//                                         message: payload,
//                                     });
//                                 }
//                             }
//                         }
//                         todo!()
//                     },
//                 },
//             )
//             .await?;
//         Ok(())
//     }

//     pub fn run(&mut self) -> anyhow::Result<()> {
//         todo!()
//     }

//     fn handle_chanproto(&mut self, req: ChanCtx) {
//         match req.payload {
//             ChanProto::CsPMsg { player_id, message } => {
//                 tracing::debug!("recv pmsg-{}. {:?}", player_id, message);
//                 self.handle_player_msg(player_id, message);
//             }
//             _um => tracing::error!("[play] receive unhandled ChanProto {:?}", _um),
//         };
//     }

//     fn sendp(&self, player_id: u64, msg: pb::ScMsg) {
//         tracing::debug!("send player-{} {:?}", player_id, msg);
//         if let Err(err) = match &self.hub {
//             Some(hub) => {
//                 hub.blocking_cast(
//                     ModuleName::Nats,
//                     ChanProto::ScPMsg {
//                         player_id,
//                         message: msg,
//                     },
//                 );
//                 Ok(())
//             }
//             None => {
//                 // unlikely
//                 Err(anyhow!("[play] hub is None."))
//             }
//         } {
//             tracing::error!("[play] fail to send player. {}", err);
//         }
//     }
// }
