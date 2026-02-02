<script lang="ts">
    // Lista de mods instalados com estado reativo
    let mods = $state([
        {
            id: 1,
            name: "Tamriel Rebuilt",
            version: "v22.11",
            size: "2.4 GB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuAgls5J9bT47HwQN44CeBvhfPyd70k7SHilWrGNzR6Bkj6s2kFYPKKPtz_kKK97DS1bzeZXLA0kbOXW19U3x0tgbg6nfQ5U2CFu_NIXvrVIsxPYIqR7MD77zvif2ecIjlVRW6ZI64n8Fp0ATppfOJPkEUelTrcdzKYp76hxmtje-Gl43DoHpBKv9TlZAeN970AFV7gipo2fUL6aK6d_L4mt_ARdF1R71Dehev8tb1WyQ2xB2cCUYvLVEut7id1N7_8B1XQfyTLxPLKN",
        },
        {
            id: 2,
            name: "MGE XE Graphics",
            version: "v0.13.7",
            size: "120 MB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuAF_xhE-he09uywNp5kJGI2AtYJXMwHpmIHb2bYyhZGKEd1tqIyfDde71NxpwTi5D0R52TxSMyplXaXN1LupuyyaDr-JQQwy4Kv6GS6vosJ7mcbs8KwB8Ghbm_mPFaagTSHQkga-YGIWTg2CnqfXAqBXeuc3dWEbaG3y5Ibf-wwmEnSDq0DUbwOKtrcMUxhROKyC2VZjDTY6t7QeXVtiXcc9Lq0ZrmA30RE4xMYMMI2KW5bcFA93V22c_W__FjZAhZqAoCTsY_AmV2S",
        },
        {
            id: 3,
            name: "Patch for Purists",
            version: "v4.0.2",
            size: "8 MB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuA0CtfxJ9ivsZPX0NfcEr0kQbPgru_HC_BIPeWESfsDP7UsZXxNOWERyuGdu5Wo58fPwrKtKpfOJZcVyCXhdggf6zqJw4Q1YbRRtU38iU7LvfiWyBI-Q3m-3CYe4_w7NVJ66-ix4UQZVaEnIdFkEgqrcucRVvX9TVKgJtibW6-EW5xGXMfnUEwRD6QQnZRgoVR2WE1PCfXtF9Ot5NqgtmHMY9UZovmrObH16YtPiLBpbZRf1FXJ_x6zsd6g3HUp9lxIHY00_J6fYh6y",
        },
        {
            id: 4,
            name: "Rise of House Telvanni",
            version: "v2.1.0",
            size: "45 MB",
            active: false,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuAIReDfAopTpSBoMf3SEAFP7IjImjgj1Zi3Mi_NKFTxRT7lAOmWtWuMNRoA813WJiLm2rhOqw9NhRvGYLaW5YugbNmkKVc9dIrpbQjCfNKo6-MvEDv9xESjFHpYGbO1b6k6vnZmfXTii7QbxQ25TwGbzhmbAb4o4Pu42kbQfv2c951T0VoXmgABFD4u-fSJ9dgrp7X0p28eq8qhk0xEVTdGeI-Ya0-A-EXFkIrD6zZHvH894E8GxDB8BYg_7bnodIljfwRPpAoO_MvD",
        },
        {
            id: 5,
            name: "Better Heads",
            version: "v1.1",
            size: "54 MB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuBEjb0siBbxlnBOfVMrA0ecU6hsNcA8rlaSQ8HPD8EcJxxRk4XUJlzVt--VXe94SughJOBW95vZBlA3uQxjiFzazZE1gO3dkMjouRCfTv1PDXQ9sILqpkMceLYhfYoY3T8gBD1jZf-fIEWXMZRhAvi8lBCPVW4rV7zjA0Tk-yyQWLksYdEuLRw_s1dbabLzS4xasj8r7VbBgGPaodO7Fvxew7n1ybaDyyrzn3-_XRx05rIeFBWPZpmW59yVS_zFH6iciAOKz7qepicY",
        },
        {
            id: 6,
            name: "Tamriel Data",
            version: "v9.0",
            size: "800 MB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCi-EaqF3Z3tRqtivmlCb2o-y3qN-U7WzPqelEprpWEDvtvzR10J3WAQxfc-JzfY8kuodhw0-WKZDa3baDr2EehBC9j_7GNdyDIrq4XaKnlgCAzv6MXoqlSHVgaiSarovNc1izblKmlMMBzlzqNNJrsW0I9tofyCCyGv8xif8zF6FoZqHaJx-adtWp2-ok7RWzy-1kFWtlAPVY_aqTqZH3o5Jcah7lWHrI6PxPTq5GKS2UEd5PhuotTPiKvPrYzml1WAFkJoezkIxsF",
        },
        {
            id: 7,
            name: "Vurt's Groundcover",
            version: "v2.3",
            size: "12 MB",
            active: false,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCL-gx5PdGMEKWRW8QeTzOmK-FNdQM2omS6q2WlU916ZCIyTbIpucZGx3CmuHcKfvzmpMoIioEnTu0XTBnfn_6-Va3c8JNJdJ-bnthBzrD-0Sm37MqLw6R59dcZG0Q99cRkuPd_kRJiEThCC1_X1g_NEeKmyL7PHcsR7PDrv3bXQ5nnzk6o4twsrgY7dGZ3f1ZIyg7mb5GmhTEe5BsX45cUquJwCIziZjcv93-S7J1UvWQ3_2hxvOfeD2LcsW-knce3YwWxtJldYF1n",
        },
        {
            id: 8,
            name: "Dwemer Armor HD",
            version: "v1.0",
            size: "15 MB",
            active: true,
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuBee1svpypBjzWfeUPCoa08_Blu9R0KolM8m5yZKfegAVWP0E-0EJLLyQd6ZaW0b7GysFdD2F8Uv7FNFMfgYkRIyk657lQdgEIl_LJ7iu1CqZAq9ETMl_X_asSjxjqnU70YI5U1lknwZutFy35jbp_I2vOCSN0hu_7kpgH_tR4_CeezzFuNnnXLq8cFW3h3VgqtBWiAmBJMvhehSj0CTQHUqhn5NB_dqrGQgjniEhcE8YpoFJTGjv6baFCgX7pkLLqHzhjB5rYMikBr",
        },
    ]);

    let searchQuery = $state("");

    const filteredMods = $derived(
        mods.filter((mod) =>
            mod.name.toLowerCase().includes(searchQuery.toLowerCase()),
        ),
    );

    function handlePlay() {
        console.log("Iniciando Morrowind...");
        // Aqui entraria a integração com o Tauri para abrir o app
    }
