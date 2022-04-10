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

    // pub fn start_game(&self, id: uuid::Uuid) -> MemeResult<()> {
    //     let mut room = self.get_room_by_id(id)?;

    //     room.start_game()?;
    //     self.repo.update_room(room)?;

    //     Ok(())
    // }

    // pub fn end_game(&self, id: uuid::Uuid) -> MemeResult<()> {
    //     let mut room = self.get_room_by_id(id)?;

    //     room.end_game()?;
    //     self.repo.update_room(room)?;

    //     println!("{:?}", room);

    //     Ok(())
    // }
}
