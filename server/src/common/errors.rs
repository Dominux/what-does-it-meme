use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use envconfig::Error as EnvconfigError;
use r2d2::Error as R2d2Error;
use reqwest::Error as ReqwestError;
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

    #[error("Situation creator cannot `{0}`")]
    SituationCreatorCant(String),

    #[error("You can't create situation at this round stage")]
    InvalidStateToCreateSituation,

    #[error("You can't react with a meme at this round stage")]
    InvalidStateToReactWithMeme,

    #[error("You can't vote at this round stage")]
    InvalidStateToVote,

    #[error("You already reacted to this situation")]
    AlreadyReactedWithMeme,

    #[error("You already voted")]
    AlreadyVoted,

    #[error("You can't vote for your own meme")]
    PlayerCannotVoteForHisMeme,

    #[error("Player does not have such meme in his hand")]
    MemeIsNotInHand,

    #[error("Memes scrapping error")]
    MemesScrapingError,

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
            Self::SituationCreatorCant(_) => StatusCode::UNAUTHORIZED,
            Self::InvalidStateToCreateSituation => StatusCode::LOCKED,
            Self::InvalidStateToReactWithMeme => StatusCode::LOCKED,
            Self::InvalidStateToVote => StatusCode::LOCKED,
            Self::AlreadyReactedWithMeme => StatusCode::LOCKED,
            Self::AlreadyVoted => StatusCode::LOCKED,
            Self::PlayerCannotVoteForHisMeme => StatusCode::CONFLICT,
            Self::MemeIsNotInHand => StatusCode::CONFLICT,
            Self::MemesScrapingError => StatusCode::INTERNAL_SERVER_ERROR,
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
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Self::NotFound,
            _ => Self::Unknown,
        }
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

impl From<ReqwestError> for MemeError {
    fn from(_: ReqwestError) -> Self {
        Self::MemesScrapingError
    }
}

impl From<std::env::VarError> for MemeError {
    fn from(_: std::env::VarError) -> Self {
        Self::Unknown
    }
}

/// Generic app result
pub type MemeResult<T> = Result<T, MemeError>;
