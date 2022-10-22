use std::time::Duration;

pub async fn build_nats(cfg: gconf::ConfigMQ) -> anyhow::Result<async_nats::Client> {
    async_nats::ConnectOptions::default()
        .connection_timeout(Duration::from_secs(5))
        .connect(format!("{}:{}", cfg.host, cfg.port))
        .await
        .map_err(Into::into)
}
