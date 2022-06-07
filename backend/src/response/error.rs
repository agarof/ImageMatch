use warp::hyper::StatusCode;

use super::{EmptyResponse, Response, ResponseBody};

pub struct Error {
    body: Option<Body>,
    code: Code,
}

pub type Body = String;

#[derive(Clone, Copy)]
pub enum Code {
    NotFound,
    Conflict,
    Internal,
}

pub fn error() -> Error {
    Error {
        body: None,
        code: Code::Internal,
    }
}

impl Error {
    pub fn with_status(self, code: Code) -> Self {
        Self { code, ..self }
    }

    pub fn body(self, body: Body) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }
}

impl<T> From<Error> for Response<T>
where
    T: ResponseBody,
{
    fn from(error: Error) -> Self {
        let body = Err(error
            .body
            .unwrap_or_else(|| error.code.as_str().to_string()));

        Self {
            body,
            code: error.code.into(),
        }
    }
}

impl From<Error> for EmptyResponse {
    fn from(error: Error) -> Self {
        let message = Some(
            error
                .body
                .unwrap_or_else(|| error.code.as_str().to_string()),
        );

        Self {
            error: message,
            code: error.code.into(),
        }
    }
}

impl Code {
    fn as_str(self) -> &'static str {
        match self {
            Code::NotFound => "Not Found",
            Code::Conflict => "Conflict",
            Code::Internal => "Internal Server Error",
        }
    }
}

impl From<Code> for StatusCode {
    fn from(code: Code) -> Self {
        match code {
            Code::NotFound => StatusCode::NOT_FOUND,
            Code::Conflict => StatusCode::CONFLICT,
            Code::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
