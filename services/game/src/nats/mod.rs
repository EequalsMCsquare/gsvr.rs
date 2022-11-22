use cspb::Message;
use tokio::sync::mpsc;
mod builder;
mod handler;
use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName},
};
pub use builder::Builder;
use gsfw::component;

pub struct NatsComponent {
    nats: async_nats::Client,
    hub: Hub,
    rx: mpsc::Receiver<ChanCtx>,
}

#[async_trait::async_trait]
impl component::Component<ChanProto, ModuleName, Error> for NatsComponent {
    fn name(&self) -> ModuleName {
        ModuleName::Nats
    }

    async fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(mut self: Box<Self>) -> Result<(), Error> {
        let mut msg = cspb::ScProto::default();
        loop {
            match self.rx.recv().await {
                Some(req) => match &req.payload {
                    ChanProto::ScPMsgNtf { player_id, message } => {
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
                    ChanProto::CtrlShutdown => {
                        tracing::info!("[{:?}]recv shutdown", ModuleName::Nats);
                        return Ok(());
                    }
                    _unhandled_msg => {
                        tracing::error!("receive unhandled ChanProto. {:?}", _unhandled_msg);
                    }
                },
                None => {
                    tracing::info!("recv None, all sender drop");
                    return Ok(())
                },
            }
        }
    }
}
