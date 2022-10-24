use crate::error::Result;
use mongodb::bson;
use std::fmt::Debug;

pub enum ChanProto {
    CsPMsg {
        player_id: u64,
        message: pb::CsMsg,
    },
    ScPMsg {
        player_id: u64,
        message: pb::ScMsg,
    },
    SubTopicReq {
        topic: String,
    },
    SubTopicAck {
        subscriber: async_nats::Subscriber,
    },
    Sub2HubReq {
        topic: String,
        decode_fn: fn(async_nats::Message) -> anyhow::Result<ChanProto>,
    },
    DBLoadReq {
        coll: String,
        filter: Option<bson::Document>,
        options: Option<mongodb::options::FindOneOptions>,
    },
    DBLoadAck(Result<bson::Binary>),
    Sub2HubAck,
}

impl Debug for ChanProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CsPMsg { player_id, message } => f
                .debug_struct("CsPMsg")
                .field("player_id", player_id)
                .field("message", message)
                .finish(),
            Self::ScPMsg { player_id, message } => f
                .debug_struct("ScPMsg")
                .field("player_id", player_id)
                .field("message", message)
                .finish(),
            Self::SubTopicReq { topic } => {
                f.debug_struct("SubTopicReq").field("topic", topic).finish()
            }
            Self::SubTopicAck { subscriber: _ } => f.debug_struct("SubTopicAck").finish(),
            ChanProto::Sub2HubReq { topic, decode_fn } => f
                .debug_struct("Sub2HubReq")
                .field("topic", topic)
                // .field("decode_fn", decode_fn)
                .finish(),
            ChanProto::Sub2HubAck => write!(f, "Sub2HubAck"),
            ChanProto::DBLoadReq {
                coll,
                filter,
                options,
            } => todo!(),
            ChanProto::DBLoadAck(res) => todo!(),
        }
    }
}
