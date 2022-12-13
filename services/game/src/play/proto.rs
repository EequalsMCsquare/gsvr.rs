use crate::hub::PMSG;

use super::player::Player;

// Worker Proto, used for communication between Play component and their workers
pub enum WProto {
    PMSG(PMSG),
    AddPlayer(Player),
}

pub enum PLProtoReq {
    AddPlayer(Player),
}

pub enum PLProtoAck {

}
