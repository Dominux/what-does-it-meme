use std::time::SystemTime;

use actix_web::cookie::time::Duration;
use actix_web::{test, web, App};
use diesel::prelude::*;
use lazy_static::lazy_static;

use crate::apps::rooms::models::{self, Room};
use crate::apps::rooms::repository::RoomsRepository;
use crate::apps::rooms::router::register_router as rooms_router;
use crate::apps::rooms::schema::rooms;
use crate::apps::rooms::state_enum::RoomState;
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
async fn test_create_and_get_room() {
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("rooms").configure(rooms_router));

    let mut app = test::init_service(app).await;

    // Creating room
    let room: models::Room = {
        let req = test::TestRequest::post().uri("/rooms").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");

        test::read_body_json(response).await
    };

    assert!(
        std::matches!(room.state, RoomState::NotStarted),
        "Room should not started"
    );

    // Getting room
    {
        let req = test::TestRequest::get()
            .uri(format!("/rooms/{}", room.id).as_str())
            .to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 200, "Sht, status should be 200 nibba");

        let room_from_get: models::Room = test::read_body_json(response).await;
        assert_eq!(room.id, room_from_get.id);
        assert!(
            std::matches!(room_from_get.state, RoomState::NotStarted),
            "Room should not started"
        );
    }
}

#[actix_web::test]
async fn test_delete_expired_rooms() {
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("rooms").configure(rooms_router));

    let mut app = test::init_service(app).await;

    // Getting db
    let db = &DB_POOL.get().expect("Can't get db connection");

    // Cleaning rooms
    diesel::delete(rooms::table)
        .execute(db)
        .expect("Error on deleting rooms");

    let config = Config::new().expect("Error on creating config");

    // Creating max rooms
    let rooms_repo = RoomsRepository::new(db);
    let mut rooms_ids = Vec::new();
    for _ in 0..config.max_rooms_count {
        let new_room_id = rooms_repo.create_room().expect("Error on creating room").id;
        rooms_ids.push(new_room_id);
    }

    // 1. Trying to create one more
    {
        let req = test::TestRequest::post().uri("/rooms").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 403, "{:?}", response.into_body());
    }

    // Expiring one of them as started
    rooms_repo
        .update_room_expiration_timestamp(
            rooms_ids.remove(0),
            SystemTime::now() - config.time_to_start_the_game,
        )
        .expect("Error on updating room");

    // Expiring one of them as ended
    rooms_repo
        .update_room(Room {
            id: rooms_ids.remove(1),
            state: RoomState::Ended,
            current_round_id: None,
            expiration_timestamp: SystemTime::now() - config.time_until_room_deletion,
        })
        .expect("Error on updating room");

    // Expiring one of them as started, but abandoned
    rooms_repo
        .update_room(Room {
            id: rooms_ids.remove(2),
            state: RoomState::Started,
            current_round_id: None,
            expiration_timestamp: SystemTime::now() - Duration::MINUTE * 10,
        })
        .expect("Error on updating room");

    // 2. Trying to create a room again
    {
        let req = test::TestRequest::post().uri("/rooms").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 201, "{:?}", response.into_body());

        let room: models::Room = test::read_body_json(response).await;
        rooms_ids.push(room.id);

        // Checking if expired rooms were deleted
        let db_rooms_ids = rooms::table
            .select(rooms::id)
            .load::<uuid::Uuid>(db)
            .expect("Error on loading rooms ids");
        assert_eq!(rooms_ids, db_rooms_ids, "Rooms ids must be equal");
    }

    // Cleaning rooms for next tests
    diesel::delete(rooms::table)
        .execute(db)
        .expect("Error on deleting rooms");
}
