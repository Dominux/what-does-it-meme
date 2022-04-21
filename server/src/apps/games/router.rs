use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid;

use crate::{
    apps::{
        games::services::GameService, memes::services::MemesService, players::models::Claims,
        rounds::models,
    },
    common::{
        config::Config,
        db::DBPool,
        errors::{MemeError, MemeResult},
        headers::AuthorizationHeader,
        jwt_service::JWTService,
    },
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

#[derive(Deserialize)]
struct ReactWithMemeJSON {
    link: String,
    player_id: uuid::Uuid,
    room_id: uuid::Uuid,
    round_id: uuid::Uuid,
}

#[post("/react_with_meme")]
async fn react_with_meme(
    auth_header: web::Header<AuthorizationHeader>,
    db_pool: web::Data<DBPool>,
    body: web::Json<ReactWithMemeJSON>,
) -> MemeResult<HttpResponse> {
    // Trying to pop meme from token, on error - Throwing error
    let secret = Config::new()?.secret;
    let jwt_service = JWTService::new(secret.as_str());
    let mut memes_in_hands = jwt_service
        .decode::<Claims>(auth_header.token.as_str())?
        .memes_in_hands;
    let index = memes_in_hands
        .iter()
        .position(|link| *link == body.link)
        .ok_or(MemeError::MemeIsNotInHand)?;
    memes_in_hands.remove(index);

    let db = db_pool.get()?;
    let body = body.into_inner();
    web::block(move || {
        GameService::new(&db).react_with_meme(
            body.link,
            body.player_id,
            body.round_id,
            body.room_id,
        )
    })
    .await??;

    // Getting new meme for a player, adding to his hand
    let new_meme = MemesService::get_random_meme().await?;
    memes_in_hands.push(new_meme);
    let claims = Claims::new(memes_in_hands);
    let new_token = jwt_service.encode(&claims)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(json!({ "token": new_token })))
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
            .service(react_with_meme)
            .service(get_general_status),
    );
}
