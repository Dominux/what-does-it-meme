use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use uuid::Uuid;

use crate::{
    apps::players::models::{Claims, InPlayer, PlayerWithMemes},
    apps::{memes::services::MemesService, players::services::PlayersService},
    common::{db::DBPool, errors::MemeResult},
};

#[post("")]
async fn add_player(
    db_pool: web::Data<DBPool>,
    in_player: web::Json<InPlayer>,
) -> MemeResult<HttpResponse> {
    let player = in_player.to_owned();
    let db = db_pool.get()?;

    // Adding player
    let player = web::block(move || PlayersService::new(&db).add_player(player)).await??;

    // Creating memes for him
    let memes = MemesService::get_random_memes().await?;
    let player_with_memes = PlayerWithMemes::new(player, memes);

    // Creating jwt token
    // TODO: implement mechanism to put all the player's memes inside jwt token and then check them everytime he tries to use one of them

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(player_with_memes))
}

#[get("/{id}")]
async fn get_player(
    db_pool: web::Data<DBPool>,
    path: web::Path<(Uuid,)>,
) -> MemeResult<HttpResponse> {
    let id = path.into_inner().0;
    let db = db_pool.get()?;
    let player = web::block(move || PlayersService::new(&db).get_player_by_id(id)).await??;
    Ok(HttpResponse::Ok().json(player))
}

pub fn register_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(add_player).service(get_player));
}
