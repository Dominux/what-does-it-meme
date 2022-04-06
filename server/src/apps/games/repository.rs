use diesel::prelude::*;
use uuid;

use crate::apps::games::models;
use crate::common::db::DBConnection;
use crate::common::errors::{AppError, AppResult};

pub struct GamesRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> GamesRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db: db }
    }

    pub fn create_game(&self) -> AppResult<models::Game> {
        use crate::apps::games::schema::games::dsl::*;

        let game = models::Game {
            id: uuid::Uuid::new_v4(),
        };

        diesel::insert_into(games).values(&game).execute(self.db)?;

        Ok(game)
    }

    pub fn get_game(&self, uid: uuid::Uuid) -> AppResult<models::Game> {
        use crate::apps::games::schema::games::dsl::*;

        let game = games
            .filter(id.eq(uid))
            .first::<models::Game>(self.db)
            .optional()?
            .ok_or(AppError::NotFound)?;
        Ok(game)
    }
}
