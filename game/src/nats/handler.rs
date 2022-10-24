use anyhow::anyhow;
use futures::StreamExt;
use game_core::broker::Broker;

use crate::hub::{ModuleName, ChanProto};

use super::NatsPlugin;

impl NatsPlugin {
    pub(super) async fn on_Sub2HubReq(
        &self,
        from: ModuleName,
        topic: String,
        decode_fn: fn(async_nats::Message) -> anyhow::Result<ChanProto>,
    ) -> anyhow::Result<()> {
        match self.nats.subscribe(topic).await {
            Ok(mut sub) => {
                let func = decode_fn;
                let sender = from;
                let hub = self.hub.clone();
                let _handle = tokio::spawn(async move {
                    while let Some(mq_msg) = sub.next().await {
                        let proto = match func(mq_msg) {
                            Ok(proto) => proto,
                            Err(err) => {
                                tracing::error!("fail to decode nats message. {}", err);
                                continue;
                            }
                        };
                        hub.cast(sender, proto).await;
                    }
                });
            }
            Err(err) => return Err(anyhow!("{}", err.to_string())),
        }
        Ok(())
    }
}

