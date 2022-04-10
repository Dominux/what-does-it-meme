use super::repository::RoundsRepository;
use super::state_enum::RoundState;
use crate::apps::rounds::models;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

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
}
