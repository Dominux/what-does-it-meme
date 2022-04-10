use diesel::prelude::*;
use uuid;

use crate::apps::players::models;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct PlayersRepository<'a> {
    pub db: &'a DBConnection,
}

impl<'a> PlayersRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    pub fn create(&self, in_player: models::InPlayer) -> MemeResult<models::Player> {
        use crate::apps::players::schema::players::dsl::*;

        let player = models::Player::from(in_player);
        diesel::insert_into(players)
            .values(&player)
            .execute(self.db)?;

        Ok(player)
    }

    pub fn list_players_ids(&self, room_id: uuid::Uuid) -> MemeResult<Vec<uuid::Uuid>> {
        use crate::apps::players::schema::players::dsl::*;

        let players_ids = players
            .select(id)
            .filter(room_id.eq(room_id))
            .load::<uuid::Uuid>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(players_ids)
    }

    pub fn get_player(&self, uid: uuid::Uuid) -> MemeResult<models::Player> {
        use crate::apps::players::schema::players::dsl::*;

        let player = players
            .filter(id.eq(uid))
            .first::<models::Player>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(player)
    }

    pub fn list_room_players(&self, _room_id: uuid::Uuid) -> MemeResult<Vec<models::Player>> {
        use crate::apps::players::schema::players::dsl::*;

        let _players = players
            .filter(room_id.eq(_room_id))
            .load::<models::Player>(self.db)?;
        Ok(_players)
    }

    // pub fn get_players_count(&self, _room_id: uuid::Uuid) -> MemeResult<i64> {
    //     use crate::apps::players::schema::players::dsl::*;

    //     let count = players.filter(room_id.eq(_room_id)).select(count_star()).first(self.db)?;
    //     Ok(count)
    // }
}
