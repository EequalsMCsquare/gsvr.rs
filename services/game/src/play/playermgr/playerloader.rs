use std::sync::Arc;

use crate::play::{DBplayer, PLProtoReq, Player};
use crate::{
    db::DBProtoReq,
    hub::{GProto, ModuleName, PMSG},
};
use crossbeam_queue::SegQueue;
use dashmap::{DashMap, DashSet};
use fxhash::FxBuildHasher;
use gsfw::chanrpc::{CallTx, CastTx};
use sqlx::FromRow;

pub struct PlayerLoader {
    loading_set: Arc<DashSet<i64, FxBuildHasher>>,
    pending_map: Arc<DashMap<i64, SegQueue<PMSG>, FxBuildHasher>>,
    db_caller: Arc<CallTx<GProto, ModuleName, crate::Error>>,
    play_caller: Arc<CallTx<GProto, ModuleName, crate::Error>>,
    play_caster: Arc<CastTx<GProto, ModuleName, crate::Error>>,
    pcs_rx: crossbeam_channel::Receiver<PMSG>,
    close_rx: crossbeam_channel::Receiver<()>,
}

impl PlayerLoader {
    pub fn new(
        db_caller: Arc<CallTx<GProto, ModuleName, crate::Error>>,
        play_caller: Arc<CallTx<GProto, ModuleName, crate::Error>>,
        play_caster: Arc<CastTx<GProto, ModuleName, crate::Error>>,
        pcs_rx: crossbeam_channel::Receiver<PMSG>,
        close_rx: crossbeam_channel::Receiver<()>,
    ) -> Self {
        Self {
            loading_set: Arc::new(DashSet::with_capacity_and_hasher(
                512,
                FxBuildHasher::default(),
            )),
            pending_map: Arc::new(DashMap::with_capacity_and_hasher(
                1024,
                FxBuildHasher::default(),
            )),
            db_caller,
            play_caller,
            play_caster,
            pcs_rx,
            close_rx,
        }
    }

    pub fn run(self) -> anyhow::Result<()> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_io()
            .enable_time()
            .build()?;
        loop {
            crossbeam_channel::select! {
                recv(self.pcs_rx) -> ret => {
                    let start = time::Instant::now();
                    let cs = ret?;
                    // check if player already loaded
                    if self.loading_set.contains(&cs.player_id) {
                        self.play_caster.blocking_cast(GProto::PMSG(cs));
                        continue;
                    }
                    // if the loading is already pended, just push the new PCS to the queue
                    if let Some(p) = self.pending_map.get(&cs.player_id) {
                        tracing::debug!(
                            "player{} is still loading, add new pcs to the queue",
                            cs.player_id
                        );
                        p.push(cs);
                    } else {
                        // if new load request recv, create a new Pending with the incoming PCS,
                        // and then spawn a new coroutine to handle load response
                        tracing::debug!("try to load player{} from database", cs.player_id);
                        let player_id = cs.player_id;
                        let new_pcsq = SegQueue::new();
                        new_pcsq.push(cs);
                        self.pending_map.insert(player_id, new_pcsq);

                        let db_caller = self.db_caller.clone();
                        let play_caster = self.play_caster.clone();
                        let play_caller = self.play_caller.clone();
                        let loading_set = self.loading_set.clone();
                        let pending_map = self.pending_map.clone();

                        let fut = async move {
                            let query = DBplayer::query_find_one(player_id);
                            let ack_future = db_caller
                                .call(GProto::DBProtoReq(DBProtoReq::Find {
                                    kind: crate::db::FindKind::Option,
                                    query,
                                }))
                                .await;
                            // wait for response from DB component
                            match ack_future.await {
                                Ok(Ok(ret)) => match ret {
                                    GProto::DBProtoAck(inner) => {
                                        match inner {
                                            crate::db::DBProtoAck::OptRow(dbres) => {
                                                let player = match dbres {
                                                    Ok(Some(row)) => DBplayer::from_row(&row)
                                                        .expect("unable to build DBPlayer from PgRow")
                                                        .into(),
                                                    Ok(None) => Player::new(player_id),
                                                    Err(err) => {
                                                        tracing::error!(
                                                            "fail to load player from database. {}",
                                                            err
                                                        );
                                                        return;
                                                    }
                                                };
                                                // ack Play module to add player
                                                let pid = player.pid;
                                                let ack = play_caller
                                                    .call(GProto::PLProtoReq(PLProtoReq::AddPlayer(player)))
                                                    .await;
                                                if let Err(err) = ack.await.unwrap() {
                                                    tracing::error!("add player fail. {}", err);
                                                } else {
                                                    // mark player loaded successfully
                                                    loading_set.insert(player_id);
                                                    // remove player from pending map
                                                    let (_, pcs_queue) = pending_map.remove(&player_id).unwrap();
                                                    for pmsg in pcs_queue {
                                                        play_caster.cast(GProto::PMSG(pmsg)).await;
                                                    }
                                                    let end = time::Instant::now();
                                                    tracing::debug!("player-{} loaded. time used: {}", pid, end-start);
                                                    // delay removing player loaded mark
                                                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                                    loading_set.remove(&player_id);
                                                }
                                            }
                                            _unexpected => todo!(),
                                        }
                                    }
                                    _unexpected => tracing::error!(
                                        "unexpected GProto: {}",
                                        Into::<&'static str>::into(_unexpected)
                                    ),
                                },
                                Ok(Err(err)) => tracing::error!("crate::Error: {}", err),
                                Err(err) => tracing::error!("oneshot error: {}", err),
                            };
                        };
                        // spawn coroutine to handle loading
                        rt.spawn(fut);
                    }
                },
                recv(self.close_rx) -> _ => return Ok(())
            }
        }
    }
}
