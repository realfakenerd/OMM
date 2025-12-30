<script lang="ts">
    import ModList from '$lib/components/ModList.svelte';
    import { ModManager } from '$lib/mods';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    const manager = new ModManager();
    let mods = $state(manager.getMods());
    let configPath = $state("/sdcard/Documents/OpenMW/openmw.cfg");
    let status = $state("");
    let error = $state("");

    onMount(async () => {
        try {
            status = "Loading config...";
            error = "";
            // Sample mods for UI demonstration
            if (mods.length === 0) {
                manager.addMod({ name: 'Morrowind.esm', enabled: true });
                manager.addMod({ name: 'Tribunal.esm', enabled: true });
                manager.addMod({ name: 'Bloodmoon.esm', enabled: true });
                mods = [...manager.getMods()];
            }
            status = "Ready";
        } catch (e: any) {
            error = "Failed to initialize: " + e.message || e;
            status = "";
        }
    });

    function handleToggle(name: String) {
        manager.toggleMod(name);
        mods = [...manager.getMods()];
    }

    function handleReorder(from: number, to: number) {
        manager.reorder(from, to);
        mods = [...manager.getMods()];
    }

    async function handleSave() {
        try {
            status = "Saving...";
            error = "";
            // Actual call would be:
            // await invoke('write_config', { path: configPath, content: "..." });
            status = "Save successful (Simulated)";
        } catch (e: any) {
            error = "Save failed: " + e.message || e;
            status = "";
        }
    }
</script>

<main class="container mx-auto p-4 max-w-2xl">
    <h1 class="text-3xl font-bold text-center border-b mb-6 pb-2">OpenMW Android Modloader</h1>
    
    <div class="mb-6 flex flex-col md:flex-row items-start md:items-center justify-between bg-white p-4 rounded-lg border shadow-sm gap-4">
        <div class="overflow-hidden w-full">
            <p class="text-xs font-bold text-gray-500 uppercase tracking-wider mb-1">Config Path</p>
            <p class="text-sm font-mono truncate text-gray-700 bg-gray-50 p-1 rounded border" title={configPath}>{configPath}</p>
        </div>
        <button 
            class="w-full md:w-auto bg-blue-600 hover:bg-blue-700 text-white font-semibold px-6 py-2 rounded-lg shadow-md transition-all active:scale-95"
            onclick={handleSave}
        >
            Save Changes
        </button>
    </div>

    {#if error}
        <div class="mb-6 p-4 bg-red-50 border-l-4 border-red-500 text-red-700 rounded shadow-sm">
            <div class="flex items-center">
                <span class="font-bold mr-2">Error:</span>
                <span>{error}</span>
            </div>
        </div>
    {/if}

    {#if status}
        <p class="text-center text-sm mb-6 text-gray-500 animate-pulse">{status}</p>
    {/if}

    <div class="bg-white rounded-lg border shadow-sm overflow-hidden">
        <ModList {mods} onToggle={handleToggle} onReorder={handleReorder} />
    </div>
</main>
