mod handler;
mod player;
mod player_mgr;
use std::cell::RefCell;

use anyhow::anyhow;
use pb::Message;
use pin_project::pin_project;
use player_mgr::PlayerMgr;
use tokio::sync::mpsc;

use crate::hub::ChanCtx;
use crate::hub::ChanProto;
use crate::hub::Hub;
use crate::hub::ModuleName;
use crate::hub::ModuleProto;

#[pin_project]
pub struct Module {
    players: RefCell<PlayerMgr>,

    rx: mpsc::Receiver<ChanCtx>,
    tx: mpsc::Sender<ChanCtx>,
    hub: Option<Hub>,
}

impl Module {
    pub fn new() -> Module {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            players: RefCell::new(PlayerMgr::new()),
            rx,
            tx,
            hub: None,
        }
    }

    pub fn chanrpc(&self) -> mpsc::Sender<ChanCtx> {
        self.tx.clone()
    }

    pub fn with_hub(&mut self, mut hub: Hub) {
        hub.module = ModuleName::Play;
        self.hub = Some(hub);
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        // subscribe play message topic
        self.hub
            .as_ref()
            .unwrap()
            .call(ModuleProto::Nats(ChanProto::Sub2HubReq {
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
                            }
                        }
                    }
                    todo!()
                },
            }))
            .await?;
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            match self.rx.blocking_recv() {
                Some(req) => self.handle_chanproto(req),
                None => return Err(anyhow!("[play] no ProtoSender left")),
            };
        }
    }

    fn handle_chanproto(&mut self, req: ChanCtx) {
        match req.payload {
            ChanProto::CsPMsg { player_id, message } => {
                tracing::debug!("recv pmsg-{}. {:?}", player_id, message);
                self.handle_player_msg(player_id, message);
            }
            _um => tracing::error!("[play] receive unhandled ChanProto {:?}", _um),
        };
    }

    fn sendp(&self, player_id: u64, msg: pb::ScMsg) {
        tracing::debug!("send player-{} {:?}", player_id, msg);
        if let Err(err) = match &self.hub {
            Some(hub) => {
                hub.blocking_cast(ModuleProto::Nats(ChanProto::ScPMsg {
                    player_id,
                    message: msg,
                }));
                Ok(())
            }
            None => {
                // unlikely
                Err(anyhow!("[play] hub is None."))
            }
        } {
            tracing::error!("[play] fail to send player. {}", err);
        }
    }
}
