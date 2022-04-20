use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use serde::Deserialize;
use uuid;

use crate::{
    apps::{games::services::GameService, rounds::models},
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
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(
        models::OutRound::from(round), // Players must not know ids of others, it's a private data
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////
///     Round moves
////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
struct CreateSituationJSON {
    player_id: uuid::Uuid,
    situation: String,
}

#[post("/create_situation")]
async fn create_situation(
    db_pool: web::Data<DBPool>,
    body: web::Json<CreateSituationJSON>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let body = body.into_inner();
    let round =
        web::block(move || GameService::new(&db).create_situation(body.player_id, body.situation))
            .await??;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(round))
}

////////////////////////////////////////////////////////////////////////////////////////////////
///     Statuses
////////////////////////////////////////////////////////////////////////////////////////////////

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
    cfg.service(
        web::scope("")
            .service(start_game)
            .service(create_situation)
            .service(get_general_status),
    );
}
