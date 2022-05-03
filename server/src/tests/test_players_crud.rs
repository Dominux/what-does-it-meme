use actix_web::{test, web, App};
use diesel::prelude::*;
use lazy_static::lazy_static;
use serde_json::json;

use crate::apps::players::models::InPlayer;
use crate::apps::players::router::register_router as players_router;
use crate::apps::players::services::PlayersService;
use crate::apps::rooms::schema::rooms;
use crate::apps::rooms::services::RoomsService;
use crate::common::{
    config::Config,
    db::{get_dbpool, DBPool},
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::new().unwrap();
        get_dbpool(config.db_url)
    };
    static ref NAMES: [&'static str; 6] =
        ["ur mom 1", "ur mom 2", "ur mom 3", "ur mom 4", "ur mom 5", "ur mom 6",];
}

#[actix_web::test]
async fn test_add_player() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("players").configure(players_router));
    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Cleaning rooms
    diesel::delete(rooms::table)
        .execute(db)
        .expect("Error on deleting rooms");

    {
        // Creating room
        let room = RoomsService::new(db)
            .create_room()
            .expect("Can't create room");

        // 1. Creating first player
        {
            let in_player = json!({
                "name": NAMES[0],
                "room_id": room.id,
            });

            let req = test::TestRequest::post()
                .uri("/players")
                .set_json(&in_player)
                .to_request();
            let response = test::call_service(&mut app, req).await;

            assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");
        };

        // 2. Trying to add more then limit players
        {
            // Creating players up to the limit
            for name in &NAMES[1..5] {
                let in_player = InPlayer {
                    name: name.to_string(),
                    room_id: room.id,
                };
                PlayersService::new(db)
                    .add_player(in_player, Vec::new())
                    .expect(format!("Can't create player with name \"{}\"", name).as_str());
            }

            // Creating one more player
            let in_player = json!({
                "name": NAMES[0],
                "room_id": room.id,
            });

            let req = test::TestRequest::post()
                .uri("/players")
                .set_json(&in_player)
                .to_request();
            let response = test::call_service(&mut app, req).await;

            assert_eq!(response.status(), 409, "Sht, status should be 409 nibba");
        }
    }

    // 3. Trying to add player if he has name that already there is inside the room
    {
        // Creating room
        let room = RoomsService::new(db)
            .create_room()
            .expect("Can't create room");

        // Creating player
        let name = "lol";

        let in_player = InPlayer {
            name: name.to_string(),
            room_id: room.id,
        };
        PlayersService::new(db)
            .add_player(in_player, Vec::new())
            .expect(format!("Can't create player with name \"{}\"", name).as_str());

        // Trying to create a player with the same name
        let in_player = json!({
            "name": name,
            "room_id": room.id,
        });
        let req = test::TestRequest::post()
            .uri("/players")
            .set_json(in_player)
            .to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 409, "Sht, status should be 409 nibba");
    }

    // 4. Trying to add player if the room already started game or after ended the game
    {
        // Creating room
        let mut room = RoomsService::new(db)
            .create_room()
            .expect("Can't create room");

        // Starting game
        room.start_game().expect("Can't start the game");
        let rooms_service = RoomsService::new(db);
        rooms_service
            .update_game(room)
            .expect("Can't update the game");

        // Trying to create a player
        let in_player = json!({
            "name": "lol",
            "room_id": room.id,
        });
        let req = test::TestRequest::post()
            .uri("/players")
            .set_json(in_player)
            .to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "Sht, status should be 423 nibba");

        // Ending game
        room.end_game().expect("Error on ending game");
        rooms_service
            .update_game(room)
            .expect("Error on updating game");

        // Trying to create a player
        let in_player = json!({
            "name": "lol",
            "room_id": room.id,
        });
        let req = test::TestRequest::post()
            .uri("/players")
            .set_json(in_player)
            .to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "Sht, status should be 423 nibba");
    }
}
