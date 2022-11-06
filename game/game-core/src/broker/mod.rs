mod ctx;
use async_trait::async_trait;
pub use ctx::ChanCtx;
pub use ctx::Proto;
use futures::StreamExt;
use hashbrown::HashMap;
use strum::IntoEnumIterator;
use tokio::sync::{mpsc, oneshot};

pub struct CastTx<P: Proto, NameEnum: Send, Error: Send> {
    tx: mpsc::Sender<ChanCtx<P, NameEnum, Error>>,
    to: NameEnum,
}

impl<P, NameEnum, Error> CastTx<P, NameEnum, Error>
where
    P: Proto,
    NameEnum: Send + Copy + Clone,
    Error: Send,
{
    pub async fn cast(&self, msg: P) {
        if let Err(err) = self.tx.send(ChanCtx::new_cast(msg, self.to)).await {
            tracing::error!("fail to cast. {}", err)
        }
    }

    pub fn blocking_cast(&self, msg: P) {
        if let Err(err) = self.tx.blocking_send(ChanCtx::new_cast(msg, self.to)) {
            tracing::error!("fail to cast. {}", err)
        }
    }
}

#[async_trait]
pub trait Broker<P, NameEnum>
where
    NameEnum: Send,
    P: Proto,
{
    type Error: Send
        + From<oneshot::error::RecvError>
        + From<mpsc::error::SendError<ChanCtx<P, NameEnum, Self::Error>>>;

    fn new(
        name: NameEnum,
        tx_map: &HashMap<NameEnum, mpsc::Sender<ChanCtx<P, NameEnum, Self::Error>>>,
    ) -> Self;

    fn name(&self) -> NameEnum;

    fn get_tx<'a>(&'a self, name: NameEnum) -> &'a mpsc::Sender<ChanCtx<P, NameEnum, Self::Error>>;

    fn cast_tx(&self, name: NameEnum) -> CastTx<P, NameEnum, Self::Error> {
        CastTx {
            to: self.name(),
            tx: self.get_tx(name).clone(),
        }
    }

    async fn cast<'a>(&'a self, to: NameEnum, msg: P)
    where
        P: 'a,
        NameEnum: 'a,
    {
        let chan = self.get_tx(to);
        if let Err(err) = chan.send(ChanCtx::new_cast(msg, self.name())).await {
            tracing::error!("fail to cast. {}", err)
        }
    }

    fn blocking_cast(&self, to: NameEnum, msg: P) {
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ChanCtx::new_cast(msg, self.name())) {
            tracing::error!("fail to cast. {}", err)
        }
    }

    async fn call<'a>(&'a self, to: NameEnum, msg: P) -> Result<P, Self::Error>
    where
        P: 'a,
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

    fn blocking_call(&self, to: NameEnum, msg: P) -> Result<P, Self::Error> {
        let (ctx, rx) = ChanCtx::new_call(msg, self.name());
        let chan = self.get_tx(to);
        if let Err(err) = chan.blocking_send(ctx) {
            tracing::error!("fail to request. {}", err);
            return Err(Self::Error::from(err));
        }
        rx.blocking_recv()?
    }
}

#[async_trait]
pub trait BroadcastBroker<P, NameEnum>: Broker<P, NameEnum>
where
    P: Proto + Clone + Sync,
    NameEnum: Send + strum::IntoEnumIterator,
{
    async fn broadcast(&self, msg: P)
    where
        <NameEnum as IntoEnumIterator>::Iterator: std::marker::Send,
        P: 'async_trait;

    fn blocking_broadcast(&self, msg: P);
}

#[async_trait]
impl<T, P, NameEnum> BroadcastBroker<P, NameEnum> for T
where
    NameEnum: Send + strum::IntoEnumIterator,
    P: Proto + Clone + Sync,
    T: Broker<P, NameEnum> + Sync,
{
    async fn broadcast(&self, msg: P)
    where
        <NameEnum as IntoEnumIterator>::Iterator: std::marker::Send,
        P: 'async_trait,
    {
        let mut stream = futures::stream::iter(NameEnum::iter());
        while let Some(e) = stream.next().await {
            self.cast(e, msg.clone()).await;
        }
    }

    fn blocking_broadcast(&self, msg: P) {
        for e in NameEnum::iter() {
            self.blocking_cast(e, msg.clone())
        }
    }
}
