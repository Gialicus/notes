use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::service::service::Bread;

use super::note::Note;

#[async_trait]
impl Bread<Note> for Note {
    async fn browse(db: &Pool<Sqlite>) -> Vec<Note> {
        let res: Vec<Note> = sqlx::query_as::<_, Note>("SELECT * FROM notes")
            .fetch_all(db)
            .await
            .unwrap();
        res
    }

    async fn read(db: &Pool<Sqlite>, id: &str) -> Option<Note> {
        let res = sqlx::query_as::<_, Note>("SELECT * FROM notes")
            .fetch_one(db)
            .await;
        res.ok()
    }

    async fn edit(db: &Pool<Sqlite>, id: &str, t: Note) {
        todo!()
    }

    async fn add(db: &Pool<Sqlite>, t: Note) {
        sqlx::query("INSERT INTO notes (title,body) VALUES (?,?)")
            .bind(t.title)
            .bind(t.body)
            .execute(db)
            .await
            .unwrap();
    }

    async fn delete(db: &Pool<Sqlite>, id: &str) {
        todo!()
    }
}
