mod ctx;
mod proto;
use std::fmt::Debug;

pub use ctx::ChanCtx;
pub use proto::ChanProto;
use tokio::sync::mpsc;

pub type ProtoSender = mpsc::Sender<ChanCtx>;

#[derive(Debug, Clone)]
pub struct Hub {
    pub module: ModuleName,
    pub play: mpsc::Sender<ChanCtx>,
    pub nats: mpsc::Sender<ChanCtx>,
    pub db: mpsc::Sender<ChanCtx>,
}

#[derive(Debug)]
pub enum ModuleProto {
    Play(ChanProto),
    Nats(ChanProto),
    DB(ChanProto),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ModuleName {
    #[default]
    Uninit,
    Play,
    Nats,
    DB,
}

impl Hub {
    pub async fn cast(&self, msg: ModuleProto) {
        if let Err(err) = match msg {
            ModuleProto::Play(msg) => self.play.send(ChanCtx::new_cast(msg, self.module)).await,
            ModuleProto::Nats(msg) => self.nats.send(ChanCtx::new_cast(msg, self.module)).await,
            ModuleProto::DB(msg) => self.db.send(ChanCtx::new_cast(msg, self.module)).await,
        } {
            tracing::error!("fail to cast. {}", err);
        }
    }

    pub fn blocking_cast(&self, msg: ModuleProto) {
        if let Err(err) = match msg {
            ModuleProto::Play(msg) => self.play.blocking_send(ChanCtx::new_cast(msg, self.module)),
            ModuleProto::Nats(msg) => self.nats.blocking_send(ChanCtx::new_cast(msg, self.module)),
            ModuleProto::DB(msg) => self.db.blocking_send(ChanCtx::new_cast(msg, self.module)),
        } {
            tracing::error!("fail to cast. {}", err);
        }
    }

    pub async fn call(&self, msg: ModuleProto) -> anyhow::Result<ChanProto> {
        let ctx;
        let rx;
        if let Err(err) = match msg {
            ModuleProto::Play(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.play.send(ctx).await
            }
            ModuleProto::Nats(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.nats.send(ctx).await
            }
            ModuleProto::DB(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.db.send(ctx).await
            }
        } {
            tracing::error!("fail to request. {}", err);
        }
        rx.await?
    }

    pub fn blocking_call(&self, msg: ModuleProto) -> anyhow::Result<ChanProto> {
        let ctx;
        let rx;
        if let Err(err) = match msg {
            ModuleProto::Play(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.play.blocking_send(ctx)
            }
            ModuleProto::Nats(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.nats.blocking_send(ctx)
            }
            ModuleProto::DB(msg) => {
                (ctx, rx) = ChanCtx::new_req(msg, self.module);
                self.db.blocking_send(ctx)
            }
        } {
            tracing::error!("fail to request. {}", err);
        }
        rx.blocking_recv()?
    }
}
