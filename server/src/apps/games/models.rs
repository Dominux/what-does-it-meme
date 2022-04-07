use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid;

use super::state_enum::GameState;
use crate::apps::games::schema::games;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: uuid::Uuid,
    pub state: GameState,
    pub timestamp: Option<SystemTime>
}
