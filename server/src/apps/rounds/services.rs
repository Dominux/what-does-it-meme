use std::time::SystemTime;

use super::repository::RoundsRepository;
use super::state_enum::RoundState;
use crate::apps::rooms::repository::RoomsRepository;
use crate::apps::rounds::models;
use crate::common::config::Config;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct RoundsService<'a> {
    repo: RoundsRepository<'a>,
}

impl<'a> RoundsService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: RoundsRepository::new(db),
        }
    }

    pub fn create_round(
        &self,
        room_id: uuid::Uuid,
        situation_creator_id: uuid::Uuid,
    ) -> MemeResult<models::Round> {
        // Filling the fields
        let round = models::Round {
            id: uuid::Uuid::new_v4(),
            room_id,
            state: RoundState::SituationCreation,
            situation: None,
            situation_creator_id,
        };

        // Saving it
        self.repo.save_round(&round)?;

        Ok(round)
    }

    pub fn get_round(&self, uid: uuid::Uuid) -> MemeResult<models::Round> {
        self.repo.get_round(uid)
    }

    pub fn get_rounds_amount(&self, room_id: uuid::Uuid) -> MemeResult<u8> {
        self.repo.count_rounds(room_id)
    }

    pub fn create_situation(
        &self,
        situation_creator_id: uuid::Uuid,
        situation: String,
    ) -> MemeResult<()> {
        let mut round = self
            .repo
            .get_round_by_situation_creator_id(situation_creator_id)?;

        // Checking if it's right state
        if !matches!(round.state, RoundState::SituationCreation) {
            return Err(MemeError::InvalidStateToCreateSituation);
        }

        // Saving it and updating state
        round.situation = Some(situation);
        round.set_to_choose_memes()?;

        self.repo.update_round(round.clone())?;

        // Updating room's expiration timestamp
        let expiration_timestamp = SystemTime::now() + Config::new()?.time_to_choose_memes;
        RoomsRepository::new(self.repo.db)
            .update_room_expiration_timestamp(round.room_id, expiration_timestamp)
    }

    pub fn set_to_vote(&self, round_id: uuid::Uuid) -> MemeResult<()> {
        let mut round = self.repo.get_round(round_id)?;
        round.set_to_vote()?;
        self.repo.update_round(round.clone())?;

        // Updating room's expiration timestamp
        let expiration_timestamp = SystemTime::now() + Config::new()?.time_to_vote;
        RoomsRepository::new(self.repo.db)
            .update_room_expiration_timestamp(round.room_id, expiration_timestamp)
    }

    pub fn end_round(&self, round_id: uuid::Uuid) -> MemeResult<()> {
        let mut round = self.repo.get_round(round_id)?;
        round.end_round()?;
        self.repo.update_round(round.clone())?;

        // Updating room's expiration timestamp
        let expiration_timestamp = SystemTime::now() + Config::new()?.time_to_create_situation;
        RoomsRepository::new(self.repo.db)
            .update_room_expiration_timestamp(round.room_id, expiration_timestamp)
    }
}
