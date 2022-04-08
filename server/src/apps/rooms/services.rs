use super::repository::RoomsRepository;
use crate::apps::rooms::models;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

pub struct RoomsService<'a> {
    repo: RoomsRepository<'a>,
}

impl<'a> RoomsService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: RoomsRepository::new(db),
        }
    }

    pub fn create_room(&self) -> MemeResult<models::Room> {
        self.repo.create_room()
    }

    pub fn get_room_by_id(&self, id: uuid::Uuid) -> MemeResult<models::Room> {
        self.repo.get_room(id)
    }
}
