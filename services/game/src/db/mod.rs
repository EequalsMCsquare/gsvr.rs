mod builder;
mod handler;

use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
pub use builder::Builder;
use gsfw::component;
use tokio::sync::mpsc;

pub struct DBComponent {
    _broker: Hub,
    rx: mpsc::Receiver<ChanCtx>,
    database: sqlx::PgPool,
}

#[async_trait::async_trait]
impl component::Component<ChanProto, ModuleName, Error> for DBComponent {
    #[inline]
    fn name(&self) -> ModuleName {
        ModuleName::DB
    }

    async fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(self: Box<Self>) -> Result<(), Error> {
        let db = self.database.clone();
        let mut rx = self.rx;
        loop {
            if let Some(req) = rx.recv().await {
                match &req.payload {
                    // UnhandledProto
                    _um => tracing::error!("unhandled proto: {:?}", _um),
                }
            }
        }
    }
}
