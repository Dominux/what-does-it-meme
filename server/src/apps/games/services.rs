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
            let sutiation_creator_id = *players_ids
                .choose(&mut thread_rng())
                .ok_or(MemeError::TooLessPlayers)?;

            self.rounds_service
                .create_round(room_id, sutiation_creator_id)?
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
        new_meme: String,
    ) -> MemeResult<bool> {
        let room_id = {
            let round = self.rounds_service.get_round(round_id)?;

            // Validating round state
            if !round.is_choosing_memes() {
                return Err(MemeError::InvalidStateToReactWithMeme);
            }

            // Validating if player is situation creator (he can't choose meme in this round)
            if round.situation_creator_id == player_id {
                return Err(MemeError::SituationCreatorCant("choose memes".to_string()));
            }

            round.room_id
        };

        // Checking if the player has the meme in his hands
        let mut players_memes = self
            .players_service
            .get_player_by_id(player_id)?
            .memes_in_hand;

        if !players_memes.contains(&link) {
            return Err(MemeError::MemeIsNotInHand);
        }

        // Saving meme
        let meme = Meme::new(round_id, player_id, link);
        let is_created = self.memes_service.save_meme_if_not_exists(meme)?;
        if !is_created {
            return Err(MemeError::AlreadyReactedWithMeme);
        }

        // Checking if all the players have already reacted
        let players_count = self.players_service.count_players(room_id)?;
        let memes_count = self.memes_service.count_memes(round_id)?;
        let max_memes_limit = players_count - 1; // Substracting 1 cause 1 player is a situation creator
        if max_memes_limit == memes_count {
            self.rounds_service.set_to_vote(round_id)?
        }

        // Adding meme to player's hand
        players_memes.push(new_meme);
        self.players_service
            .update_players_memes(player_id, players_memes)?;

        Ok(true)
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
