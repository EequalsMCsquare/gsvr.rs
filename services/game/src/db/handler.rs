use sqlx::{postgres::PgArguments, query::Query, Postgres};

use super::{DBComponent, DBProtoAck, FindKind};
impl DBComponent {
    pub async fn ctl_find(
        database: &sqlx::PgPool,
        kind: FindKind,
        query: Query<'_, Postgres, PgArguments>,
    ) -> DBProtoAck {
        match kind {
            FindKind::One => DBProtoAck::OneRow(query.fetch_one(database).await),
            FindKind::All => DBProtoAck::AllRow(query.fetch_all(database).await),
            FindKind::Option => DBProtoAck::OptRow(query.fetch_optional(database).await),
        }
    }

    pub async fn ctl_exec(
        database: &sqlx::PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> DBProtoAck {
        DBProtoAck::Exec(query.execute(database).await)
    }
}
