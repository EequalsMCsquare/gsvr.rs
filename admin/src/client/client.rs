pub struct ClientInfo {
    pub(super) id: u64,
    pub(super) _username: String,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            _username: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientCsMsg {
    pub ids: Vec<u64>,
    pub payload: cspb::CsMsg,
}

#[derive(Clone, Debug)]
pub struct ClientScMsg {
    pub id: u64,
    pub payload: cspb::ScMsg,
}
