use anyhow::anyhow;
use anyhow::bail;
use bytes::Bytes;
use futures::ready;
use futures::Future;
use futures::SinkExt;
use futures::StreamExt;
use pb::Message;
use pin_project::pin_project;
use std::task::Poll;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use tokio_util::codec::FramedRead;
use tokio_util::codec::FramedWrite;
use tracing::debug;

#[pin_project(project=AgentProject)]
pub struct Agent<E, D> {
    id: u64,
    framed_reader: FramedRead<ReadHalf<TcpStream>, D>,
    framed_writer: FramedWrite<WriteHalf<TcpStream>, E>,
    nats: async_nats::Client,
    cs_topic: String,
    sc_topic: String,
    sub: Option<async_nats::Subscriber>,
}

impl<E, D> Agent<E, D>
where
    E: Encoder<Bytes, Error = anyhow::Error>,
    D: Decoder<Item = Bytes, Error = anyhow::Error>,
{
    pub fn new(stream: TcpStream, nats: async_nats::Client, decoder: D, encoder: E) -> Self {
        let (r, w) = tokio::io::split(stream);
        Self {
            id: 0,
            nats,
            cs_topic: String::default(),
            sc_topic: String::default(),
            framed_reader: FramedRead::with_capacity(r, decoder, 1024),
            framed_writer: FramedWrite::new(w, encoder),
            sub: None,
        }
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.verify_auth().await?;
        debug!(
            "agent[{}] auth success, subscribe to topic {}",
            self.id, self.sc_topic
        );
        Ok(())
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut sub = match self.nats.subscribe(self.sc_topic.clone()).await {
            Ok(sub) => sub,
            Err(err) => bail!("{:?}", err),
        };
        loop {
            tokio::select! {
                frame = self.read_frame() => {
                    match frame {
                        Ok(frame) => {
                            tracing::debug!("client[{}] receive frame: {:?}", self.id, frame);
                            self.nats.publish(self.cs_topic.clone(), frame).await?;
                        }
                        Err(err) => {
                            tracing::error!("read frame error. {}", err);
                            return Ok(())
                        }
                    };
                },
                Some(sc) = sub.next() => {
                    self.send_agent(sc.payload).await?;
                }
            }
        }
    }

    #[inline]
    async fn send_agent(&mut self, buf: Bytes) -> anyhow::Result<()> {
        tracing::info!("send agent. {:?}", buf);
        self.framed_writer.send(buf).await
    }

    #[inline]
    async fn read_frame(&mut self) -> anyhow::Result<Bytes> {
        match self.framed_reader.next().await {
            Some(ret) => ret,
            None => Err(anyhow!("read empty frame. connection closed")),
        }
    }

    async fn verify_auth(&mut self) -> anyhow::Result<()> {
        match self.read_frame().await {
            Ok(frame) => match Self::decode_csmsg(frame)? {
                pb::CsMsg::CsFastLogin(msg) => {
                    self.id = msg.player_id;
                    self.cs_topic = format!("cspmsg.{}", self.id);
                    self.sc_topic = format!("scpmsg.{}", self.id);
                    let sc = pb::ScProto {
                        payload: Some(pb::ScMsg::ScFastLogin(pb::ScFastLogin {
                            err_code: pb::ErrCode::Success as i32,
                        })),
                    };
                    if let Err(err) = self.send_agent(Bytes::from(sc.encode_to_vec())).await {
                        tracing::error!("fail to send agent[{}]. {:?}", self.id, err);
                    }
                    Ok(())
                }
                other => {
                    bail!(
                        "first message must pb::CsFastLogin or pb::CsLogin. but receive {:?}",
                        other
                    );
                }
            },
            Err(err) => Err(err),
        }
    }

    fn decode_csmsg(frame: Bytes) -> anyhow::Result<pb::CsMsg> {
        pb::CsProto::decode(frame)?
            .payload
            .ok_or(anyhow!("empty payload"))
    }

    fn poll_read_frame(
        reader: &mut FramedRead<ReadHalf<TcpStream>, D>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<anyhow::Result<Bytes>> {
        match ready!(reader.poll_next_unpin(cx)) {
            Some(ret) => Poll::Ready(ret.map_err(Into::into)),
            None => Poll::Ready(Err(anyhow!("read empty frame"))),
        }
    }
    fn poll_send_agent(
        writer: &mut FramedWrite<WriteHalf<TcpStream>, E>,
        cx: &mut std::task::Context<'_>,
        buf: Bytes,
    ) -> Poll<anyhow::Result<()>> {
        match writer.start_send_unpin(buf).map_err(Into::into) {
            Ok(_) => {
                if let Err(err) = ready!(writer.poll_flush_unpin(cx)) {
                    return Poll::Ready(Err(err));
                }
                Poll::Ready(Ok(()))
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl<E, D> Future for Agent<E, D> {
    type Output = anyhow::Result<()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
