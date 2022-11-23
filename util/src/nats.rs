use crate::gconf::ConfigMQ;

pub async fn build_nats(cfg: ConfigMQ) -> anyhow::Result<async_nats::Client> {
    async_nats::ConnectOptions::default()
        .connection_timeout(cfg.conn_timeout.into())
        .client_capacity(cfg.client_capacity)
        .subscription_capacity(cfg.subscription_capacity)
        .request_timeout(cfg.request_timeout.map(Into::into))
        .ping_interval(cfg.ping_interval.into())
        .flush_interval(cfg.flush_interval.into())
        .connect(format!("{}:{}", cfg.host, cfg.port))
        .await
        .map_err(Into::into)
}
