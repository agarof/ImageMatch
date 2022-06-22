use serde::Deserialize;
use time::OffsetDateTime;

use db::{images, images_associations, result::Error, sessions, Pool};

use crate::response::{error, success, EmptyResponse};

#[derive(Deserialize)]
pub struct CreateModel {
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    phase1: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    phase2: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    phase3: OffsetDateTime,
}

pub async fn create(params: CreateModel, db: Pool) -> EmptyResponse {
    let result = sessions::create(
        &params.name,
        params.phase1,
        params.phase2,
        params.phase3,
        &db,
    )
    .await;

    match result {
        Ok(_) => success(()).with_status(success::Code::Created).into(),
        Err(err) => match err {
            Error::InvalidDates => error()
                .with_status(error::Code::BadRequest)
                .body(String::from("Invalid dates")),
            _ => error(),
        }
        .into(),
    }
}

#[derive(Deserialize)]
pub struct ImagesModel {
    image: i32,
    session: i32,
}

pub async fn images(params: ImagesModel, db: Pool) -> EmptyResponse {
    let result =
        images_associations::create(images::Id(params.image), sessions::Id(params.session), &db)
            .await;

    match result {
        Ok(_) => success(()).into(),
        Err(err) => match err {
            Error::InvalidSession => error()
                .with_status(error::Code::BadRequest)
                .body(String::from("Invalid session id")),
            Error::InvalidImage => error()
                .with_status(error::Code::BadRequest)
                .body(String::from("Invalid image id")),
            _ => error(),
        }
        .into(),
    }
}
