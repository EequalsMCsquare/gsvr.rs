mod conf;
mod db;
mod error;
mod hub;
mod nats;
mod play;
mod timer;
use gsfw::gs;

fn main() -> anyhow::Result<()> {

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(16)
        .build()?;

    rt.block_on(async {
        let cfg = conf::Config::parse("etc/game")?;
        util::init_logger(cfg.log);
        let nats_client = util::build_nats(cfg.mq)
            .await
            .expect("fail to build nats client");
        let mongo_client = util::build_db(cfg.database)
            .await
            .expect("fail to build mongodb client");
        gs::GameBuilder::new()
            .component(db::Builder::new().with_db(mongo_client))
            .component(timer::Builder::new())
            .component(nats::Builder::new().with_nats(nats_client))
            .component(play::Builder::new())
            .serve()?
            .await;
        Ok(())
    })
}
