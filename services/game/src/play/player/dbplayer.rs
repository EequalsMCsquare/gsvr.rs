use sqlx::{postgres::PgArguments, query::Query, types::Json, FromRow, Postgres};

use super::basic::Basic;

#[derive(FromRow)]
pub struct DBplayer {
    pub id: i64,
    pub basic: Json<Basic>,
}

impl DBplayer {
    pub fn query_find_one(id: i64) -> Query<'static, Postgres, PgArguments> {
        sqlx::query(
            r#"
            SELECT id, basic FROM public.players WHERE public.players.id = $1;
        "#,
        )
        .bind(id)
    }
}

impl Into<super::Player> for DBplayer {
    fn into(self) -> super::Player {
        super::Player {
            pid: self.id,
            basic: self.basic.0,
            state: Default::default(),
        }
    }
}
