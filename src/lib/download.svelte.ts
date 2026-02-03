import { browser } from '$app/environment';
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface DownloadProgress {
  mod_id: number;
  bytes_downloaded: number;
  total_bytes: number;
  status: string;
}

export class DownloadStore {
  queue = $state<DownloadProgress[]>([]);

  constructor() {
    if(browser) {
      this.init();
    }
  }

  private async init() {
    if (!(window as any).__TAURI_INTERNALS__) return;

    await listen<DownloadProgress>("download-progress", (event) => {
      const index = this.queue.findIndex(d => d.mod_id === event.payload.mod_id);
      if (index !== -1) {
        this.queue[index] = event.payload;
      } else {
        this.queue.push(event.payload);
      }

      if (event.payload.status === "completed") {
        // Remove from queue after a delay or move to completed
        setTimeout(() => {
          this.queue = this.queue.filter(d => d.mod_id !== event.payload.mod_id);
        }, 5000);
      }
    });
  }

  async startDownload(mod_id: number, file_id: number) {
    if (!(window as any).__TAURI_INTERNALS__) {
      console.warn("Tauri APIs not available");
      return;
    }
    try {
      await invoke("download_mod", { mod_id, fileId: file_id });
      this.queue.push({
        mod_id,
        bytes_downloaded: 0,
        total_bytes: 0,
        status: "starting"
      });
    } catch (error) {
      console.error("Failed to start download:", error);
    }
  }
}


export const downloadStore = new DownloadStore();
