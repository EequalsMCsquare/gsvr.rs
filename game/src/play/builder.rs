use std::cell::RefCell;
use game_core::component::{Component, ComponentBuilder};
use tokio::sync::mpsc;
use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
use super::{player_mgr::PlayerMgr, PlayComponent};

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl ComponentBuilder<ModuleName, ChanProto, Hub> for Builder {
    type BrkrError = Error;
    fn build(
        self: Box<Self>,
    ) -> Box<dyn Component<ModuleName, ChanProto, BrkrError = Self::BrkrError>> {
        Box::new(PlayComponent {
            players: RefCell::new(PlayerMgr::new()),
            rx: self.rx.unwrap(),
            broker: self.brkr.unwrap(),
        })
    }

    fn name(&self) -> ModuleName {
        self.name
    }
    fn set_rx(&mut self, rx: mpsc::Receiver<ChanCtx>) {
        self.rx = Some(rx)
    }
    fn set_broker(&mut self, broker: Hub) {
        self.brkr = Some(broker);
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            name: ModuleName::Play,
            rx: None,
            brkr: None,
        }
    }
}
