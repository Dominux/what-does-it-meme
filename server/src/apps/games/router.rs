use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{apps::games::services::GamesService, common::{db::DBPool, errors::AppResult}};
use crate::apps::games::models;

#[post("")]
async fn create_game(db_pool: web::Data<DBPool>) -> AppResult<models::Game> {
    let db = db_pool.get()?;
    let game = web::block(move || GamesService::new(&db).create_game()).await??;
    Ok(game)
}

#[get("/{id}")]
async fn get_game(path: web::Path<(u32,)>) -> impl Responder {
    format!("Fucked ur mom with id: {}", path.into_inner().0.to_string())
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(create_game).service(get_game));
}
