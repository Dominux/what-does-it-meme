use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    apps::{
        memes::{models::Meme, services::MemesService},
        players::{models::Player, services::PlayersService},
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

// score for one meme voter
const SCORE_COEFF: u16 = 100;

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

    // Method to vote
    pub fn vote(
        &self,
        round_id: uuid::Uuid,
        meme_id: uuid::Uuid,
        player_id: uuid::Uuid,
    ) -> MemeResult<()> {
        let room_id = {
            let round = self.rounds_service.get_round(round_id)?;

            // Validating round state
            if !round.is_voting() {
                return Err(MemeError::InvalidStateToVote);
            }

            // Validating if player is situation creator (he can't choose meme in this round)
            if round.situation_creator_id == player_id {
                return Err(MemeError::SituationCreatorCant("vote".to_string()));
            }

            // Validating if the voter of a meme is it's author
            let meme_reactor_id = self.memes_service.get_meme(meme_id)?.player_id;
            if player_id == meme_reactor_id {
                return Err(MemeError::PlayerCannotVoteForHisMeme);
            }

            round.room_id
        };

        // Saving vote
        self.memes_service.save_voter(meme_id, player_id)?;

        // Checking if all the players have already voted
        let limit_voters_amount = self.players_service.count_players(room_id)? - 1; // Substracting 1 cause 1 player is a situation creator
        let actual_voters_amount = self
            .memes_service
            .list_all_round_voters_ids(round_id)?
            .len() as u8;
        if limit_voters_amount == actual_voters_amount {
            self.rounds_service.end_round(round_id)?;

            // Creating next round, if it's None - then we need stop the game
            if matches!(self.next_round(room_id)?, None) {
                self.end_game(room_id)?
            }
        }
        Ok(())
    }

    /// Creating next round
    fn next_round(&self, room_id: uuid::Uuid) -> MemeResult<Option<Round>> {
        // Getting the room
        let mut room = self.rooms_service.get_room_by_id(room_id)?;

        // Creating a round
        let round = {
            // Checking if previous round was the last
            let rounds_amount_limit = Config::new()?.rounds_amount;
            let rounds_amount = self.rounds_service.get_rounds_amount(room_id)?;
            if rounds_amount_limit > rounds_amount {
                let players_ids = self.players_service.list_players_ids(room_id)?;

                // Choosing a situation creator
                let sutiation_creator_id = *players_ids
                    .choose(&mut thread_rng())
                    .ok_or(MemeError::TooLessPlayers)?;

                Some(
                    self.rounds_service
                        .create_round(room_id, sutiation_creator_id)?,
                )
            } else {
                None
            }
        };

        // Attaching the round to the room as the current one
        room.current_round_id = round.as_ref().map(|r| r.id);
        self.rooms_service.update_game(room)?;

        // Finally returning the new round
        Ok(round)
    }

    /// End the game
    fn end_game(&self, room_id: uuid::Uuid) -> MemeResult<()> {
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

    /// Method to calculate scores
    ///
    /// Returns dict with players names as keys and their scores as values
    pub fn calculate_scores(&self, room_id: uuid::Uuid) -> MemeResult<HashMap<String, u16>> {
        // Getting all the room's rounds and memes
        let rounds = self.rounds_service.list_rounds(room_id)?;
        let mut memes = self
            .memes_service
            .list_memes_by_rounds_ids(rounds.iter().map(|r| r.id).collect())?;

        // Firstable creating a hashmap with players names and zeros as their scores
        let mut scores: HashMap<uuid::Uuid, u16> = HashMap::new();

        // Then iterating over rounds
        for round in rounds {
            // Poping memes of the round
            let round_memes: Vec<Meme> = memes
                .drain_filter(|meme| meme.round_id == round.id)
                .collect();

            let mut first_score = 0_u16;
            let mut second_score = 0_u16;

            // Setting score for players, who reacted with a meme
            for meme in round_memes {
                let round_score = meme.voters_ids.len() as u16 * SCORE_COEFF;
                *scores.entry(meme.player_id).or_insert(0) += round_score;

                // Updating first and second scores
                if round_score > first_score {
                    second_score = first_score;
                    first_score = round_score;
                } else if round_score > second_score {
                    second_score = round_score;
                }
            }

            // Setting score for a situation creator
            // As an average score of two top round's scores, divided on 2
            let round_score = (first_score + second_score) / 4;
            *scores.entry(round.situation_creator_id).or_insert(0) += round_score;
        }

        // Substituting players ids with their names, cause their ids are sensitive data
        let mut players = self.players_service.list_players(room_id)?;
        let mut scores_with_names: HashMap<String, u16> = HashMap::with_capacity(players.len());
        for (player_id, score) in scores {
            let index = players
                .iter()
                .position(|p| p.id == player_id)
                .ok_or(MemeError::Unknown)?;
            let name = players.swap_remove(index).name;
            scores_with_names.insert(name, score);
        }
        Ok(scores_with_names)
    }
}
