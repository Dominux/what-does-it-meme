use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "PGPORT")]
    pub db_port: u16,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub db_password: String,
}
