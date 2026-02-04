<script lang="ts">
    import { Bell, Funnel, Play, Search } from "@lucide/svelte";

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

<div
    class="flex flex-col gap-4 p-4 sticky top-0 bg-base-100/70 backdrop-blur z-50"
>
    <div class="flex w-full justify-between items-center">
        <h1 class="tracking-tight text-4xl font-bold leading-tight">My Mods</h1>
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
        <input type="search" required placeholder="Filter installed mods..." />
        <Funnel />
    </label>
</div>

<!-- Mod List -->
<main class="p-4 flex flex-col gap-4">
    <div class="flex items-center justify-between pt-2 pb-1">
        <span class="text-xs font-bold text-primary/80 uppercase tracking-wider"
            >Installed ({mods.length})</span
        >
        <button class="text-xs font-semibold">Manage Load Order</button>
    </div>

    {#each filteredMods as mod (mod.id)}
        <div class="card card-side bg-base-200">
            <figure>
                <img
                    loading="lazy"
                    class="w-24 object-cover aspect-square"
                    src={mod.image}
                    alt="mod thumb"
                />
            </figure>
            <div class="card-body">
                <div class="card-title">
                    {mod.name}
                </div>

                <div class="card-action">
                    <label class="toggle">
                        <input
                            type="checkbox"
                            bind:checked={mod.active}
                            class="sr-only peer"
                        />
                    </label>
                </div>
            </div>
        </div>
    {/each}
</main>

<div class="fab bottom-24">
    <button onclick={handlePlay} class="btn btn-primary w-full">
        <Play />
        <span>Play Morrowind</span>
    </button>
</div>
