use actix_web::{web, App, HttpServer};

pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").service(api::v1::service()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
