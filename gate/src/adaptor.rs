use anyhow::anyhow;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use gsfw::{codec, network::*};
use pb::Message;
use std::sync::atomic::AtomicU64;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::codec::{DecoderImpl, EncoderImpl};

static ADAPTOR_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct NatsAdaptorBuilder {
    pub env: String,
    pub nats: async_nats::Client,
}

#[async_trait]
impl AdaptorBuilder for NatsAdaptorBuilder {
    type Adaptor = NatsAdaptor;

    async fn build(self) -> Self::Adaptor {
        let session_id = ADAPTOR_ID.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        let sctopic = format!("scp.{}", session_id);
        let sub = self.nats.subscribe(sctopic).await.unwrap();
        Self::Adaptor {
            player_id: 0,
            cstopic: String::new(),
            sub,
            nats: self.nats.clone(),
        }
    }
}

pub struct NatsAdaptor {
    player_id: u64,
    cstopic: String,
    nats: async_nats::Client,
    sub: async_nats::Subscriber,
}

#[async_trait]
impl Adaptor for NatsAdaptor {
    type RecvItem = Bytes;
    type Enc = EncoderImpl;
    type Dec = DecoderImpl;

    async fn ready<R, W>(
        &mut self,
        mut stream: FramedRead<R, Self::Dec>,
        mut sink: FramedWrite<W, Self::Enc>,
    ) -> Result<(FramedRead<R, Self::Dec>, FramedWrite<W, Self::Enc>), Box<dyn std::error::Error>>
    where
        R: AsyncRead + Send + Unpin,
        W: AsyncWrite + Send + Unpin,
    {
        if let Some(msg) = stream.next().await {
            let msg = msg?;
            let pbmsg = pb::CsProto::decode(msg)?;
            if let Some(payload) = pbmsg.payload {
                match payload {
                    pb::CsMsg::CsLogin(_msg) => {
                        todo!()
                    }
                    pb::CsMsg::CsFastLogin(msg) => {
                        self.player_id = msg.player_id;
                        let reply = pb::ScProto {
                            payload: Some(pb::ScMsg::ScFastLogin(pb::ScFastLogin {
                                err_code: pb::ErrCode::Success.into(),
                            })),
                        };
                        sink.send(reply.encode_to_vec().into()).await?;
                        self.sub = match self.nats.subscribe(format!("scp.{}", self.player_id)).await {
                            Ok(sub) => sub,
                            Err(err) => return Err(err)
                        };
                        self.cstopic = format!("csp.{}", self.player_id);
                        return Ok((stream, sink));
                    }
                    _unexpected => todo!(),
                }
            } else {
                return Err(crate::Error::PBPayload.into());
            }
        } else {
            return Err(crate::Error::ReadZero.into());
        }
    }

    // send cs to nats
    async fn send(
        &mut self,
        msg: Result<<Self::Dec as codec::Decoder>::Item, <Self::Dec as codec::Decoder>::Error>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match msg {
            Ok(msg) => {
                if self.player_id == 0 {
                    // todo!

                    return Ok(());
                }
                if let Err(err) = self.nats.publish(self.cstopic.clone(), msg).await {
                    return Err(Box::new(err));
                }
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    // recv sc from nats
    async fn recv(&mut self) -> Result<Option<Self::RecvItem>, Box<dyn std::error::Error + Send>> {
        match self.sub.next().await {
            Some(msg) => Ok(Some(msg.payload)),
            None => Err(anyhow!("channel close").into()),
        }
    }
}
