use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
#[async_trait]
pub trait Bread<T> {
    async fn browse(db: &Pool<Sqlite>) -> Vec<T>;
    async fn read(db: &Pool<Sqlite>, id: &str) -> Option<T>;
    async fn edit(db: &Pool<Sqlite>, id: &str, t: T);
    async fn add(db: &Pool<Sqlite>, t: T);
    async fn delete(db: &Pool<Sqlite>, id: &str);
}
