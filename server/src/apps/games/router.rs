use actix_web::{get, post, web, Responder};

#[post("")]
async fn create_game() -> impl Responder {
    // Using some service to create the game
    "Lol"
}


pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_game)
    );
}
