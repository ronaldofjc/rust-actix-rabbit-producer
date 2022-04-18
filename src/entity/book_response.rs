use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i32
}