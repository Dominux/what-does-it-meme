use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid;

use crate::{
    apps::{
        games::services::{GameService, StatusService},
        memes::services::MemesService,
        rounds::models,
    },
    common::{config::Config, db::DBPool, errors::MemeResult},
};

#[derive(Deserialize)]
struct QueryRoom {
    room_id: uuid::Uuid,
}

#[post("/start")]
async fn start_game(
    db_pool: web::Data<DBPool>,
    query_room: web::Query<QueryRoom>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let round = web::block(move || GameService::new(&db).start_game(query_room.room_id)).await??;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(
        models::OutRound::from(round), // Players must not know ids of others, it's a private data
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////
///     Round moves
////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
struct CreateSituationJSON {
    round_id: uuid::Uuid,
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
    let round = web::block(move || {
        GameService::new(&db).create_situation(body.round_id, body.player_id, body.situation)
    })
    .await??;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(round))
}

#[derive(Deserialize)]
struct ReactWithMemeJSON {
    link: String,
    player_id: uuid::Uuid,
    round_id: uuid::Uuid,
}

#[post("/react_with_meme")]
async fn react_with_meme(
    db_pool: web::Data<DBPool>,
    body: web::Json<ReactWithMemeJSON>,
) -> MemeResult<HttpResponse> {
    // Getting new meme for a player
    let new_meme = MemesService::get_random_meme(&Config::new()?).await?;
    let new_meme_copy = new_meme.clone();

    let db = db_pool.get()?;
    let body = body.into_inner();
    web::block(move || {
        GameService::new(&db).react_with_meme(
            body.link,
            body.player_id,
            body.round_id,
            new_meme_copy,
        )
    })
    .await??;

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(json!({ "new_meme": new_meme })))
}

#[derive(Deserialize)]
struct VoteJSON {
    meme_id: uuid::Uuid,
    player_id: uuid::Uuid,
    round_id: uuid::Uuid,
}

#[post("/vote")]
async fn vote(db_pool: web::Data<DBPool>, body: web::Json<VoteJSON>) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let body = body.into_inner();
    web::block(move || GameService::new(&db).vote(body.round_id, body.meme_id, body.player_id))
        .await??;

    Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
}

#[get("/score")]
async fn get_score(
    db_pool: web::Data<DBPool>,
    query_room: web::Query<QueryRoom>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let score =
        web::block(move || StatusService::new(&db).calculate_scores(query_room.room_id)).await??;
    Ok(HttpResponse::Ok().status(StatusCode::OK).json(score))
}

#[get("/status")]
async fn get_status(
    db_pool: web::Data<DBPool>,
    query_room: web::Query<QueryRoom>,
) -> MemeResult<HttpResponse> {
    let db = db_pool.get()?;
    let status =
        web::block(move || StatusService::new(&db).get_status(query_room.room_id)).await??;
    Ok(HttpResponse::Ok().status(StatusCode::OK).json(status))
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(start_game)
            .service(create_situation)
            .service(react_with_meme)
            .service(vote)
            .service(get_score)
            .service(get_status),
    );
}
