#[derive(::gsfw::Registry)]
#[registry(prefix="super::msg::",rename="Registry")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgId {
    #[registry(skip)]
    Reserved = 0,
    CsPing = 1,
    ScPing = 2,
    CsLogin = 101,
    ScLogin = 102,
    CsFastLogin = 103,
    ScFastLogin = 104,
    CsEcho = 201,
    ScEcho = 202,
}
impl MsgId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgId::Reserved => "__RESERVED",
            MsgId::CsPing => "CsPing",
            MsgId::ScPing => "ScPing",
            MsgId::CsLogin => "CsLogin",
            MsgId::ScLogin => "ScLogin",
            MsgId::CsFastLogin => "CsFastLogin",
            MsgId::ScFastLogin => "ScFastLogin",
            MsgId::CsEcho => "CsEcho",
            MsgId::ScEcho => "ScEcho",
        }
    }
}
