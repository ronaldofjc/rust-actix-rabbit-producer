use actix_web::web;
use crate::{CreateBook, use_case, repository, domain};
use crate::entity::BookResponse;

pub fn execute(create_book: web::Json<CreateBook>) -> BookResponse {
    let book = repository::create_new_book::execute(create_book);
    let event = domain::Book::domain_to_event(&book);
    use_case::publish_created_book_event::execute(event);
    let response = domain::Book::domain_to_response(&book);
    response
}