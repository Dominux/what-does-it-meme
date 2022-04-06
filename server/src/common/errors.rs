use std::{ops::FromResidual, convert::Infallible};

use actix_web::{ResponseError, http::StatusCode, HttpResponse};
use diesel::{r2d2::Error as R2d2Error, result::{Error as DieselError}};
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

impl FromResidual<Result<Infallible, DieselError>> for AppError {
    fn from_residual(residual: Result<Infallible, DieselError>) -> Self {
        Self::Unknown
    }
}

impl FromResidual<Result<Infallible, R2d2Error>> for AppError {
    fn from_residual(residual: Result<Infallible, R2d2Error>) -> Self {
        Self::Unknown
    }
}

/// Generic app result
pub type AppResult<T> = Result<T, AppError>;
