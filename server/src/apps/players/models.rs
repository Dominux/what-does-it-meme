use serde::{Deserialize, Serialize};
use uuid;

use crate::apps::players::schema::players;
use crate::apps::rooms::models::Room;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InPlayer {
    pub name: String,
    pub room_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Insertable,
    Associations,
)]
#[belongs_to(Room)]
pub struct Player {
    pub id: uuid::Uuid,
    pub name: String,
    pub room_id: uuid::Uuid,
    pub memes_in_hand: Vec<String>,
}

impl Player {
    pub fn new(name: String, room_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            room_id,
            memes_in_hand: Vec::new(),
        }
    }
}

impl From<InPlayer> for Player {
    fn from(in_player: InPlayer) -> Self {
        Self::new(in_player.name, in_player.room_id)
    }
}
