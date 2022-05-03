use std::{env, str::FromStr, time::Duration};

use envconfig::Envconfig;

use super::errors::{MemeError, MemeResult};

#[derive(Envconfig)]
struct EnvConfig {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,

    #[envconfig(from = "MAX_ROOMS_COUNT")]
    pub max_rooms_count: u8,

    #[envconfig(from = "PLAYERS_MINIMUM")]
    pub players_minimum: u8,

    #[envconfig(from = "PLAYERS_LIMIT")]
    pub players_limit: i64,

    #[envconfig(from = "ROUNDS_AMOUNT")]
    pub rounds_amount: u8,

    #[envconfig(from = "MEMES_IN_HAND_AMOUNT")]
    pub memes_in_hand_amount: u8,

    #[envconfig(from = "KNOW_YOUR_MEME_BASEURL")]
    pub know_your_meme_baseurl: String,

    #[envconfig(from = "KNOW_YOUR_MEME_PAGEURL")]
    pub know_your_meme_pageurl_relative: String,

    #[envconfig(from = "KNOW_YOUR_MEME_PAGES")]
    pub know_your_meme_pages: u16,

    #[envconfig(from = "KNOW_YOUR_MEME_MEMEURL")]
    pub know_your_meme_memeurl_relative: String,
}

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub max_rooms_count: u8,
    pub players_minimum: u8,
    pub players_limit: i64,
    pub rounds_amount: u8,
    pub memes_in_hand_amount: u8,

    ///////////////////////////////////////
    //  Lifecycle
    ///////////////////////////////////////
    pub time_to_start_the_game: Duration,
    pub time_to_create_situation: Duration,
    pub time_to_choose_memes: Duration,
    pub time_to_vote: Duration,
    pub time_to_show_results: Duration,
    pub time_until_room_deletion: Duration,

    ///////////////////////////////////////
    //  KNOW YOUR MEME
    ///////////////////////////////////////
    pub know_your_meme_baseurl: String,
    pub know_your_meme_pageurl_relative: String,
    pub know_your_meme_pages: u16,
    pub know_your_meme_memeurl_relative: String,
}

impl Config {
    pub fn new() -> MemeResult<Self> {
        let env_config = EnvConfig::init_from_env()?;

        let time_to_start_the_game = Duration::from_secs(
            Self::get_env_var::<u64>("TIME_TO_START_THE_GAME_IN_MINUTES")? * 60,
        );
        let time_to_create_situation = Duration::from_secs(Self::get_env_var::<u64>(
            "TIME_TO_CREATE_SITUATION_IN_SECONDS",
        )?);
        let time_to_choose_memes =
            Duration::from_secs(Self::get_env_var::<u64>("TIME_TO_CHOOSE_MEMES_IN_SECONDS")?);
        let time_to_vote =
            Duration::from_secs(Self::get_env_var::<u64>("TIME_TO_VOTE_IN_SECONDS")?);
        let time_to_show_results =
            Duration::from_secs(Self::get_env_var::<u64>("TIME_TO_SHOW_RESULTS_IN_SECONDS")?);
        let time_until_room_deletion = Duration::from_secs(
            Self::get_env_var::<u64>("TIME_UNTIL_ROOM_DELETION_IN_MINUTES")? * 60,
        );

        Ok(Self {
            host: env_config.host,
            port: env_config.port,
            db_url: env_config.db_url,
            max_rooms_count: env_config.max_rooms_count,
            players_minimum: env_config.players_minimum,
            players_limit: env_config.players_limit,
            rounds_amount: env_config.rounds_amount,
            memes_in_hand_amount: env_config.memes_in_hand_amount,
            know_your_meme_baseurl: env_config.know_your_meme_baseurl,
            know_your_meme_pageurl_relative: env_config.know_your_meme_pageurl_relative,
            know_your_meme_pages: env_config.know_your_meme_pages,
            know_your_meme_memeurl_relative: env_config.know_your_meme_memeurl_relative,
            time_to_start_the_game,
            time_to_create_situation,
            time_to_choose_memes,
            time_to_vote,
            time_to_show_results,
            time_until_room_deletion,
        })
    }

    #[inline]
    fn get_env_var<T: FromStr>(env_var: &str) -> MemeResult<T> {
        env::var(env_var)?
            .parse::<T>()
            .map_err(|_| MemeError::Unknown)
    }
}
