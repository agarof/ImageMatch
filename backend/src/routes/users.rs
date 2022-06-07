use warp::{Filter, Rejection};

use db::{users, Pool};

use crate::{controllers, extractors};

pub fn router(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("users").and(
        create(pool.clone())
            .or(confirm(pool.clone()))
            .or(login(pool.clone()))
            .or(logout(pool.clone()))
            .or(candidates(pool)),
    )
}

pub fn create(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::users::create)
}

pub fn confirm(
    pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let auth_pool = pool.clone();

    warp::path("confirm")
        .and(warp::path::param().map(users::Id))
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::any().map(move || pool.clone()))
        .and(extractors::auth::admin_auth_filter(auth_pool))
        .then(controllers::users::confirm)
}

pub fn login(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::users::login)
}

pub fn logout(pool: Pool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::path("logout")
        .and(warp::path::end())
        .and(warp::post())
        .and(extractors::auth::auth_filter(pool.clone()))
        .and(warp::any().map(move || pool.clone()))
        .then(controllers::users::logout)
}

pub fn candidates(
    pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let auth_pool = pool.clone();

    warp::path("candidates")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::any().map(move || pool.clone()))
        .and(extractors::auth::admin_auth_filter(auth_pool))
        .then(controllers::users::candidates)
}
