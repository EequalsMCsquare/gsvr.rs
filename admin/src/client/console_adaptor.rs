use anyhow::anyhow;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use gsfw::network;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{broadcast, mpsc},
};
use tokio_util::codec::{FramedRead, FramedWrite};

use super::{client::ClientInfo, codec};

pub struct ConsoleAdaptor {
    client: super::ClientInfo,
    req_rx: broadcast::Receiver<super::ClientCsMsg>,
    ack_tx: mpsc::Sender<super::ClientScMsg>,
}

#[async_trait]
impl network::Adaptor for ConsoleAdaptor {
    type RecvItem = cspb::CsMsg;
    type Enc = codec::Encoder;
    type Dec = codec::Decoder;

    async fn ready<R, W>(
        &mut self,
        mut stream: FramedRead<R, Self::Dec>,
        mut sink: FramedWrite<W, Self::Enc>,
    ) -> Result<(FramedRead<R, Self::Dec>, FramedWrite<W, Self::Enc>), Box<dyn std::error::Error>>
    where
        R: AsyncRead + Send + Unpin,
        W: AsyncWrite + Send + Unpin,
    {
        let csmsg = cspb::CsMsg::CsFastLogin(cspb::CsFastLogin {
            player_id: self.client.id,
        });
        sink.send(csmsg).await?;
        if let Some(msg) = stream.next().await {
            let msg = msg?;
            match msg {
                cspb::ScMsg::ScLogin(_msg) => todo!(),
                cspb::ScMsg::ScFastLogin(msg) => {
                    if msg.err_code() == cspb::ErrCode::Success {
                        tracing::debug!("auth success");
                        return Ok((stream, sink));
                    }
                    return Err(anyhow!("auth fail. err_code: {:?}", msg.err_code()).into());
                }
                _unexpected => todo!(),
            }
        } else {
            return Err(anyhow!("connection close").into())
        }
    }

    // send sc to console
    async fn send(
        &mut self,
        msg: Result<cspb::ScMsg, anyhow::Error>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg = msg?;
        if let Err(err) = self
            .ack_tx
            .send(super::ClientScMsg {
                id: self.client.id,
                payload: msg,
            })
            .await
        {
            return Err(err.to_string().into());
        }
        Ok(())
    }

    // recv cs from console
    async fn recv(&mut self) -> Result<Option<Self::RecvItem>, Box<dyn std::error::Error + Send>> {
        match self.req_rx.recv().await {
            Ok(msg) => {
                if msg.ids.contains(&self.client.id) {
                    return Ok(Some(msg.payload));
                }
                Ok(None)
            }
            Err(err) => return Err(anyhow::Error::from(err).into()),
        }
    }
}

#[derive(Clone)]
pub struct ConsoleAdaptorBuilder {
    req_tx: broadcast::Sender<super::ClientCsMsg>,
    ack_tx: mpsc::Sender<super::ClientScMsg>,
    id: u64,
}

impl ConsoleAdaptorBuilder {
    pub fn new(
        req_tx: broadcast::Sender<super::ClientCsMsg>,
        ack_tx: mpsc::Sender<super::ClientScMsg>,
        id: u64,
    ) -> Self {
        Self { req_tx, ack_tx, id }
    }
}

#[async_trait]
impl network::AdaptorBuilder for ConsoleAdaptorBuilder {
    type Adaptor = ConsoleAdaptor;

    async fn build(self) -> Self::Adaptor {
        Self::Adaptor {
            client: ClientInfo {
                id: self.id,
                ..Default::default()
            },
            ack_tx: self.ack_tx,
            req_rx: self.req_tx.subscribe(),
        }
    }
}
