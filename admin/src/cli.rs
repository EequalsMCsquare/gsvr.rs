#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub gate: String,

    #[command(subcommand)]
    pub subcmd: SubCmds,
}

#[derive(clap::Subcommand, Debug)]
#[clap(rename_all = "lower_case")]
pub enum SubCmds {
    ServeHttp {
        #[arg(short, long)]
        port: u16,
    },
    FClient {
        #[arg(short, long = "pid")]
        player_id: i64,
    },
    Client {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    Bench {
        #[arg(short, long)]
        client: usize,
        #[arg(short, long)]
        iter: usize,
    },
    GM,
}
