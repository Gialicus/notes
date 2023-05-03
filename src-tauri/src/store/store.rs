use std::sync::Arc;

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tauri::async_runtime::Mutex;

const DB_URL: &str = "sqlite://../temp/sqlite.db";
pub struct AppState {
    pub db: Arc<Mutex<Pool<Sqlite>>>,
}

pub async fn get_db_pool() -> AppState {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    //notes
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS notes 
    (id INTEGER PRIMARY KEY NOT NULL, 
    title VARCHAR(250) NOT NULL,
    body TEXT NOT NULL);",
    )
    .execute(&db)
    .await
    .unwrap();
    //links
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS links 
    (id INTEGER PRIMARY KEY NOT NULL, 
    target INTEGER NOT NULL,
    source INTEGER NOT NULL,
    FOREIGN KEY(target) REFERENCES notes(id),
    FOREIGN KEY(source) REFERENCES notes(id));",
    )
    .execute(&db)
    .await
    .unwrap();

    let db = Arc::new(Mutex::new(db));
    AppState { db }
}
