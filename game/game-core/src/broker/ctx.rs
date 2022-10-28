use std::fmt::Debug;

use tokio::sync::oneshot;

pub trait Proto: Send + Debug {
    fn proto_shutdown() -> Self;
}

#[derive(Debug)]
pub struct ChanCtx<P, NameEnum, Error>
where
    P: Proto,
    Error: Send,
{
    pub payload: P,
    pub from: NameEnum,
    reply_chan: Option<oneshot::Sender<Result<P, Error>>>,
}

impl<P, NameEnum, Error> ChanCtx<P, NameEnum, Error>
where
    P: Proto,
    Error: Send,
{
    pub fn new_call(
        msg: P,
        from: NameEnum,
    ) -> (
        ChanCtx<P, NameEnum, Error>,
        oneshot::Receiver<Result<P, Error>>,
    ) {
        let (tx, rx) = oneshot::channel();
        (
            Self {
                payload: msg,
                reply_chan: Some(tx),
                from,
            },
            rx,
        )
    }

    pub fn new_cast(msg: P, from: NameEnum) -> ChanCtx<P, NameEnum, Error> {
        Self {
            payload: msg,
            reply_chan: None,
            from,
        }
    }

    pub fn ok(self, reply: P) {
        if let Some(reply_chan) = self.reply_chan {
            if let Err(_) = reply_chan.send(Ok(reply)) {
                tracing::error!("ChanRpc fail to reply with Ok. receiver dropped");
            }
            return;
        }
        tracing::warn!("attempt to reply to a non request ctx");
    }

    pub fn err(self, err: Error) {
        if let Some(reply_chan) = self.reply_chan {
            if let Err(_) = reply_chan.send(Err(err)) {
                tracing::error!("ChanRpc fail to reply with Err. receiver dropped");
            }
            return;
        }
        tracing::warn!("attempt to reply to a non request ctx");
    }
}
