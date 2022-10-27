use game_core::{
    broker::Broker,
    component::{Component, ComponentJoinHandle},
};
use tokio::sync::mpsc;
use crate::{hub::{ChanCtx, ChanProto, Hub, ModuleName}, error::Error};
use mongodb::bson;
mod builder;
pub use builder::Builder;

pub struct DBPlugin {
    broker: Hub,
    rx: mpsc::Receiver<ChanCtx>,
    database: mongodb::Database
}

impl Component<ModuleName, ChanProto> for DBPlugin {
    type BrkrError = Error;
    #[inline]
    fn name(&self) -> ModuleName {
        ModuleName::DB
    }

    fn channel(&self) -> tokio::sync::mpsc::Sender<ChanCtx> {
        self.broker.get_tx(self.name()).clone()
    }

    fn run(self: Box<Self>) -> ComponentJoinHandle<anyhow::Error> {
        ComponentJoinHandle::TokioHandle(tokio::spawn(async move{ 
            let db = self.database.clone();
            let mut rx = self.rx;
            loop {
                if let Some(req) = rx.recv().await {
                    match req.payload {
                        ChanProto::DBLoadReq { coll, filter, options } => {
                            let res: Result<Option<bson::Binary>,_> = db.collection(&coll).find_one(filter, options).await;
                            match res {
                                Ok(Some(bin)) => todo!(),
                                Ok(None) => todo!(),
                                Err(err) => {
                                    // req.err(Error::DBError(err));
                                },
                            }
                        },
                        _um => todo!()
                    }
                }
            }
         }))
    }
}
