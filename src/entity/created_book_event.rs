use serde::{Serialize, Deserialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct CreatedBookEvent {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i32
}