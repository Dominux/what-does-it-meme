use actix_web::{error, ResponseError, http::StatusCode, HttpResponse};
use thiserror;

/// Generic error
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
