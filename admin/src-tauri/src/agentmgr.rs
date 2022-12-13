use std::sync::Arc;

use dashmap::DashMap;
use serde::Serialize;

use crate::agent::{FrontGateAgent, FrontPfAgent, GateAgent, GateAgentProxy, PfAgent};

#[derive(Clone)]
pub struct AgentMgr {
    inner: Arc<Inner>,
}

struct Inner {
    pfmap: DashMap<String, PfAgent>,
    gatemap: DashMap<i64, GateAgentProxy>,
    coldmap: DashMap<i64, GateAgent>,
}

#[derive(Serialize)]
pub struct FrontAgentMgr {
    pfs:  Vec<FrontPfAgent>,
    gates: Vec<FrontGateAgent>,
}

impl AgentMgr {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                pfmap: Default::default(),
                gatemap: Default::default(),
                coldmap: Default::default(),
            }),
        }
    }

    pub fn to_front(&self) -> FrontAgentMgr {
        FrontAgentMgr {
            pfs: self
                .inner
                .pfmap
                .iter()
                .map(|pair| pair.to_front())
                .collect(),
            gates: self
                .inner
                .gatemap
                .iter()
                .map(|pair| pair.to_front())
                .collect(),
        }
    }

    pub fn upsert_pf_agent(&self, agent: PfAgent) {
        self.inner.pfmap.insert(agent.username.clone(), agent);
    }

    pub fn upsert_gate_agent(&self, agent: GateAgentProxy) {
        self.inner.gatemap.insert(agent.pid, agent);
    }

    pub fn upsert_cold_agent(&self, agent: GateAgent) {
        self.inner.coldmap.insert(agent.pid, agent);
    }

    pub fn remove_pf_agent(&self, username: &str) -> Option<PfAgent> {
        self.inner.pfmap.remove(username).map(|(_, v)| v)
    }

    pub fn remove_gate_agent(&self, pid: i64) -> Option<GateAgentProxy> {
        self.inner.gatemap.remove(&pid).map(|(_, v)| v)
    }

    pub fn remove_cold_agent(&self, pid: i64) -> Option<GateAgent> {
        self.inner.coldmap.remove(&pid).map(|(_, v)| v)
    }

    pub fn get_pf_agent(
        &self,
        username: &str,
    ) -> Option<dashmap::mapref::one::RefMut<'_, std::string::String, PfAgent>> {
        self.inner.pfmap.get_mut(username)
    }

    pub fn get_gate_agent(
        &self,
        pid: i64,
    ) -> Option<dashmap::mapref::one::RefMut<'_, i64, GateAgentProxy>> {
        self.inner.gatemap.get_mut(&pid)
    }

    pub async fn broadcase(&self, msg: cspb::Registry) {
        for pair in &self.inner.gatemap {
            pair.value().send(msg.clone()).await.unwrap()
        }
    }
}
