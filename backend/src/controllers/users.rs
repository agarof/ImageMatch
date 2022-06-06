use log::*;
use serde::Deserialize;

use db::{result::Error, users, Pool};

use crate::response::{error, success, EmptyResponse};

#[derive(Deserialize)]
pub struct CreateModel {
    email: String,
    password: String,
}

pub async fn create(data: CreateModel, db: Pool) -> EmptyResponse {
    match users::create(&data.email, &data.password, &db).await {
        Ok(_) => success(()).into(),
        Err(err) => {
            error!("{err:?}");

            match err {
                Error::DuplicateEmail => error()
                    .body("Email already in use".to_string())
                    .with_status(error::Code::Conflict),
                _ => error(),
            }
        }
        .into(),
    }
}

pub async fn confirm(id: users::Id, db: Pool) -> EmptyResponse {
    match users::confirm(id, &db).await {
        Ok(()) => success(()).into(),
        Err(err) => {
            error!("{err:?}");

            match err {
                db::result::Error::InvalidUserId => error().with_status(error::Code::NotFound),
                _ => error(),
            }
            .into()
        }
    }
}
