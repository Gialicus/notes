use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::service::service::Add;

use super::link::Link;

#[async_trait]
impl Add<Link> for Link {
    async fn add(db: &Pool<Sqlite>, t: Link) {
        sqlx::query("INSERT INTO links (source,target) VALUES (?,?)")
            .bind(t.source)
            .bind(t.target)
            .execute(db)
            .await
            .unwrap();
    }
}
