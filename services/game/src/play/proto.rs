use crate::hub::PCS;

use super::player::Player;

// Worker Proto, used for communication between Play component and their workers
pub enum WProto {
    PCS(Box<PCS>),
    AddPlayer(Player),
}

pub enum PLProtoReq {
    AddPlayer(Player),
}

pub enum PLProtoAck {

}
