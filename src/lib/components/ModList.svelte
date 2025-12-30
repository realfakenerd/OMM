<script lang="ts">
    import type { Mod } from '$lib/mods';

    export let mods: Mod[] = [];
</script>

<div class="p-4">
    <h2 class="text-xl font-bold mb-4">Installed Mods</h2>
    <ul class="space-y-2">
        {#each mods as mod}
            <li class="flex items-center p-2 border rounded hover:bg-gray-100">
                <input type="checkbox" checked={mod.enabled} class="mr-2" />
                <span>{mod.name}</span>
            </li>
        {/each}
    </ul>
</div>
