pub mod commands;
pub mod config;
pub mod nexus;
pub mod db;
pub mod download;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_saf::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            }
            let db_path = app_data_dir.join("cache.db");
            let conn = db::init_db(db_path).expect("failed to initialize database");
            app.manage(std::sync::Mutex::new(conn));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::request_saf_permission,
            commands::list_mods,
            commands::read_config,
            commands::write_config,
            commands::get_featured_mods,
            commands::search_mods,
            commands::download_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_modules_linked() {
        // Just checking visibility
    }
}