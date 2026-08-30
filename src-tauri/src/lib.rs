mod db;

use db::sqlite::{
    sqlite_connect, sqlite_disconnect, sqlite_execute_query, sqlite_get_all_schemas,
    sqlite_get_database_info, sqlite_get_table_schema, sqlite_get_tables, sqlite_test_connection,
    DbState,
};
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(DbState {
            pool: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            sqlite_connect,
            sqlite_disconnect,
            sqlite_execute_query,
            sqlite_get_tables,
            sqlite_get_table_schema,
            sqlite_get_all_schemas,
            sqlite_get_database_info,
            sqlite_test_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
