mod ctx;
use async_trait::async_trait;
use hashbrown::HashMap;
use tokio::sync::{mpsc, oneshot};
pub use ctx::ChanCtx;
pub use ctx::Proto;

pub struct CastTx<P: Proto, NameEnum: Send, Error: Send> {
    tx: mpsc::Sender<ChanCtx<P, NameEnum, Error>>,
    from: NameEnum,
}

impl<P, NameEnum, Error> CastTx<P, NameEnum, Error>
where
    P: Proto,
    NameEnum: Send + Copy + Clone,
    Error: Send,
{
    pub async fn cast(&self, msg: P) {
        if let Err(err) = self.tx.send(ChanCtx::new_cast(msg, self.from)).await {
            tracing::error!("fail to cast. {}", err)
        }
    }

    pub fn blocking_cast(&self, msg: P) {
        if let Err(err) = self.tx.blocking_send(ChanCtx::new_cast(msg, self.from)) {
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
            from: self.name(),
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
