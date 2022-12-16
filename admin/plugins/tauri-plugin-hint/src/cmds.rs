use gsfw::RegistryExt;
use tauri::{Runtime, State, Window};
use tracing::instrument;

use crate::hint::{FrontHint, Hint};

#[tauri::command]
#[instrument(skip(hint))]
pub fn get_hint<R: Runtime>(_window: Window<R>, hint: State<Hint>) -> Result<String, String> {
    let fhint = FrontHint {
        pbnames: &cspb::Registry::NAMES[..],
        pbids: &cspb::Registry::IDS[..],
        pairs: &hint.pair,
    };
    serde_json::to_string(&fhint).map_err(|err| err.to_string())
}
