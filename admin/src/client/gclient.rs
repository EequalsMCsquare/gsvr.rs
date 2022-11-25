use super::{
    adaptor::ClientAdaptorBuilder,
    codec,
    misc::{AdminClient, ClientInfo},
};
use anyhow::anyhow;
use async_trait::async_trait;
use gsfw::network;
use tokio::sync::{broadcast, mpsc};
use tower::Service;

pub struct GClient {
    tx: broadcast::Sender<cspb::CsMsg>,
    rx: mpsc::Receiver<cspb::ScMsg>,
    // agent_future: Pin<Box<dyn Future<Output = Result<(), gsfw::error::Error>> + 'static + Send>>,
    _info: ClientInfo,
    _agent_join: tokio::task::JoinHandle<Result<(), gsfw::error::Error>>,
}

impl GClient {
    pub fn new(gate: String, info: ClientInfo) -> anyhow::Result<Self> {
        let stream = std::net::TcpStream::connect(gate)?;
        let stream = tokio::net::TcpStream::from_std(stream)?;
        let (req_tx, _req_rx) = broadcast::channel(128);
        let (ack_tx, ack_rx) = mpsc::channel(128);
        let mut make_agent = network::AgentService::new(
            codec::Encoder,
            codec::Decoder::default(),
            ClientAdaptorBuilder::new(req_tx.clone(), ack_tx, info.clone()),
        );
        let agent_future = make_agent.call(stream);
        Ok(Self {
            _info: info,
            tx: req_tx,
            rx: ack_rx,
            _agent_join: tokio::spawn(agent_future),
        })
    }
}

#[async_trait]
impl AdminClient for GClient {
    async fn send(&mut self, msg: cspb::CsMsg) -> anyhow::Result<()> {
        self.tx.send(msg)?;
        Ok(())
    }

    async fn recv(&mut self) -> anyhow::Result<cspb::ScMsg> {
        self.rx.recv().await.ok_or(anyhow!("recv None"))
    }
}
