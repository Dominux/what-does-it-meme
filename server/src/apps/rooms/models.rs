use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::RoomState;
use crate::{
    apps::rooms::schema::rooms,
    common::errors::{MemeError, MemeResult},
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub state: RoomState,
    pub current_round_id: Option<uuid::Uuid>,
    pub timestamp: SystemTime,
}

impl Room {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            state: RoomState::NotStarted,
            current_round_id: None,
            timestamp: SystemTime::now(),
        }
    }

    pub fn start_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::NotStarted => Ok({
                self.state = RoomState::Started;
                self.timestamp = SystemTime::now();
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn end_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::Started => Ok({
                self.state = RoomState::Ended;
                self.timestamp = SystemTime::now();
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }
}