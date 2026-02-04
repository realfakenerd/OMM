<script lang="ts">
    import { Card, CardBody, CardTitle } from "$lib/components/ui/card";
    import {
        Boxes,
        ChartColumn,
        HardDrive,
        LogOut,
        Rocket,
        TrendingUp,
    } from "@lucide/svelte";

    // Dados simulados para o profile
    const stats = [
        { label: "Mods", value: "142", Icon: Boxes },
        { label: "Ver", value: "0.48", Icon: Rocket },
        { label: "GB", value: "4.2", Icon: HardDrive },
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

<!-- Top App Bar -->
<div
    class="flex justify-between items-center p-4 sticky top-0 bg-base-100/70 backdrop-blur z-50"
>
    <h1 class="tracking-tight text-4xl font-bold leading-tight">Profile</h1>
    <div class="flex w-12 items-center justify-end">
        <button
            class="flex items-center justify-center rounded-full w-10 h-10 hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
        >
            <span class="material-symbols-outlined text-primary">edit</span>
        </button>
    </div>
</div>

<div class="px-4 pb-4 flex flex-col gap-4">
    <div class="flex flex-col items-center py-6 px-4 relative">
        <div
            class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-64 bg-primary/20 rounded-full blur-3xl pointer-events-none z-0"
        ></div>

        <div class="z-10 flex flex-col items-center gap-4">
            <div class="avatar indicator">
                <div class="indicator-item badge badge-secondary aspect-auto">
                    PRO
                </div>
                <div
                    class="ring-primary ring-offset-base-100 w-24 rounded-full ring-2 ring-offset-2"
                >
                    <img
                        src="https://img.daisyui.com/images/profile/demo/spiderperson@192.webp"
                        alt="blablabla"
                    />
                </div>
            </div>
            <div class="text-center">
                <h1 class="text-2xl font-bold tracking-tight">Nerevarine_99</h1>
                <p class="text-base-content/80 font-body text-sm mt-1">
                    OpenMW Veteran • Since 2023
                </p>
            </div>
        </div>
    </div>

    <div class="stats w-full bg-base-100 ring-base-300 ring">
        {#each stats as stat}
            <div class="stat">
                <p class="stat-title flex justify-between">
                    {stat.label}
                    <stat.Icon class="text-primary size-4" />
                </p>
                <p class="stat-value">
                    {stat.value}
                </p>
            </div>
        {/each}
    </div>

    <Card class="bg-base-100 ring-base-300 ring">
        <CardBody>
            <CardTitle class="justify-between">
                Activity (30 Days)

                <ChartColumn class="text-primary" />
            </CardTitle>

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
                    class="text-slate-500 text-xs font-bold uppercase tracking-wider"
                >
                    Wk 1
                </p>
                <p
                    class="text-slate-500 text-xs font-bold uppercase tracking-wider"
                >
                    Wk 2
                </p>
                <p
                    class="text-slate-500 text-xs font-bold uppercase tracking-wider"
                >
                    Wk 3
                </p>
                <p
                    class="text-slate-500 text-xs font-bold uppercase tracking-wider"
                >
                    Wk 4
                </p>
            </div>
        </CardBody>
    </Card>
    <button class="btn btn-error">
        <LogOut />
        Log Out
    </button>
</div>
