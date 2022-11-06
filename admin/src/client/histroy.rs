use parking_lot::RwLock;
use serde::{
    ser::{SerializeStruct},
    Serialize,
};
use slice_deque::SliceDeque;

#[derive(Debug)]
pub struct History {
    pub cs: RwLock<SliceDeque<pb::CsMsg>>,
    pub sc: RwLock<SliceDeque<pb::ScMsg>>,
}
impl Serialize for History {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("history", 2)?;
        s.serialize_field("cs", self.cs.read().as_slice())?;
        s.serialize_field("sc", self.sc.read().as_slice())?;
        s.end()
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            cs: RwLock::new(SliceDeque::with_capacity(512)),
            sc: RwLock::new(SliceDeque::with_capacity(512)),
        }
    }
}
