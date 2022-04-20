use diesel::dsl::count_star;
use diesel::prelude::*;
use uuid;

use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

use super::models;

pub struct MemesRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> MemesRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    pub fn save_meme(&self, meme: models::Meme) -> MemeResult<()> {
        unimplemented!()
    }

    pub fn get_meme(&self, meme_id: uuid::Uuid) -> MemeResult<models::Meme> {
        unimplemented!()
    }

    pub fn update_voters_ids(
        &self,
        meme_id: uuid::Uuid,
        voters_ids: Vec<uuid::Uuid>,
    ) -> MemeResult<()> {
        unimplemented!()
    }
}
