use std::task::Poll;

use super::agent::Agent;
use bytes::Bytes;
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Encoder};
use tower::Service;

pub struct MakeAgent<E, D>
where
    E: Clone + Copy + Encoder<Bytes>,
    D: Clone + Copy + Decoder,
{
    pub nats: async_nats::Client,
    pub encoder: E,
    pub decoder: D,
}

impl<E, D> Service<TcpStream> for MakeAgent<E, D>
where
    E: Clone + Copy + Encoder<Bytes, Error = anyhow::Error>,
    D: Clone + Copy + Decoder<Item = Bytes, Error = anyhow::Error>,
{
    type Response = ();

    type Error = anyhow::Error;

    type Future = Agent<E, D>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: TcpStream) -> Self::Future {
        Agent::new(req, self.nats.clone(), self.decoder, self.encoder)
    }
}
