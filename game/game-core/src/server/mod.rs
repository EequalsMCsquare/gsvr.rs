mod builder;
pub use builder::ServerBuilder;
use futures::{ready, Future, FutureExt};
use pin_project::pin_project;
use slice_deque::SliceDeque;
use std::{cell::Cell, fmt::Debug, task::Poll};
use tokio::{sync::oneshot, task::JoinHandle};

pub struct ComponentHandle<NameEnum, Error>
where
    NameEnum: Send + Debug,
{
    join: JoinHandle<Result<(), Error>>,
    ctrl: Cell<Option<oneshot::Sender<()>>>,
    name: NameEnum,
}

#[pin_project]
pub struct Server<NameEnum, Error>
where
    NameEnum: Send + Debug,
{
    // reg_components: Vec<NameEnum>,
    // component_joins: SliceDeque<JoinHandle<Result<(), Error>>>,
    component_handles: SliceDeque<ComponentHandle<NameEnum, Error>>,
    poll_component: Option<ComponentHandle<NameEnum, Error>>,
    poll_cursor: i8,
}

impl<NameEnum, Error> Future for Server<NameEnum, Error>
where
    NameEnum: Send + Debug,
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        // components joins length is checked in ServerBuilder::serve(), so there is no panic
        this.poll_component
            .replace(this.component_handles.pop_front().unwrap());
        if *this.poll_cursor == -1 {
            let mut ctrl_c_future = Box::pin(tokio::signal::ctrl_c());
            match ready!(ctrl_c_future.poll_unpin(cx)) {
                Ok(_) => {
                    *this.poll_cursor += 1;
                    this.component_handles.iter().for_each(|h| {
                        let tx = h.ctrl.take().unwrap();
                        if let Err(_) = tx.send(()) {
                            tracing::error!(
                                "error occur while sending close ctrl to component {:?}",
                                h.name
                            )
                        }
                    });
                    tracing::info!("recv SIGKILL, begin to clean up")
                }
                Err(err) => {
                    tracing::error!("ctrl_c polling error: {}. ", err)
                }
            };
        }
        loop {
            if let Some(component) = this.poll_component {
                match ready!(component.join.poll_unpin(cx)) {
                    Ok(_) => {
                        tracing::info!(
                            "component join success. {} components left",
                            this.component_handles.len()
                        )
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
