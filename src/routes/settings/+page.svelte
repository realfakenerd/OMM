<script lang="ts">
    import { Card, CardBody, CardTitle } from "$lib/components/ui/card";
    import { List, ListRow } from "$lib/components/ui/list";
    import {
        ArrowUpDown,
        ChevronDown,
        ChevronRight,
        FileBracesCorner,
        FolderDown,
        FolderTree,
        Grid3x3,
        HistoryIcon,
        Proportions,
    } from "@lucide/svelte";

    let autoUpdate = $state(false);
    let showFPS = $state(true);
    let hapticFeedback = $state(true);

    function handleDone() {
        window.location.href = "/";
    }

    const themes = [
        "default",
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
        "caramellatte",
        "abyss",
        "silk",
    ];
</script>

<div class="relative flex flex-col">
    <!-- TopAppBar -->
    <div
        class="flex justify-between items-center p-4 sticky top-0 bg-base-100/70 backdrop-blur z-50"
    >
        <h1 class="tracking-tight text-4xl font-bold leading-tight">
            Settings
        </h1>
        <button class="btn btn-primary" onclick={handleDone}> Done </button>
    </div>
    <div class="px-4 flex flex-col gap-4">
        <Card class="card-border bg-base-200">
            <CardBody>
                <CardTitle>Installation</CardTitle>

                <List>
                    <!-- ListItem: Game Data Path -->
                    <ListRow class="items-center">
                        <FolderTree class="text-primary" />
                        <div class="flex flex-col">
                            <p
                                class=" text-foreground text-base font-medium leading-normal truncate"
                            >
                                Game Data Path
                            </p>
                            <p
                                class="text-base-content/80 text-xs font-normal leading-normal truncate font-mono"
                            >
                                /var/mobile/Documents/Morrowind
                            </p>
                        </div>
                        <button class="btn btn-ghost text-primary">
                            Change
                        </button>
                    </ListRow>
                    <ListRow>
                        <FileBracesCorner class="text-primary" />
                        <p
                            class="text-base font-medium leading-normal truncate"
                        >
                            Import Morrowind.ini
                        </p>
                        <button class="btn btn-circle">
                            <ChevronRight />
                        </button>
                    </ListRow>
                </List>

                <!-- ListItem: Import INI -->
            </CardBody>
        </Card>

        <!-- Section: Mod Management -->
        <Card class="bg-base-200">
            <CardBody>
                <CardTitle>Mod Management</CardTitle>
                <List>
                    <ListRow>
                        <FolderDown class="text-accent" />
                        <div class="flex flex-col">
                            <p
                                class="text-base font-medium leading-normal truncate"
                            >
                                Mod Directory
                            </p>
                            <p
                                class="text-base-content/80 text-xs font-normal leading-normal truncate font-mono"
                            >
                                ./Data Files/Mods
                            </p>
                        </div>
                        <button class="btn btn-circle">
                            <ChevronRight />
                        </button>
                    </ListRow>
                    <ListRow>
                        <ArrowUpDown class="text-secondary" />
                        <div class="flex flex-col">
                            <p
                                class="text-foreground text-base font-medium leading-normal flex-1 truncate"
                            >
                                Load Order
                            </p>
                            <span
                                class="text-base-content/80 text-xs font-normal leading-normal truncate font-mono"
                                >12 Active</span
                            >
                        </div>
                        <button class="btn btn-circle">
                            <ChevronRight />
                        </button>
                    </ListRow>
                    <!-- Toggle: Auto Update -->
                    <ListRow>
                        <HistoryIcon class="text-green-500" />
                        <label
                            for="auto-update"
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Auto-Check Updates
                        </label>
                        <input
                            id="auto-update"
                            type="checkbox"
                            checked={true}
                            class="toggle"
                        />
                    </ListRow>
                </List>
            </CardBody>
        </Card>

        <!-- Section: Graphics -->
        <Card class="bg-base-200">
            <CardBody>
                <CardTitle>Graphics & Performance</CardTitle>
                <List>
                    <ListRow>
                        <Proportions class="text-pink-500" />
                        <div class="flex flex-col">
                            <p
                                class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                            >
                                Resolution Scale
                            </p>
                            <span
                                class="text-base-content/80 text-xs font-normal leading-normal truncate font-mono"
                                >1.0x</span
                            >
                        </div>
                        <button class="btn btn-circle">
                            <ChevronRight />
                        </button>
                    </ListRow>
                    <ListRow>
                        <Grid3x3 class="text-violet-500" />
                        <div class="flex flex-col">
                            <p
                                class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                            >
                                Texture Quality
                            </p>
                            <span
                                class="text-base-content/80 text-xs font-normal leading-normal truncate font-mono"
                                >High</span
                            >
                        </div>
                        <button class="btn btn-circle">
                            <ChevronRight />
                        </button>
                    </ListRow>
                </List>
            </CardBody>
        </Card>

        <!-- Section: App Preferences -->
        <Card class="bg-base-200">
            <CardBody>
                <CardTitle>App Preferences</CardTitle>
                <List>
                    <ListRow>
                        <div
                            class=" dark:text-gray-200 flex items-center justify-center rounded-lg bg-gray-200 dark:bg-gray-700 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >palette</span
                            >
                        </div>

                        <select
                            id="theme-changer"
                            data-choose-theme
                            class="select"
                        >
                            {#each themes as theme}
                                <option class="capitalize" value={theme}
                                    >{theme}</option
                                >
                            {/each}
                        </select>
                    </ListRow>
                </List>
            </CardBody>
        </Card>

        <!-- Danger Zone / Footer -->
        <div class="mt-8 mx-4 flex flex-col gap-4">
            <button
                class="w-full h-12 rounded-xl bg-red-500/10 hover:bg-red-500/20 text-red-500 font-bold text-base transition-colors flex items-center justify-center gap-2"
            >
                <span class="material-symbols-outlined text-[20px]"
                    >delete_forever</span
                >
                Reset Configuration
            </button>
            <div class="text-center pb-8">
                <p class=" dark:text-gray-600 text-sm font-medium">
                    Open Morrowind Loader
                </p>
                <p class=" dark:text-gray-600 text-xs mt-1">
                    Version 0.42.0 (Build 2023.10)
                </p>
            </div>
        </div>
    </div>
</div>
