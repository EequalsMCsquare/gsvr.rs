#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsFastLogin {
    #[prost(uint64, tag="1")]
    pub player_id: u64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScFastLogin {
    #[prost(enumeration="super::r#enum::ErrCode", tag="1")]
    pub err_code: i32,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsLogin {
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub player_id: i32,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScLogin {
    #[prost(enumeration="super::r#enum::ErrCode", tag="1")]
    pub err_code: i32,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsEcho {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScEcho {
    #[prost(string, tag="1")]
    pub reply: ::prost::alloc::string::String,
}
