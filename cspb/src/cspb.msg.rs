#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsFastLogin {
    #[prost(int64, tag="1")]
    pub player_id: i64,
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
    #[prost(int64, tag="2")]
    pub player_id: i64,
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
