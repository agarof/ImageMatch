use futures::TryFutureExt;
use log::*;
use serde::{Deserialize, Serialize};

use db::{
    result::Error,
    tokens,
    users::{self, Candidate},
    Pool,
};

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
}

pub async fn login(data: CredentialModel, db: Pool) -> Response<TokenResponse> {
    let dbref = &db;

    let result = users::find_by_credentials(&data.email, &data.password, dbref)
        .and_then(|id| async move { tokens::create(id, dbref).await })
        .await;

    match result {
        Ok(token) => success(TokenResponse {
            token: token.0.to_string(),
        })
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
                .map(|Candidate { id, email }| CandidateModel { id: id.0, email })
                .collect(),
        )
        .into(),
        Err(err) => {
            error!("{err:?}");

            error().into()
        }
    }
}
