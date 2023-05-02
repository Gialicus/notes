use tauri::State;

use crate::{
    modules::notes::note::Note,
    service::service::{Add, Browse, Delete, Edit, Read},
    store::store::AppState,
};

use super::note::UpdateNote;

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
pub async fn edit_note(id: &str, body: UpdateNote, ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    Note::edit(&*db, id, body).await;
    Ok(String::from("{}"))
}

#[tauri::command]
pub async fn browse_note(ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    let result = Note::browse(&*db).await;
    let result = serde_json::to_string(&result).unwrap();
    Ok(format!("{result}"))
}

#[tauri::command]
pub async fn read_note(id: &str, ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    let result = Note::read(&*db, id).await;
    match result {
        Some(r) => {
            let data = serde_json::to_string(&r).unwrap();
            Ok(format!("{data}"))
        }
        None => Ok(String::from("{}")),
    }
}

#[tauri::command]
pub async fn delete_note(id: &str, ctx: State<'_, AppState>) -> Result<String, ()> {
    let db = ctx.db.lock().await;
    Note::delete(&*db, id).await;
    Ok(String::from("{}"))
}
