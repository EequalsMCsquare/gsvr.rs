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

    async fn init(&mut self) -> Result<(), Box<dyn StdError + Send>> {
        Ok(())
    }

    async fn run(self: Box<Self>) -> Result<(), Box<dyn StdError + Send>> {
        let _db = self.database.clone();
        let mut rx = self.rx;
        loop {
            if let Some(req) = rx.recv().await {
                match req.payload() {
                    GProto::DBProtoReq(inner) => {
                        req.ok(GProto::DBProtoAck(match inner {
                            DBProtoReq::Find { kind, query } => match kind {
                                FindKind::One => {
                                    DBProtoAck::OneRow(query.fetch_one(&self.database).await)
                                }
                                FindKind::All => {
                                    DBProtoAck::AllRow(query.fetch_all(&self.database).await)
                                }
                                FindKind::Option => {
                                    DBProtoAck::OptRow(query.fetch_optional(&self.database).await)
                                }
                            },
                            DBProtoReq::Exec(query) => {
                                DBProtoAck::Exec(query.execute(&self.database).await)
                            }
                        }));
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
