use warp::hyper::StatusCode;

use super::{EmptyResponse, Response, ResponseBody};

pub struct Success<T>
where
    T: ResponseBody,
{
    body: T,
    code: Code,
}

pub enum Code {
    OK,
}

pub fn success<T>(body: T) -> Success<T>
where
    T: ResponseBody,
{
    Success {
        body,
        code: Code::OK,
    }
}

impl<T> Success<T>
where
    T: ResponseBody,
{
    pub fn with_status(self, code: Code) -> Self {
        Self { code, ..self }
    }
}

impl<T> From<Success<T>> for Response<T>
where
    T: ResponseBody,
{
    fn from(success: Success<T>) -> Self {
        Self {
            body: Ok(success.body),
            code: success.code.into(),
        }
    }
}

impl From<Success<()>> for EmptyResponse {
    fn from(success: Success<()>) -> Self {
        Self {
            error: None,
            code: success.code.into(),
        }
    }
}

impl From<Code> for StatusCode {
    fn from(code: Code) -> Self {
        match code {
            Code::OK => StatusCode::OK,
        }
    }
}
