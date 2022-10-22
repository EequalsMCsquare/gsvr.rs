use futures::{Future, FutureExt};
use tokio::sync::mpsc;

use crate::hub::{ChanCtx, Hub, ProtoSender};

pub struct Module {
    tx: ProtoSender,
    rx: mpsc::Receiver<ChanCtx>,
    hub: Option<Hub>,
}

impl Module {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self { tx, rx, hub: None }
    }

    pub fn with_hub(&mut self, hub: Hub) {
        self.hub = Some(hub)
    }

    pub fn chanrpc(&self) -> ProtoSender {
        self.tx.clone()
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Future for Module {
    type Output = anyhow::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Box::pin(self.get_mut().run()).poll_unpin(cx)
    }
}
