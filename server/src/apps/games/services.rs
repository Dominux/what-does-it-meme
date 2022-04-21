use rand::{seq::SliceRandom, thread_rng};

use crate::{
    apps::{
        memes::{models::Meme, services::MemesService},
        players::services::PlayersService,
        rooms::{services::RoomsService, state_enum::RoomState},
        rounds::{models::Round, services::RoundsService},
    },
    common::{
        config::Config,
        db::DBConnection,
        errors::{MemeError, MemeResult},
    },
};

use super::models::GeneralGameStatus;

/// Main Game runner to manage rooms, rounds and players
pub struct GameService<'a> {
    rooms_service: RoomsService<'a>,
    rounds_service: RoundsService<'a>,
    players_service: PlayersService<'a>,
    memes_service: MemesService<'a>,
}

impl<'a> GameService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            rooms_service: RoomsService::new(db),
            rounds_service: RoundsService::new(db),
            players_service: PlayersService::new(db),
            memes_service: MemesService::new(db),
        }
    }

    /// Starting the game and return first round
    pub fn start_game(&self, room_id: uuid::Uuid) -> MemeResult<Round> {
        // Getting the room
        let mut room = self.rooms_service.get_room_by_id(room_id)?;

        // Creating a round
        let round = {
            // Validating amount of players
            let players_ids = self.players_service.list_players_ids(room_id)?;
            if players_ids.len() < Config::new()?.players_minimum as usize {
                return Err(MemeError::TooLessPlayers);
            }

            // Choosing a situation creator
            let sutiation_creator = *players_ids
                .choose(&mut thread_rng())
                .ok_or(MemeError::TooLessPlayers)?;

            self.rounds_service
                .create_round(room_id, sutiation_creator)?
        };

        // Attaching the round to the room as the current one
        room.current_round_id = Some(round.id);

        // Starting room game
        room.start_game()?;
        self.rooms_service.update_game(room)?;

        // Finally returning the new round
        Ok(round)
    }

    /// Method to create a situation
    pub fn create_situation(
        &self,
        situation_creator_id: uuid::Uuid,
        situation: String,
    ) -> MemeResult<()> {
        self.rounds_service
            .create_situation(situation_creator_id, situation)
    }

    /// Method to react on situation with meme
    pub fn react_with_meme(
        &self,
        link: String,
        player_id: uuid::Uuid,
        round_id: uuid::Uuid,
        room_id: uuid::Uuid,
    ) -> MemeResult<()> {
        // Saving meme
        let meme = Meme::new(round_id, player_id, link);
        self.memes_service.save_meme(meme)?;

        // Checking if all the players have already reacted
        let players_count = self.players_service.count_players(room_id)?;
        let memes_count = self.memes_service.count_memes(round_id)?;
        if players_count == memes_count {
            self.rounds_service.set_to_vote(round_id)?
        }

        Ok(())
    }

    /// Ending the game and return first round
    pub fn end_game(&self, room_id: uuid::Uuid) -> MemeResult<()> {
        // Getting the room
        let mut room = self.rooms_service.get_room_by_id(room_id)?;

        // TODO: add logic

        // Ending
        room.end_game()?;
        self.rooms_service.update_game(room)?;

        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////////////////
    /// Statuses
    //////////////////////////////////////////////////////////////////////////////////////////

    /// Returns general game status
    pub fn get_general_status(&self, room_id: uuid::Uuid) -> MemeResult<GeneralGameStatus> {
        let room = self.rooms_service.get_room_by_id(room_id)?;

        // Returning 404 if state isn't Started
        if !matches!(room.state, RoomState::Started) {
            return Err(MemeError::NotFound);
        }

        let round_number = self.rounds_service.get_rounds_amount(room_id)?;

        let round = self
            .rounds_service
            .get_round(room.current_round_id.ok_or(MemeError::Unknown)?)?;

        Ok(GeneralGameStatus::new(
            round_number,
            round.state,
            room.expiration_timestamp,
        ))
    }
}
