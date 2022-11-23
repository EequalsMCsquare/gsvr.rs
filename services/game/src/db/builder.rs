use tokio::sync::mpsc;

use super::DBComponent;
use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
use gsfw::component;

pub struct Builder {
    name: ModuleName,
    rx: Option<mpsc::Receiver<ChanCtx>>,
    brkr: Option<Hub>,
    database: Option<sqlx::PgPool>,
}

impl component::ComponentBuilder<ChanProto, ModuleName, Hub, Error, mpsc::Receiver<ChanCtx>>
    for Builder
{
    fn build(self: Box<Self>) -> Box<dyn component::Component<ChanProto, ModuleName, Error>> {
        Box::new(DBComponent {
            _broker: self.brkr.unwrap(),
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

    pub fn with_db(mut self, database: sqlx::PgPool) -> Self {
        self.database = Some(database);
        self
    }
}
