use diesel::prelude::*;
use uuid;

use crate::apps::players::models;
use crate::apps::players::schema::players;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

pub struct PlayersRepository<'a> {
    pub db: &'a DBConnection,
}

impl<'a> PlayersRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    pub fn create(
        &self,
        in_player: models::InPlayer,
        memes: Vec<String>,
    ) -> MemeResult<models::Player> {
        let player = models::Player::new(in_player.name, in_player.room_id, memes);
        diesel::insert_into(players::table)
            .values(&player)
            .execute(self.db)?;

        Ok(player)
    }

    pub fn get_player(&self, uid: uuid::Uuid) -> MemeResult<models::Player> {
        use crate::apps::players::schema::players::dsl::*;

        let player = players
            .filter(id.eq(uid))
            .first::<models::Player>(self.db)?;
        Ok(player)
    }

    pub fn list_players_ids(&self, room_id: uuid::Uuid) -> MemeResult<Vec<uuid::Uuid>> {
        let players_ids = players::table
            .select(players::id)
            .filter(players::room_id.eq(room_id))
            .load::<uuid::Uuid>(self.db)?;
        Ok(players_ids)
    }

    pub fn list_players(&self, room_id: uuid::Uuid) -> MemeResult<Vec<models::Player>> {
        let players = players::table
            .filter(players::room_id.eq(room_id))
            .load(self.db)?;
        Ok(players)
    }

    pub fn count_players(&self, room_id: uuid::Uuid) -> MemeResult<u8> {
        let count: i64 = players::table
            .count()
            .filter(players::room_id.eq(room_id))
            .first(self.db)?;
        Ok(count as u8)
    }

    pub fn list_room_players(&self, room_id: uuid::Uuid) -> MemeResult<Vec<models::Player>> {
        let _players = players::table
            .filter(players::room_id.eq(room_id))
            .load::<models::Player>(self.db)?;
        Ok(_players)
    }

    pub fn update_players_memes(
        &self,
        player_id: uuid::Uuid,
        memes: Vec<String>,
    ) -> MemeResult<()> {
        diesel::update(players::table.filter(players::id.eq(player_id)))
            .set(players::memes_in_hand.eq(memes))
            .execute(self.db)?;
        Ok(())
    }

    // pub fn get_players_count(&self, _room_id: uuid::Uuid) -> MemeResult<i64> {
    //     use crate::apps::players::schema::players::dsl::*;

    //     let count = players.filter(room_id.eq(_room_id)).select(count_star()).first(self.db)?;
    //     Ok(count)
    // }
}
