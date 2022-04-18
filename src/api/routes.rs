use actix_web::{HttpResponse, web, get, post};
use crate::{CreateBook, Health, use_case};

#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("API Rust With Actix Web Works")
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(Health { status: "Ok".to_string() })
}

#[post("/books")]
pub async fn create_book(create_book: web::Json<CreateBook>) -> HttpResponse {
    let book = use_case::create_new_book::execute(create_book);
    HttpResponse::Ok().json(book)
}