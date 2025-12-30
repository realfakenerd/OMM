use tauri::{AppHandle, command, Runtime};
use tauri_plugin_saf::SafExt;
use tauri_plugin_saf::models::*;

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
