use gsfw::component;
use tokio::sync::mpsc;

use crate::hub::{ChanCtx, Hub, ModuleName};

use super::TimerComponet;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl component::ComponentBuilder<Hub> for Builder {
    fn name(&self) -> <Hub as gsfw::chanrpc::broker::Broker>::Name {
        self.name
    }

    fn build(self: Box<Self>) -> Box<dyn component::Component<Hub>> {
        Box::new(TimerComponet {
            broker: self.brkr.unwrap(),
            rx: self.rx.unwrap(),
        })
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
            name: ModuleName::Timer,
            rx: None,
            brkr: None,
        }
    }
}
