use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: add getting host and port
    HttpServer::new(|| {
        // Adding cors
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))? 
    .run()
    .await
}
