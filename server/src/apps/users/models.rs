use serde::{Deserialize, Serialize};
use uuid;

use crate::apps::games::models::Game;
use crate::apps::users::schema::users;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations)]
#[belongs_to(Game)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub game_id: uuid::Uuid,
}

impl User {
    pub fn new(name: String, user_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name,
            game_id: user_id,
        }
    }
}
