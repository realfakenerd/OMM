use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub mod_id: u32,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub status: String,
}

pub struct DownloadManager {
    client: reqwest::Client,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_mod<R: Runtime>(
        &self,
        app: AppHandle<R>,
        mod_id: u32,
        url: String,
        dest_path: String,
    ) -> Result<(), String> {
        let mut response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        let mut file = tokio::fs::File::create(&dest_path)
            .await
            .map_err(|e| e.to_string())?;

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| e.to_string())?;
            file.write_all(&chunk).await.map_err(|e| e.to_string())?;
            downloaded += chunk.len() as u64;

            // Emit progress event to frontend
            app.emit("download-progress", DownloadProgress {
                mod_id,
                bytes_downloaded: downloaded,
                total_bytes: total_size,
                status: "downloading".to_string(),
            }).map_err(|e| e.to_string())?;
        }

        app.emit("download-progress", DownloadProgress {
            mod_id,
            bytes_downloaded: total_size,
            total_bytes: total_size,
            status: "completed".to_string(),
        }).map_err(|e| e.to_string())?;

        Ok(())
    }
}
