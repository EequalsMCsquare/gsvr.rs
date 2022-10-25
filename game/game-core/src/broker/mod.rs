use async_trait::async_trait;
use hashbrown::HashMap;
use tokio::sync::{mpsc, oneshot};
mod ctx;

pub use ctx::ChanCtx;

#[async_trait]
pub trait Broker<Proto, NameEnum>
where
    NameEnum: Send,
    Proto: Send,
{
    type Error: Send
        + From<oneshot::error::RecvError>
        + From<mpsc::error::SendError<ChanCtx<Proto, NameEnum, Self::Error>>>;

    fn new(
        name: NameEnum,
        tx_map: &HashMap<NameEnum, mpsc::Sender<ChanCtx<Proto, NameEnum, Self::Error>>>,
    ) -> Self;

    fn name(&self) -> NameEnum;

    fn get_tx<'a>(
        &'a self,
        name: NameEnum,
    ) -> &'a mpsc::Sender<ChanCtx<Proto, NameEnum, Self::Error>>;

    async fn cast<'a>(&'a self, to: NameEnum, msg: Proto)
    where
        Proto: 'a,
        NameEnum: 'a,
    {
        let chan = self.get_tx(to);
        if let Err(err) = chan.send(ChanCtx::new_cast(msg, self.name())).await {
            tracing::error!("fail to cast. {}", err)
        }
    }

    fn blocking_cast(&self, to: NameEnum, msg: Proto) {
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ChanCtx::new_cast(msg, self.name())) {
            tracing::error!("fail to cast. {}", err)
        }
    }

    async fn call<'a>(&'a self, to: NameEnum, msg: Proto) -> Result<Proto, Self::Error>
    where
        Proto: 'a,
        NameEnum: 'a,
    {
        let (ctx, rx) = ChanCtx::new_call(msg, self.name());
        let chan = self.get_tx(to);
        if let Err(err) = chan.send(ctx).await {
            tracing::error!("fail to request. {}", err);
            return Err(Self::Error::from(err));
        }
        rx.await.map_err(|err| Self::Error::from(err))?
    }

    fn blocking_call(&self, to: NameEnum, msg: Proto) -> Result<Proto, Self::Error> {
        let (ctx, rx) = ChanCtx::new_call(msg, self.name());
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ctx) {
            tracing::error!("fail to request. {}", err);
            return Err(Self::Error::from(err));
        }
        rx.blocking_recv()?
    }
}
