#[allow(unused)]
#[derive(Debug)]
pub struct State {
    pub(super) login_at: time::Instant,
    pub(super) recent_used_at: time::Instant,
    pub(super) last_req_at: time::Instant,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for State {
    fn default() -> Self {
        let now = time::Instant::now();
        Self {
            login_at: now,
            recent_used_at: now,
            last_req_at: now,
        }
    }
}
