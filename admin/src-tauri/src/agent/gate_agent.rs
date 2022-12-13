use std::sync::Arc;

use anyhow::bail;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use super::codec;

pub struct History {
    send_history: parking_lot::RwLock<Vec<cspb::Registry>>,
    recv_history: parking_lot::RwLock<Vec<cspb::Registry>>,
}

impl History {
    pub fn new(capacity: usize) -> Self {
        Self {
            send_history: parking_lot::RwLock::new(Vec::with_capacity(capacity)),
            recv_history: parking_lot::RwLock::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn push_send(&self, msg: cspb::Registry) {
        let mut his = self.send_history.write();
        if his.len() == his.capacity() {
            his.pop();
        }
        his.push(msg);
    }
    pub fn push_recv(&self, msg: cspb::Registry) {
        let mut his = self.recv_history.write();
        if his.len() == his.capacity() {
            his.pop();
        }
        his.push(msg);
    }

    pub fn get_send(&self, limit: usize, reverse: bool) -> Vec<cspb::Registry> {
        let his = self.send_history.read();
        if reverse {
            his.iter().rev().map(|s| s.clone()).take(limit).collect()
        } else {
            his.iter().take(limit).map(|s| s.clone()).collect()
        }
    }

    pub fn get_recv(&self, limit: usize, reverse: bool) -> Vec<cspb::Registry> {
        let his = self.recv_history.read();
        if reverse {
            his.iter().rev().map(|s| s.clone()).collect()
        } else {
            his.iter().take(limit).map(|s| s.clone()).collect()
        }
    }
}

unsafe impl Send for History {}
unsafe impl Sync for History {}

pub struct GateAgentProxy {
    pub pid: i64,
    req_tx: mpsc::Sender<cspb::Registry>,
    ack_rx: mpsc::Receiver<cspb::Registry>,
    close_tx: mpsc::Sender<()>,
    history: Arc<History>,
    join: tokio::task::JoinHandle<anyhow::Result<GateAgent>>,
}

#[derive(Serialize)]
pub struct FrontGateAgent {
    pub _pid: i64
}

impl GateAgentProxy {

    pub fn to_front(&self) -> FrontGateAgent {
        FrontGateAgent { _pid: self.pid }
    }

    pub async fn stop(self) -> anyhow::Result<GateAgent> {
        self.close_tx.send(()).await?;
        self.join.await?
    }

    pub async fn send(
        &self,
        msg: cspb::Registry,
    ) -> Result<(), mpsc::error::SendError<cspb::Registry>> {
        self.req_tx.send(msg).await
    }

    pub async fn recv(&mut self) -> Option<cspb::Registry> {
        self.ack_rx.recv().await
    }

    pub fn history(&self) -> Arc<History> {
        self.history.clone()
    }
}

pub struct GateAgent {
    fr: FramedRead<ReadHalf<TcpStream>, codec::Decoder>,
    fw: FramedWrite<WriteHalf<TcpStream>, codec::Encoder>,
    his: Arc<History>,
    authorized: bool,
    pub pid: i64,
}

impl GateAgent {
    pub fn new(pid: i64, stream: TcpStream) -> Self {
        let (rd, wr) = tokio::io::split(stream);
        Self {
            fr: FramedRead::with_capacity(rd, codec::Decoder::default(), 1024),
            fw: FramedWrite::new(wr, codec::Encoder),
            his: Arc::new(History::new(256)),
            authorized: false,
            pid,
        }
    }

    pub async fn fast_login(&mut self) -> anyhow::Result<()> {
        self.send(cspb::CsFastLogin {
            player_id: self.pid,
        })
        .await?;
        let ack: cspb::ScFastLogin = self.recv().await.map(|msg| msg.try_into().unwrap())?;
        if ack.err_code() == cspb::ErrCode::Success {
            self.authorized = true;
            Ok(())
        } else {
            bail!("fail to auth. {:?}", ack.err_code())
        }
    }

    pub async fn login(&mut self, token: &str) -> anyhow::Result<()> {
        self.send(cspb::CsLogin {
            token: token.into(),
            player_id: self.pid,
        })
        .await?;
        let ack: cspb::ScLogin = self.recv().await.map(|msg| msg.try_into().unwrap())?;
        if ack.err_code() == cspb::ErrCode::Success {
            self.authorized = true;
            Ok(())
        } else {
            bail!("fail to auth. {:?}", ack.err_code())
        }
    }

    async fn send<T>(&mut self, msg: T) -> anyhow::Result<()>
    where
        T: Into<cspb::Registry>,
    {
        self.fw.send(msg.into()).await?;
        Ok(())
    }

    async fn recv(&mut self) -> anyhow::Result<cspb::Registry> {
        if let Some(msg) = self.fr.next().await {
            msg
        } else {
            bail!("recv none from FramedRead")
        }
    }

    pub fn run(mut self) -> anyhow::Result<GateAgentProxy> {
        if !self.authorized {
            bail!("unauthorized agent")
        }
        let (req_tx, mut req_rx) = mpsc::channel(16);
        let (ack_tx, ack_rx) = mpsc::channel(16);
        let (close_tx, mut close_rx) = mpsc::channel(1);
        let pid = self.pid;
        let his = self.his.clone();
        let his2 = his.clone();
        let join = tokio::spawn(async move {
            loop {
                tokio::select! {
                    req = req_rx.recv() => {
                        if let Some::<cspb::Registry>(req) = req {
                            his.push_send(req.clone());
                            self.fw.send(req).await.unwrap();
                        } else {
                            bail!("recv None from Request channel")
                        }
                    },
                    Some(ack) = self.fr.next() => {
                        match ack {
                            Ok(ack) => {
                                his.push_recv(ack.clone());
                                ack_tx.send(ack).await.unwrap();
                            },
                            Err(err) => {
                                bail!("error occur when recv from FramedRead. {}", err)
                            }
                        }
                    },
                    _ = close_rx.recv() => {
                        return Ok(self)
                    }
                }
            }
        });
        Ok(GateAgentProxy {
            pid,
            req_tx,
            ack_rx,
            close_tx,
            join,
            history: his2,
        })
    }
}
