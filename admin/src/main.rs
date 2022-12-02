use clap::Parser;
mod cli;
mod client;
mod cmd;
use client::{
    gclient::GClientBuilder,
    misc::ClientInfo,
    nclient::NClient,
    pfclient::PfClient,
};
use cmd::{
    bench::run_bench,
    client::{run_api_client, run_game_client},
};
use util::gconf::ConfigLog;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::logger::init(ConfigLog {
        level: Default::default(),
        output: Some("stdout".to_string()),
        ..Default::default()
    });
    let args = cli::Args::parse();
    let pfclient = PfClient::new("http://localhost:8100".to_string());
    match args.subcmd {
        cli::SubCmds::FClient { player_id } => {
            let gclient = GClientBuilder::new()
                .gate(&args.gate)
                .info(ClientInfo::FastLogin { player_id })
                .build()
                .await?;
            run_game_client(gclient, format!("[fclient-{}]", player_id)).await?;
        }
        cli::SubCmds::Client { username, password } => {
            let prompt = format!("[{}]", username);
            let nclient = NClient::new(pfclient, args.gate, username, password)?;
            run_api_client(nclient, prompt).await?;
        }
        cli::SubCmds::Bench { client, iter } => {
            run_bench(args.gate.clone(), iter, client).await?;
        }
        _unhandle => {
            panic!("unhandle subcmd: {:?}", _unhandle)
        }
    }
    Ok(())
}
