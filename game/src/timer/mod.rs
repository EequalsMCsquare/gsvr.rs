use crate::{
    error::Error,
    hub::{ChanCtx, ChanProto, Hub, ModuleName, TimerArgs},
};
use parking_lot::RwLock;
use slice_deque::SliceDeque;
use tokio::sync::mpsc;
mod builder;
mod handler;
pub use builder::Builder;
use gsfw::component;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TimerMeta {
    pub(super) typ: u16,
    pub(super) save: bool,
    pub(super) from: ModuleName,
    pub(super) deadline: std::time::Instant,
    pub(super) args: TimerArgs,
}

#[allow(dead_code)]
pub struct TimerComponent {
    hub: Hub,
    rx: mpsc::Receiver<ChanCtx>,
    timers: RwLock<SliceDeque<TimerMeta>>,
    curr: RwLock<Option<TimerMeta>>,
    curr_handle: tokio::task::JoinHandle<()>,
    timer_rx: mpsc::Receiver<()>,
    timer_tx: mpsc::Sender<()>,
}

#[async_trait::async_trait]
impl component::Component<ChanProto, ModuleName, Error> for TimerComponent {
    fn name(&self) -> ModuleName {
        ModuleName::Timer
    }

    async fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(mut self: Box<Self>) -> Result<(), Error>{
        loop {
            tokio::select! {
                Some(req) = self.rx.recv() => {
                    match req.payload {
                        ChanProto::NewTimerReq { typ, deadline, args } => {
                            self.on_NewTimerReq(
                                req.from,
                                typ,
                                deadline.clone(),
                                args
                            );
                        }
                        // ChanProto::NewTickerReq { typ, interval, start_time, args } => todo!(),
                        ChanProto::CtrlShutdown => {
                            tracing::info!("[{:?}]recv shutdown", ModuleName::Timer);
                            return Ok(());
                        }
                        _um => panic!("recv unhandled message: {:?}", _um),
                    }
                },
                _ = self.timer_rx.recv() => self.on_TimerTrigger().await,
            }
        }
    }
}
