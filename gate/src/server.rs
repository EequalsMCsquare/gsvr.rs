use std::fmt::Debug;
use tokio::net::TcpListener;
use tracing::debug;

use crate::agent::Agent;

pub struct Server {
    inner: TcpListener,
    nats: async_nats::Client,
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs + Debug>(
        listen_addr: T,
        nats: async_nats::Client,
    ) -> Self {
        let lis = std::net::TcpListener::bind(&listen_addr)
            .expect(format!("fail to listen on {:?}", listen_addr).as_str());
        let lis = TcpListener::from_std(lis).unwrap();
        Self { inner: lis, nats }
    }

    pub async fn serve(self)
    // where
    //     S: tower::Service<TcpStream>,
    //     S::Future: futures::Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    {
        loop {
            match self.inner.accept().await {
                Ok((stream, addr)) => {
                    debug!("incoming connection: {:?}", addr);
                    let mut agent = Agent::new(
                        stream,
                        self.nats.clone(),
                        pb::codec::RawDecoder::new(),
                        pb::codec::RawEncoder::default(),
                    );
                    tokio::spawn(async move {
                        match agent.init().await {
                            Ok(_) => {}
                            Err(err) => {
                                tracing::error!("agent init fail. {}", err);
                                return;
                            }
                        }
                        match agent.run().await {
                            Ok(_) => {}
                            Err(err) => {
                                tracing::error!("agent close. {}", err);
                                return;
                            }
                        }
                    });
                }
                Err(err) => {
                    debug!("fail to accpect. {}", err);
                }
            }
        }
    }
}
