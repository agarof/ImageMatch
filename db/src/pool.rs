use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::result::{DbResult, Error};

pub type Pool = PgPool;

pub async fn connect(
    user: &str,
    password: &str,
    host: &str,
    port: u32,
    database: &str,
) -> DbResult<Pool> {
    let url = format!("postgresql://{user}:{password}@{host}:{port}/{database}");

    PgPoolOptions::new()
        .connect(&url)
        .await
        .map_err(Error::Sqlx)
}
