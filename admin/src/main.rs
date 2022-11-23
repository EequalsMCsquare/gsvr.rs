use clap::Parser;
mod cli;
mod client;
mod cmd;
use util::gconf::ConfigLog;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::logger::init(ConfigLog {
        level: Default::default(),
        output: Some("stdout".to_string()),
        ..Default::default()
    });
    let args = cli::Args::parse();
    match args.subcmd {
        cli::SubCmds::FClient { playerid } => {
            cmd::fclient::run(args.gate, playerid).await?
        }
        _unhandle => {
            panic!("unhandle subcmd: {:?}", _unhandle)
        }
    }
    Ok(())
}
