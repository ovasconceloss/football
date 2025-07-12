mod core;
mod application;
mod infrastructure;

use application::commands::save::new_save;
use crate::infrastructure::database::connection::create_connection;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let connection = create_connection().await;

    tauri::Builder::default()
        .manage(connection)
        .invoke_handler(tauri::generate_handler![
            new_save
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}