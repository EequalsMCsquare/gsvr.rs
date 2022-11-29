use crate::hub::GProto;

pub enum MQProtoReq {
    SubTopicReq(String),

    Sub2HubReq {
        topic: String, 
        decode_fn: fn(async_nats::Message) -> anyhow::Result<GProto>
    },
}

pub enum MQProtoAck {
    SubTopicAck(async_nats::Subscriber),
}