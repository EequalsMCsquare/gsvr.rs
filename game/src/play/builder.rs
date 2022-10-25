use std::cell::RefCell;

use game_core::plugin::{Plugin, PluginBuilder};
use tokio::sync::mpsc;

use crate::{hub::{ChanCtx, ChanProto, Hub, ModuleName}, error::Error};

use super::{player_mgr::PlayerMgr, PlayPlugin};

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl PluginBuilder<ModuleName, ChanProto, Hub> for Builder {
    type BrkrError = Error;
    fn build(self: Box<Self>) -> Box<dyn Plugin<ModuleName, ChanProto, BrkrError =  Self::BrkrError>> {
        Box::new(PlayPlugin {
            players: RefCell::new(PlayerMgr::new()),
            rx: self.rx.unwrap(),
            hub: self.brkr.unwrap(),
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
