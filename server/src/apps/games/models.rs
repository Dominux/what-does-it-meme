use std::time::SystemTime;

use serde::Serialize;

use crate::apps::{rooms::state_enum::RoomState, rounds::state_enum::RoundState};

#[derive(Debug, Serialize)]
pub struct GameStatusRoundMeme {
    pub link: String,
    pub voters_names: Option<Vec<String>>,
    pub author_name: Option<String>,
}

impl GameStatusRoundMeme {
    pub fn new(
        link: String,
        voters_names: Option<Vec<String>>,
        author_name: Option<String>,
    ) -> Self {
        Self {
            link,
            voters_names,
            author_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GameStatusRound {
    pub round_number: u8,
    pub round_state: RoundState,
    pub situation_creator_name: String,
    pub situation: Option<String>,
    pub memes: Option<Vec<GameStatusRoundMeme>>,
    pub reacted_players_names: Option<Vec<String>>,
}

impl GameStatusRound {
    pub fn new(
        round_number: u8,
        round_state: RoundState,
        situation_creator_name: String,
        situation: Option<String>,
        memes: Option<Vec<GameStatusRoundMeme>>,
        reacted_players_names: Option<Vec<String>>,
    ) -> Self {
        Self {
            round_number,
            round_state,
            situation_creator_name,
            situation,
            memes,
            reacted_players_names,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GameStatus {
    pub state: RoomState,
    pub players_names: Vec<String>,
    pub round: Option<GameStatusRound>,
    pub expiration_timestamp: SystemTime,
}

impl GameStatus {
    pub fn new(
        state: RoomState,
        players_names: Vec<String>,
        round: Option<GameStatusRound>,
        expiration_timestamp: SystemTime,
    ) -> Self {
        Self {
            state,
            players_names,
            round,
            expiration_timestamp,
        }
    }
}
