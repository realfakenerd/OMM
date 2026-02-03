<script lang="ts">
    import { Avatar } from "bits-ui";

    // Dados simulados para o profile
    const stats = [
        { label: "Mods", value: "142", icon: "extension" },
        { label: "Ver", value: "0.48", icon: "rocket_launch" },
        { label: "GB", value: "4.2", icon: "hard_drive" },
    ];

    const menuItems = [
        {
            section: "Data Management",
            items: [
                {
                    label: "Mod cache",
                    icon: "cached",
                    color: "text-emerald-500",
                    bgColor: "bg-emerald-500/10",
                },
                {
                    label: "Saved Games",
                    icon: "save",
                    color: "text-yellow-500",
                    bgColor: "bg-yellow-500/10",
                },
            ],
        },
        {
            section: "Support",
            items: [
                {
                    label: "Join Discord",
                    icon: "forum",
                    color: "text-indigo-500",
                    bgColor: "bg-indigo-500/10",
                    actionIcon: "open_in_new",
                },
                {
                    label: "Report a Bug",
                    icon: "bug_report",
                    color: "text-red-500",
                    bgColor: "bg-red-500/10",
                },
            ],
        },
    ];
</script>

<div class="pb-8">
    <!-- Top App Bar -->
    <div
        class="sticky top-0 z-20 flex items-center bg-background-light/80 dark:bg-background-dark/80 backdrop-blur-md p-4 pb-2 justify-between border-b border-gray-200 dark:border-white/5 max-w-md mx-auto"
    >
        <div class="w-12"></div>
        <h2 class="text-xl font-bold leading-tight tracking-tight text-center">
            Profile
        </h2>
        <div class="flex w-12 items-center justify-end">
            <button
                class="flex items-center justify-center rounded-full w-10 h-10 hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            >
                <span class="material-symbols-outlined text-primary">edit</span>
            </button>
        </div>
    </div>

    <div class="max-w-md mx-auto">
        <!-- Profile Header -->
        <div
            class="flex flex-col items-center pt-6 pb-6 px-4 relative overflow-hidden"
        >
            <!-- Abstract Background Glow -->
            <div
                class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-64 bg-primary/20 rounded-full blur-[80px] pointer-events-none z-0"
            ></div>

            <div class="relative z-10 flex flex-col items-center gap-4">
                <div class="relative">
                    <div
                        class="absolute -inset-1 bg-linear-to-tr from-primary to-secondary rounded-full opacity-70 blur-sm"
                    ></div>
                    <div
                        class="absolute bottom-1 right-1 bg-primary text-white text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-background-dark"
                    >
                        PRO
                    </div>
                </div>
                <div class="text-center">
                    <h1 class="text-2xl font-bold tracking-tight">
                        Nerevarine_99
                    </h1>
                    <p
                        class="text-slate-500 dark:text-slate-400 font-body text-sm mt-1"
                    >
                        OpenMW Veteran • Since 2023
                    </p>
                </div>
            </div>
        </div>

        <!-- Stats Dashboard -->
        <div class="px-4 mb-2">
            <div class="grid grid-cols-3 gap-3">
                {#each stats as stat}
                    <div
                        class="flex flex-col items-center justify-center gap-1 rounded-xl p-4 bg-white dark:bg-card-dark shadow-sm ring-1 ring-black/5 dark:ring-white/5"
                    >
                        <span
                            class="material-symbols-outlined text-primary mb-1 text-[28px]"
                            >{stat.icon}</span
                        >
                        <p class="text-2xl font-bold leading-none">
                            {stat.value}
                        </p>
                        <p
                            class="text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wide"
                        >
                            {stat.label}
                        </p>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Activity Chart -->
        <div class="px-4 py-6">
            <div
                class="rounded-xl bg-white dark:bg-card-dark p-5 shadow-sm ring-1 ring-black/5 dark:ring-white/5"
            >
                <div class="flex items-center justify-between mb-4">
                    <div>
                        <p
                            class="text-slate-500 dark:text-slate-400 text-sm font-medium"
                        >
                            Activity (30 Days)
                        </p>
                        <div class="flex items-baseline gap-2">
                            <h3 class="text-2xl font-bold">24 hrs</h3>
                            <span
                                class="text-emerald-500 text-sm font-medium flex items-center"
                            >
                                <span
                                    class="material-symbols-outlined text-sm mr-0.5"
                                    >trending_up</span
                                >
                                +12%
                            </span>
                        </div>
                    </div>
                    <div
                        class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center text-primary"
                    >
                        <span class="material-symbols-outlined">bar_chart</span>
                    </div>
                </div>
                <div class="h-32 w-full relative">
                    <svg
                        fill="none"
                        height="100%"
                        preserveAspectRatio="none"
                        viewBox="0 0 478 150"
                        width="100%"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <defs>
                            <linearGradient
                                gradientUnits="userSpaceOnUse"
                                id="chartGradient"
                                x1="236"
                                x2="236"
                                y1="25"
                                y2="150"
                            >
                                <stop stop-color="#7f13ec" stop-opacity="0.5"
                                ></stop>
                                <stop
                                    offset="1"
                                    stop-color="#7f13ec"
                                    stop-opacity="0"
                                ></stop>
                            </linearGradient>
                        </defs>
                        <path
                            d="M0 109C18.1538 109 18.1538 21 36.3077 21C54.4615 21 54.4615 41 72.6154 41C90.7692 41 90.7692 93 108.923 93C127.077 93 127.077 33 145.231 33C163.385 33 163.385 101 181.538 101C199.692 101 199.692 61 217.846 61C236 61 236 45 254.154 45C272.308 45 272.308 121 290.462 121C308.615 121 308.615 149 326.769 149C344.923 149 344.923 1 363.077 1C381.231 1 381.231 81 399.385 81C417.538 81 417.538 129 435.692 129C453.846 129 453.846 25 472 25V149H0V109Z"
                            fill="url(#chartGradient)"
                        ></path>
                        <path
                            d="M0 109C18.1538 109 18.1538 21 36.3077 21C54.4615 21 54.4615 41 72.6154 41C90.7692 41 90.7692 93 108.923 93C127.077 93 127.077 33 145.231 33C163.385 33 163.385 101 181.538 101C199.692 101 199.692 61 217.846 61C236 61 236 45 254.154 45C272.308 45 272.308 121 290.462 121C308.615 121 308.615 149 326.769 149C344.923 149 344.923 1 363.077 1C381.231 1 381.231 81 399.385 81C417.538 81 417.538 129 435.692 129C453.846 129 453.846 25 472 25"
                            stroke="#7f13ec"
                            stroke-linecap="round"
                            stroke-width="3"
                        ></path>
                    </svg>
                </div>
                <div class="flex justify-between mt-3 px-2">
                    <p
                        class="text-slate-400 dark:text-slate-500 text-xs font-bold uppercase tracking-wider"
                    >
                        Wk 1
                    </p>
                    <p
                        class="text-slate-400 dark:text-slate-500 text-xs font-bold uppercase tracking-wider"
                    >
                        Wk 2
                    </p>
                    <p
                        class="text-slate-400 dark:text-slate-500 text-xs font-bold uppercase tracking-wider"
                    >
                        Wk 3
                    </p>
                    <p
                        class="text-slate-400 dark:text-slate-500 text-xs font-bold uppercase tracking-wider"
                    >
                        Wk 4
                    </p>
                </div>
            </div>
        </div>

        <!-- Settings Menu -->
        <div class="px-4 pb-4">
            {#each menuItems as section}
                <h3 class="text-lg font-bold px-1 pb-3 pt-2">
                    {section.section}
                </h3>
                <div
                    class="bg-white dark:bg-card-dark rounded-xl overflow-hidden shadow-sm ring-1 ring-black/5 dark:ring-white/5 mb-6"
                >
                    {#each section.items as item}
                        <button
                            class="w-full flex items-center justify-between p-4 hover:bg-black/5 dark:hover:bg-white/10 transition-colors border-b border-gray-100 dark:border-white/5 last:border-0 text-left"
                        >
                            <div class="flex items-center gap-3">
                                <div
                                    class="w-8 h-8 rounded-lg {item.bgColor} flex items-center justify-center"
                                >
                                    <span
                                        class="material-symbols-outlined {item.color} text-[20px]"
                                        >{item.icon}</span
                                    >
                                </div>
                                <span class="font-medium text-base"
                                    >{item.label}</span
                                >
                            </div>
                            <div class="flex items-center gap-2">
                                {#if "value" in item && item.value}
                                    <span class="text-sm text-slate-500"
                                        >{item.value}</span
                                    >
                                {/if}
                                <span
                                    class="material-symbols-outlined text-slate-400 text-[20px]"
                                    >{"actionIcon" in item
                                        ? item.actionIcon
                                        : "chevron_right"}</span
                                >
                            </div>
                        </button>
                    {/each}
                </div>
            {/each}

            <!-- Logout -->
            <button
                class="w-full rounded-xl bg-red-500/10 hover:bg-red-500/20 text-red-500 dark:text-red-400 font-bold p-4 flex items-center justify-center gap-2 transition-colors"
            >
                <span class="material-symbols-outlined">logout</span>
                Log Out
            </button>
            <p
                class="text-center text-xs text-slate-400 dark:text-slate-600 mt-6 mb-8 font-mono"
            >
                OpenMW Loader v0.48.0 (Build 2394)
            </p>
        </div>
    </div>
</div>
