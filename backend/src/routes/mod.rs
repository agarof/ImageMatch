use log::error;
use warp::{
    filters::body::BodyDeserializeError, hyper::StatusCode, reply, Filter, Rejection, Reply,
};

use crate::extractors::{auth::InvalidToken, InternalError};

mod sessions;
mod users;

pub fn routes(pool: db::Pool) -> impl Filter<Extract = impl warp::Reply> + Clone {
    users::router(pool.clone())
        .or(sessions::router(pool))
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    error!("{err:?}");

    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if err.find::<InvalidToken>().is_some() {
        Ok(reply::with_status("Invalid Token", StatusCode::FORBIDDEN))
    } else if err.find::<BodyDeserializeError>().is_some() {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else if err.find::<InternalError>().is_some() {
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
