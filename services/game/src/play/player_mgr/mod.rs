pub mod api;

use hashbrown::HashMap;

use super::player;
pub struct PlayerMgr {
    active_players: HashMap<u64, player::Model>,
}