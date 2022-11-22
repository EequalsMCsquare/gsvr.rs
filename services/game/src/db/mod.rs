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
    database: mongodb::Database,
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
                    // DBLoadReq
                    ChanProto::DBLoadReq { coll, filter } => {
                        match Self::on_DBLoadReq(&db, coll, filter).await {
                            Ok(bin) => req.ok(ChanProto::DBLoadAck(bin)),
                            Err(err) => req.err(err),
                        }
                    }
                    ChanProto::CtrlShutdown => {
                        tracing::info!("[{:?}]recv shutdown", ModuleName::DB);
                        return Ok(());
                    }
                    // UnhandledProto
                    _um => tracing::error!("unhandled proto: {:?}", _um),
                }
            }
        }
    }
}
