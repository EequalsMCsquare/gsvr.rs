#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub gate: String,

    #[command(subcommand)]
    pub subcmd: SubCmds,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCmds {
    ServeHttp {
        #[arg(short, long)]
        port: u16,
    },
    FClient {
        #[arg(short, long)]
        playerid: u64,
    },
    Client {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
        #[arg(short, long)]
        register: bool,
    },
    Bench,
    GM,
}
