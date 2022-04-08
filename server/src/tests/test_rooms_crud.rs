use actix_web::{test, web, App};
use envconfig::Envconfig;
use lazy_static::lazy_static;

use crate::apps::rooms::models;
use crate::apps::rooms::router::register_router as rooms_router;
use crate::apps::rooms::state_enum::RoomState;
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
        "Room should be stopped"
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
            "Room should be stopped"
        );
    }
}
