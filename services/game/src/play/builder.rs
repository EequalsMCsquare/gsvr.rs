use super::{
    worker::{Worker, WorkerHandle},
    PlayComponent,
};
use crate::hub::{ChanCtx, Hub, ModuleName};
use gsfw::component;
use tokio::sync::mpsc;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
    worker_num: usize,
}

impl component::ComponentBuilder<Hub> for Builder {
    fn build(self: Box<Self>) -> Box<dyn component::Component<Hub>> {
        let (ptx, prx) = crossbeam_channel::bounded(8196);
        Box::new(PlayComponent {
            rx: self.rx.unwrap(),
            broker: self.brkr.unwrap(),
            workers: vec![
                (
                    crossbeam_channel::bounded(4096),
                    crossbeam_channel::bounded(1)
                );
                self.worker_num
            ]
            .into_iter()
            .map(|((wtx, wrx), (ctx, crx))| WorkerHandle {
                close_tx: ctx,
                wtx,
                worker: Worker::new(wrx, ptx.clone(), crx),
            })
            .collect(),
            pmsg_rx: prx,
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
            worker_num: 4,
        }
    }

    pub fn worker_num(mut self, num: usize) -> Self {
        self.worker_num = num;
        self
    }
}
