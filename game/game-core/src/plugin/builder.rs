use super::{Plugin};
use crate::broker::{self, Broker};
use tokio::sync::mpsc;

pub trait PluginBuilder<NameEnum, Proto, Brkr>
where
    NameEnum: Send,
    Proto: Send,
    Brkr: Broker<Proto, NameEnum>,
{
    fn build(self: Box<Self>) -> Box<dyn Plugin<NameEnum, Proto>>;
    fn name(&self) -> NameEnum;
    fn set_rx(&mut self, rx: mpsc::Receiver<broker::ChanCtx<Proto, NameEnum>>);
    fn set_broker(&mut self, broker: Brkr);
}