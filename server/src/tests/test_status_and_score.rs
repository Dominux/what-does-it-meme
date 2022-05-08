use std::time::{SystemTime, UNIX_EPOCH};

use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App, Error};
use lazy_static::lazy_static;

use crate::{
    apps::{
        games::{
            models::GameStatus, router::register_router as games_router, services::GameService,
        },
        players::{
            models::{InPlayer, Player},
            services::PlayersService,
        },
        rooms::{repository::RoomsRepository, services::RoomsService, state_enum::RoomState},
        rounds::{repository::RoundsRepository, state_enum::RoundState},
    },
    common::{
        config::Config,
        db::{get_dbpool, DBPool},
    },
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::new().unwrap();
        get_dbpool(config.db_url)
    };
}

#[actix_web::test]
async fn test_status_and_score() {
    // Creating the app
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("games").configure(games_router));
    let app = &mut test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Creating room
    let room = RoomsService::new(db)
        .create_room()
        .expect("Can't create room");

    let players_service = PlayersService::new(db);

    let mut players: Vec<Player> = vec![];

    // 1. Before the game starting
    {
        let status = get_status(&room.id, app).await;

        assert!(status.round.is_none());
        assert!(eq_timestamps(
            &status.expiration_timestamp,
            &room.expiration_timestamp
        ));
        assert!(matches!(status.state, RoomState::NotStarted));
        assert_eq!(
            status.players_names,
            players
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
        );

        for name in ["John", "Vasya", "PussyDestroyer", "underdog"] {
            // Generating memes for them
            let memes = ["meme 1", "meme 2"]
                .into_iter()
                .map(|meme| format!("{} {}", name, meme))
                .collect();

            let player = players_service
                .add_player(
                    InPlayer {
                        name: name.to_string(),
                        room_id: room.id,
                    },
                    memes,
                )
                .expect("Error on creating player");
            players.push(player);

            let status = get_status(&room.id, app).await;

            assert!(status.round.is_none());
            assert!(eq_timestamps(
                &status.expiration_timestamp,
                &room.expiration_timestamp
            ));
            assert!(matches!(status.state, RoomState::NotStarted));
            assert_eq!(
                status.players_names,
                players
                    .iter()
                    .map(|p| p.name.clone())
                    .collect::<Vec<String>>()
            );
        }
    }

    let rooms_repo = RoomsRepository::new(db);
    let rounds_repo = RoundsRepository::new(db);
    let game_service = GameService::new(db);

    // 2. When the game is running
    {
        // Starting game
        let round = game_service
            .start_game(room.id)
            .expect("Errro on starting game");

        let situation_creator_name = players
            .iter()
            .find(|p| p.id == round.situation_creator_id)
            .expect("Didn't find player")
            .name
            .clone();

        // 2.1. Checking status after the round was started
        let room = rooms_repo.get_room(room.id).expect("Error on getting room");
        let status = get_status(&room.id, app).await;
        assert!(matches!(status.state, RoomState::Started));
        assert!(eq_timestamps(
            &status.expiration_timestamp,
            &room.expiration_timestamp
        ),);
        assert_eq!(
            status.players_names,
            players
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
        );
        let status_round = status.round.expect("Should be Some");
        assert_eq!(status_round.round_number, 1);
        assert!(
            matches!(status_round.round_state, RoundState::SituationCreation),
            "{:?}, {:?}",
            status_round.round_state,
            RoundState::SituationCreation
        );
        assert_eq!(status_round.situation_creator_name, situation_creator_name);
        assert!(status_round.situation.is_none());
        assert!(status_round.reacted_players_names.is_none());
        assert!(status_round.memes.is_none());

        // Creating situation
        game_service
            .create_situation(round.id, round.situation_creator_id, "Lol joke".to_string())
            .expect("Error on creating situation");

        // 2.2. Checking status after situation creation
        let room = rooms_repo.get_room(room.id).expect("Error on getting room");
        let round = rounds_repo
            .get_round(round.id)
            .expect("Error on getting round");
        let status = get_status(&room.id, app).await;
        assert!(matches!(status.state, RoomState::Started));
        assert!(eq_timestamps(
            &status.expiration_timestamp,
            &room.expiration_timestamp
        ),);
        assert_eq!(
            status.players_names,
            players
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
        );
        let status_round = status.round.expect("Should be Some");
        assert_eq!(status_round.round_number, 1);
        assert!(
            matches!(status_round.round_state, RoundState::ChoosingMemes),
            "{:?}, {:?}",
            status_round.round_state,
            RoundState::SituationCreation
        );
        assert_eq!(status_round.situation_creator_name, situation_creator_name);
        assert_eq!(status_round.situation, round.situation);
        assert_eq!(status_round.reacted_players_names, Some(Vec::new()));
        assert!(status_round.memes.is_none());

        // Reacting with memes by two players
        let not_situation_creators: Vec<&Player> = players
            .iter()
            .filter(|p| p.id != round.situation_creator_id)
            .collect();
        for player in &not_situation_creators[..2] {
            let new_meme = format!("{} meme got in 1 round", player.name);
            game_service
                .react_with_meme(
                    player.memes_in_hand[0].clone(),
                    player.id,
                    round.id,
                    new_meme,
                )
                .expect("Error on reacting with a meme");
        }

        // 2.3. Checking status during choosing memes
        let status = get_status(&room.id, app).await;
        assert!(matches!(status.state, RoomState::Started));
        assert!(eq_timestamps(
            &status.expiration_timestamp,
            &room.expiration_timestamp
        ),);
        let status_round = status.round.expect("Should be Some");
        assert_eq!(status_round.round_number, 1);
        assert!(matches!(
            status_round.round_state,
            RoundState::ChoosingMemes
        ),);
        assert_eq!(status_round.situation_creator_name, situation_creator_name);
        assert_eq!(status_round.situation, round.situation);
        assert_eq!(
            status_round.reacted_players_names,
            Some(
                not_situation_creators[..2]
                    .iter()
                    .map(|p| p.name.clone())
                    .collect()
            )
        );
        assert!(status_round.memes.is_none());

        // Choosing a last meme
        let player = not_situation_creators[2];
        let new_meme = format!("{} meme got in 1 round", player.name);
        game_service
            .react_with_meme(
                player.memes_in_hand[0].clone(),
                player.id,
                round.id,
                new_meme,
            )
            .expect("Error on reacting with a meme");

        // 2.4. Checking status after last reaction in round
        let room = rooms_repo.get_room(room.id).expect("Error on getting room");
        let status = get_status(&room.id, app).await;
        assert!(matches!(status.state, RoomState::Started));
        assert!(eq_timestamps(
            &status.expiration_timestamp,
            &room.expiration_timestamp
        ),);
        let status_round = status.round.expect("Should be Some");
        assert_eq!(status_round.round_number, 1);
        assert!(matches!(status_round.round_state, RoundState::Voting),);
        assert_eq!(status_round.situation_creator_name, situation_creator_name);
        assert_eq!(status_round.situation, round.situation);
        // assert!(status_round.reacted_players_names.is_none());
        // let memes = memes_repo
        //     .list_memes(round.id)
        //     .expect("Error on getting memes")
        //     .into_iter()
        //     .map(|m| {
        //         Ok(GameStatusRoundMeme::new(
        //             m.link,
        //             Some(self.get_memes_voters_names(m.voters_ids, &players)?),
        //             None,
        //         ))
        //     })
        //     .collect::<MemeResult<_>>()?;
    }

    // TODO: create tests for left tests
}

/// Fixture to retreive statuses
async fn get_status(
    room_id: &uuid::Uuid,
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
) -> GameStatus {
    let req = test::TestRequest::get()
        .uri(format!("/games/status?room_id={}", room_id).as_str())
        .to_request();
    let response = test::call_service(app, req).await;
    assert_eq!(response.status(), 200, "{:?}", response.into_body());

    test::read_body_json(response).await
}

fn eq_timestamps(a: &SystemTime, b: &SystemTime) -> bool {
    a.duration_since(UNIX_EPOCH)
        .expect("Error on getting duration")
        .as_micros()
        == b.duration_since(UNIX_EPOCH)
            .expect("Error on getting duration")
            .as_micros()
}
