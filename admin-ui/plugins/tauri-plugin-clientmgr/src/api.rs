use std::sync::Arc;

use admin::{proto, History};
use serde::Serialize;
use tauri::State;
use tokio::sync::oneshot;
use tracing::instrument;

use crate::{
    error::{Error, Result},
    state::{ClientMgr, ClientProxy},
};

#[tauri::command]
#[instrument]
pub fn add_fclient(player_id: u64, mgr: State<'_, ClientMgr<u64>>) -> Result<()> {
    // check client unique
    if let Some(_) = mgr.clients.read().unwrap().get(&player_id) {
        return Err(Error::DupFClient(player_id));
    }
    let (close_tx, close_rx) = oneshot::channel();
    let client =
        admin::FastLoginClient::new(player_id, mgr.br_tx.subscribe(), mgr.tx.clone(), close_rx)?;
    let his = client.history();
    let h = tauri::async_runtime::spawn(async move { client.run().await });
    mgr.clients.write().unwrap().insert(
        player_id,
        ClientProxy {
            id: player_id,
            closechan: close_tx,
            join: h,
            history: his,
        },
    );
    return Ok(());
}

#[tauri::command]
#[instrument]
pub async fn drop_fclient(player_id: u64, mgr: State<'_, ClientMgr<u64>>) -> Result<()> {
    let handle: ClientProxy<u64>;
    {
        if let Some(h) = mgr.clients.write().unwrap().remove(&player_id) {
            handle = h;
        } else {
            return Err(Error::FClientNotFound(player_id));
        }
    }
    match handle.closechan.send(()) {
        Ok(_) => {
            if let Err(err) = handle.join.await {
                tracing::error!("fclient-{} join error. {}", player_id, err);
                return Err(Error::Tauri(err));
            }
            return Ok(());
        }
        Err(_) => {
            return Err(Error::Any(format!(
                "fail to send fclient-{} close signal",
                player_id
            )))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn fclient_history(player_id: u64, mgr: State<'_, ClientMgr<u64>>) -> Result<String> {
    if let Some(client) = mgr.clients.read().unwrap().get(&player_id) {
        let res = serde_json::to_string_pretty(client.history.as_ref())?;
        tracing::debug!("{}", res);
        return Ok(res);
    }
    return Err(Error::FClientNotFound(player_id));
}

#[tauri::command]
#[instrument]
pub async fn fclient_request(
    player_id: u64,
    content: &str,
    mgr: State<'_, ClientMgr<u64>>,
) -> Result<String> {
    let proto: pb::CsMsg = serde_json::from_str(content).map_err(Error::Serde)?;
    if let Some(caps) = mgr.pb_name_re.captures(content) {
        let pb_name = &caps["pbName"];
        tracing::debug!("fclient-{} send {:?}", player_id, proto);
        let msg = proto::TagCsMsg {
            msg: proto,
            to: Some(player_id),
        };
        mgr.br_tx.send(msg).map_err(|_| Error::CloseSend)?;
        return Ok(pb_name.into());
    }
    Err(Error::Any(format!(
        "invalid pb content {}. unable to capture pb name",
        content
    )))
}
