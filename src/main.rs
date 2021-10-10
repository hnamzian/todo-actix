use actix_web::{HttpServer, App};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
