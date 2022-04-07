use actix_web::{test, web, App};
use envconfig::Envconfig;
use lazy_static::lazy_static;

use crate::apps::games::models;
use crate::apps::games::router::register_router as games_router;
use crate::apps::games::state_enum::GameState;
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
async fn test_create_and_get_game() {
    let app = App::new()
        .app_data(web::Data::new(DB_POOL.clone()))
        .service(web::scope("games").configure(games_router));

    let mut app = test::init_service(app).await;

    // Creating game
    let game: models::Game = {
        let req = test::TestRequest::post().uri("/games").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 201, "Sht, status should be 201 nibba");

        test::read_body_json(response).await
    };

    assert!(std::matches!(game.state, GameState::NotStarted), "Game should be stopped");

    // Getting game
    {
        let req = test::TestRequest::get()
            .uri(format!("/games/{}", game.id).as_str())
            .to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), 200, "Sht, status should be 200 nibba");

        let game_from_get: models::Game = test::read_body_json(response).await;
        assert_eq!(game.id, game_from_get.id);
        assert!(std::matches!(game_from_get.state, GameState::NotStarted), "Game should be stopped");
    }
}
