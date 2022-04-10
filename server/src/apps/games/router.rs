use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use uuid::Uuid;

use crate::{
    apps::games::services::GameService,
    common::{db::DBPool, errors::MemeResult},
};

#[post("/start")]
async fn start_game(
    db_pool: web::Data<DBPool>,
    room_id: web::Query<uuid::Uuid>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let round =
        web::block(move || GameService::new(&db).start_game(room_id.into_inner())).await??;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(round))
}

#[get("/status")]
async fn get_general_status(
    db_pool: web::Data<DBPool>,
    room_id: web::Query<uuid::Uuid>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let status = web::block(move || GameService::new(&db).get_general_status(room_id.into_inner()))
        .await??;
    Ok(HttpResponse::Ok().status(StatusCode::OK).json(status))
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(start_game));
}
