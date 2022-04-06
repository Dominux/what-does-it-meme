use super::repository::GamesRepository;
use crate::apps::games::models;
use crate::common::db::DBConnection;
use crate::common::errors::AppResult;

pub struct GamesService<'a> {
    repo: GamesRepository<'a>,
}

impl<'a> GamesService<'a> {
    // TODO: add trait to create services
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: GamesRepository::new(db),
        }
    }

    pub fn create_game(&self) -> AppResult<models::Game> {
        self.repo.create_game()
    }

    pub fn get_game_by_id(&self, id: uuid::Uuid) -> AppResult<models::Game> {
        self.repo.get_game(id)
    }
}
