use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "PGPORT")]
    pub db_port: u16,

    #[envconfig(from = "POSTGRES_USERNAME")]
    pub db_username: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub db_password: String,

    #[envconfig(from = "DB_NAME")]
    pub db_name: String,
}

impl Config {
    pub fn get_db_uri(&self) -> String {
        format!(
            "postgresql://{username}:{password}@{host}:{port}/{dbname}]",
            username = self.db_username,
            password = self.db_password,
            host = self.host,
            port = self.db_port,
            dbname = self.db_name,
        )
    }
}
