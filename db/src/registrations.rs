use sqlx::PgExecutor;

use crate::{
    result::{DbResult, Error},
    sessions, users,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

pub async fn create<'a, E>(user: users::Id, session: sessions::Id, db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into registrations(user_id,session_id)values($1,$2)returning id";

    sqlx::query_as(QUERY)
        .bind(user.0)
        .bind(session.0)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(Error::Sqlx)
}

pub async fn by_user<'a, E>(user: users::Id, db: E) -> DbResult<Vec<sessions::Session>>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select s.id,s.phase1,s.phase2,s.phase3 from registrations r inner join sessions s on s.id=r.session_id where r.user_id=$1";

    sqlx::query_as(QUERY)
        .bind(user.0)
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}

pub async fn by_session<'a, E>(session: sessions::Id, db: E) -> DbResult<Vec<users::Summary>>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "select u.id,u.email from registrations r inner join users u on u.id=r.user_id where r.session_id=$1";

    sqlx::query_as(QUERY)
        .bind(session.0)
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}
