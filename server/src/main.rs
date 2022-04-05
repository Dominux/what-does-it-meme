use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use envconfig::Envconfig;

mod config;

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
    })
    .bind((config.host, config.port))? 
    .run()
    .await
}
