#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub (crate) enum Error {
    #[error("fail to decode Bytes to PB")]
    DecodeToPB,
    #[error("invalid pb payload")]
    PBPayload,
    #[error("unauthorized agent, first CsLogin or CsFastLogin first")]
    UnAuth,
    #[error("connection close")]
    ReadZero,
    #[error("nats subscribe error")]
    NatsSub
}