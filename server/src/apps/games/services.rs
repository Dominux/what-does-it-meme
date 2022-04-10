use rand::{seq::SliceRandom, thread_rng};

use crate::{
    apps::{
        players::services::PlayersService,
        rooms::services::RoomsService,
        rounds::{models::Round, services::RoundsService},
    },
    common::{
        config::Config,
        db::DBConnection,
        errors::{MemeError, MemeResult},
    },
};

/// Main Game runner to manage rooms, rounds and players
pub struct GameService<'a> {
    rooms_service: RoomsService<'a>,
    rounds_service: RoundsService<'a>,
    players_service: PlayersService<'a>,
}

impl<'a> GameService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            rooms_service: RoomsService::new(db),
            rounds_service: RoundsService::new(db),
            players_service: PlayersService::new(db),
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

    fn get_random_player_id(&self, room_id: uuid::Uuid) -> MemeResult<uuid::Uuid> {
        let player_id = *self
            .players_service
            .list_players_ids(room_id)?
            .choose(&mut thread_rng())
            .ok_or(MemeError::TooLessPlayers)?;

        Ok(player_id)
    }
}
