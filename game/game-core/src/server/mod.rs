use futures::{ready, Future, FutureExt};
use hashbrown::HashSet;

use crate::component::ComponentJoinHandle;

mod builder;
pub use builder::ServerBuilder;

pub struct Server<NameEnum, Error> {
    registered_plugins: HashSet<NameEnum>,
    plugin_join_handles: Vec<ComponentJoinHandle<Error>>,
}

impl<NameEnum, Error> Future for Server<NameEnum, Error> {
    type Output = Result<(), std::io::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut ctrl_c_future = Box::pin(tokio::signal::ctrl_c());
        // todo: reversely send close signal and wait for handles to join
        match ready!(ctrl_c_future.poll_unpin(cx)) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
