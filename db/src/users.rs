use sqlx::{postgres::PgRow, PgExecutor, Row};

use crate::result::{DbResult, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

#[derive(Debug)]
pub struct Candidate {
    pub id: Id,
    pub email: String,
    pub password: String,
}

const LIST_CANDIDATES_QUERY: &str = "select id,email,password from users";

impl<'r> sqlx::FromRow<'r, PgRow> for Candidate {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id(row.try_get(0)?),
            email: row.try_get(1)?,
            password: row.try_get(2)?,
        })
    }
}

pub async fn create<'a, E>(email: &str, password: &str, db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into users(email,password)values($1,$2)returning id";

    sqlx::query_as(QUERY)
        .bind(email)
        .bind(password)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(|e| {
            if e.as_database_error()
                .and_then(|e| e.code())
                .map(|code| code == "23505")
                .unwrap_or(false)
            {
                Error::DuplicateEmail
            } else {
                Error::Sqlx(e)
            }
        })
}

pub async fn confirm<'a, E>(id: Id, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "update users set confirm_limit=null where id=$1";

    sqlx::query(QUERY)
        .bind(id.0)
        .execute(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(|result| match result.rows_affected() {
            0 => Err(Error::InvalidUserId),
            1.. => Ok(()),
        })
}

pub async fn list_candidates<'a, E>(pool: E) -> DbResult<Vec<Candidate>>
where
    E: PgExecutor<'a>,
{
    sqlx::query_as(LIST_CANDIDATES_QUERY)
        .fetch_all(pool)
        .await
        .map_err(Error::Sqlx)
}
