#[derive(Debug)]
pub enum Error {
    InvalidToken,
    InvalidUserId,
    DuplicateEmail,
    Sqlx(sqlx::Error),
}

pub type DbResult<T> = Result<T, Error>;

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sqlx(e)
    }
}
