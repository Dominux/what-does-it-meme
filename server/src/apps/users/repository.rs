use diesel::prelude::*;
use uuid;

use crate::apps::games::models;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct GamesRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> GamesRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db: db }
    }

    pub fn create_game(&self) -> MemeResult<models::Game> {
        use crate::apps::games::schema::games::dsl::*;

        let game = models::Game::new();
        diesel::insert_into(games).values(&game).execute(self.db)?;

        Ok(game)
    }

    pub fn get_game(&self, uid: uuid::Uuid) -> MemeResult<models::Game> {
        use crate::apps::games::schema::games::dsl::*;

        let game = games
            .filter(id.eq(uid))
            .first::<models::Game>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(game)
    }
}
