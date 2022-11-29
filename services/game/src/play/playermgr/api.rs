use anyhow::anyhow;

use crate::play::player;

use super::PlayerMgr;

impl PlayerMgr {
    pub fn get_player(&mut self, player_id: i64) -> Option<&mut player::Player> {
        if let Some(player_ref) = self.active_players.get_mut(&player_id) {
            return Some(player_ref);
        }
        return None;
    }

    pub fn add_player(&mut self, player: player::Player) -> anyhow::Result<&mut player::Player> {
        self.active_players
            .try_insert(player.pid, player)
            .map_err(|err| anyhow!("{:?}", err))
    }
}
