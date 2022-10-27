use crate::broker;
use tokio::sync::mpsc;
mod builder;
pub use builder::ComponentBuilder;

pub enum ComponentJoinHandle<Error> {
    TokioHandle(tokio::task::JoinHandle<Result<(), Error>>),
    ThreadHandle(std::thread::JoinHandle<Result<(), Error>>),
}

pub trait Component<NameEnum, Proto>
where
    NameEnum: Send,
    Proto: Send,
{
    type BrkrError: Send;

    fn name(&self) -> NameEnum;
    fn channel(&self) -> mpsc::Sender<broker::ChanCtx<Proto, NameEnum, Self::BrkrError>>;
    fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn run(self: Box<Self>) -> ComponentJoinHandle<anyhow::Error>;
}
