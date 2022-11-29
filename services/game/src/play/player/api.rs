impl super::Player {
    pub fn new(player_id: i64) -> Self {
        Self {
            pid: player_id,
            basic: super::basic::Basic::new(player_id),
            state: super::state::State::new(),
        }
    }
}
