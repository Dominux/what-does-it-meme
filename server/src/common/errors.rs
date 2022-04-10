use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use envconfig::Error as EnvconfigError;
use r2d2::Error as R2d2Error;
use thiserror;

/// Generic app error
#[derive(thiserror::Error, Debug)]
pub enum MemeError {
    #[error("Not Found")]
    NotFound,

    #[error("State transition not allowed")]
    NotAllowedStateTransition,

    #[error("Enterring a room is impossible once the game starts")]
    EnterringRoomAfterStart,

    #[error("Players limit is already achieved")]
    AchivedPlayersLimit,

    #[error("At least three players needed")]
    TooLessPlayers,

    #[error("Another player in the room already has this name")]
    DuplicatedName,

    #[error("Unknown")]
    Unknown,
}

impl ResponseError for MemeError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::NotAllowedStateTransition => StatusCode::LOCKED,
            Self::EnterringRoomAfterStart => StatusCode::LOCKED,
            Self::AchivedPlayersLimit => StatusCode::CONFLICT,
            Self::TooLessPlayers => StatusCode::LOCKED,
            Self::DuplicatedName => StatusCode::CONFLICT,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for MemeError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        Self::Unknown
    }
}

impl From<BlockingError> for MemeError {
    fn from(_: BlockingError) -> Self {
        Self::Unknown
    }
}

impl From<DieselError> for MemeError {
    fn from(_: DieselError) -> Self {
        Self::Unknown
    }
}

impl From<R2d2Error> for MemeError {
    fn from(_: R2d2Error) -> Self {
        Self::Unknown
    }
}

impl From<EnvconfigError> for MemeError {
    fn from(_: EnvconfigError) -> Self {
        Self::Unknown
    }
}

/// Generic app result
pub type MemeResult<T> = Result<T, MemeError>;
