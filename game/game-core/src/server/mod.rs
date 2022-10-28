mod builder;
pub use builder::ServerBuilder;
use futures::{ready, Future, FutureExt};
use pin_project::pin_project;
use slice_deque::SliceDeque;
use std::{fmt::Debug, task::Poll};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ComponentHandle<NameEnum, Error>
where
    NameEnum: Send + Debug,
    Error: Debug,
{
    join: JoinHandle<Result<(), Error>>,
    name: NameEnum,
}

#[pin_project]
pub struct Server<NameEnum, Error>
where
    NameEnum: Send + Debug,
    Error: Debug,
{
    component_handles: SliceDeque<ComponentHandle<NameEnum, Error>>,
    poll_component: Option<ComponentHandle<NameEnum, Error>>,
    ctrl_c_future: JoinHandle<()>,
    poll_cursor: i8,
}

impl<NameEnum, Error> Future for Server<NameEnum, Error>
where
    NameEnum: Send + Debug,
    Error: Debug,
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        // components joins length is checked in ServerBuilder::serve(), so there is no panic
        if *this.poll_cursor == -1 {
            match ready!(this.ctrl_c_future.poll_unpin(cx)) {
                Ok(_) => {
                    *this.poll_cursor += 1;
                }
                Err(err) => {
                    tracing::error!("ctrl_c polling error: {}. ", err)
                }
            };
            this.poll_component
                .replace(this.component_handles.pop_front().unwrap());
        }
        loop {
            if let Some(component) = this.poll_component {
                match ready!(component.join.poll_unpin(cx)) {
                    Ok(_) => {
                        tracing::info!("[{:?}] join success", component.name)
                    }
                    Err(err) => {
                        tracing::error!("error occur while wait for component join: {}", err)
                    }
                }
                *this.poll_cursor += 1;
                *this.poll_component = this.component_handles.pop_front();
            } else {
                return Poll::Ready(());
            }
        }
    }
}
