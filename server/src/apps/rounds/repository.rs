use diesel::dsl::count_star;
use diesel::prelude::*;
use uuid;

use crate::apps::rounds::models;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

pub struct RoundsRepository<'a> {
    pub db: &'a DBConnection,
}

impl<'a> RoundsRepository<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self { db }
    }

    pub fn save_round(&self, round: &models::Round) -> MemeResult<()> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        diesel::insert_into(rounds).values(round).execute(self.db)?;

        Ok(())
    }

    pub fn get_round(&self, uid: uuid::Uuid) -> MemeResult<models::Round> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        let round = rounds.filter(id.eq(uid)).first::<models::Round>(self.db)?;
        Ok(round)
    }

    pub fn get_round_by_situation_creator_id(
        &self,
        situation_creator_id: uuid::Uuid,
    ) -> MemeResult<models::Round> {
        use crate::apps::rounds::schema::rounds;

        let round = rounds::table
            .filter(rounds::columns::situation_creator_id.eq(situation_creator_id))
            .first::<models::Round>(self.db)?;
        Ok(round)
    }

    pub fn count_rounds(&self, _room_id: uuid::Uuid) -> MemeResult<u8> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        let round: i64 = rounds
            .select(count_star())
            .filter(room_id.eq(_room_id))
            .first(self.db)?;
        Ok(round as u8)
    }

    /// Now we updating only state
    pub fn update_round(&self, round: models::Round) -> MemeResult<()> {
        use crate::apps::rounds::schema::rounds::dsl::*;

        diesel::update(rounds.filter(id.eq(round.id)))
            .set((state.eq(round.state), situation.eq(round.situation)))
            .execute(self.db)?;

        Ok(())
    }
}
