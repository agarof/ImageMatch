use sqlx::{postgres::PgRow, PgExecutor, Row};

use crate::{
    result::{DbResult, Error},
    utils::at_least_one,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

#[derive(Debug)]
pub struct Candidate {
    pub id: Id,
    pub email: String,
}

const LIST_CANDIDATES_QUERY: &str = "select id,email from users where confirm_limit is not null";

impl<'r> sqlx::FromRow<'r, PgRow> for Candidate {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id(row.try_get(0)?),
            email: row.try_get(1)?,
        })
    }
}

pub async fn create<'a, E>(email: &str, password: &str, db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "insert into users(email,password)values($1,crypt($2, gen_salt('bf')))returning id";

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
        .and_then(at_least_one(Error::InvalidUserId))
}

pub async fn list_candidates<'a, E>(db: E) -> DbResult<Vec<Candidate>>
where
    E: PgExecutor<'a>,
{
    sqlx::query_as(LIST_CANDIDATES_QUERY)
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}

pub async fn find_by_credentials<'a, E>(email: &str, password: &str, db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select id from users where confirm_limit is null and email=$1 and password=crypt($2,password)";

    sqlx::query_as(QUERY)
        .bind(email)
        .bind(password)
        .fetch_optional(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(|opt| opt.map(|(id,)| Id(id)).ok_or(Error::InvalidCredentials))
}

pub async fn set_admin<'a, E>(id: Id, admin: bool, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "update users set admin=$1 where id=$2";

    sqlx::query(QUERY)
        .bind(admin)
        .bind(id.0)
        .execute(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(at_least_one(Error::InvalidUserId))
}
