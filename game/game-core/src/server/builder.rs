use super::Server;
use crate::{
    broker::{self, Broker, ChanCtx, Proto},
    component,
    server::ComponentHandle,
};
use anyhow::anyhow;
use component::ComponentBuilder;
use hashbrown::{HashMap, HashSet};
use slice_deque::SliceDeque;
use std::{fmt::Debug, hash::Hash};
use tokio::sync::mpsc;

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

        // get all tx
        let tx_map2 = tx_map.clone();
        let ctrl_c_future = tokio::spawn(async move {
            // wait for user to press ctrl_c
            if let Err(err) = tokio::signal::ctrl_c().await {
                tracing::error!("ctrl_c error: {}", err);
            }
            tracing::info!("ctrl+c receive, begin to clean up");
            // send proto_shutdown to all components
            for (k, tx) in tx_map2 {
                tracing::debug!("sending shutdown to {:?}", k);
                let k = k.clone();
                if let Err(err) = tx.send(ChanCtx::new_cast(Proto::proto_shutdown(), k)).await {
                    tracing::error!("fail to send shutdown to {:?}: {}", k, err);
                }
            }
        });

        // set up components
        let components: Vec<_> = self
            .component_builders
            .into_iter()
            .map(|mut builder| {
                builder.set_broker(Brkr::new(builder.name(), &tx_map));
                builder.set_rx(broker_map.remove(&builder.name()).unwrap().1);
                tracing::debug!("ComponentBuilder {:?} setup complete", builder.name());
                let ret = builder.build();
                tracing::debug!("component {:?} setup complete", ret.name());
                ret
            })
            .collect();
        component_futures = components
            .into_iter()
            .map(|mut component| {
                let name = component.name();
                ComponentHandle {
                    join: tokio::spawn(async move {
                        component.init().await.map_err(|err| anyhow!("{:?}", err))?;
                        component.run().await
                    }),
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
            ctrl_c_future,
        })
    }
}
