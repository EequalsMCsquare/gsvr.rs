use tokio::sync::{mpsc, oneshot};

use crate::hub::ChanCtx;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("bag is at max capacity: {0}")]
    BagMax(u16),
    #[error(
        "item is at max capacity. {cap_needed} empty spots are required but only {remain} left"
    )]
    ItemMax { cap_needed: u64, remain: u64 },
    #[error(transparent)]
    Database(#[from] mongodb::error::Error),
    #[error(transparent)]
    ChanRecv(#[from] oneshot::error::RecvError),
    #[error("send error: {0}")]
    ChanSend(String),
    #[error("nats subscribe error: {0}")]
    NatsSub(Box<dyn std::error::Error + Send + Sync>),
    #[error("nats publish error: {0}")]
    NatsPub(Box<dyn std::error::Error + Send + Sync>)
}

impl From<mpsc::error::SendError<ChanCtx>> for Error {
    fn from(err: mpsc::error::SendError<ChanCtx>) -> Self {
        Self::ChanSend(err.to_string())
    }
}
