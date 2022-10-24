use game_core::{
    broker::Broker,
    plugin::{Plugin, PluginJoinHandle},
};

use crate::hub::{ChanCtx, ChanProto, Hub, ModuleName};

mod builder;
pub use builder::Builder;

pub struct DBPlugin {
    broker: Hub,
}

impl Plugin<ModuleName, ChanProto> for DBPlugin {
    #[inline]
    fn name(&self) -> ModuleName {
        ModuleName::DB
    }

    fn channel(&self) -> tokio::sync::mpsc::Sender<ChanCtx> {
        self.broker.get_tx(self.name()).clone()
    }

    fn run(self: Box<Self>) -> PluginJoinHandle<anyhow::Error> {
        PluginJoinHandle::TokioHandle(tokio::spawn(async { Ok(()) }))
    }
}
