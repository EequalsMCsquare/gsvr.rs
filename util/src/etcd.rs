use anyhow::bail;

use crate::gconf::ConfigEtcd;

pub async fn build(cfg: ConfigEtcd) -> anyhow::Result<etcd_client::Client> {
    let option = etcd_client::ConnectOptions::new()
        .with_connect_timeout(cfg.conn_timeout.into())
        .with_timeout(cfg.request_timeout.into())
        .with_keep_alive(cfg.keepalive_interval.into(), cfg.keepalive_timeout.into())
        .with_keep_alive_while_idle(cfg.keepalive_idle);

    if cfg.user.is_some() && cfg.password.is_some() {
        etcd_client::Client::connect(
            cfg.endpoints,
            option
                .with_user(cfg.user.unwrap(), cfg.password.unwrap())
                .into(),
        )
        .await
    } else if cfg.user.is_none() && cfg.password.is_none() {
        etcd_client::Client::connect(cfg.endpoints, option.into()).await
    } else {
        bail!("user and password must be provided at the same time")
    }
    .map_err(Into::into)
}
