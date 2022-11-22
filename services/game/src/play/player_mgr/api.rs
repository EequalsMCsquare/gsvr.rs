use anyhow::anyhow;

use crate::play::player;

use super::PlayerMgr;

impl PlayerMgr {
    pub fn new() -> Self {
        Self {
            active_players: Default::default(),
        }
    }

    pub fn get_player(&self, player_id: u64) -> Option<&player::Model> {
        if let Some(player_ref) = self.active_players.get(&player_id) {
            return Some(player_ref);
        }
        return None;
        // TODO: load from database
        todo!()
    }

    pub fn add_player(&mut self, player: player::Model) -> anyhow::Result<&mut player::Model> {
        self.active_players
            .try_insert(player.pid, player)
            .map_err(|err| anyhow!("{:?}", err))
    }
}
