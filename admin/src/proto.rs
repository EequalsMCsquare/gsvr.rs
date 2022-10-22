#[derive(Debug, Clone, serde::Serialize)]
pub struct TagScMsg<T>
where
    T: PartialEq + serde::Serialize,
{
    pub from: T,
    pub msg: pb::ScMsg,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TagCsMsg<T>
where
    T: PartialEq,
{
    pub to: Option<T>,
    pub msg: pb::CsMsg,
}
