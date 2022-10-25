use tokio::sync::oneshot;

#[derive(Debug)]
pub struct ChanCtx<Proto, NameEnum, Error>
where
    Proto: Send,
    Error: Send,
{
    pub payload: Proto,
    pub from: NameEnum,
    reply_chan: Option<oneshot::Sender<Result<Proto, Error>>>,
}

impl<Proto, NameEnum, Error> ChanCtx<Proto, NameEnum, Error>
where
    Proto: Send,
    Error: Send,
{
    pub fn new_call(
        msg: Proto,
        from: NameEnum,
    ) -> (
        ChanCtx<Proto, NameEnum, Error>,
        oneshot::Receiver<Result<Proto, Error>>,
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

    pub fn new_cast(msg: Proto, from: NameEnum) -> ChanCtx<Proto, NameEnum, Error> {
        Self {
            payload: msg,
            reply_chan: None,
            from,
        }
    }

    pub fn ok(self, reply: Proto) {
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
