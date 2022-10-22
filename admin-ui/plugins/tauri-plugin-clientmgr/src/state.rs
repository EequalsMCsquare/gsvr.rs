use admin::proto;
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};
use tauri::regex::Regex;
use tokio::sync::{broadcast, mpsc, oneshot};

#[derive(Debug)]
pub(crate) struct ClientProxy<T> {
    pub(crate) id: T,
    pub(crate) history: Arc<admin::Histroy>,
    pub(crate) closechan: oneshot::Sender<()>,
    pub(crate) join: tauri::async_runtime::JoinHandle<anyhow::Result<()>>,
}

#[derive(Debug)]
pub struct ClientMgr<T: serde::Serialize + PartialEq> {
    pub(crate) clients: RwLock<HashMap<u64, ClientProxy<T>>>,
    pub(crate) pb_name_re: Regex,
    pub(crate) br_tx: broadcast::Sender<proto::TagCsMsg<T>>,
    pub(crate) tx: mpsc::Sender<proto::TagScMsg<T>>,
}

impl ClientMgr<u64> {
    pub fn new() -> (Self, mpsc::Receiver<proto::TagScMsg<u64>>) {
        let (br_tx, _) = broadcast::channel(1024);
        let (tx, rx) = mpsc::channel(1024);
        (
            Self {
                clients: RwLock::new(HashMap::new()),
                pb_name_re: Regex::new(r#"\{\s*"(?P<pbName>\w+)":"#).unwrap(),
                br_tx,
                tx,
            },
            rx,
        )
    }
}

impl ClientMgr<String> {
    pub fn new() -> (Self, mpsc::Receiver<proto::TagScMsg<String>>) {
        let (br_tx, _) = broadcast::channel(1024);
        let (tx, rx) = mpsc::channel(1024);
        (
            Self {
                pb_name_re: Regex::new(r#"\{\s*"(?P<pbName>\w+)":"#).unwrap(),
                clients: RwLock::new(HashMap::new()),
                br_tx,
                tx,
            },
            rx,
        )
    }
}
