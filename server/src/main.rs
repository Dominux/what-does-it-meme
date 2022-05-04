#![feature(drain_filter)]
#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

use apps::games::router::register_router as games_router;
use apps::players::router::register_router as players_router;
use apps::rooms::router::register_router as rooms_router;
use common::config::Config;
use common::cors::build_cors;
use common::db;

mod apps;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new().expect("Error on config setup");
    let db_pool = db::get_dbpool(config.db_url);

    HttpServer::new(move || {
        // Adding cors
        let cors = build_cors(config.allowed_origins.as_str());
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
