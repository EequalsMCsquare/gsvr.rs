use anyhow::anyhow;
use game_core::{
    component::{Component, ComponentJoinHandle}, broker::Broker,
};
use pb::Message;
use tokio::sync::mpsc;
mod builder;
mod handler;
use crate::{hub::{ChanProto, Hub, ModuleName, ChanCtx}, error::Error};
pub use builder::Builder;

pub struct NatsPlugin {
    nats: async_nats::Client,
    hub: Hub,
    rx: mpsc::Receiver<ChanCtx>,
}

impl Component<ModuleName, ChanProto> for NatsPlugin {
    type BrkrError = Error;

    fn name(&self) -> ModuleName {
        ModuleName::Nats
    }

    fn channel(&self) -> mpsc::Sender<ChanCtx> {
        self.hub.get_tx(self.name()).clone()
    }

    fn run(mut self: Box<Self>) -> ComponentJoinHandle<anyhow::Error> {
        ComponentJoinHandle::TokioHandle(tokio::spawn(async move {
            let mut msg = pb::ScProto::default();
            loop {
                match self.rx.recv().await {
                    Some(req) => match &req.payload {
                        ChanProto::ScPMsg { player_id, message } => {
                            msg.payload = Some(message.clone());
                            if let Err(err) = self
                                .nats
                                .publish(
                                    format!("scpmsg.{}", player_id),
                                    msg.encode_to_vec().into(),
                                )
                                .await
                            {
                                tracing::error!("fail to publish scpmsg.*. {}", err);
                            }
                        }
                        ChanProto::SubTopicReq { topic } => {
                            match self.nats.subscribe(topic.clone()).await {
                                Ok(sub) => req.ok(ChanProto::SubTopicAck { subscriber: sub }),
                                Err(err) => req.err(Error::NatsSub(err)),
                            }
                        }
                        ChanProto::Sub2HubReq { topic, decode_fn } => {
                            match self
                                .on_Sub2HubReq(req.from.clone(), topic.clone(), decode_fn.clone())
                                .await
                            {
                                Ok(_) => req.ok(ChanProto::Sub2HubAck),
                                Err(err) => {
                                    req.err(err);
                                }
                            }
                        }
                        _unhandled_msg => {
                            tracing::error!("receive unhandled ChanProto. {:?}", _unhandled_msg);
                        }
                    },
                    None => return Err(anyhow!("receive none")),
                }
            }
        }))
    }
}