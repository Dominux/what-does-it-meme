use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::GameState;
use crate::{
    apps::games::schema::games,
    common::errors::{MemeError, MemeResult},
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: uuid::Uuid,
    pub state: GameState,
    pub timestamp: Option<SystemTime>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            state: GameState::NotStarted,
            // Setting current time
            timestamp: Some(SystemTime::now()),
        }
    }

    pub fn start(&mut self) -> MemeResult<()> {
        match self.state {
            GameState::NotStarted => Ok({
                self.state = GameState::Started;
                // Setting time to None cause at this state we don't need it
                self.timestamp = None;
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn end(&mut self) -> MemeResult<()> {
        match self.state {
            GameState::Started => Ok({
                self.state = GameState::Ended;
                // Setting current time
                self.timestamp = Some(SystemTime::now());
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }
}
