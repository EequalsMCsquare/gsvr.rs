use gsfw::component;
use parking_lot::RwLock;
use tokio::sync::mpsc;

use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};

use super::TimerComponent;

pub struct Builder {
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
}

impl component::ComponentBuilder<ChanProto, ModuleName, Hub, Error, mpsc::Receiver<ChanCtx>>
    for Builder
{
    fn build(self: Box<Self>) -> Box<dyn component::Component<ChanProto, ModuleName, Error>> {
        let (tx, rx) = mpsc::channel(4);
        Box::new(TimerComponent {
            hub: self.brkr.unwrap(),
            rx: self.rx.unwrap(),
            timers: Default::default(),
            curr: RwLock::new(None),
            curr_handle: tokio::spawn(async {}),
            timer_rx: rx,
            timer_tx: tx,
        })
    }

    fn name(&self) -> ModuleName {
        ModuleName::Timer
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
            rx: None,
            brkr: None,
        }
    }
}