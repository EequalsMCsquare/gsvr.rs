mod proto;
use gsfw::chanrpc;
pub use proto::{ChanProto, TimerArgs};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::sync::mpsc;

use crate::error::Error;

pub type ChanCtx = chanrpc::ChanCtx<proto::ChanProto, ModuleName, crate::error::Error>;

#[derive(Debug, Clone)]
pub struct Hub {
    name: ModuleName,
    play: mpsc::Sender<ChanCtx>,
    nats: mpsc::Sender<ChanCtx>,
    db: mpsc::Sender<ChanCtx>,
    timer: mpsc::Sender<ChanCtx>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
pub enum ModuleName {
    Play,
    Nats,
    DB,
    Timer,
}

impl
    chanrpc::broker::Broker<
        proto::ChanProto,
        ModuleName,
        Error,
        mpsc::Sender<ChanCtx>,
        mpsc::Receiver<ChanCtx>,
    > for Hub
{
    fn name(&self) -> ModuleName {
        self.name
    }

    fn tx<'a>(&'a self, name: ModuleName) -> &'a mpsc::Sender<ChanCtx> {
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

    fn channel(size: usize) -> (mpsc::Sender<ChanCtx>, mpsc::Receiver<ChanCtx>) {
        mpsc::channel(size)
    }
}

// impl
//     gsfw::chanrpc::broker::AsyncBroker<
//         ChanProto,
//         ModuleName,
//         Error,
//         mpsc::Sender<ChanCtx>,
//         mpsc::Receiver<ChanCtx>,
//     > for Hub
// {
// }
