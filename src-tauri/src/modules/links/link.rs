use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Link {
    pub id: i32,
    pub source: i32,
    pub target: i32,
}
