use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use envconfig::Envconfig;

use apps::games::router::{register_router as games_router};

mod config;
mod common;
mod apps;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::init_from_env().unwrap();

    HttpServer::new(|| {
        // Adding cors
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("games").configure(games_router))
    })
    .bind((config.host, config.port))? 
    .run()
    .await
}
