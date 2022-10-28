use derivative::Derivative;
use game_core::broker::Proto;
use mongodb::bson;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Derivative)]
#[derivative(Debug)]
pub enum ChanProto {
    CsPMsgNtf {
        player_id: u64,
        message: pb::CsMsg,
    },
    ScPMsgNtf {
        player_id: u64,
        message: pb::ScMsg,
    },

    // subscribe nats topic and get a subscriber
    SubTopicReq {
        topic: String,
    },
    SubTopicAck {
        #[derivative(Debug = "ignore")]
        subscriber: async_nats::Subscriber,
    },

    // subscribe nats topic to rx
    Sub2HubReq {
        topic: String,
        decode_fn: fn(async_nats::Message) -> anyhow::Result<ChanProto>,
    },
    Sub2HubAck,

    // load data from mongodb
    DBLoadReq {
        coll: String,
        filter: Option<bson::Document>,
    },
    DBLoadAck(bson::Binary),

    // start a new timer
    NewTimerReq {
        typ: u16,
        deadline: std::time::Instant,
        args: TimerArgs,
    },
    NewTimerAck,

    // start a new ticker
    NewTickerReq {
        typ: u16,
        interval: std::time::Duration,
        start_time: std::time::Instant,
        args: TimerArgs,
    },
    NewTickerAck,

    // notify timer/ticker trigger
    TimerTriggerNtf {
        typ: u16,
        args: TimerArgs,
    },

    CtrlShutdown,
}

impl Proto for ChanProto {
    fn proto_shutdown() -> Self {
        Self::CtrlShutdown
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TimerArgs {
    Empty,
}
