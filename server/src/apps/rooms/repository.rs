use diesel::prelude::*;
use uuid;

use crate::apps::rooms::models;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct RoomsRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> RoomsRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db: db }
    }

    pub fn create_room(&self) -> MemeResult<models::Room> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        let room = models::Room::new();
        diesel::insert_into(rooms).values(&room).execute(self.db)?;

        Ok(room)
    }

    pub fn get_room(&self, uid: uuid::Uuid) -> MemeResult<models::Room> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        let room = rooms
            .filter(id.eq(uid))
            .first::<models::Room>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(room)
    }

    pub fn update_room(&self, room: models::Room) -> MemeResult<()> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        diesel::update(rooms.filter(id.eq(room.id)))
            .set((state.eq(room.state), timestamp.eq(room.timestamp)))
            .execute(self.db)?;

        Ok(())
    }
}
