import { invoke } from "@tauri-apps/api/core";

export interface NexusMod {
    name: string;
    summary: string;
    author: string;
    version: string;
    downloads: number;
    endorsements: number;
    updated_timestamp: number;
    picture_url: string;
    mod_id: number;
}

export async function getFeaturedMods(): Promise<NexusMod[]> {
    try {
        return await invoke<NexusMod[]>("get_featured_mods");
    } catch (error) {
        console.error("Failed to fetch featured mods:", error);
        throw error;
    }
}

export async function searchMods(query: string): Promise<NexusMod[]> {
    try {
        return await invoke<NexusMod[]>("search_mods", { query });
    } catch (error) {
        console.error("Failed to search mods:", error);
        throw error;
    }
}
