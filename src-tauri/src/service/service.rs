use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

#[async_trait]
pub trait Browse<T> {
    async fn browse(db: &Pool<Sqlite>) -> Vec<T>;
}
#[async_trait]
pub trait Read<T> {
    async fn read(db: &Pool<Sqlite>, id: &str) -> Option<T>;
}
#[async_trait]
pub trait Edit<T> {
    async fn edit(db: &Pool<Sqlite>, id: &str, t: T);
}

#[async_trait]
pub trait Add<T> {
    async fn add(db: &Pool<Sqlite>, t: T);
}

#[async_trait]
pub trait Delete<T> {
    async fn delete(db: &Pool<Sqlite>, id: &str);
}
