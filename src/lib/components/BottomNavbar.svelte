<script lang="ts">
    import { page } from "$app/state";
    import { downloadStore } from "$lib/download.svelte";
    import {
        CompassIcon,
        HouseIcon,
        SettingsIcon,
        UserIcon,
    } from "@lucide/svelte";

    const activeTab = $derived(page.url.pathname);

    const activeDownloads = $derived(
        downloadStore.queue.filter((d) => d.status === "downloading"),
    );
    const totalProgress = $derived.by(() =>
        activeDownloads.length > 0
            ? activeDownloads.reduce(
                  (acc, d) => acc + d.bytes_downloaded / (d.total_bytes || 1),
                  0,
              ) / activeDownloads.length
            : 0,
    );

    const progress = $derived(totalProgress * 100);

    const links = [
        {
            href: "/",
            title: "home",
            Icon: HouseIcon,
        },
        {
            href: "/explore",
            title: "explore",
            Icon: CompassIcon,
        },
        {
            href: "/profile",
            title: "profile",
            Icon: UserIcon,
        },
        {
            href: "/settings",
            title: "settings",
            Icon: SettingsIcon,
        },
    ];
</script>

<nav class="dock">
    {#if activeDownloads.length > 0}
        <div
            class="absolute top-0 left-0 h-0.5 bg-primary transition-all duration-300"
            style="width: {progress * 100}%"
        ></div>
    {/if}

    {#each links as { href, Icon, title } (href)}
        <a {href} class={activeTab === href ? "dock-active" : ""}>
            <Icon />
            <span class="dock-label">{title}</span>
        </a>
    {/each}
</nav>
