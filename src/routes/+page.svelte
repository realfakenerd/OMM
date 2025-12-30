<script lang="ts">
    import ModList from '$lib/components/ModList.svelte';
    import { ModManager } from '$lib/mods';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    const manager = new ModManager();
    let mods = $state(manager.getMods());
    let configPath = $state("/sdcard/Documents/OpenMW/openmw.cfg");
    let status = $state("");

    onMount(async () => {
        try {
            status = "Loading config...";
            // Sample mods for UI demonstration
            if (mods.length === 0) {
                manager.addMod({ name: 'Morrowind.esm', enabled: true });
                manager.addMod({ name: 'Tribunal.esm', enabled: true });
                manager.addMod({ name: 'Bloodmoon.esm', enabled: true });
                mods = [...manager.getMods()];
            }
            status = "Ready";
        } catch (e) {
            status = "Error: " + e;
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
            // Actual call would be:
            // await invoke('write_config', { path: configPath, content: "..." });
            status = "Save successful (Simulated)";
        } catch (e) {
            status = "Save failed: " + e;
        }
    }
</script>

<main class="container mx-auto p-4">
    <h1 class="text-3xl font-bold text-center border-b mb-4 pb-2">OpenMW Android Modloader</h1>
    
    <div class="mb-4 flex items-center justify-between bg-gray-50 p-4 rounded border">
        <div>
            <p class="text-sm font-semibold text-gray-600">Config Path:</p>
            <p class="text-xs font-mono">{configPath}</p>
        </div>
        <button 
            class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition"
            onclick={handleSave}
        >
            Save Changes
        </button>
    </div>

    {#if status}
        <p class="text-center text-sm mb-4 text-blue-600 font-medium">{status}</p>
    {/if}

    <ModList {mods} onToggle={handleToggle} onReorder={handleReorder} />
</main>