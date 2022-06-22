mod controllers;
mod extractors;
mod response;
mod routes;

use std::{net::SocketAddr, str::FromStr};

use routes::routes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config();
    let pool = db::connect(
        &config.db_user,
        &config.db_pass,
        &config.db_host,
        config.db_port,
        &config.db_name,
    )
    .await
    .expect("Failed to connect to database");

    println!("Starting server on {}", config.addr);

    warp::serve(routes(pool)).run(config.addr).await;
}

fn config() -> Config {
    Config {
        addr: var_with_default("HOST", || SocketAddr::from(([0, 0, 0, 0], 6060))),
        db_port: var_with_default("DB_PORT", || 6300),
        db_host: var_with_default("DB_HOST", || String::from("0.0.0.0")),
        db_name: var_with_default("DB_NAME", || String::from("db")),
        db_user: var_with_default("DB_USER", || String::from("postgre")),
        db_pass: var_with_default("DB_PASS", || String::from("postgre")),
    }
}

struct Config {
    addr: SocketAddr,
    db_port: i16,
    db_host: String,
    db_name: String,
    db_user: String,
    db_pass: String,
}

fn var_with_default<T, F>(var: &str, default: F) -> T
where
    T: FromStr,
    F: FnOnce() -> T,
{
    std::env::var(var)
        .map(|var| {
            var.parse()
                .unwrap_or_else(|_| panic!("Invalid value for {var}"))
        })
        .unwrap_or_else(|e| match e {
            std::env::VarError::NotPresent => default(),
            std::env::VarError::NotUnicode(_) => panic!("Invalid unicode in {var}"),
        })
}
