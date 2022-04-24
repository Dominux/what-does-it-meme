use std::time::SystemTime;

use diesel::prelude::*;
use uuid;

use crate::apps::rooms::models;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

pub struct RoomsRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> RoomsRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    pub fn create_room(&self) -> MemeResult<models::Room> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        let room = models::Room::new()?;
        diesel::insert_into(rooms).values(&room).execute(self.db)?;

        Ok(room)
    }

    pub fn get_room(&self, uid: uuid::Uuid) -> MemeResult<models::Room> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        let room = rooms.filter(id.eq(uid)).first::<models::Room>(self.db)?;
        Ok(room)
    }

    pub fn update_room(&self, room: models::Room) -> MemeResult<()> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        diesel::update(rooms.filter(id.eq(room.id)))
            .set((
                state.eq(room.state),
                current_round_id.eq(room.current_round_id),
                expiration_timestamp.eq(room.expiration_timestamp),
            ))
            .execute(self.db)?;

        Ok(())
    }

    pub fn update_room_expiration_timestamp(
        &self,
        room_id: uuid::Uuid,
        et: SystemTime,
    ) -> MemeResult<()> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        diesel::update(rooms.filter(id.eq(room_id)))
            .set((expiration_timestamp.eq(et),))
            .execute(self.db)?;

        Ok(())
    }
}
