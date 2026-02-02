<script lang="ts">
    import { page } from "$app/state";
    import { downloadStore } from '$lib/download.svelte';

    const activeTab = $derived(page.url.pathname);

    const activeDownloads = $derived(downloadStore.queue.filter(d => d.status === 'downloading'));
    const totalProgress = $derived(
        activeDownloads.length > 0 
            ? activeDownloads.reduce((acc, d) => acc + (d.bytes_downloaded / (d.total_bytes || 1)), 0) / activeDownloads.length
            : 0
    );
</script>

<nav
    class="fixed bottom-0 w-full bg-background pb-6 pt-3 px-6 flex justify-between items-center z-50 left-1/2 -translate-x-1/2"
>
    {#if activeDownloads.length > 0}
        <div class="absolute top-0 left-0 h-0.5 bg-primary transition-all duration-300" style="width: {totalProgress * 100}%"></div>
    {/if}
    
    <a
        href="/"
        class="flex flex-col items-center gap-1 {activeTab === '/'
            ? 'text-primary'
            : 'text-slate-400 hover:text-primary'} transition-colors"
    >
        <span
            class="material-symbols-outlined"
            style="font-variation-settings: 'FILL' {activeTab === '/' ? 1 : 0}"
            >home</span
        >
        <span class="text-[10px] font-medium">Home</span>
    </a>
    <a
        href="/explore"
        class="flex flex-col items-center gap-1 {activeTab === '/explore'
            ? 'text-primary'
            : 'text-slate-400 hover:text-primary'} transition-colors"
    >
        <span
            class="material-symbols-outlined"
            style="font-variation-settings: 'FILL' {activeTab === '/explore'
                ? 1
                : 0}">explore</span
        >
        <span class="text-[10px] font-medium">Explore</span>
    </a>
    <a
        href="/profile"
        class="flex flex-col items-center gap-1 {activeTab === '/profile'
            ? 'text-primary'
            : 'text-slate-400 hover:text-primary'} transition-colors"
    >
        <span
            class="material-symbols-outlined"
            style="font-variation-settings: 'FILL' {activeTab === '/profile'
                ? 1
                : 0}">person</span
        >
        <span class="text-[10px] font-medium">Profile</span>
    </a>
    <a
        href="/settings"
        class="flex flex-col items-center gap-1 {activeTab === '/settings'
            ? 'text-primary'
            : 'text-slate-400 hover:text-primary'} transition-colors"
    >
        <span
            class="material-symbols-outlined"
            style="font-variation-settings: 'FILL' {activeTab === '/settings'
                ? 1
                : 0}">settings</span
        >
        <span class="text-[10px] font-medium">Settings</span>
    </a>
</nav>
