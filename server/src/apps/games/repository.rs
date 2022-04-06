use diesel::prelude::*;
use uuid;

use crate::apps::games::models;
use crate::common::errors::AppResult;
use crate::common::db::DBConnection;

pub struct GamesRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> GamesRepository<'a> {
    // TODO: add repo trait to locate the logic for creating new repo in one place
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db: db }
    }

    pub fn create_game(&self) -> AppResult<models::Game> {
        use crate::apps::games::schema::games::dsl::*;

        let game = models::Game {
            id: uuid::Uuid::new_v4(),
        };

        diesel::insert_into(games)
            .values(&game)
            .execute(self.db)?;

        Ok(game)
    }
}