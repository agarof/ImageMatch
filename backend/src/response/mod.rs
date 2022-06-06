pub mod error;
pub mod success;

pub use error::error;
pub use success::success;

use serde::Serialize;
use warp::{
    hyper::StatusCode,
    reply::{json, with_status},
};

pub trait ResponseBody: Serialize + Send {}

impl<T> ResponseBody for T where T: Serialize + Send {}

pub struct Response<T>
where
    T: ResponseBody,
{
    body: Result<T, error::Body>,
    code: StatusCode,
}

pub struct EmptyResponse {
    error: Option<error::Body>,
    code: StatusCode,
}

impl<T> warp::Reply for Response<T>
where
    T: ResponseBody,
{
    fn into_response(self) -> warp::reply::Response {
        let body = match &self.body {
            Ok(body) => json(body),
            Err(body) => json(body),
        };

        with_status(body, self.code).into_response()
    }
}

impl warp::Reply for EmptyResponse {
    fn into_response(self) -> warp::reply::Response {
        match self.error {
            None => with_status(warp::reply(), self.code).into_response(),
            Some(error) => with_status(json(&error), self.code).into_response(),
        }
    }
}
