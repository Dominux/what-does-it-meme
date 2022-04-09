use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::RoundState;
use crate::{
    apps::rounds::schema::rounds,
    common::errors::{MemeError, MemeResult},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Round {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub number: i16,
    pub state: RoundState,
    pub situation: String,
    pub situation_creater_id: uuid::Uuid,
}
