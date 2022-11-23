pub async fn build_nats(cfg: gconf::ConfigMQ) -> anyhow::Result<async_nats::Client> {
    async_nats::ConnectOptions::default()
        .connection_timeout(cfg.conn_timeout)
        .client_capacity(cfg.client_capacity)
        .subscription_capacity(cfg.subscription_capacity)
        .request_timeout(Some(cfg.request_timeout))
        .ping_interval(cfg.ping_interval)
        .flush_interval(cfg.flush_interval)
        .connect(format!("{}:{}", cfg.host, cfg.port))
        .await
        .map_err(Into::into)
}
