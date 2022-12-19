mod adaptor;
mod codec;
mod conf;
mod error;
use bytes::Bytes;
use error::Error;
use gsfw::network::{AgentService, Gate};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = conf::Config::parse("etc/gate")?;
    let _guard = util::logger::init(c.log);

    let nats = util::nats::build(c.mq).await?;
    tracing::info!("NATS connected");
    let auth = spb::AuthServiceClient::connect(c.pf_url).await?;
    tracing::info!("AUTH_SVC connected");
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
    tracing::info!("gate listen on 0.0.0.0:{}", c.port);
    Gate::new(format!("0.0.0.0:{}", c.port))
        .serve(service)
        .await;
    Ok(())
}
