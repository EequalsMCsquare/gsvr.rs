use clap::Parser;
mod cli;
mod client;
mod cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::init_logger(gconf::ConfigLog {
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
