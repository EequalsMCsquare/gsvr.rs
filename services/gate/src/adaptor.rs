use anyhow::anyhow;
use async_trait::async_trait;
use bytes::Bytes;
use cspb::Message;
use futures::{SinkExt, StreamExt};
use gsfw::{codec, network::*};
use spb::auth::VerifyTokenReq;
use std::sync::atomic::AtomicU64;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::codec::{DecoderImpl, EncoderImpl};

static ADAPTOR_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct NatsAdaptorBuilder {
    pub env: String,
    pub nats: async_nats::Client,
    pub auth: spb::AuthServiceClient<tonic::transport::Channel>,
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
            auth: self.auth.clone(),
        }
    }
}

pub struct NatsAdaptor {
    player_id: i64,
    cstopic: String,
    nats: async_nats::Client,
    sub: async_nats::Subscriber,
    auth: spb::AuthServiceClient<tonic::transport::Channel>,
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
            let pbmsg = cspb::CsProto::decode(msg)?;
            if let Some(payload) = pbmsg.payload {
                match payload {
                    cspb::CsMsg::CsLogin(msg) => {
                        match self
                            .auth
                            .verify_token(VerifyTokenReq { token: msg.token })
                            .await
                        {
                            Ok(ack) => {
                                let ack = ack.into_inner();
                                tracing::info!("verify_token success. {:?}", ack);
                                let reply = cspb::ScProto {
                                    payload: Some(cspb::ScMsg::ScLogin(cspb::ScLogin {
                                        err_code: cspb::ErrCode::Success as i32,
                                    })),
                                };
                                sink.send(reply.encode_to_vec().into()).await?;
                                self.player_id = msg.player_id;
                                self.sub = match self
                                    .nats
                                    .subscribe(format!("scp.{}", self.player_id))
                                    .await
                                {
                                    Ok(sub) => sub,
                                    Err(err) => return Err(err),
                                };
                                self.cstopic = format!("csp.{}", self.player_id);
                                Ok((stream, sink))
                            }
                            Err(err) => {
                                tracing::error!("verify_token error. {}", err);
                                let reply = cspb::ScProto {
                                    payload: Some(cspb::ScMsg::ScLogin(cspb::ScLogin {
                                        err_code: cspb::ErrCode::Internal as i32,
                                    })),
                                };
                                sink.send(reply.encode_to_vec().into()).await?;
                                Err(crate::Error::VerToken(err).into())
                            }
                        }
                    }
                    cspb::CsMsg::CsFastLogin(msg) => {
                        self.player_id = msg.player_id;
                        let reply = cspb::ScProto {
                            payload: Some(cspb::ScMsg::ScFastLogin(cspb::ScFastLogin {
                                err_code: cspb::ErrCode::Success.into(),
                            })),
                        };
                        sink.send(reply.encode_to_vec().into()).await?;
                        self.sub =
                            match self.nats.subscribe(format!("scp.{}", self.player_id)).await {
                                Ok(sub) => sub,
                                Err(err) => return Err(err),
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
