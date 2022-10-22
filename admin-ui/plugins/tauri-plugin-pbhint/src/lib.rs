use hashbrown::HashMap;
use serde::{ser::Serializer, Serialize};
use strum::{IntoEnumIterator, VariantNames};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};
use tracing::instrument;

use std::ops::Deref;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no hint found for {0}")]
    NotRecord(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug)]
struct PbHintState {
    hint: HashMap<&'static str, String>,
    options: Vec<&'static str>,
}

impl PbHintState {
    pub fn new() -> Self {
        Self {
            hint: std::iter::zip(
                pb::CsMsg::VARIANTS.iter().map(Deref::deref),
                pb::CsMsg::iter().map(|f| serde_json::to_string_pretty(&f).unwrap()),
            )
            .collect(),
            options: pb::CsMsg::VARIANTS
                .iter()
                .filter(|&&x| x != "CsLogin" && x != "CsFastLogin")
                .map(Deref::deref)
                .collect(),
        }
    }
}

#[tauri::command]
#[instrument]
fn get_options<'a>(state: State<'a, PbHintState>) -> Vec<&'static str> {
    state.options.clone()
}

#[tauri::command]
#[instrument]
fn try_hint<'a>(state: State<'a, PbHintState>, key: &str) -> Result<String> {
    if let Some(val) = state.hint.get(key) {
        return Ok(val.clone());
    }
    Err(Error::NotRecord(key.to_string()))
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("pbhint")
        .invoke_handler(tauri::generate_handler![get_options, try_hint])
        .setup(|app| {
            app.manage(PbHintState::new());
            Ok(())
        })
        .build()
}
