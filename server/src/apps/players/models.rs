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
}

impl Player {
    pub fn new(name: String, room_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            room_id,
        }
    }
}

impl From<InPlayer> for Player {
    fn from(in_player: InPlayer) -> Self {
        Self::new(in_player.name, in_player.room_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub memes_in_hands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerWithMemes {
    pub id: uuid::Uuid,
    pub name: String,
    pub room_id: uuid::Uuid,
    pub memes_in_hands: Vec<String>,
}

impl PlayerWithMemes {
    pub fn new(player: Player, memes_in_hands: Vec<String>) -> Self {
        Self {
            id: player.id,
            name: player.name,
            room_id: player.room_id,
            memes_in_hands,
        }
    }
}
