use tokio::runtime;

mod conf;
mod db;
mod hub;
mod nats;
mod play;

fn main() -> anyhow::Result<()> {
    let rt = runtime::Builder::new_multi_thread().enable_all().build()?;
    let cfg = conf::Config::parse("etc/game")?;
    util::init_logger(cfg.log);
    rt.block_on(async {
        let nats_client = util::build_nats(cfg.mq).await.unwrap();
        tracing::debug!("NATS init success");
        // build modules
        let mut play = play::Module::new();
        let mut nats = nats::Module::new(nats_client);
        let mut db = db::Module::new();
        // init ChanRpc Hub
        let h = hub::Hub {
            play: play.chanrpc(),
            db: db.chanrpc(),
            nats: nats.chanrpc(),
            module: hub::ModuleName::default(),
        };

        // set modules hub
        play.with_hub(h.clone());
        nats.with_hub(h.clone());
        db.with_hub(h.clone());

        // spawn module future 顺序不能错
        let nats_handle = rt.spawn(async move {
            if let Err(err) = nats.init().await {
                panic!("Nats module init fail. {}", err);
            }
            tracing::info!("Nats module init success");
            nats.run().await
        });

        // let db_handle = tokio::spawn(db);
        if let Err(err) = play.init().await {
            panic!("Play module init fail. {}", err);
        }
        tracing::info!("Play module init success");
        std::thread::spawn(move || {
            if let Err(err) = play.run() {
                tracing::error!("error occur while running play module. {}", err);
            }
        });

        let (nats_ret) = tokio::join! {
            nats_handle,
            // db_handle
        };
    });
    Ok(())
}
