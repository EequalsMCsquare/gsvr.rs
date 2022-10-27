use game_core::component::{Component, ComponentBuilder};
use tokio::sync::mpsc;

use crate::{hub::{ChanCtx, ChanProto, Hub, ModuleName}, error::Error};

use super::DBPlugin;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
    database: mongodb::Database,
}

impl ComponentBuilder<ModuleName, ChanProto, Hub> for Builder {
    type BrkrError = Error;
    fn build(self: Box<Self>) -> Box<dyn Component<ModuleName, ChanProto, BrkrError = Self::BrkrError>> {
        Box::new(DBPlugin {
            broker: self.brkr.unwrap(),
            database: self.database,
            rx: self.rx.unwrap()
        })
    }

    fn name(&self) -> ModuleName {
        self.name
    }

    fn set_rx(&mut self, rx: mpsc::Receiver<ChanCtx>) {
        self.rx = Some(rx);
    }

    fn set_broker(&mut self, broker: Hub) {
        self.brkr = Some(broker);
    }
}

impl Builder {
    pub fn new(database: mongodb::Database) -> Self {
        Self {
            name: ModuleName::DB,
            database,
            rx: None,
            brkr: None,
        }
    }
}
