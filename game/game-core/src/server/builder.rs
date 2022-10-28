use crate::{
    broker::{self, Broker, ChanCtx, Proto},
    component,
    server::ComponentHandle,
};
use anyhow::anyhow;
use component::ComponentBuilder;
use hashbrown::{HashMap, HashSet};
use slice_deque::SliceDeque;
use std::{cell::Cell, fmt::Debug, hash::Hash};
use tokio::sync::{mpsc, oneshot};

use super::Server;

pub struct ServerBuilder<NameEnum, P, Brkr>
where
    NameEnum: Hash + Eq + Send + Debug,
    P: Proto,
    Brkr: Broker<P, NameEnum>,
{
    component_set: HashSet<NameEnum>,
    component_builders: Vec<Box<dyn ComponentBuilder<NameEnum, P, Brkr, BrkrError = Brkr::Error>>>,
}

impl<NameEnum, P, Brkr> ServerBuilder<NameEnum, P, Brkr>
where
    NameEnum: Hash + Eq + Send + Debug + Copy + 'static,
    P: Proto + 'static,
    Brkr: broker::Broker<P, NameEnum> + 'static,
{
    pub fn new() -> Self {
        Self {
            component_set: Default::default(),
            component_builders: Default::default(),
        }
    }

    pub fn component<PB>(mut self, component_builder: PB) -> Self
    where
        PB: ComponentBuilder<NameEnum, P, Brkr, BrkrError = Brkr::Error> + 'static,
    {
        if let Some(_) = self.component_set.get(&component_builder.name()) {
            panic!(
                "component[{:?}] already registered",
                component_builder.name()
            );
        }
        self.component_set.insert(component_builder.name());
        self.component_builders.push(Box::new(component_builder));
        self
    }

    pub fn serve(self) -> anyhow::Result<Server<NameEnum, anyhow::Error>> {
        if self.component_builders.len() == 0 {
            return Err(anyhow!("no components"));
        }
        let component_futures: Vec<_>;
        let mut broker_map: HashMap<
            NameEnum,
            (
                mpsc::Sender<ChanCtx<P, NameEnum, Brkr::Error>>,
                mpsc::Receiver<ChanCtx<P, NameEnum, Brkr::Error>>,
            ),
        > = self
            .component_builders
            .iter()
            .map(|builder| (builder.name(), mpsc::channel(1024)))
            .collect();
        let tx_map: HashMap<NameEnum, mpsc::Sender<ChanCtx<P, NameEnum, Brkr::Error>>> = broker_map
            .iter()
            .map(|(k, (tx, _))| (k.clone(), tx.clone()))
            .collect();
        // set up components
        let components: Vec<_> = self
            .component_builders
            .into_iter()
            .map(|mut builder| {
                let (tx, rx) = oneshot::channel();
                builder.set_broker(Brkr::new(builder.name(), &tx_map));
                builder.set_rx(broker_map.remove(&builder.name()).unwrap().1);
                builder.set_ctrl(rx);
                tracing::debug!("ComponentBuilder {:?} setup complete", builder.name());
                let ret = builder.build();
                tracing::debug!("component {:?} setup complete", ret.name());
                (ret, tx)
            })
            .collect();
        component_futures = components
            .into_iter()
            .map(|mut component| {
                let name = component.0.name();
                ComponentHandle {
                    join: tokio::spawn(async move {
                        component
                            .0
                            .init()
                            .await
                            .map_err(|err| anyhow!("{:?}", err))?;
                        component.0.run().await
                    }),
                    ctrl: Cell::new(Some(component.1)),
                    name: name,
                }
            })
            .collect();

        tracing::info!(
            "all componentss launch complete, running: {:?}",
            self.component_set
        );
        tracing::info!("press ctrl+c to terminate the app");
        Ok(Server {
            component_handles: SliceDeque::from_iter(component_futures.into_iter()),
            poll_cursor: -1,
            poll_component: None,
        })
    }
}
