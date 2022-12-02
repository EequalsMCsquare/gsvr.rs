use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio::{sync::{mpsc, broadcast}, io::{AsyncRead, AsyncWrite}};
use gsfw::network;
use tokio_util::codec::{FramedRead, FramedWrite};
use super::{codec, misc::ClientInfo};


pub struct ClientAdaptor {
    info: ClientInfo,
    req_rx: broadcast::Receiver<cspb::CsMsg>,
    ack_tx: mpsc::Sender<cspb::ScMsg>,
}

#[derive(Clone)]
pub struct ClientAdaptorBuilder {
    info: ClientInfo,
    req_tx: Arc<broadcast::Sender<cspb::CsMsg>>,
    ack_tx: mpsc::Sender<cspb::ScMsg>,
}

impl ClientAdaptorBuilder {
    pub fn new(
        req_tx: Arc<broadcast::Sender<cspb::CsMsg>>,
        ack_tx: mpsc::Sender<cspb::ScMsg>,
        info: ClientInfo,
    ) -> Self {
        Self {
            info,
            req_tx,
            ack_tx,
        }
    }
}

#[async_trait]
impl network::AdaptorBuilder for ClientAdaptorBuilder {
    type Adaptor = ClientAdaptor;

    async fn build(self) -> Self::Adaptor {
        Self::Adaptor {
            req_rx: self.req_tx.subscribe(),
            ack_tx: self.ack_tx,
            info: self.info,
        }
    }
}

#[async_trait]
impl network::Adaptor for ClientAdaptor {
    type RecvItem = cspb::CsMsg;
    type Enc = codec::Encoder;
    type Dec = codec::Decoder;

    async fn ready<R, W>(
        &mut self,
        mut fr: FramedRead<R, Self::Dec>,
        mut fw: FramedWrite<W, Self::Enc>,
    ) -> Result<(FramedRead<R, Self::Dec>, FramedWrite<W, Self::Enc>), Box<dyn std::error::Error>>
    where
        R: AsyncRead + Send + Unpin,
        W: AsyncWrite + Send + Unpin,
    {
        let csmsg = match &self.info {
            ClientInfo::FastLogin { player_id } => {
                cspb::CsMsg::CsFastLogin(cspb::CsFastLogin {
                    player_id: player_id.clone(),
                })
            }
            ClientInfo::Normal { player_id, token } => cspb::CsMsg::CsLogin(cspb::CsLogin {
                token: token.clone(),
                player_id: player_id.clone(),
            }),
        };
        sink.send(csmsg).await?;
        let ack = if let Some(msg) = stream.next().await {
            msg?
        } else {
            return Err(anyhow!("connection close").into());
        };
        match &self.info {
            ClientInfo::FastLogin { player_id: _ } => match ack {
                cspb::ScMsg::ScFastLogin(msg) => {
                    if msg.err_code() == cspb::ErrCode::Success {
                        tracing::debug!("auth success");
                        Ok((stream, sink))
                    } else {
                        Err(anyhow!("auth fail. err_code: {:?}", msg.err_code()).into())
                    }
                }
                _unexpected => Err(anyhow!("unexpect message: {:?}", _unexpected).into()),
            },
            ClientInfo::Normal {
                player_id: _,
                token: _,
            } => match ack {
                cspb::ScMsg::ScLogin(msg) => {
                    if msg.err_code() == cspb::ErrCode::Success {
                        tracing::debug!("auth success");
                        Ok((stream, sink))
                    } else {
                        Err(anyhow!("auth fail. err_code: {:?}", msg.err_code()).into())
                    }
                }
                _unexpected => Err(anyhow!("unexpect message: {:?}", _unexpected).into()),
            },
        }
    }

    // send sc to console
    async fn send(
        &mut self,
        msg: Result<cspb::ScMsg, anyhow::Error>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg = msg?;
        self.ack_tx.send(msg).await?;
        Ok(())
    }

    // recv cs from console
    async fn recv(&mut self) -> Result<Option<Self::RecvItem>, Box<dyn std::error::Error + Send>> {
        match self.req_rx.recv().await {
            Ok(msg) => Ok(Some(msg)),
            Err(err) => Err(anyhow!("channel close {}", err.to_string()).into()),
        }
    }
}