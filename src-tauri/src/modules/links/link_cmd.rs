use tauri::State;

use crate::{service::service::Add, store::store::AppState};

use super::link::Link;

#[tauri::command]
pub async fn add_link(target: i32, source: i32, ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    Link::add(
        &*db,
        Link {
            id: Default::default(),
            target,
            source,
        },
    )
    .await;
    Ok(String::from("{}"))
}
