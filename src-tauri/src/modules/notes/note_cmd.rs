use tauri::State;

use crate::{modules::notes::note::Note, service::service::Bread, store::store::AppState};

#[tauri::command]
pub async fn add_note(title: &str, body: &str, ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    Note::add(
        &*db,
        Note {
            id: Default::default(),
            title: title.to_string(),
            body: body.to_string(),
        },
    )
    .await;
    Ok(String::from("{}"))
}

#[tauri::command]
pub async fn browse_note(ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    let result = Note::browse(&*db).await;
    let result = serde_json::to_string(&result).unwrap();
    Ok(format!("{result}"))
}
