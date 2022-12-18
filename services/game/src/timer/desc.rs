use crate::hub::ModuleName;

use super::kind::TimerKind;

#[derive(Debug)]
pub(super) struct Description {
    pub from: ModuleName,
    pub data: TimerKind,
    pub repeat: bool
}
