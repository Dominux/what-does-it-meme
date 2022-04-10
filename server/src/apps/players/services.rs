use super::models::InPlayer;
use super::repository::PlayersRepository;
use crate::apps::rooms::repository::RoomsRepository;
use crate::apps::rooms::state_enum::RoomState;
use crate::apps::players::models;
use crate::common::config::Config;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

pub struct PlayersService<'a> {
    repo: PlayersRepository<'a>,
}

impl<'a> PlayersService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: PlayersRepository::new(db),
        }
    }

    pub fn add_player(&self, in_player: InPlayer) -> MemeResult<models::Player> {
        // Forbidding enterring room if the game already started
        {
            let room = RoomsRepository::new(self.repo.db).get_room(in_player.room_id)?;
            match room.state {
                RoomState::NotStarted => (),
                _ => return Err(MemeError::EnterringRoomAfterStart),
            };
        }

        {
            let players = self.repo.list_room_players(in_player.room_id)?;

            // Forbidding enterring room if players limit was already achieved
            if players.len() as i64 == Config::new()?.players_limit {
                return Err(MemeError::AchivedPlayersLimit)
            }
            
            // Forbidding enterring room if player's desired name already there is in the room
            for player in players {
                if player.name == in_player.name {
                    return Err(MemeError::DuplicatedName)
                }
            }
        }

        self.repo.create(in_player)
    }

    pub fn list_players_ids(&self, room_id: uuid::Uuid) -> MemeResult<Vec<uuid::Uuid>> {
        self.repo.list_players_ids(room_id)
    }

    pub fn get_player_by_id(&self, id: uuid::Uuid) -> MemeResult<models::Player> {
        self.repo.get_player(id)
    }
}
