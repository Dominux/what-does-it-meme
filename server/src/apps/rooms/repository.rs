use std::time::SystemTime;

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

    // pub fn get_room_by_player_id(&self, player_id: uuid::Uuid) -> MemeResult<models::Room> {
    //     use crate::apps::players::schema::players;
    //     use crate::apps::rooms::schema::rooms;

    //     let rooms_table = rooms::table;
    //     let players_table = players::table;

    //     allow_tables_to_appear_in_same_query!(rooms, players);

    //     let room = rooms_table
    //         .inner_join(players_table.on(players::dsl::id.eq(player_id)))
    //         .select((
    //             rooms::columns::id,
    //             rooms::columns::state,
    //             rooms::columns::current_round_id,
    //             rooms::columns::expiration_timestamp,
    //         ))
    //         .first::<models::Room>(self.db)
    //         .optional()?
    //         .ok_or(MemeError::NotFound)?;
    //     Ok(room)
    // }

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

    ////////////////////////////////////////////
    //  Timestamp getting/setting
    ////////////////////////////////////////////

    pub fn get_expiration_timestamp(&self, room_id: uuid::Uuid) -> MemeResult<SystemTime> {
        use crate::apps::rooms::schema::rooms::dsl::*;

        let _expiration_timestamp = rooms
            .select(expiration_timestamp)
            .filter(id.eq(room_id))
            .first::<SystemTime>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(_expiration_timestamp)
    }

    ////////////////////////////////////////////
    //  Current round getting/setting
    ////////////////////////////////////////////
}
