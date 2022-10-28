use crate::broker::Proto;
mod builder;
pub use builder::ComponentBuilder;
use std::error::Error as StdError;

#[async_trait::async_trait]
pub trait Component<NameEnum, P>: Send
where
    NameEnum: Send,
    P: Proto,
{
    type BrkrError: Send;

    fn name(&self) -> NameEnum;
    async fn init(&mut self) -> Result<(), Box<dyn StdError + Send>> {
        Ok(())
    }
    async fn run(self: Box<Self>) -> anyhow::Result<()>;
}
