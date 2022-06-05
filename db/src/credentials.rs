use sqlx::{types::Uuid, PgExecutor};

use crate::{
    result::{DbResult, Error},
    users,
};

pub async fn create<'a, E>(id: users::Id, db: E) -> DbResult<Uuid>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into credentials(user_id)values($1)returning token";

    sqlx::query_as(QUERY)
        .bind(id.0)
        .fetch_one(db)
        .await
        .map(|(uuid,)| uuid)
        .map_err(Error::Sqlx)
}

pub async fn auth<'a, E>(uuid: Uuid, db: E) -> DbResult<users::Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select user_id from credentials where token=$1 and expiration > CURRENT_TIMESTAMP";

    sqlx::query_as(QUERY)
        .bind(uuid)
        .fetch_one(db)
        .await
        .map(|(id,)| users::Id(id))
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::InvalidToken,
            e => Error::Sqlx(e),
        })
}

pub async fn delete<'a, E>(uuid: Uuid, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "delete from credentials where token=$1";

    sqlx::query(QUERY)
        .bind(uuid)
        .execute(db)
        .await
        .map(|_| ())
        .map_err(Error::Sqlx)
}

pub async fn logout_user<'a, E>(id: users::Id, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "delete from credentials where user_id=$1";

    sqlx::query(QUERY)
        .bind(id.0)
        .execute(db)
        .await
        .map(|_| ())
        .map_err(Error::Sqlx)
}
