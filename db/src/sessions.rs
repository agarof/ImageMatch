use sqlx::{postgres::PgRow, types::time::OffsetDateTime, PgExecutor, Row};

use crate::{
    filters::DateFilter,
    result::{code_to_error, codes, DbResult, Error},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

pub struct Session {
    pub id: Id,
    pub name: String,
    pub phase1: OffsetDateTime,
    pub phase2: OffsetDateTime,
    pub phase3: OffsetDateTime,
}

pub enum State {}

const LIST_QUERY: &str = "select id,phase1,phase2,phase3 from sessions";

impl<'r> sqlx::FromRow<'r, PgRow> for Session {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id(row.try_get(0)?),
            name: row.try_get(1)?,
            phase1: row.try_get(2)?,
            phase2: row.try_get(3)?,
            phase3: row.try_get(4)?,
        })
    }
}

pub async fn create<'a, E>(
    name: &str,
    phase1: OffsetDateTime,
    phase2: OffsetDateTime,
    phase3: OffsetDateTime,
    db: E,
) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "insert into sessions(name,phase1,phase2,phase3)values($1,$2,$3,$4)returning id";

    sqlx::query_as(QUERY)
        .bind(name)
        .bind(phase1)
        .bind(phase2)
        .bind(phase3)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(code_to_error(&[(codes::CHECK, |_| Error::InvalidDates)]))
}

pub async fn list<'a, E>(
    phase1: DateFilter,
    phase2: DateFilter,
    phase3: DateFilter,
    db: E,
) -> DbResult<Vec<Session>>
where
    E: PgExecutor<'a>,
{
    sqlx::query_as(LIST_QUERY)
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}
