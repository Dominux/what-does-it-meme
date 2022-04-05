use diesel::prelude::PgConnection;
use diesel::r2d2;

type DBPool = r2d2::ConnectionManager<PgConnection>;

pub fn get_dbpool(connection_uri: String) -> r2d2::Pool<DBPool> {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(connection_uri);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool")
}
