mod proto;
use crate::error::Error;
use game_core::broker::{self, Broker};
use hashbrown::HashMap;
pub use proto::{ChanProto, TimerArgs};
use std::fmt::Debug;
use tokio::sync::mpsc;

pub type ChanCtx = broker::ChanCtx<proto::ChanProto, ModuleName, crate::error::Error>;

#[derive(Debug, Clone)]
pub struct Hub {
    name: ModuleName,
    play: mpsc::Sender<ChanCtx>,
    nats: mpsc::Sender<ChanCtx>,
    db: mpsc::Sender<ChanCtx>,
    timer: mpsc::Sender<ChanCtx>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ModuleName {
    Play,
    Nats,
    DB,
    Timer,
}

impl Broker<proto::ChanProto, ModuleName> for Hub {
    type Error = Error;
    fn name(&self) -> ModuleName {
        self.name
    }

    fn get_tx<'a>(&'a self, name: ModuleName) -> &'a mpsc::Sender<ChanCtx> {
        match name {
            ModuleName::Play => &self.play,
            ModuleName::Nats => &self.nats,
            ModuleName::DB => &self.db,
            ModuleName::Timer => &self.timer,
        }
    }

    fn new(name: ModuleName, tx_map: &HashMap<ModuleName, mpsc::Sender<ChanCtx>>) -> Self {
        Self {
            name,
            play: tx_map.get(&ModuleName::Play).unwrap().clone(),
            nats: tx_map.get(&ModuleName::Nats).unwrap().clone(),
            db: tx_map.get(&ModuleName::DB).unwrap().clone(),
            timer: tx_map.get(&ModuleName::Timer).unwrap().clone(),
        }
    }
}
