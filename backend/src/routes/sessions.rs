use warp::{Filter, Rejection};

use db::Pool;

use crate::controllers;

pub fn router(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("sessions").and(create(pool.clone()).or(images(pool)))
}

pub fn create(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::sessions::create)
}

pub fn images(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::path("images")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::sessions::images)
}
