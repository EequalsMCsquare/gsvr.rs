use crate::hub::{ChanProto, Hub, ModuleName};
use game_core::{broker::ChanCtx, plugin::PluginBuilder};
use tokio::sync::mpsc;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx<ChanProto, ModuleName>>>,
    nats: Option<async_nats::Client>,
    brkr: Option<Hub>,
}

impl Builder {
    pub fn with_nats(mut self, nats: async_nats::Client) -> Self {
        self.nats = Some(nats);
        self
    }
}

impl PluginBuilder<ModuleName, ChanProto, Hub> for Builder {
    fn build(self: Box<Self>) -> Box<dyn game_core::plugin::Plugin<ModuleName, ChanProto>> {
        Box::new(super::NatsPlugin {
            nats: self.nats.unwrap(),
            hub: self.brkr.unwrap(),
            rx: self.rx.unwrap(),
        })
    }
    fn name(&self) -> ModuleName {
        self.name
    }
    fn set_rx(
        &mut self,
        rx: tokio::sync::mpsc::Receiver<game_core::broker::ChanCtx<ChanProto, ModuleName>>,
    ) {
        self.rx = Some(rx);
    }
    fn set_broker(&mut self, broker: Hub) {
        self.brkr = Some(broker);
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            name: ModuleName::Nats,
            rx: None,
            nats: None,
            brkr: None,
        }
    }
}
