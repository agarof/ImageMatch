mod controllers;
mod response;
mod routes;

use std::net::SocketAddr;

use routes::routes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 6060));
    let pool = db::connect("postgre", "postgre", "0.0.0.0", 6300, "db")
        .await
        .expect("Failed to connect to database");

    println!("Starting server on {addr}");

    warp::serve(routes(pool)).run(addr).await;
}
