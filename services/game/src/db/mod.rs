mod builder;
mod handler;
mod proto;
use crate::hub::{ChanCtx, GProto, Hub, ModuleName};
pub use builder::Builder;
use gsfw::component;
pub use proto::*;
use std::error::Error as StdError;
use tokio::sync::mpsc;

pub struct DBComponent {
    _broker: Hub,
    rx: mpsc::Receiver<ChanCtx>,
    database: sqlx::PgPool,
}

#[async_trait::async_trait]
impl component::Component<Hub> for DBComponent {
    #[inline]
    fn name(&self) -> ModuleName {
        ModuleName::DB
    }

    async fn init(
        self: Box<Self>,
    ) -> Result<Box<dyn component::Component<Hub>>, Box<dyn StdError + Send>> {
        Ok(self)
    }

    async fn run(self: Box<Self>) -> Result<(), Box<dyn StdError + Send>> {
        let _db = self.database.clone();
        let mut rx = self.rx;
        loop {
            if let Some(req) = rx.recv().await {
                match req.payload() {
                    GProto::DBProtoReq(inner) => {
                        let ret = match inner {
                            DBProtoReq::Find { kind, query } => {
                                Self::ctl_find(&self.database, kind, query).await
                            }
                            DBProtoReq::Exec(query) => Self::ctl_exec(&self.database, query).await,
                        };
                        req.ok(GProto::DBProtoAck(ret));
                    }

                    GProto::CtrlShutdown => return Ok(()),
                    // UnhandledProto
                    _unexpected => panic!(
                        "unhandled proto: {:?}",
                        Into::<&'static str>::into(_unexpected)
                    ),
                }
            }
        }
    }
}
