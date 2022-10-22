impl super::Model {
    pub fn new(player_id: u64) -> Self {
        Self {
            pid: player_id,
            basic: super::basic::SubModel {
                name: format!("player-{}", player_id),
                level: 1,
                exp: 0,
                gender: pb::Gender::Hidden,
            },
        }
    }
}
