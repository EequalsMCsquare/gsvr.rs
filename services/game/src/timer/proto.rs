use crate::hub::GProto;
use super::kind::TimerKind;

pub enum TMProtoReq {
    NewTimeout {
        duration: std::time::Duration,
        kind: TimerKind,
    },
    NewInterval {
        duration: std::time::Duration,
        kind: TimerKind,
    },
    NewDeadline {
        deadline: std::time::Instant,
        kind: TimerKind,
    },
}

impl Into<GProto> for TMProtoReq {
    fn into(self) -> GProto {
        GProto::TMProtoReq(self)
    }
}

#[derive(Debug)]
pub enum TMProtoAck {
    TimeoutSnapshot {
        id: u64,
        start: std::time::Instant,
        duration: std::time::Duration,
        repeat: bool,
    },
    DeadlineSnapshot {
        id: u64,
        start: std::time::Instant,
        deadline: std::time::Instant,
    },
}

impl Into<GProto> for TMProtoAck {
    fn into(self) -> GProto {
        GProto::TMProtoAck(self)
    }
}


#[derive(Debug)]
pub struct TMProtoNtf {
    pub id: u64,
    pub start: std::time::Instant,
    pub end: std::time::Instant,
    pub data: TimerKind,
}

impl Into<GProto> for TMProtoNtf {
    fn into(self) -> GProto {
        GProto::TMProtoNtf(self)
    }
}
