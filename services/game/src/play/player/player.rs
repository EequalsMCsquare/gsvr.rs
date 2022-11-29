#[derive(Debug)]
pub struct Player {
    pub pid: i64,
    pub basic: super::basic::Basic,

    pub state: super::state::State,
}
