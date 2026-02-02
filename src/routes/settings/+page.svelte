<script lang="ts">
    let autoUpdate = $state(false);
    let showFPS = $state(true);
    let hapticFeedback = $state(true);

    function handleDone() {
        window.location.href = "/";
    }
</script>

<div
    class=" bg-background font-display antialiased text-foreground transition-colors duration-200 min-h-screen"
>
    <div
        class="relative flex h-screen w-full flex-col overflow-hidden max-w-md mx-auto shadow-2xl bg-background"
    >
        <!-- TopAppBar -->
        <div
            class="flex flex-col gap-2 p-4 pb-2 z-10 /90 bg-background/90 backdrop-blur-md sticky top-0"
        >
            <div class="flex items-center h-12 justify-between">
                <div class="w-12"></div>
                <!-- Spacer -->
                <div class="flex w-12 items-center justify-end">
                    <button
                        onclick={handleDone}
                        class="text-primary hover:text-primary/80 text-base font-bold leading-normal tracking-[0.015em] shrink-0 transition-colors"
                    >
                        Done
                    </button>
                </div>
            </div>
            <h1
                class=" text-foreground tracking-tight text-[32px] font-bold leading-tight"
            >
                Settings
            </h1>
        </div>

        <h3
            class=" dark:text-[#ab9db9] text-xs font-bold leading-tight tracking-[0.05em] px-6 pb-2 pt-6 uppercase"
        >
            Installation
        </h3>
        <div
            class="mx-4 bg-surface-light dark:bg-surface-dark rounded-xl overflow-hidden shadow-sm border border-gray-200 dark:border-white/5"
        >
            <!-- ListItem: Game Data Path -->
            <button
                class="w-full relative flex items-center gap-3 px-4 min-h-18 py-3 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left group"
            >
                <div class="flex items-center gap-3 overflow-hidden">
                    <div
                        class="text-primary flex items-center justify-center rounded-lg bg-primary-settings/10 shrink-0 size-10 group-hover:scale-105 transition-transform"
                    >
                        <span class="material-symbols-outlined text-[24px]"
                            >folder_open</span
                        >
                    </div>
                    <div class="flex flex-col justify-center min-w-0">
                        <p
                            class=" text-foreground text-base font-medium leading-normal truncate"
                        >
                            Game Data Path
                        </p>
                        <p
                            class=" dark:text-[#ab9db9] text-xs font-normal leading-normal truncate font-mono"
                        >
                            /var/mobile/Documents/Morrowind
                        </p>
                    </div>
                </div>
                <div class="shrink-0 pl-2">
                    <div
                        class="flex items-center justify-center overflow-hidden rounded-full h-8 px-4 bg-primary-settings/10 text-primary text-sm font-semibold leading-normal w-fit transition-colors"
                    >
                        Change
                    </div>
                </div>
            </button>
            <!-- ListItem: Import INI -->
            <button
                class="w-full relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left group"
            >
                <div class="flex items-center gap-3">
                    <div
                        class="text-primary flex items-center justify-center rounded-lg bg-primary-settings/10 shrink-0 size-10 group-hover:scale-105 transition-transform"
                    >
                        <span class="material-symbols-outlined text-[24px]"
                            >description</span
                        >
                    </div>
                    <p
                        class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                    >
                        Import Morrowind.ini
                    </p>
                </div>
                <div class="shrink-0 dark:">
                    <span class="material-symbols-outlined">chevron_right</span>
                </div>
            </button>
        </div>

        <!-- Section: Mod Management -->
        <div>
            <h3
                class=" dark:text-[#ab9db9] text-xs font-bold leading-tight tracking-[0.05em] px-6 pb-2 pt-6 uppercase"
            >
                Mod Management
            </h3>
            <div
                class="mx-4 bg-surface-light dark:bg-surface-dark rounded-xl overflow-hidden shadow-sm border border-gray-200 dark:border-white/5"
            >
                <button
                    class="w-full relative flex items-center gap-3 px-4 min-h-16 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left group"
                >
                    <div class="flex items-center gap-3 overflow-hidden">
                        <div
                            class="text-blue-500 flex items-center justify-center rounded-lg bg-blue-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >topic</span
                            >
                        </div>
                        <div class="flex flex-col justify-center min-w-0">
                            <p
                                class=" text-foreground text-base font-medium leading-normal truncate"
                            >
                                Mod Directory
                            </p>
                            <p
                                class=" dark:text-[#ab9db9] text-xs font-normal leading-normal truncate font-mono"
                            >
                                ./Data Files/Mods
                            </p>
                        </div>
                    </div>
                    <div class="shrink-0 dark:">
                        <span class="material-symbols-outlined"
                            >chevron_right</span
                        >
                    </div>
                </button>
                <button
                    class="w-full relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left group"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-orange-500 flex items-center justify-center rounded-lg bg-orange-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >reorder</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Load Order
                        </p>
                    </div>
                    <div class="shrink-0 flex items-center gap-2">
                        <span class=" dark: text-sm">12 Active</span>
                        <span class="material-symbols-outlined dark:"
                            >chevron_right</span
                        >
                    </div>
                </button>
                <!-- Toggle: Auto Update -->
                <div
                    class="relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-green-500 flex items-center justify-center rounded-lg bg-green-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >update</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Auto-Check Updates
                        </p>
                    </div>
                    <label
                        class="shrink-0 relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in cursor-pointer"
                    >
                        <input
                            type="checkbox"
                            bind:checked={autoUpdate}
                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 border-gray-300 dark:border-[#302839] appearance-none cursor-pointer transition-all duration-300 left-0 checked:left-6 checked:border-primary-settings"
                        />
                        <div
                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 dark:bg-[#302839] cursor-pointer transition-colors duration-300"
                        ></div>
                    </label>
                </div>
            </div>
        </div>

        <!-- Section: Graphics -->
        <div>
            <h3
                class=" dark:text-[#ab9db9] text-xs font-bold leading-tight tracking-[0.05em] px-6 pb-2 pt-6 uppercase"
            >
                Graphics & Performance
            </h3>
            <div
                class="mx-4 bg-surface-light dark:bg-surface-dark rounded-xl overflow-hidden shadow-sm border border-gray-200 dark:border-white/5"
            >
                <button
                    class="w-full relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-pink-500 flex items-center justify-center rounded-lg bg-pink-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >aspect_ratio</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Resolution Scale
                        </p>
                    </div>
                    <div class="shrink-0 flex items-center gap-2">
                        <span class=" dark: text-sm">1.0x</span>
                        <span class="material-symbols-outlined dark:"
                            >chevron_right</span
                        >
                    </div>
                </button>
                <button
                    class="w-full relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-purple-500 flex items-center justify-center rounded-lg bg-purple-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >texture</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Texture Quality
                        </p>
                    </div>
                    <div class="shrink-0 flex items-center gap-2">
                        <span class=" dark: text-sm">High</span>
                        <span class="material-symbols-outlined dark:"
                            >chevron_right</span
                        >
                    </div>
                </button>
                <!-- Toggle: FPS -->
                <div
                    class="relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-yellow-500 flex items-center justify-center rounded-lg bg-yellow-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >speed</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Show FPS Counter
                        </p>
                    </div>
                    <label
                        class="shrink-0 relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in cursor-pointer"
                    >
                        <input
                            type="checkbox"
                            bind:checked={showFPS}
                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 border-gray-300 dark:border-[#302839] appearance-none cursor-pointer transition-all duration-300 left-0 checked:left-6 checked:border-primary-settings"
                        />
                        <div
                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 dark:bg-[#302839] cursor-pointer transition-colors duration-300"
                        ></div>
                    </label>
                </div>
            </div>
        </div>

        <!-- Section: App Preferences -->
        <div>
            <h3
                class=" dark:text-[#ab9db9] text-xs font-bold leading-tight tracking-[0.05em] px-6 pb-2 pt-6 uppercase"
            >
                App Preferences
            </h3>
            <div
                class="mx-4 bg-surface-light dark:bg-surface-dark rounded-xl overflow-hidden shadow-sm border border-gray-200 dark:border-white/5"
            >
                <button
                    class="w-full relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-left"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class=" dark:text-gray-200 flex items-center justify-center rounded-lg bg-gray-200 dark:bg-gray-700 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >palette</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Theme
                        </p>
                    </div>
                    <div class="shrink-0 flex items-center gap-2">
                        <span class=" dark: text-sm">System</span>
                        <span class="material-symbols-outlined dark:"
                            >chevron_right</span
                        >
                    </div>
                </button>
                <!-- Toggle: Haptics -->
                <div
                    class="relative flex items-center gap-3 px-4 min-h-14 py-2 justify-between list-group-item"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="text-teal-500 flex items-center justify-center rounded-lg bg-teal-500/10 shrink-0 size-10"
                        >
                            <span class="material-symbols-outlined text-[24px]"
                                >vibration</span
                            >
                        </div>
                        <p
                            class=" text-foreground text-base font-medium leading-normal flex-1 truncate"
                        >
                            Haptic Feedback
                        </p>
                    </div>
                    <label
                        class="shrink-0 relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in cursor-pointer"
                    >
                        <input
                            type="checkbox"
                            bind:checked={hapticFeedback}
                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 border-gray-300 dark:border-[#302839] appearance-none cursor-pointer transition-all duration-300 left-0 checked:left-6 checked:border-primary-settings"
                        />
                        <div
                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 dark:bg-[#302839] cursor-pointer transition-colors duration-300"
                        ></div>
                    </label>
                </div>
            </div>
        </div>

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

<style>
    /* Toggle Switch styling */
    .toggle-checkbox:checked {
        right: 0;
        border-color: #7f13ec;
    }
    .toggle-checkbox:checked + .toggle-label {
        background-color: #7f13ec;
    }

    /* iOS-like grouped list separation lines */
    .list-group-item:not(:last-child)::after {
        content: "";
        position: absolute;
        bottom: 0;
        left: 60px; /* Aligned with text start */
        right: 0;
        height: 1px;
        background-color: rgba(171, 157, 185, 0.1);
    }
</style>
