use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_saf);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Saf<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.saf", "SafPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_saf)?;
  Ok(Saf(handle))
}

/// Access to the saf APIs.
pub struct Saf<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Saf<R> {
  pub fn request_permission(&self) -> crate::Result<RequestPermissionResponse> {
    self
      .0
      .run_mobile_plugin("requestPermission", ())
      .map_err(Into::into)
  }

  pub fn check_permission(&self, payload: CheckPermissionRequest) -> crate::Result<CheckPermissionResponse> {
    self
      .0
      .run_mobile_plugin("checkPermission", payload)
      .map_err(Into::into)
  }

  pub fn list_dir(&self, payload: ListDirRequest) -> crate::Result<ListDirResponse> {
    self
      .0
      .run_mobile_plugin("listDir", payload)
      .map_err(Into::into)
  }

  pub fn read_file(&self, payload: ReadFileRequest) -> crate::Result<ReadFileResponse> {
    self
      .0
      .run_mobile_plugin("readFile", payload)
      .map_err(Into::into)
  }

  pub fn write_file(&self, payload: WriteFileRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("writeFile", payload)
      .map_err(Into::into)
  }
}