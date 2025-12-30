<script lang="ts">
    import ModList from '$lib/components/ModList.svelte';
    import { ModManager } from '$lib/mods';
    import { onMount } from 'svelte';

    const manager = new ModManager();
    let mods = $state(manager.getMods());

    onMount(() => {
        manager.addMod({ name: 'Morrowind.esm', enabled: true });
        manager.addMod({ name: 'Tribunal.esm', enabled: true });
        manager.addMod({ name: 'Bloodmoon.esm', enabled: true });
        mods = [...manager.getMods()];
    });
</script>

<main class="container mx-auto">
    <h1 class="text-3xl font-bold p-4">OpenMW Android Modloader</h1>
    <ModList {mods} />
</main>