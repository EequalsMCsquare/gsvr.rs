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

mod test {
    use super::{RootCmd};
    use clap::Parser;
    #[test]
    fn test_parse_long() {
        let input = "- login --username eequalsmc2 --password 123aaa";
        let ret = RootCmd::parse_from(input.split_whitespace());
        debug_assert_eq!(
            ret,
            RootCmd::Login {
                username: "eequalsmc2".to_string(),
                password: "123aaa".to_string()
            }
        );
    }

    #[test]
    fn test_parse_short() {
        let input = "- login -u eequalsmc2 -p 123aaa";
        let ret = RootCmd::parse_from(input.split_whitespace());
        debug_assert_eq!(
            ret,
            RootCmd::Login {
                username: "eequalsmc2".to_string(),
                password: "123aaa".to_string()
            }
        );
    }
}
