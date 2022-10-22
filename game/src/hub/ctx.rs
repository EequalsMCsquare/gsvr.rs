use tokio::sync::oneshot;

use super::{ChanProto, ModuleName};

pub struct ChanCtx {
    pub payload: ChanProto,
    pub from: ModuleName,
    reply_chan: Option<oneshot::Sender<anyhow::Result<ChanProto>>>,
}

impl ChanCtx {
    pub fn new_req(
        msg: ChanProto,
        from: ModuleName,
    ) -> (ChanCtx, oneshot::Receiver<anyhow::Result<ChanProto>>) {
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

    pub fn new_cast(msg: ChanProto, from: ModuleName) -> ChanCtx {
        Self {
            payload: msg,
            reply_chan: None,
            from,
        }
    }

    pub fn ok(self, reply: ChanProto) {
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
