use hashbrown::HashMap;

mod api;
use super::item::Item;

#[derive(Debug, Default)]
pub struct ItemMgr(HashMap<u32, Item>);
