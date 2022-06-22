use log::error;
use sqlx::postgres::{PgDatabaseError, PgQueryResult};

#[derive(Debug)]
pub enum Error {
    InvalidToken,
    InvalidUserId,
    InvalidCredentials,
    InvalidImageAssociation,
    InvalidImage,
    InvalidSession,
    InvalidDates,
    DuplicateEmail,
    UnknownForeignKey,
    Sqlx(sqlx::Error),
}

pub type DbResult<T> = Result<T, Error>;

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sqlx(e)
    }
}

pub fn at_least_one(error: Error) -> impl FnOnce(PgQueryResult) -> DbResult<()> {
    move |result| match result.rows_affected() {
        0 => Err(error),
        1.. => Ok(()),
    }
}

pub type ErrorMatch<'a> = (&'a str, fn(&PgDatabaseError) -> Error);

pub mod codes {
    pub const FOREIGN_KEY: &str = "23503";
    pub const CHECK: &str = "23514";
}

pub fn code_to_error<'a>(matches: &'a [ErrorMatch<'a>]) -> impl 'a + FnOnce(sqlx::Error) -> Error {
    |error| {
        error
            .as_database_error()
            .and_then(|e| e.try_downcast_ref())
            .and_then(|e: &PgDatabaseError| {
                let code = e.code();

                matches
                    .iter()
                    .find(|(exp_code, _)| *exp_code == code)
                    .map(|(_, f)| f(e))
            })
            .unwrap_or(Error::Sqlx(error))
    }
}

pub const FOREIGN_KEYS_HANDLER: ErrorMatch = (codes::FOREIGN_KEY, foreign_key_handler);

fn foreign_key_handler(error: &PgDatabaseError) -> Error {
    type Association<'a> = (&'a str, fn() -> Error);
    const FOREIGN_KEYS: [Association; 2] = [
        ("session_id_fkey", || Error::InvalidSession),
        ("image_id_fkey", || Error::InvalidImage),
    ];

    error
        .constraint()
        .and_then(|constraint| {
            FOREIGN_KEYS
                .iter()
                .find(|(key, _)| constraint.ends_with(key))
                .or_else(|| {
                    error!("Non handled foreign key error: {constraint}");
                    None
                })
        })
        .map(|(_, f)| f())
        .unwrap_or_else(|| Error::UnknownForeignKey)
}
