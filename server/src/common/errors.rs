use actix_web::{ResponseError, http::StatusCode, HttpResponse, error::BlockingError};
use diesel::result::{Error as DieselError};
use r2d2::{Error as R2d2Error};
use thiserror;

/// Generic app error
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,

    #[error("Unknown")]
    Unknown,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        Self::Unknown
    }
}

impl From<BlockingError> for AppError {
    fn from(_: BlockingError) -> Self {
        Self::Unknown
    }
}

impl From<DieselError> for AppError {
    fn from(_: DieselError) -> Self {
        Self::Unknown
    }
}

impl From<R2d2Error> for AppError {
    fn from(_: R2d2Error) -> Self {
        Self::Unknown
    }
}

/// Generic app result
pub type AppResult<T> = Result<T, AppError>;
