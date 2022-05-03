use actix_web::{test, web, App};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::json;

use crate::apps::games::router::register_router as games_router;
use crate::apps::games::services::GameService;
use crate::apps::memes::models::Meme;
use crate::apps::memes::repository::MemesRepository;
use crate::apps::players::models::{InPlayer, Player};
use crate::apps::players::router::register_router as players_router;
use crate::apps::players::services::PlayersService;
use crate::apps::rooms::repository::RoomsRepository;
use crate::apps::rooms::services::RoomsService;
use crate::apps::rounds::repository::RoundsRepository;
use crate::apps::rounds::services::RoundsService;
use crate::apps::rounds::state_enum::RoundState;
use crate::common::{
    config::Config,
    db::{get_dbpool, DBPool},
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::new().unwrap();
        get_dbpool(config.db_url)
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
        .add_player(in_player, Vec::new())
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
            .add_player(in_player, Vec::new())
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
        let round = RoundsRepository::new(db)
            .get_round(round.id)
            .expect("Error on refreshing round");
        assert!(matches!(round.state, RoundState::ChoosingMemes));
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

        assert!(
            response.status().is_client_error(),
            "Sht, status should be 404 or 423 nibba"
        );
    }
}

#[derive(Debug, Deserialize)]
struct ReactWithMemeJson {
    new_meme: String,
}

