<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { downloadStore } from "$lib/download.svelte";
    import { Button } from "$lib/components/ui/button";
    import type { Snippet } from "svelte";

    let { children }: { children: Snippet } = $props();

    const downloads = $derived(downloadStore.queue);

    function formatBytes(bytes: number) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
</script>

<Dialog.Root>
    <Dialog.Trigger>
        {@render children()}
    </Dialog.Trigger>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Downloads</Dialog.Title>
            <Dialog.Description>
                Manage your active mod downloads.
            </Dialog.Description>
        </Dialog.Header>
        
        <div class="flex flex-col gap-4 py-4">
            {#if downloads.length === 0}
                <p class="text-center text-slate-500 py-8">No active downloads.</p>
            {:else}
                {#each downloads as download}
                    <div class="flex flex-col gap-2 p-3 rounded-lg bg-card border">
                        <div class="flex justify-between items-center">
                            <span class="font-bold text-sm">Mod ID: {download.mod_id}</span>
                            <span class="text-xs uppercase font-bold text-primary">{download.status}</span>
                        </div>
                        
                        <div class="w-full h-2 bg-slate-200 dark:bg-slate-800 rounded-full overflow-hidden">
                            <div 
                                class="h-full bg-primary transition-all duration-300" 
                                style="width: {(download.bytes_downloaded / (download.total_bytes || 1)) * 100}%"
                            ></div>
                        </div>
                        
                        <div class="flex justify-between text-[10px] text-slate-500">
                            <span>{formatBytes(download.bytes_downloaded)} / {formatBytes(download.total_bytes)}</span>
                            <span>{Math.round((download.bytes_downloaded / (download.total_bytes || 1)) * 100)}%</span>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </Dialog.Content>
</Dialog.Root>
