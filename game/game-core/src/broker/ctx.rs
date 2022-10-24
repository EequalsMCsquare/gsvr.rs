use tokio::sync::oneshot;

pub struct ChanCtx<T, M>
where
    T: Send,
{
    pub payload: T,
    pub from: M,
    reply_chan: Option<oneshot::Sender<anyhow::Result<T>>>,
}

impl<T, M> ChanCtx<T, M>
where
    T: Send,
{
    pub fn new_call(msg: T, from: M) -> (ChanCtx<T, M>, oneshot::Receiver<anyhow::Result<T>>) {
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

    pub fn new_cast(msg: T, from: M) -> ChanCtx<T, M> {
        Self {
            payload: msg,
            reply_chan: None,
            from,
        }
    }

    pub fn ok(self, reply: T) {
        if let Some(reply_chan) = self.reply_chan {
            if let Err(_) = reply_chan.send(Ok(reply)) {
                tracing::error!("ChanRpc fail to reply with Ok. receiver dropped");
            }
            return;
        }
        tracing::warn!("attempt to reply to a non request ctx");
    }

    pub fn err(self, err: anyhow::Error) {
        if let Some(reply_chan) = self.reply_chan {
            if let Err(_) = reply_chan.send(Err(err)) {
                tracing::error!("ChanRpc fail to reply with Err. receiver dropped");
            }
            return;
        }
        tracing::warn!("attempt to reply to a non request ctx");
    }
}
