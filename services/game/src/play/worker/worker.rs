use crate::{
    hub::PMSG,
    play::{playermgr::PlayerMgr, proto::WProto},
};
use std::{cell::RefCell, sync::atomic::AtomicU32};

pub struct WorkerHandle {
    pub close_tx: crossbeam_channel::Sender<()>,
    pub wtx: crossbeam_channel::Sender<WProto>,
    pub worker: Worker,
}

pub struct Worker {
    pcount: AtomicU32, // currently handle player count
    players: RefCell<PlayerMgr>,
    rx: crossbeam_channel::Receiver<WProto>,
    close_rx: crossbeam_channel::Receiver<()>,
    ptx: crossbeam_channel::Sender<PMSG>,
}

impl Worker {
    pub fn new(
        rx: crossbeam_channel::Receiver<WProto>,
        ptx: crossbeam_channel::Sender<PMSG>,
        close_rx: crossbeam_channel::Receiver<()>
    ) -> Self {
        Self {
            pcount: 0.into(),
            players: Default::default(),
            rx,
            ptx,
            close_rx,
        }
    }

    pub fn sendp(&self, player_id: i64, message: cspb::Registry) {
        if let Err(err) = self.ptx.send(PMSG { player_id, message }) {
            tracing::error!("sendp error. {}", err)
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut ref_playermgr = self.players.borrow_mut();
        loop {
            crossbeam_channel::select! {
                recv(self.rx) -> msg => {
                    let msg = msg?;
                    match msg {
                        WProto::PMSG(pmsg) => {
                            let player = ref_playermgr.get_player(pmsg.player_id).unwrap();
                            self.handle_pcs(player, pmsg.message);
                        }
                        WProto::AddPlayer(p) => {
                            ref_playermgr.add_player(p).unwrap();
                            self.pcount
                                .fetch_add(1, std::sync::atomic::Ordering::Acquire);
                        }
                    }
                },
                recv(self.close_rx) -> _ => {
                    tracing::debug!("worker recv close signal");
                    return Ok(())
                }
            }
        }
    }
}
