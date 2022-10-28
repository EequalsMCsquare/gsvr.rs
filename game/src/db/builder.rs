use game_core::component::{Component, ComponentBuilder};
use tokio::sync::mpsc;

use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};

use super::DBComponent;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
    database: Option<mongodb::Database>,
}

impl ComponentBuilder<ModuleName, ChanProto, Hub> for Builder {
    type BrkrError = Error;
    fn build(
        self: Box<Self>,
    ) -> Box<dyn Component<ModuleName, ChanProto, BrkrError = Self::BrkrError>> {
        Box::new(DBComponent {
            broker: self.brkr.unwrap(),
            database: self.database.unwrap(),
            rx: self.rx.unwrap(),
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
    pub fn new() -> Self {
        Self {
            name: ModuleName::DB,
            database: None,
            rx: None,
            brkr: None,
        }
    }

    pub fn with_db(mut self, database: mongodb::Database) -> Self {
        self.database = Some(database);
        self
    }
}
