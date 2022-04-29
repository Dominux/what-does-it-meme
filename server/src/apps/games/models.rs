use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::apps::{rooms::state_enum::RoomState, rounds::state_enum::RoundState};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatusRoundMeme {
    pub meme_id: uuid::Uuid,
    pub link: String,
    pub voters_names: Option<Vec<String>>,
    pub author_name: Option<String>,
}

impl GameStatusRoundMeme {
    pub fn new(
        meme_id: uuid::Uuid,
        link: String,
        voters_names: Option<Vec<String>>,
        author_name: Option<String>,
    ) -> Self {
        Self {
            meme_id,
            link,
            voters_names,
            author_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatusRound {
    pub id: uuid::Uuid,
    pub round_number: u8,
    pub round_state: RoundState,
    pub situation_creator_name: String,
    pub situation: Option<String>,
    pub memes: Option<Vec<GameStatusRoundMeme>>,
    pub reacted_players_names: Option<Vec<String>>,
}

impl GameStatusRound {
    pub fn new(
        id: uuid::Uuid,
        round_number: u8,
        round_state: RoundState,
        situation_creator_name: String,
        situation: Option<String>,
        memes: Option<Vec<GameStatusRoundMeme>>,
        reacted_players_names: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            round_number,
            round_state,
            situation_creator_name,
            situation,
            memes,
            reacted_players_names,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
