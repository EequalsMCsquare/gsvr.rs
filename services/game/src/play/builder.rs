use super::{player_mgr::PlayerMgr, PlayComponent};
use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
use gsfw::component;
use std::cell::RefCell;
use tokio::sync::mpsc;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl component::ComponentBuilder<ChanProto, ModuleName, Hub, Error, mpsc::Receiver<ChanCtx>>
    for Builder
{
    fn build(self: Box<Self>) -> Box<dyn component::Component<ChanProto, ModuleName, Error>> {
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
