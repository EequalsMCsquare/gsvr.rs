use futures::{Future, FutureExt};
use hashbrown::HashSet;

use crate::plugin::PluginJoinHandle;

mod builder;
pub use builder::ServerBuilder;

pub struct Server<NameEnum, Error> {
    registered_plugins: HashSet<NameEnum>,
    plugin_join_handles: Vec<PluginJoinHandle<Error>>
}

impl<NameEnum, Error> Future for Server<NameEnum, Error> {
    type Output = Result<(), std::io::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut ctrl_c_future = Box::pin(tokio::signal::ctrl_c());
        ctrl_c_future.poll_unpin(cx)
    }
}
