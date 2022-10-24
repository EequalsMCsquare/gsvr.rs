use anyhow::bail;
use async_trait::async_trait;
use hashbrown::HashMap;
use tokio::sync::mpsc;
mod ctx;
pub use ctx::ChanCtx;

#[async_trait]
pub trait Broker<T, M>
where
    M: Send,
    T: Send,
{
    fn new(name: M, tx_map: &HashMap<M, mpsc::Sender<ChanCtx<T, M>>>) -> Self;

    fn name(&self) -> M;

    fn get_tx<'a>(&'a self, name: M) -> &'a mpsc::Sender<ChanCtx<T, M>>;

    async fn cast<'a>(&'a self, to: M, msg: T)
    where
        T: 'a,
        M: 'a,
    {
        let chan = self.get_tx(to);
        if let Err(err) = chan.send(ChanCtx::new_cast(msg, self.name())).await {
            tracing::error!("fail to cast. {}", err)
        }
    }

    fn blocking_cast(&self, to: M, msg: T) {
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ChanCtx::new_cast(msg, self.name())) {
            tracing::error!("fail to cast. {}", err)
        }
    }

    async fn call<'a>(&'a self, to: M, msg: T) -> anyhow::Result<T>
    where
        T: 'a,
        M: 'a,
    {
        let (ctx, rx) = ChanCtx::new_call(msg, self.name());
        let chan = self.get_tx(to);
        if let Err(err) = chan.send(ctx).await {
            tracing::error!("fail to request. {}", err);
            bail!(err.to_string())
        }
        rx.await?
    }

    fn blocking_call(&self, to: M, msg: T) -> anyhow::Result<T> {
        let (ctx, rx) = ChanCtx::new_call(msg, self.name());
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ctx) {
            tracing::error!("fail to request. {}", err);
            bail!(err.to_string())
        }
        rx.blocking_recv()?
    }
}
