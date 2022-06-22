use sqlx::PgExecutor;

use crate::result::{DbResult, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub i32);

pub async fn create<'a, E>(db: E) -> DbResult<Id>
where
    E: PgExecutor<'a>,
{
    const QUERY: &str = "insert into images values(default)returning id";

    sqlx::query_as(QUERY)
        .fetch_one(db)
        .await
        .map(|(id,)| Id(id))
        .map_err(Error::Sqlx)
}
