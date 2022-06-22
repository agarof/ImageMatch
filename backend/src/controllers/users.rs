use futures::TryFutureExt;
use log::*;
use serde::{Deserialize, Serialize};

use db::{
    result::Error,
    tokens,
    users::{self, Summary},
    Pool,
};
use time::OffsetDateTime;

use crate::{
    extractors::auth::{AdminAuth, Auth},
    response::{error, success, EmptyResponse, Response},
};

#[derive(Deserialize)]
pub struct CredentialModel {
    email: String,
    password: String,
}

pub async fn create(data: CredentialModel, db: Pool) -> EmptyResponse {
    match users::create(&data.email, &data.password, &db).await {
        Ok(_) => success(()).with_status(success::Code::Created).into(),
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

pub async fn confirm(id: users::Id, db: Pool, _: AdminAuth) -> EmptyResponse {
    match users::confirm(id, &db).await {
        Ok(()) => success(()).into(),
        Err(err) => {
            error!("{err:?}");

            match err {
                Error::InvalidUserId => error().with_status(error::Code::NotFound),
                _ => error(),
            }
            .into()
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
    #[serde(with = "time::serde::rfc3339")]
    expiration: OffsetDateTime,
    admin: bool,
}

pub async fn login(data: CredentialModel, db: Pool) -> Response<TokenResponse> {
    let dbref = &db;

    let result = users::find_by_credentials(&data.email, &data.password, dbref)
        .and_then(|(id, admin)| async move {
            tokens::create(id, dbref)
                .await
                .map(|(token, expiration)| (token.0.to_string(), expiration, admin))
        })
        .await;

    match result {
        Ok((token, expiration, admin)) => success(TokenResponse {
            token,
            expiration,
            admin,
        })
        .with_status(success::Code::Created)
        .into(),
        Err(Error::InvalidCredentials) => error()
            .with_status(error::Code::Forbidden)
            .body(String::from("Invalid credentials"))
            .into(),
        Err(err) => {
            error!("{err:?}");

            error().into()
        }
    }
}

pub async fn logout(auth: Auth, db: Pool) -> EmptyResponse {
    match tokens::delete(auth.token(), &db).await {
        Ok(()) => success(()).into(),
        Err(err) => {
            error!("{err:?}");

            error().into()
        }
    }
}

#[derive(Serialize)]
pub struct CandidateModel {
    id: i32,
    email: String,
}

pub async fn candidates(db: Pool, _: AdminAuth) -> Response<Vec<CandidateModel>> {
    match users::list_candidates(&db).await {
        Ok(candidates) => success(
            candidates
                .into_iter()
                .map(|Summary { id, email }| CandidateModel { id: id.0, email })
                .collect(),
        )
        .into(),
        Err(err) => {
            error!("{err:?}");

            error().into()
        }
    }
}
