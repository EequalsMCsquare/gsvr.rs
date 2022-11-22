pub mod api;
mod basic;

#[derive(Debug)]
pub struct Model {
    pub pid: u64,
    pub basic: basic::SubModel,
    pub friends: Vec<u64>,
}