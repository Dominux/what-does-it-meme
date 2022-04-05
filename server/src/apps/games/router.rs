use actix_web::{get, post, web, Responder};

// use crate::apps::games::models;

#[post("")]
async fn create_game() -> impl Responder {
    // Using some service to create the game
    "Lol"
}

#[get("/{id}")]
async fn get_game(path: web::Path<(u32,)>) -> impl Responder {
    format!("Fucked ur mom with id: {}", path.into_inner().0.to_string())
}


pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_game)
            .service(get_game)
    );
}
