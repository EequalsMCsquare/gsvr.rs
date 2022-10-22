use anyhow::anyhow;
use clap::Parser;
use dialoguer::Input;
use tokio::sync::{broadcast, mpsc};

use crate::cmd::{self, AgentCmd};

pub struct Executor {
    recvchan: mpsc::Receiver<pb::ScMsg>,
    sendchan: broadcast::Sender<pb::CsMsg>,
}

impl Executor {
    pub fn new() -> (
        Self,
        mpsc::Sender<pb::ScMsg>,
        broadcast::Receiver<pb::CsMsg>,
    ) {
        let (br_tx, br_rx) = broadcast::channel(128);
        let (tx, rx) = mpsc::channel(128);
        (
            Self {
                recvchan: rx,
                sendchan: br_tx,
            },
            tx,
            br_rx,
        )
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let input: String = Input::new()
                .with_prompt("> ")
                .interact_text()
                .map_err(|err| anyhow!("{:?}", err))?;

            let cli = match AgentCmd::try_parse_from(format!("- {}", input).split_whitespace()) {
                Ok(cmd) => cmd,
                Err(err) => {
                    tracing::error!("invalid input. {}", err);
                    continue;
                }
            };
            match cli {
                AgentCmd::Send => {
                    let rest_args: Vec<&str> =
                        input.split_whitespace().into_iter().skip(1).collect();
                }
            }
        }
        Ok(())
    }
}
