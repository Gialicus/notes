// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use menu::menu::{build_app_menu, handle_app_menu};
use store::store::get_db_pool;
use tauri::{async_runtime::block_on, Manager};

mod menu;
mod modules;
mod service;
mod store;

fn main() {
    let db = get_db_pool();
    let db = block_on(db);
    let menu = build_app_menu();
    tauri::Builder::default()
        .manage(db)
        .menu(menu)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.emit("event-name", "Tauri is awesome!").unwrap();
            Ok(())
        })
        .on_menu_event(|event| handle_app_menu(event))
        .invoke_handler(tauri::generate_handler![
            modules::notes::note_cmd::add_note,
            modules::notes::note_cmd::browse_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
