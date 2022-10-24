use game_core::plugin::{PluginBuilder, Plugin};
use tokio::sync::mpsc;

use crate::hub::{ChanCtx, ChanProto, Hub, ModuleName};

use super::DBPlugin;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl PluginBuilder<ModuleName, ChanProto, Hub> for Builder {
    fn build(self: Box<Self>) -> Box<dyn Plugin<ModuleName, ChanProto>> {
        Box::new(DBPlugin {
            broker: self.brkr.unwrap(),
        })
    }

    fn name(&self) -> ModuleName {
        self.name
    }

    fn set_rx(&mut self, rx: mpsc::Receiver<ChanCtx>) {
        self.rx = Some(rx);
    }

    fn set_broker(&mut self, broker: Hub) {
        self.brkr = Some(broker);
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            name: ModuleName::DB,
            rx: None,
            brkr: None,
        }
    }
}
