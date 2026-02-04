<script lang="ts">
    import { onMount } from "svelte";
    import { getFeaturedMods, searchMods, type NexusMod } from "$lib/nexus";
    import {
        Bell,
        Funnel,
        Search,
        ChevronRight,
        Plus,
        Download,
        Star,
    } from "@lucide/svelte";
    // import DownloadsDrawer from "$lib/components/DownloadsDrawer.svelte";

    let featuredMods = $state<NexusMod[]>([]);
    let searchResults = $state<NexusMod[]>([]);
    let searchQuery = $state("");
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
            title: "Better Heads",
            author: "Motoki",
            rating: "4.9",
            downloads: "240k",
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuBEjb0siBbxlnBOfVMrA0ecU6hsNcA8rlaSQ8HPD8EcJxxRk4XUJlzVt--VXe94SughJOBW95vZBlA3uQxjiFzazZE1gO3dkMjouRCfTv1PDXQ9sILqpkMceLYhfYoY3T8gBD1jZf-fIEWXMZRhAvi8lBCPVW4rV7zjA0Tk-yyQWLksYdEuLRw_s1dbabLzS4xasj8r7VbBgGPaodO7Fvxew7n1ybaDyyrzn3-_XRx05rIeFBWPZpmW59yVS_zFH6iciAOKz7qepicY",
        },
        {
            title: "Patch for Purists",
            author: "Half11",
            rating: "4.8",
            downloads: "180k",
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuA0CtfxJ9ivsZPX0NfcEr0kQbPgru_HC_BIPeWESfsDP7UsZXxNOWERyuGdu5Wo58fPwrKtKpfOJZcVyCXhdggf6zqJw4Q1YbRRtU38iU7LvfiWyBI-Q3m-3CYe4_w7NVJ66-ix4UQZVaEnIdFkEgqrcucRVvX9TVKgJtibW6-EW5xGXMfnUEwRD6QQnZRgoVR2WE1PCfXtF9Ot5NqgtmHMY9UZovmrObH16YtPiLBpbZRf1FXJ_x6zsd6g3HUp9lxIHY00_J6fYh6y",
        },
    ];

    const highestRated = [
        {
            title: "Rise of House Telvanni",
            category: "Quest Expansion",
            rating: "5.0",
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuAIReDfAopTpSBoMf3SEAFP7IjImjgj1Zi3Mi_NKFTxRT7lAOmWtWuMNRoA813WJiLm2rhOqw9NhRvGYLaW5YugbNmkKVc9dIrpbQjCfNKo6-MvEDv9xESjFHpYGbO1b6k6vnZmfXTii7QbxQ25TwGbzhmbAb4o4Pu42kbQfv2c951T0VoXmgABFD4u-fSJ9dgrp7X0p28eq8qhk0xEVTdGeI-Ya0-A-EXFkIrD6zZHvH894E8GxDB8BYg_7bnodIljfwRPpAoO_MvD",
        },
        {
            title: "Tamriel Data",
            category: "Resource",
            rating: "4.9",
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCi-EaqF3Z3tRqtivmlCb2o-y3qN-U7WzPqelEprpWEDvtvzR10J3WAQxfc-JzfY8kuodhw0-WKZDa3baDr2EehBC9j_7GNdyDIrq4XaKnlgCAzv6MXoqlSHVgaiSarovNc1izblKmlMMBzlzqNNJrsW0I9tofyCCyGv8xif8zF6FoZqHaJx-adtWp2-ok7RWzy-1kFWtlAPVY_aqTqZH3o5Jcah7lWHrI6PxPTq5GKS2UEd5PhuotTPiKvPrYzml1WAFkJoezkIxsF",
        },
        {
            title: "Vurt's Groundcover",
            category: "Environment",
            rating: "4.8",
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCL-gx5PdGMEKWRW8QeTzOmK-FNdQM2omS6q2WlU916ZCIyTbIpucZGx3CmuHcKfvzmpMoIioEnTu0XTBnfn_6-Va3c8JNJdJ-bnthBzrD-0Sm37MqLw6R59dcZG0Q99cRkuPd_kRJiEThCC1_X1g_NEeKmyL7PHcsR7PDrv3bXQ5nnzk6o4twsrgY7dGZ3f1ZIyg7mb5GmhTEe5BsX45cUquJwCIziZjcv93-S7J1UvWQ3_2hxvOfeD2LcsW-knce3YwWxtJldYF1n",
        },
    ];
</script>

