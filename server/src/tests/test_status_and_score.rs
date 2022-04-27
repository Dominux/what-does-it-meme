use std::time::{SystemTime, UNIX_EPOCH};

use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App, Error};
use lazy_static::lazy_static;

use crate::apps::games::models::GameStatus;
use crate::apps::games::router::register_router as games_router;
use crate::apps::players::models::{InPlayer, Player};
use crate::apps::players::services::PlayersService;
use crate::apps::rooms::services::RoomsService;
use crate::apps::rooms::state_enum::RoomState;
use crate::common::{
    config::Config,
    db::{get_dbpool, DBPool},
};

lazy_static! {
    static ref DB_POOL: DBPool = {
        let config = Config::new().unwrap();
        get_dbpool(config.get_db_uri())
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
    let mut room = RoomsService::new(db)
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
            let player = players_service
                .add_player(
                    InPlayer {
                        name: name.to_string(),
                        room_id: room.id,
                    },
                    Vec::new(),
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

    // 2. When the game is running
    let rounds_service = RoomsService::new(db);
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
        .as_millis()
        == b.duration_since(UNIX_EPOCH)
            .expect("Error on getting duration")
            .as_millis()
}
