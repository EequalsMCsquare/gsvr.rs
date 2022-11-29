use gsfw::util::dirty::DirtyMark;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, gsfw::util::Dirty)]
pub struct Basic {
    pub(super) name: String,
    pub(super) level: u32,
    pub(super) exp: u64,
    pub(super) gender: cspb::Gender,

    #[serde(skip)]
    #[dirty]
    __dirty__: DirtyMark,
}

impl Basic {
    pub fn new(player_id: i64) -> Self {
        Self {
            name: format!("player-{}", player_id),
            level: 1,
            exp: 0,
            gender: cspb::Gender::Hidden,
            __dirty__: Default::default(),
        }
    }
}
