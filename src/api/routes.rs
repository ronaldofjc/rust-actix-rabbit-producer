use actix_web::{HttpResponse, web, get, post};
use crate::{CreateBook, Health, use_case, domain};

#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Producer Events with Rust and Actix Web it's working!")
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(Health { status: "Ok".to_string() })
}

#[post("/books")]
pub async fn create_book(create_book: web::Json<CreateBook>) -> HttpResponse {
    let book_model = domain::Book::dto_to_domain(create_book);
    let book = use_case::create_new_book::execute(book_model);
    HttpResponse::Ok().json(book)
}