use sqlx::postgres::PgQueryResult;

use crate::result::{DbResult, Error};

pub fn at_least_one(error: Error) -> impl FnOnce(PgQueryResult) -> DbResult<()> {
    move |result| match result.rows_affected() {
        0 => Err(error),
        1.. => Ok(()),
    }
}
