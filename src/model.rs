use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub created_at: u64,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}
