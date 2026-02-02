<script lang="ts">
    import { onMount } from 'svelte';
    import { getFeaturedMods, searchMods, type NexusMod } from '$lib/nexus';
    import DownloadsDrawer from '$lib/components/DownloadsDrawer.svelte';

    let featuredMods = $state<NexusMod[]>([]);
    let searchResults = $state<NexusMod[]>([]);
    let searchQuery = $state('');
    let isLoading = $state(true);
    let isSearching = $state(false);
    let error = $state<string | null>(null);

    let searchTimeout: ReturnType<typeof setTimeout>;

    $effect(() => {
        if (searchQuery.trim().length > 2) {
            clearTimeout(searchTimeout);
            isSearching = true;
            searchTimeout = setTimeout(async () => {
                try {
                    searchResults = await searchMods(searchQuery);
                } catch (e) {
                    console.error(e);
                } finally {
                    isSearching = false;
                }
            }, 500);
        } else {
            searchResults = [];
            isSearching = false;
        }
    });

    onMount(async () => {
        try {
            featuredMods = await getFeaturedMods();
        } catch (e) {
            error = "Failed to load mods from Nexus.";
            console.error(e);
        } finally {
            isLoading = false;
        }
    });

    // Mock data for other sections
    const popularMods = [
        {
            title: 'Better Heads',
            author: 'Motoki',
            rating: '4.9',
            downloads: '240k',
            image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuBEjb0siBbxlnBOfVMrA0ecU6hsNcA8rlaSQ8HPD8EcJxxRk4XUJlzVt--VXe94SughJOBW95vZBlA3uQxjiFzazZE1gO3dkMjouRCfTv1PDXQ9sILqpkMceLYhfYoY3T8gBD1jZf-fIEWXMZRhAvi8lBCPVW4rV7zjA0Tk-yyQWLksYdEuLRw_s1dbabLzS4xasj8r7VbBgGPaodO7Fvxew7n1ybaDyyrzn3-_XRx05rIeFBWPZpmW59yVS_zFH6iciAOKz7qepicY'
        },
        {
            title: 'Patch for Purists',
            author: 'Half11',
            rating: '4.8',
            downloads: '180k',
            image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuA0CtfxJ9ivsZPX0NfcEr0kQbPgru_HC_BIPeWESfsDP7UsZXxNOWERyuGdu5Wo58fPwrKtKpfOJZcVyCXhdggf6zqJw4Q1YbRRtU38iU7LvfiWyBI-Q3m-3CYe4_w7NVJ66-ix4UQZVaEnIdFkEgqrcucRVvX9TVKgJtibW6-EW5xGXMfnUEwRD6QQnZRgoVR2WE1PCfXtF9Ot5NqgtmHMY9UZovmrObH16YtPiLBpbZRf1FXJ_x6zsd6g3HUp9lxIHY00_J6fYh6y'
        }
    ];

    const highestRated = [
        { title: 'Rise of House Telvanni', category: 'Quest Expansion', rating: '5.0', image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuAIReDfAopTpSBoMf3SEAFP7IjImjgj1Zi3Mi_NKFTxRT7lAOmWtWuMNRoA813WJiLm2rhOqw9NhRvGYLaW5YugbNmkKVc9dIrpbQjCfNKo6-MvEDv9xESjFHpYGbO1b6k6vnZmfXTii7QbxQ25TwGbzhmbAb4o4Pu42kbQfv2c951T0VoXmgABFD4u-fSJ9dgrp7X0p28eq8qhk0xEVTdGeI-Ya0-A-EXFkIrD6zZHvH894E8GxDB8BYg_7bnodIljfwRPpAoO_MvD' },
        { title: 'Tamriel Data', category: 'Resource', rating: '4.9', image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuCi-EaqF3Z3tRqtivmlCb2o-y3qN-U7WzPqelEprpWEDvtvzR10J3WAQxfc-JzfY8kuodhw0-WKZDa3baDr2EehBC9j_7GNdyDIrq4XaKnlgCAzv6MXoqlSHVgaiSarovNc1izblKmlMMBzlzqNNJrsW0I9tofyCCyGv8xif8zF6FoZqHaJx-adtWp2-ok7RWzy-1kFWtlAPVY_aqTqZH3o5Jcah7lWHrI6PxPTq5GKS2UEd5PhuotTPiKvPrYzml1WAFkJoezkIxsF' },
        { title: "Vurt's Groundcover", category: 'Environment', rating: '4.8', image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuCL-gx5PdGMEKWRW8QeTzOmK-FNdQM2omS6q2WlU916ZCIyTbIpucZGx3CmuHcKfvzmpMoIioEnTu0XTBnfn_6-Va3c8JNJdJ-bnthBzrD-0Sm37MqLw6R59dcZG0Q99cRkuPd_kRJiEThCC1_X1g_NEeKmyL7PHcsR7PDrv3bXQ5nnzk6o4twsrgY7dGZ3f1ZIyg7mb5GmhTEe5BsX45cUquJwCIziZjcv93-S7J1UvWQ3_2hxvOfeD2LcsW-knce3YwWxtJldYF1n' }
    ];
</script>

<div class="relative mx-auto h-full min-h-screen w-full max-w-md overflow-hidden bg-background-light dark:bg-background-dark font-display antialiased">
    <!-- Header -->
    <header class="sticky top-0 z-20 bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-md pt-2">
        <div class="flex items-center justify-between px-4 py-3">
            <div class="flex items-center gap-3">
                <button class="flex h-10 w-10 items-center justify-center rounded-full bg-surface-light dark:bg-surface-dark text-gray-600 dark:text-white hover:bg-gray-100 dark:hover:bg-white/10 transition-colors">
                    <span class="material-symbols-outlined text-[24px]">menu</span>
                </button>
                <h1 class="text-2xl font-bold tracking-tight">Explore</h1>
            </div>
            <div class="flex gap-2">
                <DownloadsDrawer>
                    <button class="flex h-10 w-10 items-center justify-center rounded-full bg-surface-light dark:bg-surface-dark text-gray-600 dark:text-white hover:bg-gray-100 dark:hover:bg-white/10 transition-colors relative">
                        <span class="material-symbols-outlined text-[24px]">download</span>
                    </button>
                </DownloadsDrawer>
                <button class="flex h-10 w-10 items-center justify-center rounded-full bg-surface-light dark:bg-surface-dark text-gray-600 dark:text-white hover:bg-gray-100 dark:hover:bg-white/10 transition-colors relative">
                    <span class="material-symbols-outlined text-[24px]">notifications</span>
                    <span class="absolute top-2 right-2 h-2.5 w-2.5 rounded-full bg-primary-settings ring-2 ring-background-dark"></span>
                </button>
            </div>
        </div>

        <!-- Search Bar -->
        <div class="px-4 pb-4">
            <div class="relative flex w-full items-center">
                <span class="absolute left-4 text-[#ab9db9] material-symbols-outlined">search</span>
                <input 
                    bind:value={searchQuery}
                    class="w-full rounded-2xl border-none bg-surface-light dark:bg-surface-dark py-3.5 pl-11 pr-4 text-sm font-medium text-gray-900 dark:text-white placeholder-[#ab9db9] focus:ring-2 focus:ring-primary-settings shadow-sm" 
                    placeholder="Search mods, authors, categories..." 
                    type="text"
                />
                {#if isSearching}
                    <div class="absolute right-12 animate-spin size-4 border-2 border-primary-settings border-t-transparent rounded-full"></div>
                {/if}
                <button class="absolute right-3 p-1.5 rounded-lg text-[#ab9db9] hover:bg-background-dark/20 dark:hover:bg-white/10 transition-colors">
                    <span class="material-symbols-outlined text-[20px]">tune</span>
                </button>
            </div>
        </div>
    </header>

    <!-- Main Content -->
    <main class="relative z-10 w-full overflow-y-auto pb-24 space-y-8 no-scrollbar">
        {#if searchQuery.trim().length > 2}
            <section class="px-4">
                <h2 class="text-xl font-bold mb-4">Search Results</h2>
                <div class="flex flex-col gap-3">
                    {#each searchResults as mod}
                        <div class="flex gap-3 rounded-xl bg-surface-light dark:bg-surface-dark p-3 shadow-sm hover:ring-1 hover:ring-primary-settings/50 transition-all">
                            <div class="h-20 w-20 shrink-0 overflow-hidden rounded-lg bg-gray-800">
                                <div class="h-full w-full bg-cover bg-center" style="background-image: url('{mod.picture_url}');"></div>
                            </div>
                            <div class="flex flex-1 flex-col justify-between">
                                <div>
                                    <h3 class="font-bold text-sm leading-tight mb-1">{mod.name}</h3>
                                    <p class="text-xs text-[#ab9db9]">by {mod.author}</p>
                                </div>
                                <div class="flex items-center gap-3 mt-2">
                                    <div class="flex items-center gap-1 text-yellow-500">
                                        <span class="material-symbols-outlined text-[14px] fill-current">star</span>
                                        <span class="text-xs font-bold text-gray-700 dark:text-gray-300">{mod.endorsements}</span>
                                    </div>
                                    <div class="flex items-center gap-1 text-[#ab9db9]">
                                        <span class="material-symbols-outlined text-[14px]">download</span>
                                        <span class="text-xs">{mod.downloads}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    {:else}
                        {#if !isSearching}
                            <p class="text-center text-[#ab9db9] py-8">No results found for "{searchQuery}"</p>
                        {/if}
                    {/each}
                </div>
            </section>
        {:else}
            <!-- Featured Carousel -->
            <section class="mt-2">
                <div class="flex items-center justify-between px-4 mb-3">
                    <h2 class="text-xl font-bold">Featured</h2>
                </div>
                <div class="flex snap-x snap-mandatory gap-4 overflow-x-auto px-4 pb-4 hide-scrollbar">
                    {#if isLoading}
                        {#each Array(3) as _}
                            <div class="h-48 w-80 shrink-0 snap-center rounded-2xl bg-surface-dark animate-pulse"></div>
                        {/each}
                    {:else if error}
                        <div class="px-4 py-8 text-center text-red-400 text-sm">
                            {error}
                        </div>
                    {:else}
                        {#each featuredMods as mod}
                            <div class="relative h-48 w-80 shrink-0 snap-center overflow-hidden rounded-2xl bg-surface-dark group cursor-pointer shadow-lg">
                                <div class="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-105" style="background-image: url('{mod.picture_url}');"></div>
                                <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent"></div>
                                <div class="absolute bottom-0 left-0 p-4 w-full text-white">
                                    <div class="mb-1 flex items-center gap-2">
                                        <span class="rounded bg-primary-settings px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider">Top Pick</span>
                                    </div>
                                    <h3 class="text-lg font-bold">{mod.name}</h3>
                                    <p class="text-xs text-gray-300 line-clamp-1">{mod.summary}</p>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>
            </section>

            <!-- Most Popular List -->
            <section class="px-4">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-lg font-bold">Most Popular</h2>
                    <button class="text-xs font-semibold text-primary-settings flex items-center gap-0.5 hover:opacity-80">
                        See All <span class="material-symbols-outlined text-[16px]">arrow_forward_ios</span>
                    </button>
                </div>
                <div class="flex flex-col gap-3">
                    {#each popularMods as mod}
                        <div class="flex gap-3 rounded-xl bg-surface-light dark:bg-surface-dark p-3 shadow-sm hover:ring-1 hover:ring-primary-settings/50 transition-all">
                            <div class="h-20 w-20 shrink-0 overflow-hidden rounded-lg bg-gray-800">
                                <div class="h-full w-full bg-cover bg-center" style="background-image: url('{mod.image}');"></div>
                            </div>
                            <div class="flex flex-1 flex-col justify-between">
                                <div>
                                    <h3 class="font-bold text-sm leading-tight mb-1">{mod.title}</h3>
                                    <p class="text-xs text-[#ab9db9]">by {mod.author}</p>
                                </div>
                                <div class="flex items-center gap-3 mt-2">
                                    <div class="flex items-center gap-1 text-yellow-500">
                                        <span class="material-symbols-outlined text-[14px] fill-current">star</span>
                                        <span class="text-xs font-bold text-gray-700 dark:text-gray-300">{mod.rating}</span>
                                    </div>
                                    <div class="flex items-center gap-1 text-[#ab9db9]">
                                        <span class="material-symbols-outlined text-[14px]">download</span>
                                        <span class="text-xs">{mod.downloads}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="flex items-center">
                                <button class="flex h-8 w-8 items-center justify-center rounded-full bg-primary-settings/10 text-primary-settings hover:bg-primary-settings hover:text-white transition-colors">
                                    <span class="material-symbols-outlined text-[20px]">add</span>
                                </button>
                            </div>
                        </div>
                    {/each}
                </div>
            </section>

            <!-- Highest Rated -->
            <section>
                <div class="flex items-center justify-between px-4 mb-4">
                    <h2 class="text-lg font-bold">Highest Rated</h2>
                    <button class="text-xs font-semibold text-primary-settings flex items-center gap-0.5 hover:opacity-80">
                        See All <span class="material-symbols-outlined text-[16px]">arrow_forward_ios</span>
                    </button>
                </div>
                <div class="flex snap-x snap-mandatory gap-4 overflow-x-auto px-4 pb-4 hide-scrollbar">
                    {#each highestRated as mod}
                        <div class="w-40 shrink-0 snap-start flex flex-col gap-2 group">
                            <div class="relative w-full aspect-square rounded-xl overflow-hidden bg-surface-dark shadow-md">
                                <div class="absolute inset-0 bg-cover bg-center group-hover:scale-110 transition-transform duration-300" style="background-image: url('{mod.image}');"></div>
                                <div class="absolute top-2 right-2 bg-black/60 backdrop-blur-sm rounded-md px-1.5 py-0.5 flex items-center gap-0.5">
                                    <span class="material-symbols-outlined text-[10px] text-yellow-400 fill-current">star</span>
                                    <span class="text-[10px] font-bold text-white">{mod.rating}</span>
                                </div>
                            </div>
                            <div>
                                <h3 class="font-bold text-sm truncate">{mod.title}</h3>
                                <p class="text-[10px] text-[#ab9db9] uppercase tracking-wide">{mod.category}</p>
                            </div>
                        </div>
                    {/each}
                </div>
            </section>
        {/if}
    </main>
</div>

<style>
    .hide-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .hide-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
</style>