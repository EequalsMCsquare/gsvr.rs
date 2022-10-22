mod agent;
mod conf;
mod make_agent;
mod server;
use server::Server;
use util::{build_nats, init_logger};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = conf::Config::parse("etc/gate")?;
    init_logger(c.log);
    debug!("logger init complete");

    let nats = build_nats(c.mq).await?;
    debug!("NATS connected");
    Server::new(format!("0.0.0.0:{}", c.port), nats).serve().await;
    Ok(())
}
