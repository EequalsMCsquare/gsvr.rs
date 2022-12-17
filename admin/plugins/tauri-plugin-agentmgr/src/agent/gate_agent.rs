use std::sync::Arc;

use anyhow::bail;
use futures::{SinkExt, StreamExt};
use gsfw::RegistryExt;
use serde::Serialize;
use tauri::Runtime;
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use super::codec;

#[derive(Serialize, Clone)]
pub struct FrontHistoryData<'a> {
    msgid: i32,
    name: &'static str,
    payload: &'a cspb::Registry,
}

impl<'a> FrontHistoryData<'a> {
    pub fn new(msg: &'a cspb::Registry) -> Self {
        Self {
            msgid: msg.msgid(),
            name: msg.name(),
            payload: msg,
        }
    }
}

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
            his.iter().rev().map(|s| s.clone()).take(limit).collect()
        } else {
            his.iter().take(limit).map(|s| s.clone()).collect()
        }
    }

    pub fn get_send_json(&self, limit: usize, reverse: bool) -> String {
        let his = self.send_history.read();
        if reverse {
            let ret: Vec<_> = his
                .iter()
                .rev()
                .map(|s| FrontHistoryData {
                    msgid: s.msgid(),
                    name: s.name(),
                    payload: s,
                })
                .take(limit)
                .collect();
            serde_json::to_string(&ret).unwrap()
        } else {
            let ret: Vec<_> = his
                .iter()
                .map(|s| FrontHistoryData {
                    msgid: s.msgid(),
                    name: s.name(),
                    payload: s,
                })
                .take(limit)
                .collect();
            serde_json::to_string(&ret).unwrap()
        }
    }

    pub fn get_recv_json(&self, limit: usize, reverse: bool) -> String {
        let his = self.recv_history.read();
        if reverse {
            let ret: Vec<_> = his
                .iter()
                .rev()
                .map(|s| FrontHistoryData {
                    msgid: s.msgid(),
                    name: s.name(),
                    payload: s,
                })
                .take(limit)
                .collect();
            serde_json::to_string(&ret).unwrap()
        } else {
            let ret: Vec<_> = his
                .iter()
                .map(|s| FrontHistoryData {
                    msgid: s.msgid(),
                    name: s.name(),
                    payload: s,
                })
                .take(limit)
                .collect();
            serde_json::to_string(&ret).unwrap()
        }
    }
}

unsafe impl Send for History {}
unsafe impl Sync for History {}

#[derive(Serialize)]
pub struct FrontGateAgent {
    pub _pid: i64,
}

#[derive(Serialize)]
pub struct GamePing {
    start: i64,
    end: i64,
    dur: i64,
}

pub struct GateAgentProxy<R: Runtime> {
    pub pid: i64,
    req_tx: mpsc::Sender<cspb::Registry>,
    ack_rx: mpsc::Receiver<cspb::Registry>,
    ctrl_tx: mpsc::Sender<AgentCtrl>,
    history: Arc<History>,
    join: tokio::task::JoinHandle<anyhow::Result<GateAgent<R>>>,
}

impl<R: Runtime> GateAgentProxy<R> {
    pub fn to_front(&self) -> FrontGateAgent {
        FrontGateAgent { _pid: self.pid }
    }

    pub async fn stop(self) -> anyhow::Result<GateAgent<R>> {
        self.ctrl_tx.send(AgentCtrl::Close).await?;
        self.join.await?
    }

    pub async fn listen_ack(&self, topic: String) -> anyhow::Result<()> {
        self.ctrl_tx.send(AgentCtrl::ListenAck(topic)).await?;
        Ok(())
    }

    pub async fn unlisten_ack(&self) -> anyhow::Result<()> {
        self.ctrl_tx.send(AgentCtrl::UnlistenAck).await?;
        Ok(())
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

#[derive(Debug, Clone)]
pub enum AgentCtrl {
    Close,
    ListenAck(String),
    UnlistenAck,
}

pub struct GateAgent<R: Runtime> {
    fr: FramedRead<ReadHalf<TcpStream>, codec::Decoder>,
    fw: FramedWrite<WriteHalf<TcpStream>, codec::Encoder>,
    his: Arc<History>,
    authorized: bool,
    window: tauri::Window<R>,
    pub pid: i64,
}

impl<R: Runtime> GateAgent<R> {
    pub fn new(pid: i64, stream: TcpStream, window: tauri::Window<R>) -> Self {
        let (rd, wr) = tokio::io::split(stream);
        Self {
            fr: FramedRead::with_capacity(rd, codec::Decoder::default(), 1024),
            fw: FramedWrite::new(wr, codec::Encoder),
            his: Arc::new(History::new(256)),
            authorized: false,
            window,
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

    pub fn run(mut self) -> anyhow::Result<GateAgentProxy<R>> {
        if !self.authorized {
            bail!("unauthorized agent")
        }
        let (req_tx, mut req_rx) = mpsc::channel(256);
        let (ack_tx, ack_rx) = mpsc::channel(256);
        let (ctrl_tx, mut ctrl_rx) = mpsc::channel(1);
        let mut ack_topic: Option<String> = None;
        let pid = self.pid;
        let his = self.his.clone();
        let his2 = his.clone();
        
        let join = tokio::spawn(async move {
            loop {
                tokio::select! {
                    req = req_rx.recv() => {
                        if let Some::<cspb::Registry>(req) = req {
                            if req.msgid() != cspb::MsgId::CsPing as i32{
                                his.push_send(req.clone());
                            }
                            self.fw.send(req).await.unwrap();
                        } else {
                            bail!("recv None from Request channel")
                        }
                    },
                    Some(ack) = self.fr.next() => {
                        match ack {
                            Ok(ack) => {
                                if ack.msgid() != cspb::MsgId::ScPing as i32 {
                                    his.push_recv(ack.clone());
                                }
                                if let Some(topic) = &ack_topic {
                                    let new_his = FrontHistoryData{ msgid: ack.msgid(), name: ack.name(), payload: &ack };
                                    match serde_json::to_string(&new_his) {
                                        Ok(ret) => self.window.emit(topic, ret).unwrap(),
                                        Err(err) => tracing::error!("{}", err)
                                    }
                                }
                                // only send reply when agent is not listen
                                if ack_topic.is_none() {
                                    ack_tx.send(ack).await.unwrap();
                                }
                            },
                            Err(err) => {
                                bail!("error occur when recv from FramedRead. {}", err)
                            }
                        }
                    },
                    ctrl = ctrl_rx.recv() => {
                        let Some(ctrl) = ctrl else {
                            bail!("recv None from ctrl")
                        };
                        match ctrl {
                            AgentCtrl::Close =>return Ok(self),
                            AgentCtrl::ListenAck(topic) => ack_topic = Some(topic),
                            AgentCtrl::UnlistenAck => ack_topic = None
                        }

                    }
                }
            }
        });
        Ok(GateAgentProxy {
            pid,
            req_tx,
            ack_rx,
            ctrl_tx,
            join,
            history: his2,
        })
    }
}
