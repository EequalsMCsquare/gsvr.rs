mod agent_cmd;
use clap::Parser;
pub use agent_cmd::AgentCmd;

#[derive(PartialEq, PartialOrd, Debug, Parser)]
pub enum RootCmd {
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    Flogin {
        playerid: u64,
    },
    Bench {
        count: u64,
    },
}
