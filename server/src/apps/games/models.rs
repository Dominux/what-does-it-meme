use serde::{Deserialize, Serialize};
use uuid;

// use crate::apps::games::tables;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: uuid::Uuid,
}
