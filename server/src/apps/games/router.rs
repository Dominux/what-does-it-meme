use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::{apps::games::services::GamesService, common::{db::DBPool, errors::AppResult}};

#[post("")]
async fn create_game(db_pool: web::Data<DBPool>) -> AppResult<HttpResponse> {
    let db = db_pool.get()?;
    let game = web::block(move || GamesService::new(&db).create_game()).await??;
    Ok(HttpResponse::Ok().json(game))
}

#[get("/{id}")]
async fn get_game(db_pool: web::Data<DBPool>, path: web::Path<(Uuid,)>) -> AppResult<HttpResponse> {
    let id = path.into_inner().0;
    let db = db_pool.get()?;
    let game = web::block(move || GamesService::new(&db).get_game_by_id(id)).await??;
    Ok(HttpResponse::Ok().json(game))
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(create_game).service(get_game));
}
