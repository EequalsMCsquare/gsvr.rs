use std::collections::HashMap;

use gsfw::RegistryExt;
use serde::Serialize;

#[derive(Serialize)]
pub struct FrontNamePayloadPair {
    name: &'static str,
    payload: String,
}

#[derive(Serialize)]
pub struct FrontHint<'a> {
    pub pbnames: &'a [&'static str],
    pub pbids: &'a [i32],
    pub pairs: &'a [FrontNamePayloadPair],
}

pub struct Hint {
    pub name2payload: HashMap<&'static str, String>,
    pub pair: Vec<FrontNamePayloadPair>,
}

impl Hint {
    pub fn new() -> Self {
        Self {
            name2payload: cspb::Registry::NAME_MAP
                .iter()
                .map(|(&k, v)| (k, serde_json::to_string_pretty(v).unwrap()))
                .collect(),
            pair: cspb::Registry::NAME_MAP
                .iter()
                .map(|(&k, v)| {
                    let payload = serde_json::to_string_pretty(v).unwrap();
                    let payload = payload.split_once(':').unwrap().1.trim_end_matches('}');
                    FrontNamePayloadPair {
                        name: k,
                        payload: payload.into(),
                    }
                })
                .collect(),
        }
    }
}
