use warp::{Filter, Rejection};

use db::{users, Pool};

use crate::controllers;

pub fn router(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("users").and(create(pool.clone()).or(confirm(pool)))
}

pub fn create(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::users::create)
}

pub fn confirm(
    pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::path::param().map(users::Id))
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::users::confirm)
}
