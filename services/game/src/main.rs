mod conf;
mod db;
mod error;
mod hub;
mod nats;
mod play;
mod timer;
use error::Error;
use gsfw::gs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = conf::Config::parse("etc/game")?;
    let _guard = util::logger::init(cfg.log);

    let nats_client = util::nats::build(cfg.mq)
        .await
        .expect("fail to build nats client");
    let db = util::pgpool::build(cfg.database)
        .await
        .expect("fail to build PgPool");
    gs::GameBuilder::new()
        .component(db::Builder::new().with_db(db))
        .component(nats::Builder::new().with_nats(nats_client))
        .component(timer::Builder::new())
        .component(play::Builder::new().worker_num(1))
        .serve()?
        .await;
    Ok(())
}
