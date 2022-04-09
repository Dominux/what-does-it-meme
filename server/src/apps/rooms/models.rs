use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::RoomState;
use crate::{
    apps::rooms::schema::rooms,
    common::errors::{MemeError, MemeResult},
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub state: RoomState,
    pub timestamp: Option<SystemTime>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            state: RoomState::NotStarted,
            // Setting current time
            timestamp: Some(SystemTime::now()),
        }
    }

    pub fn start_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::NotStarted => Ok({
                self.state = RoomState::Started;
                // Setting time to None cause at this state we don't need it
                self.timestamp = None;
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn end_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::Started => Ok({
                self.state = RoomState::Ended;
                // Setting current time
                self.timestamp = Some(SystemTime::now());
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }
}
