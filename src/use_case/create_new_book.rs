use crate::{use_case, repository, domain, Book};
use crate::entity::BookResponse;

pub fn execute(book: Book) -> BookResponse {
    let new_book = repository::create_new_book::execute(book);
    let event = domain::Book::domain_to_event(&new_book);
    use_case::publish_created_book_event::execute(event);
    domain::Book::domain_to_response(&new_book)
}