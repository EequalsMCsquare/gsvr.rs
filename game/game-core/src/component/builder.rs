use super::{Component};
use crate::broker::{self, Broker};
use tokio::sync::mpsc;

pub trait ComponentBuilder<NameEnum, Proto, Brkr>
where
    NameEnum: Send,
    Proto: Send,
    Brkr: Broker<Proto, NameEnum>,
{
    type BrkrError: Send;

    fn build(self: Box<Self>) -> Box<dyn Component<NameEnum, Proto, BrkrError = Self::BrkrError>>;
    fn name(&self) -> NameEnum;
    fn set_rx(&mut self, rx: mpsc::Receiver<broker::ChanCtx<Proto, NameEnum, Self::BrkrError>>);
    fn set_broker(&mut self, broker: Brkr);
}