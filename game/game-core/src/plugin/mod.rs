use crate::broker;
use tokio::sync::mpsc;
mod builder;
pub use builder::PluginBuilder;

pub enum PluginJoinHandle<Error> {
    TokioHandle(tokio::task::JoinHandle<Result<(), Error>>),
    ThreadHandle(std::thread::JoinHandle<Result<(), Error>>),
}

pub trait Plugin<NameEnum, Proto>
where
    NameEnum: Send,
    Proto: Send,
{
    fn name(&self) -> NameEnum;
    fn channel(&self) -> mpsc::Sender<broker::ChanCtx<Proto, NameEnum>>;
    fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn run(self: Box<Self>) -> PluginJoinHandle<anyhow::Error>;
}
