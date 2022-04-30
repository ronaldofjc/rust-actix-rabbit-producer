use crate::{Book};
use crate::repository::BookModel;

pub fn execute(book: Book) -> Book {
    let book_model = BookModel {
        id: book.id.to_string(),
        title: book.title.to_string(),
        author: book.author.to_string(),
        pages: book.pages
    };
    tracing::info!("New Book created, ID: {}", book.id);
    Book::model_to_domain(book_model)
}