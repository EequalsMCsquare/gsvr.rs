use super::Component;
use crate::{
    broker::{self, Broker, Proto},
};
use tokio::sync::{mpsc, oneshot};

pub trait ComponentBuilder<NameEnum, P, Brkr>
where
    NameEnum: Send,
    P: Proto,
    Brkr: Broker<P, NameEnum>,
{
    type BrkrError: Send;

    fn build(self: Box<Self>) -> Box<dyn Component<NameEnum, P, BrkrError = Self::BrkrError>>;
    fn name(&self) -> NameEnum;
    fn set_rx(&mut self, rx: mpsc::Receiver<broker::ChanCtx<P, NameEnum, Self::BrkrError>>);
    fn set_broker(&mut self, broker: Brkr);
    fn set_ctrl(&mut self, rx: oneshot::Receiver<()>);
}
