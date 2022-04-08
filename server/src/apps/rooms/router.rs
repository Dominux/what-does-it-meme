use actix_web::{get, post, web, HttpResponse, http::StatusCode};
use uuid::Uuid;

use crate::{
    apps::rooms::services::RoomsService,
    common::{db::DBPool, errors::MemeResult},
};

#[post("")]
async fn create_room(db_pool: web::Data<DBPool>) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let room = web::block(move || RoomsService::new(&db).create_room()).await??;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(room))
}

#[get("/{id}")]
async fn get_room(db_pool: web::Data<DBPool>, path: web::Path<(Uuid,)>) -> MemeResult<HttpResponse> {
    let id = path.into_inner().0;
    let db = db_pool.get()?;
    let room = web::block(move || RoomsService::new(&db).get_room_by_id(id)).await??;
    Ok(HttpResponse::Ok().json(room))
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(create_room).service(get_room));
}
