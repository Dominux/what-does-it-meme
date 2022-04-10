use envconfig::Envconfig;

use super::errors::MemeResult;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "PGPORT")]
    pub db_port: u16,

    #[envconfig(from = "POSTGRES_USER")]
    pub db_playername: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub db_password: String,

    #[envconfig(from = "POSTGRES_DB")]
    pub db_name: String,

    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "PLAYERS_MINIMUM")]
    pub players_minimum: u8,

    #[envconfig(from = "PLAYERS_LIMIT")]
    pub players_limit: i64,

    #[envconfig(from = "ROUNDS_AMOUNT")]
    pub rounds_amount: u8,
}

impl Config {
    pub fn new() -> MemeResult<Self> {
        Ok(Self::init_from_env()?)
    }

    pub fn get_db_uri(&self) -> String {
        format!(
            "postgresql://{playername}:{password}@{host}:{port}/{dbname}",
            playername = self.db_playername,
            password = self.db_password,
            host = self.db_host,
            port = self.db_port,
            dbname = self.db_name,
        )
    }
}
