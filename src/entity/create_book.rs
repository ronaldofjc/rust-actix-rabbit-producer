use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub pages: i32
}