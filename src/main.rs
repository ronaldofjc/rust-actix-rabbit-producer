use actix_web::{App, HttpServer};
use tracing_subscriber;
use crate::domain::Book;
use crate::entity::{CreateBook, Health};

mod api;
mod domain;
mod entity;
mod use_case;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8090".to_string());
    let address = format!("127.0.0.1:{}", port);

    tracing::info!("Starting server on {}", address);

    HttpServer::new(move || {
        App::new()
            .service(api::hello)
            .service(api::health)
            .service(api::create_book)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Couldn't start the server in port {}: {:?}", port, err)
    })
    .run()
    .await
}

