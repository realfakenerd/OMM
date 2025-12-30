use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SafExt;

#[command]
pub(crate) async fn request_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<RequestPermissionResponse> {
    app.saf().request_permission()
}

#[command]
pub(crate) async fn check_permission<R: Runtime>(
    app: AppHandle<R>,
    payload: CheckPermissionRequest,
) -> Result<CheckPermissionResponse> {
    app.saf().check_permission(payload)
}

#[command]
pub(crate) async fn list_dir<R: Runtime>(
    app: AppHandle<R>,
    payload: ListDirRequest,
) -> Result<ListDirResponse> {
    app.saf().list_dir(payload)
}

#[command]
pub(crate) async fn read_file<R: Runtime>(
    app: AppHandle<R>,
    payload: ReadFileRequest,
) -> Result<ReadFileResponse> {
    app.saf().read_file(payload)
}

#[command]
pub(crate) async fn write_file<R: Runtime>(
    app: AppHandle<R>,
    payload: WriteFileRequest,
) -> Result<()> {
    app.saf().write_file(payload)
}