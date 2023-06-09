use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub body: Option<String>,
}
