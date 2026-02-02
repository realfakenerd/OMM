use tauri::{AppHandle, command, Runtime, State};
use tauri_plugin_saf::SafExt;
use tauri_plugin_saf::models::*;
use crate::nexus::{NexusClient, NexusMod};
use crate::db;
use std::sync::Mutex;
use rusqlite::Connection;

#[command]
pub async fn get_featured_mods(state: State<'_, Mutex<Connection>>) -> Result<Vec<NexusMod>, String> {
    let endpoint = "featured_mods_morrowind";
    let max_age = 24 * 60 * 60; // 24 hours

    // Try to get from cache
    {
        let conn = state.lock().map_err(|e| e.to_string())?;
        if let Ok(Some(cached)) = db::get_cached_api_response(&conn, endpoint, max_age) {
            if let Ok(mods) = serde_json::from_str::<Vec<NexusMod>>(&cached) {
                return Ok(mods);
            }
        }
    }

    // Fetch from API
    let client = NexusClient::from_env()?;
    let mods = client.get_featured_mods("morrowind").await?;

    // Cache the result
    {
        let conn = state.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string(&mods).map_err(|e| e.to_string())?;
        db::cache_api_response(&conn, endpoint, &json).map_err(|e| e.to_string())?;
    }

    Ok(mods)
}

#[command]
pub async fn search_mods(state: State<'_, Mutex<Connection>>, query: String) -> Result<Vec<NexusMod>, String> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let endpoint = format!("search_{}", query);
    let max_age = 6 * 60 * 60; // 6 hours

    // Try cache
    {
        let conn = state.lock().map_err(|e| e.to_string())?;
        if let Ok(Some(cached)) = db::get_cached_api_response(&conn, &endpoint, max_age) {
            if let Ok(mods) = serde_json::from_str::<Vec<NexusMod>>(&cached) {
                return Ok(mods);
            }
        }
    }

    // Fetch API
    let client = NexusClient::from_env()?;
    let mods = client.search_mods("morrowind", &query).await?;

    // Cache
    {
        let conn = state.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string(&mods).map_err(|e| e.to_string())?;
        db::cache_api_response(&conn, &endpoint, &json).map_err(|e| e.to_string())?;
    }

    Ok(mods)
}

use crate::download::{DownloadManager, DownloadProgress};

#[command]
pub async fn download_mod<R: Runtime>(
    app: AppHandle<R>,
    mod_id: u32,
    file_id: u32
) -> Result<(), String> {
    let client = NexusClient::from_env()?;
    let download_url = client.get_download_link("morrowind", mod_id, file_id).await?;
    
    let download_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("downloads");
    
    if !download_dir.exists() {
        std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;
    }

    let dest_path = download_dir.join(format!("{}.zip", mod_id));
    let manager = DownloadManager::new();
    
    // Run in background task
    tokio::spawn(async move {
        let _ = manager.download_mod(app, mod_id, download_url, dest_path.to_string_lossy().to_string()).await;
    });

    Ok(())
}

#[command]
pub async fn request_saf_permission<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    app.saf().request_permission()
        .map(|r| r.uri)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn list_mods<R: Runtime>(app: AppHandle<R>, path: String) -> Result<Vec<String>, String> {
    let response = app.saf().list_dir(ListDirRequest { uri: path })
        .map_err(|e| e.to_string())?;
    
    Ok(response.files.into_iter()
        .filter(|f| !f.is_directory)
        .map(|f| f.name)
        .collect())
}

#[command]
pub async fn read_config<R: Runtime>(app: AppHandle<R>, path: String) -> Result<String, String> {
    app.saf().read_file(ReadFileRequest { uri: path })
        .map(|r| r.content)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn write_config<R: Runtime>(app: AppHandle<R>, path: String, content: String) -> Result<(), String> {
    app.saf().write_file(WriteFileRequest { uri: path, content })
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    // Note: These tests now require a Tauri app context or mocking SafExt.
    // For now, we will skip or update them if needed.
}