</script>

<!-- Header -->
<header class="sticky top-0 z-20 bg-background/95 backdrop-blur-md pt-2">
    <div class="flex items-center justify-between px-4 py-3">
        <div class="flex items-center gap-3">
            <button
                class="flex h-10 w-10 items-center justify-center rounded-full bg-card text-card-foreground"
            >
                <span class="material-symbols-outlined text-[24px]">menu</span>
            </button>
            <h1 class="text-2xl font-bold tracking-tight">My Mods</h1>
        </div>
        <div class="flex gap-2">
            <button
                class="flex h-10 w-10 items-center justify-center rounded-full bg-card text-card-foreground hover:bg-gray-100 dark:hover:bg-white/10 transition-colors relative"
            >
                <span class="material-symbols-outlined text-[24px]"
                    >notifications</span
                >
                <span
                    class="absolute top-2 right-2 h-2.5 w-2.5 rounded-full bg-primary-settings ring-2 ring-background-dark"
                ></span>
            </button>
        </div>
    </div>

    <!-- Search -->
    <div class="px-4 pb-4">
        <div class="relative flex w-full items-center">
            <span
                class="absolute left-4 text-[#ab9db9] material-symbols-outlined"
                >search</span
            >
            <input
                bind:value={searchQuery}
                class="w-full rounded-2xl border-none bg-card py-3.5 pl-11 pr-4 text-sm font-medium text-gray-900 placeholder-[#ab9db9] focus:ring-2 focus:ring-primary-settings shadow-sm"
                placeholder="Filter installed mods..."
                type="text"
            />
            <button
                class="absolute right-3 p-1.5 rounded-lg text-[#ab9db9] hover:bg-background-dark/20 dark:hover:bg-white/10 transition-colors"
            >
                <span class="material-symbols-outlined text-[20px]"
                    >filter_list</span
                >
            </button>
        </div>
    </div>
</header>

<!-- Mod List -->
<main class="flex-1 overflow-y-auto pb-44 px-4 space-y-3 no-scrollbar">
    <div class="flex items-center justify-between pt-2 pb-1">
        <span class="text-xs font-bold text-[#ab9db9] uppercase tracking-wider"
            >Installed ({mods.length})</span
        >
        <button
            class="text-xs font-semibold text-primary-settings hover:opacity-80"
            >Manage Load Order</button
        >
    </div>

    {#each filteredMods as mod (mod.id)}
        <div
            class="flex items-center gap-4 rounded-2xl bg-card p-3 pr-4 shadow-sm border border-transparent dark:border-white/5 hover:border-primary-settings/30 transition-all {!mod.active
                ? 'opacity-80 grayscale-[0.3]'
                : ''}"
        >
            <div
                class="h-16 w-16 shrink-0 overflow-hidden rounded-xl bg-gray-800 shadow-inner"
            >
                <div
                    class="h-full w-full bg-cover bg-center"
                    style="background-image: url('{mod.image}');"
                ></div>
            </div>
            <div class="flex flex-1 flex-col">
                <h3 class="font-bold text-base text-gray-900 leading-tight">
                    {mod.name}
                </h3>
                <p class="text-xs text-[#ab9db9] font-medium mt-1">
                    {mod.version} • {mod.size}
                </p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
                <input
                    type="checkbox"
                    bind:checked={mod.active}
                    class="sr-only peer"
                />
                <div
                    class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-primary-settings shadow-inner"
                ></div>
            </label>
        </div>
    {/each}
</main>

<div
    class="fixed bottom-24 left-1/2 -translate-x-1/2 w-full max-w-md px-4 z-30 pb-4 pt-6"
>
    <button
        onclick={handlePlay}
        class="w-full bg-primary hover:opacity-90 active:scale-[0.98] transition-all h-14 rounded-2xl font-bold text-lg shadow-md shadow-primary/30 text-white flex items-center justify-center gap-3"
    >
        <span class="material-symbols-outlined fill-current text-[28px]"
            >play_arrow</span
        >
        <span>Play Morrowind</span>
    </button>
</div>
