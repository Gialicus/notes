// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use store::store::get_db_pool;
use tauri::async_runtime::block_on;

mod modules;
mod service;
mod store;

fn main() {
    let db = get_db_pool();
    let db = block_on(db);
    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            modules::notes::note_cmd::add_note,
            modules::notes::note_cmd::browse_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
