use hashbrown::HashMap;

use crate::play::player::Player;

#[derive(Default)]
pub struct PlayerMgr {
    pub(super) active_players: HashMap<i64, Player>,
}