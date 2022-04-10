#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use envconfig::Envconfig;

use apps::games::router::register_router as games_router;
use apps::players::router::register_router as players_router;
use apps::rooms::router::register_router as rooms_router;
use common::config::Config;
use common::db;

mod apps;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init_from_env().unwrap();
    let db_pool = db::get_dbpool(config.get_db_uri());

    HttpServer::new(move || {
        // Adding cors
        let cors = Cors::default().allow_any_origin();
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors)
            .service(web::scope("rooms").configure(rooms_router))
            .service(web::scope("players").configure(players_router))
            .service(web::scope("games").configure(games_router))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests;
