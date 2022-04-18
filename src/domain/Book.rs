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
            id: self.id.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            pages: self.pages.clone()
        }
    }

    pub fn domain_to_response(&self) -> BookResponse {
        BookResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            pages: self.pages.clone()
        }
    }
}