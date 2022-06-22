use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    PgExecutor,
};

use crate::{
    result::{at_least_one, DbResult, Error},
    users,
};

#[derive(Clone, Copy)]
pub struct Token(pub Uuid);

pub async fn create<'a, E>(id: users::Id, db: E) -> DbResult<(Token, OffsetDateTime)>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into tokens(user_id)values($1)returning token,expiration";

    sqlx::query_as(QUERY)
        .bind(id.0)
        .fetch_one(db)
        .await
        .map(|(uuid, expiration)| (Token(uuid), expiration))
        .map_err(Error::Sqlx)
}

pub async fn auth<'a, E>(token: Token, db: E) -> DbResult<users::Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select user_id from tokens where token=$1 and expiration > CURRENT_TIMESTAMP";

    sqlx::query_as(QUERY)
        .bind(token.0)
        .fetch_one(db)
        .await
        .map(|(id,)| users::Id(id))
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::InvalidToken,
            e => Error::Sqlx(e),
        })
}

pub async fn auth_admin<'a, E>(token: Token, db: E) -> DbResult<users::Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select id from tokens inner join users on user_id = id where token = $1 and admin = true";

    sqlx::query_as(QUERY)
        .bind(token.0)
        .fetch_one(db)
        .await
        .map(|(id,)| users::Id(id))
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::InvalidToken,
            e => Error::Sqlx(e),
        })
}

pub async fn delete<'a, E>(token: Token, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "delete from tokens where token=$1";

    sqlx::query(QUERY)
        .bind(token.0)
        .execute(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(at_least_one(Error::InvalidToken))
}

pub async fn logout_user<'a, E>(id: users::Id, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "delete from tokens where user_id=$1";

    sqlx::query(QUERY)
        .bind(id.0)
        .execute(db)
        .await
        .map(|_| ())
        .map_err(Error::Sqlx)
}
