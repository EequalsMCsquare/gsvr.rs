mod basic;
pub mod api;

#[derive(Debug)]
pub struct Model {
    pub pid: u64,
    pub basic: basic::SubModel,
}
