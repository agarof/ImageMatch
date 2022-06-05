use sqlx::{postgres::PgRow, types::time::OffsetDateTime, PgExecutor, Row};

use crate::result::{DbResult, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

pub struct Session {
    id: Id,
    start: OffsetDateTime,
    end: OffsetDateTime,
}

pub enum State {}

const LIST_QUERY: &str = "select id,start,end from sessions";

impl<'r> sqlx::FromRow<'r, PgRow> for Session {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id(row.try_get(0)?),
            start: row.try_get(1)?,
            end: row.try_get(2)?,
        })
    }
}

pub async fn create<'a, E>(
    name: &str,
    start: OffsetDateTime,
    end: OffsetDateTime,
    db: E,
) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into sessions(name,start,end)values($1,$2,$3)returning id";

    sqlx::query_as(QUERY)
        .bind(name)
        .bind(start)
        .bind(end)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(Error::Sqlx)
}

pub async fn list<'a, E>(db: E) -> DbResult<Vec<Session>>
where
    E: PgExecutor<'a>,
{
    sqlx::query_as(LIST_QUERY)
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}
