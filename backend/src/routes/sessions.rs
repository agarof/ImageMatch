use warp::{Filter, Rejection};

use db::Pool;

use crate::{controllers, extractors};

pub fn router(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("sessions").and(create(pool.clone()).or(images(pool.clone())).or(list(pool)))
}

pub fn create(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let auth_pool = pool.clone();

    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and(extractors::auth::admin_auth_filter(auth_pool))
        .then(controllers::sessions::create)
}

pub fn images(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let auth_pool = pool.clone();

    warp::path("images")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and(extractors::auth::admin_auth_filter(auth_pool))
        .then(controllers::sessions::images)
}

pub fn list(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let auth_pool = pool.clone();

    warp::path::end()
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and(extractors::auth::auth_filter(auth_pool))
        .then(controllers::sessions::list)
}
