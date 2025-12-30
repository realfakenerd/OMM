use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use std::fs;
use std::path::Path;

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Saf<R>> {
  Ok(Saf(app.clone()))
}

/// Access to the saf APIs.
pub struct Saf<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Saf<R> {
  pub fn request_permission(&self) -> crate::Result<RequestPermissionResponse> {
    // On desktop, we don't have a direct equivalent of SAF tree selection here
    // In a real app, this might open a folder picker.
    // For now, return a dummy URI or just succeed.
    Ok(RequestPermissionResponse {
        uri: "/".to_string(),
    })
  }

  pub fn check_permission(&self, _payload: CheckPermissionRequest) -> crate::Result<CheckPermissionResponse> {
    // Desktop usually has permission if the app can run
    Ok(CheckPermissionResponse {
        granted: true,
    })
  }

  pub fn list_dir(&self, payload: ListDirRequest) -> crate::Result<ListDirResponse> {
    let path = Path::new(&payload.uri);
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        files.push(FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            uri: entry.path().to_string_lossy().to_string(),
        });
    }
    Ok(ListDirResponse { files })
  }

  pub fn read_file(&self, payload: ReadFileRequest) -> crate::Result<ReadFileResponse> {
    let content = fs::read_to_string(&payload.uri)?;
    Ok(ReadFileResponse { content })
  }

  pub fn write_file(&self, payload: WriteFileRequest) -> crate::Result<()> {
    fs::write(&payload.uri, &payload.content)?;
    Ok(())
  }
}
