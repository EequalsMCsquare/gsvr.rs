mod adaptor;
mod codec;
mod error;
mod conf;
use bytes::Bytes;
use gsfw::network::{AgentService, Gate};
use tracing::debug;
use util::{build_nats, init_logger};
use error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = conf::Config::parse("etc/gate")?;
    init_logger(c.log);
    debug!("logger init complete");

    let nats = build_nats(c.mq).await?;
    debug!("NATS connected");
    let adaptor_builder = adaptor::NatsAdaptorBuilder {
        env: c.env.into(),
        nats,
    };
    let service = AgentService::<_, _, _, Bytes>::new(
        codec::EncoderImpl::default(),
        codec::DecoderImpl::default(),
        adaptor_builder,
    );
    Gate::new(format!("0.0.0.0:{}", c.port))
        .serve(service)
        .await;
    Ok(())
}
