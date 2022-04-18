use serde::{Serialize, Deserialize};
use crate::entity::{BookResponse, CreatedBookEvent};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i32
}

impl Book {
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