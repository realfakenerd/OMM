import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface DownloadProgress {
    mod_id: number;
    bytes_downloaded: number;
    total_bytes: number;
    status: string;
}

export class DownloadStore {
    queue = $state<DownloadProgress[]>([]);

    constructor() {
        this.init();
    }

    private async init() {
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
