use sqlx::{postgres::PgRow, PgExecutor, Row};

use crate::{
    images,
    result::{at_least_one, code_to_error, DbResult, Error, FOREIGN_KEYS_HANDLER},
    sessions,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

pub async fn create<'a, E>(image: images::Id, session: sessions::Id, db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "insert into images_associations(image_id,session_id)values($1,$2)returning id";

    sqlx::query_as(QUERY)
        .bind(image.0)
        .bind(session.0)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(code_to_error(&[FOREIGN_KEYS_HANDLER]))
}

pub async fn delete<'a, E>(id: Id, db: E) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "delete from images_associations where id=$1 and (select phase1 from sessions where id=images_associations.session_id) > CURRENT_TIMESTAMP";

    sqlx::query(QUERY)
        .bind(id.0)
        .execute(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(at_least_one(Error::InvalidImageAssociation))
}

pub async fn delete_by_session<'a, E>(
    image: images::Id,
    session: sessions::Id,
    db: E,
) -> DbResult<()>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str =
        "delete from images_associations where image_id=$1 and session_id=$2 and (select phase1 from sessions where id=images_associations.session_id) > CURRENT_TIMESTAMP";

    sqlx::query(QUERY)
        .bind(image.0)
        .bind(session.0)
        .execute(db)
        .await
        .map_err(Error::Sqlx)
        .and_then(at_least_one(Error::InvalidImageAssociation))
}

pub async fn by_session<'a, E>(session: sessions::Id, db: E) -> DbResult<Vec<images::Id>>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "select image_id from images_associations where session_id=$1";

    sqlx::query(QUERY)
        .bind(session.0)
        .try_map(|row: PgRow| Ok(images::Id(row.try_get(0)?)))
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}

pub async fn by_image<'a, E>(image: images::Id, db: E) -> DbResult<Vec<sessions::Id>>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "select session_id from images_associations where image_id=$1";

    sqlx::query(QUERY)
        .bind(image.0)
        .try_map(|row: PgRow| Ok(sessions::Id(row.try_get(0)?)))
        .fetch_all(db)
        .await
        .map_err(Error::Sqlx)
}
