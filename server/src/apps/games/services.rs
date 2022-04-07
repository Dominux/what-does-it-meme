use super::repository::GamesRepository;
use crate::apps::games::models;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

pub struct GamesService<'a> {
    repo: GamesRepository<'a>,
}

impl<'a> GamesService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: GamesRepository::new(db),
        }
    }

    pub fn create_game(&self) -> MemeResult<models::Game> {
        self.repo.create_game()
    }

    pub fn get_game_by_id(&self, id: uuid::Uuid) -> MemeResult<models::Game> {
        self.repo.get_game(id)
    }
}
