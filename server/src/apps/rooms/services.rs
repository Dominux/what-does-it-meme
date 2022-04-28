use super::repository::RoomsRepository;
use crate::apps::rooms::models;
use crate::common::config::Config;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeResult, MemeError};

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
        // Deleting expired rooms
        self.repo.delete_expired_rooms()?;

        if self.repo.get_rooms_count()? >= Config::new()?.max_rooms_count {
            return Err(MemeError::TooManyRooms)
        }

        self.repo.create_room()
    }

    pub fn get_room_by_id(&self, id: uuid::Uuid) -> MemeResult<models::Room> {
        self.repo.get_room(id)
    }

    pub fn update_game(&self, room: models::Room) -> MemeResult<()> {
        self.repo.update_room(room)
    }
}
