use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use db::{images, Pool};
use futures::{StreamExt, TryStreamExt};
use log::error;
use serde::Serialize;
use tokio::io::AsyncWriteExt;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Buf, Filter, Rejection,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config();

    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&config.storage_path)
        .expect("Could not create storage directory");

    let pool = db::connect(
        &config.db_user,
        &config.db_pass,
        &config.db_host,
        config.db_port,
        &config.db_name,
    )
    .await
    .expect("Failed to connect to database");
    let routes =
        get_image(config.storage_path.clone()).or(add_images_route(config.storage_path, pool));

    println!("Starting server on {}", config.addr);

    warp::serve(routes).run(config.addr).await;
}

fn get_image(path: PathBuf) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::get().and(warp::fs::dir(path))
}

fn add_images_route(
    path: PathBuf,
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::filters::multipart::form().max_length(128_000_000))
        .map(move |m| (m, path.clone(), db.clone()))
        .untuple_one()
        .then(add_images)
}

#[derive(Serialize)]
struct AddResponse {
    ok: Vec<(usize, i32)>,
    errors: Vec<usize>,
}

async fn add_images(data: FormData, path: PathBuf, db: Pool) -> impl warp::Reply {
    let results: Vec<_> = data
        .map_err(|err| format!("add_images: FormData content error: {err}"))
        .and_then(|part| add_image(part, path.clone(), db.clone()))
        .map_err(|e| error!("Error while receiving image: {e}"))
        .enumerate()
        .collect()
        .await;

    let response = results.into_iter().fold(
        AddResponse {
            ok: Vec::new(),
            errors: Vec::new(),
        },
        |mut acc, (index, result)| {
            match result {
                Ok(id) => acc.ok.push((index, id.0)),
                Err(()) => acc.errors.push(index),
            };
            acc
        },
    );

    warp::reply::with_status(
        warp::reply::json(&response),
        if !response.ok.is_empty() {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        },
    )
}

async fn add_image(part: Part, mut path: PathBuf, db: Pool) -> Result<images::Id, String> {
    let mut trans = db
        .begin()
        .await
        .map_err(|e| format!("Failed to create transaction on database: {e:?}"))?;
    let id = images::create(&mut trans)
        .await
        .map_err(|e| format!("Failed to create image in database: {e:?}"))?;

    path.push(id.0.to_string());
    write_image(part, &path).await?;

    trans
        .commit()
        .await
        .map(|()| id)
        .map_err(|e| format!("Failed to commit transaction on database: {e:?}"))
}

async fn write_image(part: Part, path: &Path) -> Result<(), String> {
    let mut stream = part.stream().map_err(|e| format!("Part stream error: {e}"));
    let mut file = tokio::fs::File::create(path).await.map_err(|e| {
        format!(
            "Failed to open {} with write permissions: {e}",
            path.display()
        )
    })?;

    while let Some(buf) = stream.next().await {
        write_buf(buf?, &mut file).await?;
    }

    Ok(())
}

async fn write_buf(mut buf: impl Buf, file: &mut tokio::fs::File) -> Result<(), String> {
    let size = buf.remaining();
    let mut vec = vec![0; size];

    buf.copy_to_slice(&mut vec);
    file.write_all(&vec)
        .await
        .map_err(|e| format!("Failed to write Part buffer to file: {e}"))
}

fn config() -> Config {
    Config {
        addr: var_with_default("HOST", || SocketAddr::from(([0, 0, 0, 0], 3030))),
        db_port: var_with_default("DB_PORT", || 6300),
        db_host: var_with_default("DB_HOST", || String::from("0.0.0.0")),
        db_name: var_with_default("DB_NAME", || String::from("db")),
        db_user: var_with_default("DB_USER", || String::from("postgre")),
        db_pass: var_with_default("DB_PASS", || String::from("postgre")),
        storage_path: var_with_default("STORAGE_PATH", || PathBuf::from("./images")),
    }
}

struct Config {
    addr: SocketAddr,
    db_port: i16,
    db_host: String,
    db_name: String,
    db_user: String,
    db_pass: String,
    storage_path: PathBuf,
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