#[actix_web::test]
async fn test_react_with_memes() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("players").configure(players_router))
        .service(web::scope("games").configure(games_router));
    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Creating room
    let room = RoomsService::new(db)
        .create_room()
        .expect("Can't create room");

    // Adding first player
    let player: Player = {
        let body = json!({
            "name": "ur mom 1",
            "room_id": room.id,
        });

        let req = test::TestRequest::post()
            .uri("/players")
            .set_json(&body)
            .to_request();

        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");

        test::read_body_json(response).await
    };

    // 1. We can't react with a meme before starting the game
    //    cause we must pass a round id to the request body
    //    so we have no one before the game started

    // Adding two more players
    let mut all_players = vec![player];
    for name in ["ur mom 2", "ur mom 3"] {
        let in_player = InPlayer {
            name: name.to_string(),
            room_id: room.id,
        };
        let new_player = PlayersService::new(db)
            .add_player(in_player, vec!["test_meme".to_string()])
            .expect("Error on user adding");
        all_players.push(new_player)
    }

    // Starting the game
    let game_service = GameService::new(db);
    let mut round = game_service
        .start_game(room.id)
        .expect("Error on game starting");

    let situation_creator_index = all_players
        .iter()
        .position(|player| player.id == round.situation_creator_id)
        .expect("Error on finding situation creator among all players");
    let situation_creator = all_players.remove(situation_creator_index);

    // 2. Trying to react with a meme before the situation was created
    {
        let req = {
            let body = json!({
                "link": all_players[0].memes_in_hand[0],
                "player_id": all_players[0].id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "{:?}", response.into_body());
    }

    round
        .set_to_choose_memes()
        .expect("Error on setting round to choose memes");
    RoundsRepository::new(db)
        .update_round(round.clone())
        .expect("Error on round updating");

    // 3. Trying to react with a meme in the right moment of the round
    {
        let req = {
            let body = json!({
                "link": all_players[0].memes_in_hand[0],
                "player_id": all_players[0].id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 201, "{:?}", response.into_body());

        let json: ReactWithMemeJson = test::read_body_json(response).await;
        json.new_meme
    };

    // 4. Trying to react with a meme one more time
    {
        let req = {
            let body = json!({
                "link": all_players[0].memes_in_hand[0],
                "player_id": all_players[0].id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "{:?}", response.into_body());
    }

    // 5. Trying to react with a meme that is not in our collection
    {
        let req = {
            let body = json!({
                "link": "http://kindergarder-jokes.com/top-dankest-joke-ever",
                "player_id": all_players[0].id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 409, "{:?}", response.into_body());
    }

    // 6. Trying to react with a meme as situation creator
    {
        let req = {
            let body = json!({
                "link": situation_creator.memes_in_hand[0],
                "player_id": situation_creator.id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 401, "{:?}", response.into_body());
    }

    // 7. Trying to react with a meme the last time
    // (consider round to be setted to vote)
    {
        let req = {
            let body = json!({
                "link": all_players[1].memes_in_hand[0],
                "player_id": all_players[1].id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/react_with_meme")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 201, "{:?}", response.into_body());

        let _json: ReactWithMemeJson = test::read_body_json(response).await;

        // Checking if we came to the next stage
        let round = RoundsRepository::new(db)
            .get_round(round.id)
            .expect("Error on getting round");
        assert!(round.is_voting())
    };

    // 8. Trying to react with newly added meme in the next round
    // TODO
}

#[actix_web::test]
async fn test_vote() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("players").configure(players_router))
        .service(web::scope("games").configure(games_router));
    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Creating room
    let room = RoomsService::new(db)
        .create_room()
        .expect("Can't create room");

    // 1. We can't vote before starting the game
    //    cause we must pass a round id to the request body
    //    so we have no one before the game started

    // Adding all players
    let mut all_players = Vec::new();
    for name in ["ur_mom 1", "ur mom 2", "ur mom 3"] {
        let in_player = InPlayer {
            name: name.to_string(),
            room_id: room.id,
        };
        let new_player = PlayersService::new(db)
            .add_player(in_player, vec!["test_meme".to_string()])
            .expect("Error on user adding");
        all_players.push(new_player)
    }

    // Starting the game
    let game_service = GameService::new(db);
    let mut round = game_service
        .start_game(room.id)
        .expect("Error on game starting");

    // Getting situation creator outta all players
    let situation_creator_index = all_players
        .iter()
        .position(|player| player.id == round.situation_creator_id)
        .expect("Error on finding situation creator among all players");
    let situation_creator = all_players.remove(situation_creator_index);

    // 2. We can't vote before memes were created cause we need their ids to choose from

    // Filling memes and setting round to the voting stage
    let memes_repo = MemesRepository::new(db);
    let memes = [(all_players[0].id, "meme 1"), (all_players[1].id, "meme 2")].map(
        |(player_id, meme_link)| {
            let meme = Meme::new(round.id, player_id, meme_link.to_string());
            memes_repo
                .save_meme_if_not_exists(meme.clone())
                .expect("Error on saving meme");
            meme
        },
    );
    round
        .set_to_choose_memes()
        .expect("Error on setting round to choose memes");
    round.set_to_vote().expect("Error on setting round to vote");
    RoundsRepository::new(db)
        .update_round(round.clone())
        .expect("Error on round updating");

    // 3. Trying to vote in the right stage of the round
    {
        let req = {
            let body = json!({
                "meme_id": memes[0].id,
                "player_id": memes[1].player_id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/vote")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 204, "{:?}", response.into_body());
        assert_eq!(test::read_body(response).await, "")
    }

    // 4. Trying to vote one more time for the same meme
    {
        let req = {
            let body = json!({
                "meme_id": memes[0].id,
                "player_id": memes[1].player_id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/vote")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 423, "{:?}", response.into_body());
    }

    // 5. Trying to vote as a situation creator
    {
        let req = {
            let body = json!({
                "meme_id": memes[0].id,
                "player_id": situation_creator.id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/vote")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 401, "{:?}", response.into_body());
    }

    // 6. Trying to vote as a meme author
    {
        let req = {
            let body = json!({
                "meme_id": memes[0].id,
                "player_id": memes[0].player_id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/vote")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 401, "{:?}", response.into_body());
    }

    // 7. Trying to vote the last time
    {
        let req = {
            let body = json!({
                "meme_id": memes[1].id,
                "player_id": memes[0].player_id,
                "round_id": round.id
            });
            test::TestRequest::post()
                .uri("/games/vote")
                .set_json(&body)
                .to_request()
        };
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 204, "{:?}", response.into_body());

        // Checking if we came to the next stage
        let round = RoundsRepository::new(db)
            .get_round(round.id)
            .expect("Error on getting round");
        assert!(round.is_showing_results());
    }
}
