use serde::{Deserialize, Serialize};
use uuid;

use crate::apps::games::schema::games;

use super::state_enum::GameState;


#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: uuid::Uuid,
    pub state: GameState,
}
