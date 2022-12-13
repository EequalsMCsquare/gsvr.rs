use crate::{
    db::{DBProtoAck, DBProtoReq},
    nats::{MQProtoAck, MQProtoReq},
    play::{PLProtoAck, PLProtoReq},
};
use gsfw::chanrpc;
use std::fmt::Debug;

// #[derive(Debug)]
// pub struct PCS {
//     pub player_id: i64,
//     pub message: cspb::CsMsg,
// }

// #[derive(Debug)]
// pub struct PSC {
//     pub player_id: i64,
//     pub message: cspb::ScMsg,
// }

pub struct PMSG {
    pub player_id: i64,
    pub message: cspb::Registry,
}

#[allow(dead_code)]
#[derive(strum::IntoStaticStr)]
pub enum GProto {
    // PCS(Box<PCS>),
    // PSC(Box<PSC>),
    PMSG(PMSG),
    Ok,
    CtrlShutdown,

    MQProtoReq(MQProtoReq),
    MQProtoAck(MQProtoAck),
    DBProtoReq(DBProtoReq),
    DBProtoAck(DBProtoAck),
    PLProtoReq(PLProtoReq),
    PLProtoAck(PLProtoAck),
}

impl chanrpc::Proto for GProto {
    fn proto_shutdown() -> Self {
        Self::CtrlShutdown
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TimerArgs {
    Empty,
}
