mod builder;
mod desc;
mod handler;
mod kind;
mod proto;
use crate::hub::{ChanCtx, GProto, Hub, ModuleName};
use async_trait::async_trait;
pub use builder::Builder;
use desc::Description;
use gsfw::{chanrpc::broker::Broker, component, util::timer::Wheel};
pub use kind::TimerKind;
pub use proto::*;
use std::error::Error as StdError;
use tokio::sync::mpsc;

pub struct TimerComponet {
    broker: Hub,
    rx: mpsc::Receiver<ChanCtx>,
}

#[async_trait]
impl component::Component<Hub> for TimerComponet {
    #[inline]
    fn name(&self) -> ModuleName {
        ModuleName::Timer
    }

    async fn init(
        self: Box<Self>,
    ) -> Result<Box<dyn component::Component<Hub>>, Box<dyn StdError + Send>> {
        Ok(self)
    }

    async fn run(self: Box<Self>) -> Result<(), Box<dyn StdError + Send>> {
        let mut rx = self.rx;
        let now = std::time::Instant::now();

        let mut y2m_wheel =
            Wheel::<Description>::new(12, time::Duration::days(30).try_into().unwrap(), now);
        let mut m2d_wheel =
            Wheel::<Description>::new(30, time::Duration::days(1).try_into().unwrap(), now);
        let mut d2h_wheel =
            Wheel::<Description>::new(24, time::Duration::hours(1).try_into().unwrap(), now);
        let mut h2m_wheel =
            Wheel::<Description>::new(60, time::Duration::minutes(1).try_into().unwrap(), now);
        let mut m2s_wheel =
            Wheel::<Description>::new(60, time::Duration::seconds(1).try_into().unwrap(), now);
        let mut s2ms_wheel =
            Wheel::<Description>::new(5, std::time::Duration::from_millis(200), now);

        macro_rules! find_wheel {
            ($deadline: ident) => {
                if s2ms_wheel.within_round($deadline) {
                    &mut s2ms_wheel
                } else if m2s_wheel.within_round($deadline) {
                    &mut m2s_wheel
                } else if h2m_wheel.within_round($deadline) {
                    &mut h2m_wheel
                } else if d2h_wheel.within_round($deadline) {
                    &mut d2h_wheel
                } else if m2d_wheel.within_round($deadline) {
                    &mut m2d_wheel
                } else {
                    &mut y2m_wheel
                }
            };
        }

        loop {
            tokio::select! {
                Some(req) = rx.recv() => {
                    let payload = req.payload();
                    match payload {
                        GProto::TMProtoReq(payload) => {
                            let ret = match payload {
                                TMProtoReq::NewTimeout { duration, kind } =>  {
                                    // let wheel = find_wheel();
                                    let deadline = std::time::Instant::now() + duration;
                                    let wheel = find_wheel!(deadline);
                                    Self::ctl_new_timeout(wheel, req.from().clone(),duration, kind).await
                                }
                                TMProtoReq::NewInterval { duration, kind } => {
                                    let deadline = std::time::Instant::now() + duration;
                                    let wheel = find_wheel!(deadline);
                                    Self::ctl_new_interval(wheel, req.from().clone(), duration, kind).await
                                },
                                TMProtoReq::NewDeadline { deadline, kind } => {
                                    let wheel = find_wheel!(deadline);
                                    Self::ctl_new_deadline(wheel, req.from().clone(), deadline, kind).await
                                }
                            };
                            match ret {
                                Ok(ack) => req.ok(ack.into()),
                                Err(err) => req.err(err)
                            }
                        }
                        _unexpected => {
                            tracing::error!(
                                "receive unhandled ChanProto. {}",
                                Into::<&'static str>::into(_unexpected)
                            );
                        }
                    }
                },
                el = s2ms_wheel.tick() => {
                    for meta in el {
                        let Some(desc) = meta.data else {
                            tracing::error!("Meta.data is None");
                            continue;
                        };
                        if desc.repeat {
                            let now = std::time::Instant::now();
                            let interval = meta.end - meta.start;
                            let new_deadline = now + interval;
                            let wheel = find_wheel!(new_deadline);
                            match Self::ctl_new_interval(wheel, desc.from, interval, desc.data.clone()).await {
                                Ok(ack) => {
                                    tracing::info!("dispatch new interval for {:?}, snapshot: {:?}", desc.data, ack);
                                },
                                Err(err) => {
                                    tracing::error!("dispatch new interval for {:?} fail. {}", desc.data, err);
                                }
                            };
                        }
                        self.broker.cast(desc.from.clone(), TMProtoNtf{
                            id: meta.id,
                            start: meta.start,
                            end: meta.end,
                            data: desc.data
                        }.into()).await;
                    }
                },
                month_el = y2m_wheel.tick() => {
                    m2d_wheel.batch_add_even_elapse(month_el).await.unwrap();
                },
                day_el = m2d_wheel.tick() => {
                    d2h_wheel.batch_add_even_elapse(day_el).await.unwrap();
                },
                hour_el = d2h_wheel.tick() => {
                    h2m_wheel.batch_add_even_elapse(hour_el).await.unwrap();
                },
                min_el = h2m_wheel.tick() => {
                    m2s_wheel.batch_add_even_elapse(min_el).await.unwrap();
                },
                sec_el = m2s_wheel.tick() => {
                    s2ms_wheel.batch_add_even_elapse(sec_el).await.unwrap();
                }
            }
        }
    }
}
