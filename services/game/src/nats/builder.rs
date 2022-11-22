use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
use gsfw::component;
use tokio::sync::mpsc;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    nats: Option<async_nats::Client>,
    brkr: Option<Hub>,
}

impl Builder {
    pub fn with_nats(mut self, nats: async_nats::Client) -> Self {
        self.nats = Some(nats);
        self
    }
}

impl component::ComponentBuilder<ChanProto, ModuleName, Hub, Error, mpsc::Receiver<ChanCtx>>
    for Builder
{
    fn build(self: Box<Self>) -> Box<dyn component::Component<ChanProto, ModuleName, Error>> {
        Box::new(super::NatsComponent {
            nats: self.nats.unwrap(),
            hub: self.brkr.unwrap(),
            rx: self.rx.unwrap(),
        })
    }
    fn name(&self) -> ModuleName {
        self.name
    }
    fn set_rx(&mut self, rx: tokio::sync::mpsc::Receiver<ChanCtx>) {
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
