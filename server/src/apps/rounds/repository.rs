use diesel::dsl::count_star;
use diesel::prelude::*;
use uuid;

use crate::apps::rounds::models;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct RoundsRepository<'a> {
    db: &'a DBConnection,
}

impl<'a> RoundsRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db: db }
    }

    pub fn save_round(&self, round: &models::Round) -> MemeResult<()> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        diesel::insert_into(rounds).values(round).execute(self.db)?;

        Ok(())
    }

    pub fn get_round(&self, uid: uuid::Uuid) -> MemeResult<models::Round> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        let round = rounds
            .filter(id.eq(uid))
            .first::<models::Round>(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(round)
    }

    pub fn count_rounds(&self, _room_id: uuid::Uuid) -> MemeResult<u8> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        let round: i64 = rounds
            .select(count_star())
            .filter(room_id.eq(_room_id))
            .first(self.db)
            .optional()?
            .ok_or(MemeError::NotFound)?;
        Ok(round as u8)
    }

    /// Now we updating only state
    pub fn update_round(&self, room: models::Round) -> MemeResult<()> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        diesel::update(rounds.filter(id.eq(room.id)))
            .set(state.eq(room.state))
            .execute(self.db)?;

        Ok(())
    }
}
