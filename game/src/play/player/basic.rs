#[derive(Debug)]
pub struct SubModel{
    pub(super) name: String,
    pub(super) level: u32,
    pub(super) exp: u64,
    pub(super) gender: pb::Gender,
}