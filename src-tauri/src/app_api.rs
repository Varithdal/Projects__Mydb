// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::db_api::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(_db: &Database) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
