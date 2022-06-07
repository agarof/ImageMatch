//! Contains types to be extracted from a request and utility functions to extract them.

#[derive(Debug)]
pub struct InternalError {}

impl warp::reject::Reject for InternalError {}

pub mod auth {
    use std::str::FromStr;

    use db::{result::Error, tokens::Token, Pool};
    use warp::{Filter, Rejection};

    use super::InternalError;

    pub struct Auth {
        id: db::users::Id,
        token: Token,
    }

    pub struct AdminAuth {
        id: db::users::Id,
        token: Token,
    }

    impl Auth {
        pub fn id(&self) -> db::users::Id {
            self.id
        }

        pub fn token(&self) -> Token {
            self.token
        }
    }

    impl AdminAuth {
        pub fn id(&self) -> db::users::Id {
            self.id
        }

        pub fn token(&self) -> Token {
            self.token
        }
    }

    #[derive(Debug)]
    pub struct InvalidToken {}

    impl warp::reject::Reject for InvalidToken {}

    pub fn auth_filter(pool: Pool) -> impl Filter<Extract = (Auth,), Error = Rejection> + Clone {
        bearer_filter().and_then(move |token| {
            let pool = pool.clone();

            async move {
                match db::tokens::auth(token, &pool).await {
                    Ok(id) => Ok(Auth { id, token }),
                    Err(Error::InvalidToken) => Err(warp::reject::custom(InvalidToken {})),
                    Err(_) => Err(warp::reject::custom(InternalError {})),
                }
            }
        })
    }

    pub fn admin_auth_filter(
        pool: Pool,
    ) -> impl Filter<Extract = (AdminAuth,), Error = Rejection> + Clone {
        bearer_filter().and_then(move |token| {
            let pool = pool.clone();

            async move {
                match db::tokens::auth_admin(token, &pool).await {
                    Ok(id) => Ok(AdminAuth { id, token }),
                    Err(Error::InvalidToken) => Err(warp::reject::custom(InvalidToken {})),
                    Err(_) => Err(warp::reject::custom(InternalError {})),
                }
            }
        })
    }

    fn bearer_filter() -> impl Filter<Extract = (Token,), Error = warp::Rejection> + Clone {
        warp::header("Authorization").and_then(|auth: String| async move {
            let mut parts = auth.trim().split(' ');

            match (
                parts.next(),
                parts.next().map(FromStr::from_str),
                parts.next(),
            ) {
                (Some("Bearer"), Some(Ok(token)), None) => Ok(Token(token)),
                _ => Err(warp::reject::custom(InvalidToken {})),
            }
        })
    }
}
