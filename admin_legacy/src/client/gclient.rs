use super::{codec, misc::ClientInfo};
use anyhow::bail;
use futures::{SinkExt, StreamExt};
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::{TcpStream, ToSocketAddrs},
};
use tokio_util::codec::{FramedRead, FramedWrite};

pub struct GClientBuilder<A> {
    gate: Option<A>,
    info: Option<ClientInfo>,
}

impl<A> GClientBuilder<A>
where
    A: ToSocketAddrs,
{
    pub fn new() -> Self {
        Self {
            gate: None,
            info: None,
        }
    }

    pub fn gate(mut self, addr: A) -> Self {
        self.gate = Some(addr);
        self
    }

    pub fn info(mut self, info: ClientInfo) -> Self {
        self.info = Some(info);
        self
    }

    pub async fn build(self) -> anyhow::Result<GClient> {
        let stream = TcpStream::connect(self.gate.unwrap()).await?;
        let (rd, wr) = tokio::io::split(stream);
        let fr = FramedRead::with_capacity(rd, codec::Decoder::default(), 1024);
        let fw = FramedWrite::new(wr, codec::Encoder);
        Ok(GClient {
            rd: fr,
            wr: fw,
            _info: self.info.unwrap(),
        })
    }
}

pub struct GClient {
    rd: FramedRead<ReadHalf<TcpStream>, codec::Decoder>,
    wr: FramedWrite<WriteHalf<TcpStream>, codec::Encoder>,
    _info: ClientInfo,
}

impl GClient {
    async fn auth_fast_login(&mut self) -> anyhow::Result<()> {
        if let ClientInfo::FastLogin { player_id } = self._info {
            let msg = cspb::CsFastLogin { player_id };
            self.wr.send(msg.into()).await?;
            if let Some(Ok(cspb::Registry::ScFastLogin(ack))) = self.rd.next().await {
                if ack.err_code() == cspb::ErrCode::Success {
                    Ok(())
                } else {
                    bail!("fail to auth. {:?}", ack.err_code())
                }
            } else {
                bail!("recv invalid reply")
            }
        } else {
            panic!()
        }
    }

    async fn auth_normal(&mut self) -> anyhow::Result<()> {
        if let ClientInfo::Normal { player_id, token } = &self._info {
            let msg = cspb::CsLogin {
                token: token.clone(),
                player_id: *player_id,
            };
            self.wr.send(msg.into()).await?;
            if let Some(Ok(cspb::Registry::ScLogin(ack))) = self.rd.next().await {
                if ack.err_code() == cspb::ErrCode::Success {
                    Ok(())
                } else {
                    bail!("fail to auth. {:?}", ack.err_code())
                }
            } else {
                bail!("recv invalid reply")
            }
        } else {
            panic!()
        }
    }

    pub async fn authenticate(&mut self) -> anyhow::Result<()> {
        match &self._info {
            ClientInfo::Normal {
                token: _,
                player_id: _,
            } => self.auth_normal().await,
            ClientInfo::FastLogin { player_id: _ } => self.auth_fast_login().await,
        }
    }

    pub async fn send(&mut self, msg: cspb::Registry) -> anyhow::Result<()> {
        self.wr.send(msg).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<cspb::Registry> {
        if let Some(msg) = self.rd.next().await {
            msg
        } else {
            bail!("recv none from FramedRead")
        }
    }
}
