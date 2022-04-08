use actix_web::{test, web, App};
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde_json::json;

use crate::apps::players::models;
use crate::apps::rooms::services::RoomsService;
use crate::apps::players::router::register_router as players_router;
use crate::common::{
    config::Config,
    db::{get_dbpool, DBPool},
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::init_from_env().unwrap();
        get_dbpool(config.get_db_uri())
    };

    static ref NAMES: [&'static str; 6] = [
        "ur mom 1",
        "ur mom 2",
        "ur mom 3",
        "ur mom 4",
        "ur mom 5",
        "ur mom 6",
    ];
}

#[actix_web::test]
async fn test_players_crud() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("players").configure(players_router));
    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Creating room
    let room = RoomsService::new(db)
        .create_room()
        .expect("Can't create room");

    // Creating first player
    let player_1 = {
        let in_player = json!({
            "name": NAMES[0],
            "room_id": room.id,
        });

        let req = test::TestRequest::post().uri("/players").set_json(&in_player).to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");
        
        let player: models::Player = test::read_body_json(response).await;
        assert_eq!(player.name, in_player["name"]);
        assert_eq!(player.room_id, room.id);
    };

    // TODO: add cases for:
    // 1. Trying to add more then limit players
    // 2. Trying to add player if he has name that already there is inside the room
    // 3. Trying to add player if the room already started game or after ended the game

}
