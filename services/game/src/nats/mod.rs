use tokio::sync::mpsc;
mod builder;
mod handler;
mod proto;
use crate::{
    error::Error,
    hub::{ChanCtx, GProto, Hub, ModuleName},
};
pub use builder::Builder;
use gsfw::component;
use gsfw::RegistryExt;
pub use proto::{MQProtoAck, MQProtoReq};
use std::error::Error as StdError;

pub struct NatsComponent {
    nats: async_nats::Client,
    hub: Hub,
    rx: mpsc::Receiver<ChanCtx>,
}

#[async_trait::async_trait]
impl component::Component<Hub> for NatsComponent {
    fn name(&self) -> ModuleName {
        ModuleName::Nats
    }

    async fn init(self: Box<Self>) -> Result<Box<dyn component::Component<Hub>>, Box<dyn StdError + Send>> {
        Ok(self)
    }

    async fn run(mut self: Box<Self>) -> Result<(), Box<dyn StdError + Send>> {
        loop {
            match self.rx.recv().await {
                Some(req) => {
                    match req.payload() {
                        GProto::PMSG(pmsg) => {
                            // todo: try not call clone
                            if let Err(err) = self
                                .nats
                                .publish(format!("scp.{}", pmsg.player_id), pmsg.message.encode())
                                .await
                            {
                                tracing::error!("fail to publish scp.*. {}", err);
                            }
                        }
                        GProto::MQProtoReq(inner) => match inner {
                            MQProtoReq::Sub2HubReq { topic, decode_fn } => {
                                if let Err(err) = self
                                    .on_Sub2HubReq(
                                        req.from().clone(),
                                        topic.clone(),
                                        decode_fn.clone(),
                                    )
                                    .await
                                {
                                    req.err(err);
                                } else {
                                    req.ok(GProto::Ok);
                                }
                            }
                            MQProtoReq::SubTopicReq(topic) => {
                                match self.nats.subscribe(topic.clone()).await {
                                    Ok(sub) => {
                                        req.ok(GProto::MQProtoAck(MQProtoAck::SubTopicAck(sub)))
                                    }
                                    Err(err) => req.err(Error::NatsSub(err)),
                                }
                            }
                        },
                        GProto::CtrlShutdown => {
                            tracing::info!("[{:?}]recv shutdown", ModuleName::Nats);
                            return Ok(());
                        }
                        _unexpected => {
                            tracing::error!(
                                "receive unhandled ChanProto. {}",
                                Into::<&'static str>::into(_unexpected)
                            );
                        }
                    }
                }
                None => {
                    tracing::info!("recv None, all sender drop");
                    return Ok(());
                }
            }
        }
    }
}
