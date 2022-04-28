use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::RoomState;
use crate::{
    apps::rooms::schema::rooms,
    common::{
        config::Config,
        errors::{MemeError, MemeResult},
    },
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub state: RoomState,
    pub current_round_id: Option<uuid::Uuid>,
    pub expiration_timestamp: SystemTime,
}

impl Room {
    pub fn new() -> MemeResult<Self> {
        Ok(Self {
            id: uuid::Uuid::new_v4(),
            state: RoomState::NotStarted,
            current_round_id: None,
            expiration_timestamp: SystemTime::now() + Config::new()?.time_to_start_the_game,
        })
    }

    pub fn start_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::NotStarted => Ok({
                self.state = RoomState::Started;
                self.expiration_timestamp =
                    SystemTime::now() + Config::new()?.time_to_create_situation;
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    pub fn end_game(&mut self) -> MemeResult<()> {
        match self.state {
            RoomState::Started => Ok({
                self.state = RoomState::Ended;
                self.expiration_timestamp =
                    SystemTime::now() + Config::new()?.time_until_room_deletion;
            }),
            _ => Err(MemeError::NotAllowedStateTransition),
        }
    }

    #[inline]
    pub fn is_expired(&self) -> bool {
        self.expiration_timestamp.elapsed().is_ok()
    }

    #[inline]
    pub fn is_ended(&self) -> bool {
        matches!(self.state, RoomState::Ended)
    }
}
