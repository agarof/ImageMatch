use warp::Filter;

mod users;

pub fn routes(pool: db::Pool) -> impl Filter<Extract = impl warp::Reply> + Clone {
    users::router(pool)
}
