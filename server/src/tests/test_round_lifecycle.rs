use actix_web::{test, web, App};
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde_json::json;

use crate::apps::games::router::register_router as games_router;
use crate::apps::games::services::GameService;
use crate::apps::players::models::{self, InPlayer};
use crate::apps::players::services::PlayersService;
use crate::apps::rooms::services::RoomsService;
use crate::common::jwt_service::JWTService;
use crate::common::{
    config::Config,
    db::{get_dbpool, DBPool},
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::init_from_env().unwrap();
        get_dbpool(config.get_db_uri())
    };
}

#[actix_web::test]
async fn test_create_situation() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("games").configure(games_router));
    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Creating room
    let room = RoomsService::new(db)
        .create_room()
        .expect("Can't create room");

    // Adding first player
    let in_player = InPlayer {
        name: "ur mom 1".to_string(),
        room_id: room.id,
    };
    let player = PlayersService::new(db)
        .add_player(in_player)
        .expect("Error on user adding");

    // Trying to create situation before starting the game
    let req = {
        let body = json!({
            "player_id": player.id, 
            "situation": "What would u do if u come into ur mom's room and see how I fuck her hard?"
        });
        test::TestRequest::post()
            .uri("/games/create_situation")
            .set_json(&body)
            .to_request()
    };
    let response = test::call_service(&mut app, req).await;

    assert_eq!(response.status(), 404, "Sht, status should be 404 nibba");
}
