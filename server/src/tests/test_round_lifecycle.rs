use actix_web::{test, web, App};
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde_json::json;

use crate::apps::games::router::register_router as games_router;
use crate::apps::games::services::GameService;
use crate::apps::players::models::InPlayer;
use crate::apps::players::services::PlayersService;
use crate::apps::rooms::repository::RoomsRepository;
use crate::apps::rooms::services::RoomsService;
use crate::apps::rounds::services::RoundsService;
use crate::apps::rounds::state_enum::RoundState;
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
    let mut room = RoomsService::new(db)
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

    // 1. Trying to create situation before starting the game
    {
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

    let game_service = GameService::new(db);
    // Adding two more players
    let other_players = ["ur mom 2", "ur mom 3"].map(|player_name_no_one_cares_of| {
        let in_player = InPlayer {
            name: player_name_no_one_cares_of.to_string(),
            room_id: room.id,
        };
        PlayersService::new(db)
            .add_player(in_player)
            .expect("Error on user adding")
    });
    let round = game_service
        .start_game(room.id)
        .expect("Error on starting the game");

    // 2. Trying to create situation after the game started
    {
        let req = {
            let body = json!({
                "player_id": round.situation_creator_id,
                "situation": "Some hilarious joke"
            });
            test::TestRequest::post()
                .uri("/games/create_situation")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");

        // Checking the round state
        let round = game_service
            .get_general_status(room.id)
            .expect("Error on getting general status");
        assert!(matches!(round.round_state, RoundState::ChoosingMemes));
    }

    // 3. Trying to create situation in another round stage
    {
        let req = {
            let body = json!({
                "player_id": round.situation_creator_id,
                "situation": "Some hilarious joke"
            });
            test::TestRequest::post()
                .uri("/games/create_situation")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "Sht, status should be 423 nibba");
    }

    // 4. Trying to create situation in another round (or after the game ends)
    {
        let new_round = RoundsService::new(db)
            .create_round(room.id, round.situation_creator_id)
            .expect("Error on creating new round");
        room.current_round_id = Some(new_round.id);
        RoomsRepository::new(db)
            .update_room(room)
            .expect("Error on updating room");

        let req = {
            let body = json!({
                "player_id": round.situation_creator_id,
                "situation": "Some hilarious joke"
            });
            test::TestRequest::post()
                .uri("/games/create_situation")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "Sht, status should be 423 nibba");
    }

    // 5. Trying to create situation as not situation creator
    {
        let req = {
            let body = json!({
                "player_id": other_players[0].id,
                "situation": "Some hilarious joke"
            });
            test::TestRequest::post()
                .uri("/games/create_situation")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 404, "Sht, status should be 404 nibba");
    }
}
