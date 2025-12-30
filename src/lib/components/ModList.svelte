<script lang="ts">
    import type { Mod } from '$lib/mods';

    interface Props {
        mods: Mod[];
        onToggle: (name: String) => void;
        onReorder: (from: number, to: number) => void;
    }

    let { mods = [], onToggle, onReorder }: Props = $props();
</script>

<div class="p-4">
    <h2 class="text-xl font-bold mb-4">Installed Mods</h2>
    <ul class="space-y-2">
        {#each mods as mod, i}
            <li class="flex items-center p-2 border rounded hover:bg-gray-50 bg-white shadow-sm">
                <div class="flex flex-col mr-4">
                    <button 
                        class="text-xs hover:bg-gray-200 px-1 rounded disabled:opacity-30" 
                        disabled={i === 0}
                        onclick={() => onReorder(i, i - 1)}
                    >
                        ▲
                    </button>
                    <button 
                        class="text-xs hover:bg-gray-200 px-1 rounded disabled:opacity-30" 
                        disabled={i === mods.length - 1}
                        onclick={() => onReorder(i, i + 1)}
                    >
                        ▼
                    </button>
                </div>
                <input 
                    type="checkbox" 
                    checked={mod.enabled} 
                    onchange={() => onToggle(mod.name)}
                    class="mr-2 h-4 w-4" 
                />
                <span class="flex-grow font-medium">{mod.name}</span>
                <span class="text-xs text-gray-400 ml-2">#{i + 1}</span>
            </li>
        {/each}
    </ul>
</div>