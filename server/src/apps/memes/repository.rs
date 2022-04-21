use diesel::prelude::*;
use uuid;

use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

use super::models;
use super::schema::memes;

pub struct MemesRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> MemesRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    /// Saving meme only if voter have not saved one yet
    pub fn save_meme_if_not_exists(&self, meme: models::Meme) -> MemeResult<()> {
        match self.get_meme_by_player_id_and_round_id(meme.player_id, meme.round_id) {
            Err(MemeError::NotFound) => self.save_meme(&meme),
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn save_meme(&self, meme: &models::Meme) -> MemeResult<()> {
        diesel::insert_into(memes::table)
            .values(meme)
            .execute(self.db)?;
        Ok(())
    }

    pub fn get_meme(&self, meme_id: uuid::Uuid) -> MemeResult<models::Meme> {
        let meme = memes::table
            .filter(memes::columns::id.eq(meme_id))
            .first::<models::Meme>(self.db)?;
        Ok(meme)
    }

    pub fn get_meme_by_player_id_and_round_id(
        &self,
        player_id: uuid::Uuid,
        round_id: uuid::Uuid,
    ) -> MemeResult<models::Meme> {
        let meme = memes::table
            .filter(memes::columns::player_id.eq(player_id))
            .filter(memes::columns::round_id.eq(round_id))
            .first::<models::Meme>(self.db)?;
        Ok(meme)
    }

    pub fn memes_count(&self, round_id: uuid::Uuid) -> MemeResult<u8> {
        let count: i64 = memes::table.count()
            .filter(memes::columns::round_id.eq(round_id))
            .first(self.db)?;
        Ok(count as u8)
    }

    pub fn update_voters_ids(
        &self,
        meme_id: uuid::Uuid,
        voters_ids: Vec<uuid::Uuid>,
    ) -> MemeResult<()> {
        unimplemented!()
    }
}
