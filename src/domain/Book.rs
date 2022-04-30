use actix_web::web;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::CreateBook;
use crate::entity::{BookResponse, CreatedBookEvent};
use crate::repository::BookModel;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i32
}

impl Book {
    pub fn dto_to_domain(create_book: web::Json<CreateBook>) -> Book {
        Book {
            id: Uuid::new_v4().to_string(),
            title: create_book.title.to_owned(),
            author: create_book.author.to_owned(),
            pages: create_book.pages.to_owned()
        }
    }

    pub fn model_to_domain(book_model: BookModel) -> Book {
        Book {
            id: book_model.id.to_owned(),
            title: book_model.title.to_owned(),
            author: book_model.author.to_owned(),
            pages: book_model.pages.to_owned()
        }
    }

    pub fn domain_to_event(&self) -> CreatedBookEvent {
        CreatedBookEvent {
            id: self.id.to_owned(),
            title: self.title.to_owned(),
            author: self.author.to_owned(),
            pages: self.pages.to_owned()
        }
    }

    pub fn domain_to_response(&self) -> BookResponse {
        BookResponse {
            id: self.id.to_owned(),
            title: self.title.to_owned(),
            author: self.author.to_owned(),
            pages: self.pages.to_owned()
        }
    }
}