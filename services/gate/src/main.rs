mod adaptor;
mod codec;
mod conf;
mod error;
use bytes::Bytes;
use error::Error;
use gsfw::network::{AgentService, Gate};
use tracing::debug;
use util::{build_nats, init_logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = conf::Config::parse("etc/gate")?;
    init_logger(c.log);
    debug!("logger init complete");

    let nats = build_nats(c.mq).await?;
    debug!("NATS connected");
    let auth = spb::AuthServiceClient::connect("localhost:8101").await?;
    debug!("AUTH_SVC connected");
    let adaptor_builder = adaptor::NatsAdaptorBuilder {
        env: c.env.into(),
        nats,
        auth,
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
