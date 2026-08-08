use std::fs;

use tauri::Manager;

use crate::database::{Database, DATABASE_FILENAME};

mod commands;
mod database;
mod errors;

struct AppState {
    database: Database,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_local_data_dir = app.path().app_local_data_dir()?;

            fs::create_dir_all(&app_local_data_dir)?;

            let db_path = app_local_data_dir.join(DATABASE_FILENAME);
            let db_path = db_path.to_str().expect("cannot get db path");

            let database =
                tauri::async_runtime::block_on(async { Database::build(db_path).await })?;

            let app_state = AppState { database };

            app.manage(app_state);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::select_all_items_not_archived,
            commands::insert_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
