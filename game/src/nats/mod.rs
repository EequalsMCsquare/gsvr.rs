use std::sync::Arc;

use anyhow::anyhow;
use futures::StreamExt;
use pb::Message;
use tokio::sync::mpsc;

use crate::hub::{ChanCtx, ChanProto, Hub, ModuleName, ModuleProto, ProtoSender};

pub struct Module {
    nats: async_nats::Client,
    hub: Option<Arc<crate::hub::Hub>>,
    tx: ProtoSender,
    rx: mpsc::Receiver<ChanCtx>,
}

impl Module {
    pub fn new(client: async_nats::Client) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            nats: client,
            hub: None,
            tx,
            rx,
        }
    }

    pub fn with_hub(&mut self, mut hub: Hub) {
        hub.module = ModuleName::Nats;
        self.hub = Some(Arc::new(hub))
    }

    pub fn chanrpc(&self) -> ProtoSender {
        self.tx.clone()
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut msg = pb::ScProto::default();
        loop {
            match self.rx.recv().await {
                Some(req) => match &req.payload {
                    ChanProto::ScPMsg { player_id, message } => {
                        msg.payload = Some(message.clone());
                        if let Err(err) = self
                            .nats
                            .publish(format!("scpmsg.{}", player_id), msg.encode_to_vec().into())
                            .await
                        {
                            tracing::error!("fail to publish scpmsg.*. {}", err);
                        }
                    }
                    ChanProto::SubTopicReq { topic } => {
                        match self.nats.subscribe(topic.clone()).await {
                            Ok(sub) => req.ok(ChanProto::SubTopicAck { subscriber: sub }),
                            Err(err) => req.err(anyhow!("{}", err.to_string())),
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
    }

    async fn on_Sub2HubReq(
        &self,
        from: ModuleName,
        topic: String,
        decode_fn: fn(async_nats::Message) -> anyhow::Result<ChanProto>,
    ) -> anyhow::Result<()> {
        match self.nats.subscribe(topic).await {
            Ok(mut sub) => {
                let func = decode_fn;
                let sender = from;
                let hub = self.hub.as_ref().unwrap().clone();
                let _handle = tokio::spawn(async move {
                    while let Some(mq_msg) = sub.next().await {
                        let proto = match func(mq_msg) {
                            Ok(proto) => proto,
                            Err(err) => {
                                tracing::error!("fail to decode nats message. {}", err);
                                continue;
                            }
                        };
                        let proto = match sender {
                            crate::hub::ModuleName::Play => ModuleProto::Play(proto),
                            crate::hub::ModuleName::Nats => ModuleProto::Nats(proto),
                            crate::hub::ModuleName::DB => ModuleProto::DB(proto),
                            crate::hub::ModuleName::Uninit => panic!("receive from uninit sender"),
                        };
                        hub.cast(proto).await;
                    }
                });
            }
            Err(err) => return Err(anyhow!("{}", err.to_string())),
        }
        Ok(())
    }
}
