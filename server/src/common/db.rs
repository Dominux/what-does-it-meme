use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DBConnection = PgConnection;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_dbpool(connection_uri: String) -> DBPool {
    let manager = r2d2::ConnectionManager::<DBConnection>::new(connection_uri);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool")
}
