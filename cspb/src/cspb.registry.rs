#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsProto {
    #[prost(oneof="cs_proto::Payload", tags="1, 2, 3")]
    pub payload: ::core::option::Option<cs_proto::Payload>,
}
/// Nested message and enum types in `CsProto`.
pub mod cs_proto {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[derive(::strum::EnumIter, ::strum::EnumVariantNames)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="1")]
        CsLogin(super::super::msg::CsLogin),
        #[prost(message, tag="2")]
        CsEcho(super::super::msg::CsEcho),
        #[prost(message, tag="3")]
        CsFastLogin(super::super::msg::CsFastLogin),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScProto {
    #[prost(oneof="sc_proto::Payload", tags="6000, 6001, 3")]
    pub payload: ::core::option::Option<sc_proto::Payload>,
}
/// Nested message and enum types in `ScProto`.
pub mod sc_proto {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[derive(::strum::EnumIter, ::strum::EnumVariantNames)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="6000")]
        ScLogin(super::super::msg::ScLogin),
        #[prost(message, tag="6001")]
        ScEcho(super::super::msg::ScEcho),
        #[prost(message, tag="3")]
        ScFastLogin(super::super::msg::ScFastLogin),
    }
}
