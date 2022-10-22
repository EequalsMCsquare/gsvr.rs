use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("fclient-{0} exist")]
    DupFClient(u64),
    #[error("fclient-{0} not found")]
    FClientNotFound(u64),
    #[error("nclient-{0} exist")]
    DupNClient(String),
    #[error("nclient-{0} not found")]
    NClientNotFound(String),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("tokio join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("{0}")]
    Any(String),
    #[error("attempt to send to close channel")]
    CloseSend,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
