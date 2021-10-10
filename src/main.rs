mod models;
use models::Status;

use actix_web::{HttpServer, App, Responder, web};

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status::new("UP".to_string()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
