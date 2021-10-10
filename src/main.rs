mod models;
use models::Status;

mod configs;
use configs::Config;

use dotenv::dotenv;
use actix_web::{HttpServer, App, Responder, web};

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status::new("UP".to_string()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
