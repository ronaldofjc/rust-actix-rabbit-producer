use actix_web::web;
use uuid::Uuid;
use crate::{Book, CreateBook};

pub fn execute(create_book: web::Json<CreateBook>) -> Book {
    let book = Book {
        id: Uuid::new_v4().to_string(),
        title: create_book.title.to_string(),
        author: create_book.author.to_string(),
        pages: create_book.pages
    };
    tracing::info!("New Book created, ID: {}", book.id);
    book
}