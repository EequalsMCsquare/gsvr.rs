mod proto;
use gsfw::chanrpc;
pub use proto::{GProto, TimerArgs, PMSG};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::sync::mpsc;

pub type ChanCtx = chanrpc::ChanCtx<proto::GProto, ModuleName, crate::error::Error>;

#[derive(Debug, Clone)]
pub struct Hub {
    name: ModuleName,
    play: mpsc::Sender<ChanCtx>,
    nats: mpsc::Sender<ChanCtx>,
    db: mpsc::Sender<ChanCtx>,
    // timer: mpsc::Sender<ChanCtx>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
pub enum ModuleName {
    Play,
    Nats,
    DB,
    // Timer,
}

impl chanrpc::Name for ModuleName {}

impl chanrpc::broker::Broker for Hub {
    type Proto = GProto;

    type Name = ModuleName;

    type Err = crate::Error;

    fn name(&self) -> ModuleName {
        self.name
    }

    fn tx<'a>(&'a self, name: ModuleName) -> &'a mpsc::Sender<ChanCtx> {
        match name {
            ModuleName::Play => &self.play,
            ModuleName::Nats => &self.nats,
            ModuleName::DB => &self.db,
            // ModuleName::Timer => &self.timer,
        }
    }

    fn new(name: ModuleName, tx_map: &HashMap<ModuleName, mpsc::Sender<ChanCtx>>) -> Self {
        Self {
            name,
            play: tx_map.get(&ModuleName::Play).unwrap().clone(),
            nats: tx_map.get(&ModuleName::Nats).unwrap().clone(),
            db: tx_map.get(&ModuleName::DB).unwrap().clone(),
            // timer: tx_map.get(&ModuleName::Timer).unwrap().clone(),
        }
    }
}
