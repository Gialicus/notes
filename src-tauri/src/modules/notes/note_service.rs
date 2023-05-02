use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::service::service::{Add, Browse, Delete, Edit, Read};

use super::note::{Note, UpdateNote};

#[async_trait]
impl Browse<Note> for Note {
    async fn browse(db: &Pool<Sqlite>) -> Vec<Note> {
        let res: Vec<Note> = sqlx::query_as::<_, Note>("SELECT * FROM notes")
            .fetch_all(db)
            .await
            .unwrap();
        res
    }
}

#[async_trait]
impl Read<Note> for Note {
    async fn read(db: &Pool<Sqlite>, id: &str) -> Option<Note> {
        let res = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id=?")
            .bind(id)
            .fetch_one(db)
            .await;
        res.ok()
    }
}

#[async_trait]
impl Edit<UpdateNote> for Note {
    async fn edit(db: &Pool<Sqlite>, id: &str, dto: UpdateNote) {
        sqlx::query("UPDATE notes SET title=?, body=? WHERE id=?")
            .bind(dto.title)
            .bind(dto.body)
            .bind(id)
            .execute(db)
            .await
            .unwrap();
    }
}

#[async_trait]
impl Add<Note> for Note {
    async fn add(db: &Pool<Sqlite>, t: Note) {
        sqlx::query("INSERT INTO notes (title,body) VALUES (?,?)")
            .bind(t.title)
            .bind(t.body)
            .execute(db)
            .await
            .unwrap();
    }
}

#[async_trait]
impl Delete<Note> for Note {
    async fn delete(db: &Pool<Sqlite>, id: &str) {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
            .unwrap();
    }
}
