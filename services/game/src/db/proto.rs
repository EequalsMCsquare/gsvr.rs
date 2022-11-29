use sqlx::{
    postgres::{PgArguments, PgQueryResult, PgRow},
    query::Query,
    Postgres,
};

pub enum FindKind {
    One,
    All,
    Option,
}

pub enum DBProtoReq {
    Find {
        kind: FindKind,
        query: Query<'static, Postgres, PgArguments>,
    },
    Exec(Query<'static, Postgres, PgArguments>),
}

pub enum DBProtoAck {
    OneRow(sqlx::Result<PgRow>),
    AllRow(sqlx::Result<Vec<PgRow>>),
    OptRow(sqlx::Result<Option<PgRow>>),
    Exec(sqlx::Result<PgQueryResult>),
}
