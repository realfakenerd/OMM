// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tes_modloader::commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::request_saf_permission,
            commands::list_mods,
            commands::read_config,
            commands::write_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}