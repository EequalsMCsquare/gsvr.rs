use std::sync::{Arc, RwLock};

use crate::proto::*;
use anyhow::{anyhow, bail};
use futures::{SinkExt, StreamExt};
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use super::histroy::Histroy;

pub struct FastLoginClient {
    pub id: u64,
    framed_reader: FramedRead<ReadHalf<TcpStream>, pb::codec::ScProtoDecoder>,
    framed_writer: FramedWrite<WriteHalf<TcpStream>, pb::codec::CsProtoEncoder>,
    reqchan: tokio::sync::broadcast::Receiver<TagCsMsg<u64>>,
    ackchan: tokio::sync::mpsc::Sender<TagScMsg<u64>>,
    closechan: tokio::sync::oneshot::Receiver<()>,
    history: Arc<Histroy>,
}

impl FastLoginClient {
    pub fn new(
        id: u64,
        reqchan: tokio::sync::broadcast::Receiver<TagCsMsg<u64>>,
        ackchan: tokio::sync::mpsc::Sender<TagScMsg<u64>>,
        closechan: tokio::sync::oneshot::Receiver<()>,
    ) -> std::io::Result<FastLoginClient> {
        let stream = TcpStream::from_std(std::net::TcpStream::connect("127.0.0.1:8001")?)?;
        let (r, w) = tokio::io::split(stream);
        Ok(Self {
            history: Arc::new(Histroy::new()),
            id,
            ackchan,
            reqchan,
            closechan,
            framed_reader: FramedRead::with_capacity(r, pb::codec::ScProtoDecoder::new(), 1024),
            framed_writer: FramedWrite::new(w, pb::codec::CsProtoEncoder::new()),
        })
    }

    async fn verify_auth(&mut self) -> anyhow::Result<()> {
        let cslogin = pb::CsMsg::CsFastLogin(pb::CsFastLogin { player_id: self.id });
        match self.framed_writer.send(cslogin).await {
            Ok(_) => {
                // wait for auth result
                match self.framed_reader.next().await {
                    Some(Ok(msg)) => match msg {
                        pb::ScMsg::ScFastLogin(msg) => {
                            if msg.err_code() != pb::ErrCode::Success {
                                return Err(anyhow!("client-{} auth fail", self.id));
                            }
                            tracing::debug!("client-{} auth success", self.id);
                            return Ok(());
                        }
                        _unexpected => {
                            return Err(anyhow!("expect ScFastLogin, but recv: {:?}", _unexpected));
                        }
                    },
                    Some(Err(err)) => return Err(err.into()),
                    None => return Err(anyhow!("client-{} connection close", self.id)),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        self.verify_auth().await?;
        loop {
            tokio::select! {
                frame_ret = self.framed_reader.next() => match frame_ret {
                    Some(Ok(frame)) => {
                        &self.history.sc.push(frame.clone());
                        self.ackchan.send(TagScMsg {
                            msg: frame,
                            from: self.id
                    }).await.unwrap();
                    },
                    Some(Err(err)) => {
                        tracing::error!("read frame error. {}", err);
                    }
                    None => {
                        bail!("connection closed");
                    }
                },
                req = self.reqchan.recv() => match req {
                    Ok(req) => {
                        if let Some(id) = req.to {
                            if self.id != id {
                                continue
                            }
                        }
                        if let Err(err) = self.framed_writer.send(req.msg.clone()).await {
                            tracing::error!("fail to send {:?}. {}", req, err);
                            continue;

                        }
                        &self.history.cs.push(req.msg);
                    }
                    Err(err) => return Err(err.into()),
                },
                _ = &mut self.closechan => {
                    return Ok(())
                }
            }
        }
    }

    pub fn history(&self) -> Arc<Histroy> {
        self.history.clone()
    }
}
