use std::{fmt::Debug, hash::Hash};

use hashbrown::{HashMap, HashSet};
use plugin::PluginBuilder;
use tokio::sync::mpsc;

use crate::{
    broker::{self, Broker, ChanCtx},
    plugin::{self, PluginJoinHandle},
};

use super::Server;

pub struct ServerBuilder<NameEnum, Proto, Brkr>
where
    NameEnum: Hash + Eq + Send + Debug,
    Proto: Send,
    Brkr: Broker<Proto, NameEnum>,
{
    plugin_set: HashSet<NameEnum>,
    plugin_builders: Vec<Box<dyn PluginBuilder<NameEnum, Proto, Brkr>>>,
}

impl<NameEnum, Proto, Brkr> ServerBuilder<NameEnum, Proto, Brkr>
where
    NameEnum: Hash + Eq + Send + Debug + Copy,
    Proto: Send,
    Brkr: broker::Broker<Proto, NameEnum>,
{
    pub fn new() -> Self {
        Self {
            plugin_set: Default::default(),
            plugin_builders: Default::default(),
        }
    }

    pub fn plugin<PB>(mut self, plugin_builder: PB) -> Self
    where
        PB: PluginBuilder<NameEnum, Proto, Brkr> + 'static,
    {
        if let Some(_) = self.plugin_set.get(&plugin_builder.name()) {
            panic!("plugin[{:?}] already registered", plugin_builder.name());
        }
        self.plugin_set.insert(plugin_builder.name());
        self.plugin_builders.push(Box::new(plugin_builder));
        self
    }

    pub fn serve<Error>(self) -> Server<NameEnum, Error>
    where
        Vec<PluginJoinHandle<Error>>: FromIterator<PluginJoinHandle<anyhow::Error>>,
    {
        let join_handles: Vec<PluginJoinHandle<Error>>;
        let mut broker_map: HashMap<
            NameEnum,
            (
                mpsc::Sender<ChanCtx<Proto, NameEnum>>,
                mpsc::Receiver<ChanCtx<Proto, NameEnum>>,
            ),
        > = self
            .plugin_builders
            .iter()
            .map(|builder| (builder.name(), mpsc::channel(1024)))
            .collect();
        tracing::debug!("broker map: {:?}", broker_map);
        // set up plugins
        let tx_map: HashMap<NameEnum, mpsc::Sender<ChanCtx<Proto, NameEnum>>> = broker_map
            .iter()
            .map(|(k, (tx, _))| (k.clone(), tx.clone()))
            .collect();
        tracing::debug!("tx map: {:?}", tx_map);
        join_handles = self
            .plugin_builders
            .into_iter()
            .map(|mut builder| {
                builder.set_broker(Brkr::new(builder.name(), &tx_map));
                builder.set_rx(broker_map.remove(&builder.name()).unwrap().1);
                tracing::debug!("PluginBuilder {:?} setup complete", builder.name());
                let mut plugin = builder.build();
                tracing::debug!("plugin {:?} build success", plugin.name());
                if let Err(err) = plugin.init() {
                    panic!("plugin {:?} init error. {}", plugin.name(), err);
                }
                tracing::debug!("plugin {:?} init success, begin to run", plugin.name());
                plugin.run()
            })
            .collect();
        tracing::info!("all plugins launch complete, running: {:?}", self.plugin_set);

        Server {
            registered_plugins: self.plugin_set,
            plugin_join_handles: join_handles,
        }
    }
}
