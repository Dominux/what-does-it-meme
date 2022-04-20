use std::time::SystemTime;

use serde::Serialize;

use crate::apps::rounds::state_enum::RoundState;

#[derive(Debug, Serialize)]
pub struct GeneralGameStatus {
    pub round_number: u8,
    pub round_state: RoundState,
    pub expiration_timestamp: SystemTime,
}

impl GeneralGameStatus {
    pub fn new(
        round_number: u8,
        round_state: RoundState,
        expiration_timestamp: SystemTime,
    ) -> Self {
        Self {
            round_number,
            round_state,
            expiration_timestamp,
        }
    }
}
