use serde::{Deserialize, Serialize};
use uuid;

use crate::{apps::memes::schema::memes, apps::rounds::models::Round};

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations,
)]
#[belongs_to(Round)]
pub struct Meme {
    pub id: uuid::Uuid,
    pub round_id: uuid::Uuid,
    pub player_id: uuid::Uuid,
    pub voters_ids: Vec<uuid::Uuid>,
    pub link: String,
}

impl Meme {
    fn new(round_id: uuid::Uuid, player_id: uuid::Uuid, link: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            round_id,
            player_id,
            voters_ids: Vec::new(),
            link,
        }
    }
}
