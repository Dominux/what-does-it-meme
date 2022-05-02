use std::time::SystemTime;

use actix_web::cookie::time::Duration;
use diesel::prelude::*;
use uuid;

use crate::apps::rooms::models;
use crate::apps::rooms::state_enum::RoomState;
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

    pub fn get_rooms_count(&self) -> MemeResult<u8> {
        use crate::apps::rooms::schema::rooms;

        let count = rooms::table.count().first::<i64>(self.db)? as u8;
        Ok(count)
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

    pub fn delete_expired_rooms(&self) -> MemeResult<()> {
        use crate::apps::rooms::schema::rooms;

        let now = SystemTime::now();

        let not_started_predicate = rooms::state
            .eq(RoomState::NotStarted)
            .and(rooms::expiration_timestamp.le(now));
        let ended_predicate = rooms::state
            .eq(RoomState::Ended)
            .and(rooms::expiration_timestamp.le(now));

        // Also deleting abandonded games (all started games, that have been expired af 
        // (e.g for a 10 minutes in our case))
        let started_predicate = rooms::state
            .eq(RoomState::Started)
            .and(rooms::expiration_timestamp.le(now - Duration::MINUTE * 10));

        diesel::delete(
            rooms::table.filter(
                not_started_predicate
                    .or(ended_predicate)
                    .or(started_predicate),
            ),
        )
        .execute(self.db)?;

        Ok(())
    }
}
