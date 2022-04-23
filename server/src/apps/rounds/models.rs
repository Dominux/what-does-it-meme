use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::RoundState;
use crate::{
    apps::rooms::models::Room,
    apps::rounds::schema::rounds,
    common::errors::{MemeError, MemeResult},
};

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations,
)]
#[belongs_to(Room)]
pub struct Round {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub state: RoundState,
    pub situation: Option<String>,
    pub situation_creator_id: uuid::Uuid,
}

impl Round {
    pub fn new(room_id: uuid::Uuid, situation_creator_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            room_id,
            state: RoundState::SituationCreation,
            situation: None,
            situation_creator_id,
        }
    }

    pub fn set_to_choose_memes(&mut self) -> MemeResult<()> {
        match self.state {
            RoundState::SituationCreation => Ok(self.state = RoundState::ChoosingMemes),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn set_to_vote(&mut self) -> MemeResult<()> {
        match self.state {
            RoundState::ChoosingMemes => Ok(self.state = RoundState::Voting),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn end_round(&mut self) -> MemeResult<()> {
        match self.state {
            RoundState::Voting => Ok(self.state = RoundState::Ended),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn is_choosing_memes(&self) -> bool {
        matches!(self.state, RoundState::ChoosingMemes)
    }

    pub fn is_voting(&self) -> bool {
        matches!(self.state, RoundState::Voting)
    }

    pub fn is_ended(&self) -> bool {
        matches!(self.state, RoundState::Ended)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutRound {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub state: RoundState,
    pub situation: Option<String>,
}

impl From<Round> for OutRound {
    fn from(r: Round) -> Self {
        OutRound {
            id: r.id,
            room_id: r.room_id,
            state: r.state,
            situation: r.situation,
        }
    }
}
