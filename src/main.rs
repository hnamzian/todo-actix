use actix_web::{HttpServer, App, Responder, web};

async fn status() -> impl Responder {
    "{\"status\": \"UP\"}"
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