<div class="relative mx-auto w-full font-display antialiased">
    <!-- Header -->

    <div
        class="flex flex-col gap-4 p-4 sticky top-0 bg-base-100/70 backdrop-blur z-50"
    >
        <div class="flex w-full justify-between items-center">
            <h1 class="tracking-tight text-4xl font-bold leading-tight">
                Explore Mods
            </h1>
            <button class="btn btn-ghost btn-circle">
                <div class="indicator">
                    <span
                        aria-label="status"
                        class="indicator-item status status-primary"
                    ></span>
                    <Bell />
                </div>
            </button>
        </div>

        <label class="input w-full">
            <Search />
            <input type="search" required placeholder="Search for mods..." />
            <Funnel />
        </label>
    </div>

    <!-- Main Content -->
    <main class="relative w-full overflow-y-auto pb-24 space-y-8 no-scrollbar">
        {#if searchQuery.trim().length > 2}
            <section class="px-4">
                <h2 class="text-xl font-bold mb-4">Search Results</h2>
                <div class="flex flex-col gap-3">
                    {#each searchResults as mod}
                        <div
                            class="flex gap-3 rounded-xl bg-surface-light dark:bg-surface-dark p-3 shadow-sm hover:ring-1 hover:ring-primary-settings/50 transition-all"
                        >
                            <div
                                class="h-20 w-20 shrink-0 overflow-hidden rounded-lg bg-gray-800"
                            >
                                <div
                                    class="h-full w-full bg-cover bg-center"
                                    style="background-image: url('{mod.picture_url}');"
                                ></div>
                            </div>
                            <div class="flex flex-1 flex-col justify-between">
                                <div>
                                    <h3
                                        class="font-bold text-sm leading-tight mb-1"
                                    >
                                        {mod.name}
                                    </h3>
                                    <p class="text-xs text-[#ab9db9]">
                                        by {mod.author}
                                    </p>
                                </div>
                                <div class="flex items-center gap-3 mt-2">
                                    <div
                                        class="flex items-center gap-1 text-yellow-500"
                                    >
                                        <span
                                            class="material-symbols-outlined text-[14px] fill-current"
                                            >star</span
                                        >
                                        <span
                                            class="text-xs font-bold text-gray-700 dark:text-gray-300"
                                            >{mod.endorsements}</span
                                        >
                                    </div>
                                    <div
                                        class="flex items-center gap-1 text-[#ab9db9]"
                                    >
                                        <span
                                            class="material-symbols-outlined text-[14px]"
                                            >download</span
                                        >
                                        <span class="text-xs"
                                            >{mod.downloads}</span
                                        >
                                    </div>
                                </div>
                            </div>
                        </div>
                    {:else}
                        {#if !isSearching}
                            <p class="text-center text-[#ab9db9] py-8">
                                No results found for "{searchQuery}"
                            </p>
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
                <div
                    class="flex snap-x snap-mandatory gap-4 overflow-x-auto px-4 pb-4 hide-scrollbar"
                >
                    {#if isLoading}
                        {#each Array(3) as _}
                            <div
                                class="h-48 w-80 shrink-0 snap-center rounded-2xl bg-surface-dark animate-pulse"
                            ></div>
                        {/each}
                    {:else if error}
                        <div class="px-4 py-8 text-center text-red-400 text-sm">
                            {error}
                        </div>
                    {:else}
                        {#each featuredMods as mod}
                            <div
                                class="relative h-48 w-80 shrink-0 snap-center overflow-hidden rounded-2xl bg-surface-dark group cursor-pointer shadow-lg"
                            >
                                <div
                                    class="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-105"
                                    style="background-image: url('{mod.picture_url}');"
                                ></div>
                                <div
                                    class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent"
                                ></div>
                                <div
                                    class="absolute bottom-0 left-0 p-4 w-full text-white"
                                >
                                    <div class="mb-1 flex items-center gap-2">
                                        <span
                                            class="rounded bg-primary-settings px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider"
                                            >Top Pick</span
                                        >
                                    </div>
                                    <h3 class="text-lg font-bold">
                                        {mod.name}
                                    </h3>
                                    <p
                                        class="text-xs text-gray-300 line-clamp-1"
                                    >
                                        {mod.summary}
                                    </p>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>
            </section>

            <!-- Most Popular List -->
            <section class="p-4 flex flex-col gap-4">
                <div class="flex items-center justify-between">
                    <h2
                        class="text-xs font-bold uppercase tracking-wider text-primary/80"
                    >
                        Most Popular
                    </h2>
                    <button
                        class="btn btn-ghost btn-xs uppercase tracking-wider"
                    >
                        See All
                        <ChevronRight />
                    </button>
                </div>
                <div class="flex flex-col gap-3">
                    {#each popularMods as mod}
                        <div class="card card-side bg-base-200">
                            <figure>
                                <img
                                    loading="lazy"
                                    class="w-24 aspect-square"
                                    src={mod.image}
                                    alt="mod thumbnail"
                                />
                            </figure>

                            <div class="card-body">
                                <div>
                                    <h3 class="card-title">
                                        {mod.title}
                                    </h3>
                                    <p class="text-xs text-content-base/70">
                                        by {mod.author}
                                    </p>
                                </div>
                                <div class="flex items-center gap-3">
                                    <div
                                        class="flex items-center gap-1 text-yellow-500"
                                    >
                                        <Star class="size-4" />
                                        <span class="text-xs font-bold"
                                            >{mod.rating}</span
                                        >
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <Download class="size-4" />
                                        <span class="text-xs"
                                            >{mod.downloads}</span
                                        >
                                    </div>
                                </div>
                                <div class="card-actions">
                                    <button class="btn">
                                        <Plus />
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </section>

            <!-- Highest Rated -->
            <section class="flex flex-col gap-4">
                <div class="flex items-center justify-between px-4">
                    <h2
                        class="text-xs font-bold uppercase tracking-wider text-primary/80"
                    >
                        Highest Rated
                    </h2>

                    <button
                        class="btn btn-ghost btn-xs uppercase tracking-wider"
                    >
                        See All
                        <ChevronRight />
                    </button>
                </div>
                <div class="carousel carousel-start bg-base-300 p-4 gap-x-4">
                    {#each highestRated as mod}
                        <div class="carousel-item indicator">
                            <div class="indicator-item badge text-yellow-400">
                                <Star class="size-4" />
                                <span class="text-xs font-bold"
                                    >{mod.rating}</span
                                >
                            </div>

                            <div class="card image-full card-sm bg-base-200">
                                <figure>
                                    <img
                                        loading="lazy"
                                        class="w-full h-40"
                                        src={mod.image}
                                        alt="mod thumbnail"
                                    />
                                </figure>
                                <div class="card-body">
                                    <div class="card-title">
                                        {mod.title}
                                    </div>
                                    <p
                                        class="text-xs text-primary/60 uppercase tracking-wide"
                                    >
                                        {mod.category}
                                    </p>
                                </div>
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
