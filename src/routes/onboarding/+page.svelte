<script lang="ts">
    const features = [
        {
            title: 'Plugin Manager',
            description: 'Organize ESP/ESM load order easily with drag-and-drop sorting.',
            icon: 'extension',
            image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuATfJSgFb-_KidbqzDkJ-hbxO9yb1YWx-eEdzckJwfBv3FoZ4xYEyF9qxwcHOiW6vIM8qeKfMtFmJZST8-Ov8SDGuMltLKbPgdY93YMn6-2B5WoAk05u7xIQnhOsi4-UDne3cifVZFN6ZYiQuyNvZQZCPV3nTwGlF70sp8zHLIKGPgpVnwchCeYBn9nDBkzq8bXb42GTZ_qdBN_R7PhwtptoE5hy92Dxy8n8H5UygUObUDs_VelGVcy1dPe-AxagBwEvtPKSfnahC8p'
        },
        {
            title: 'One-Click Install',
            description: 'Install mods directly from ZIP/7z archives without manual extraction.',
            icon: 'download_for_offline',
            image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuC2rFq4Z4s85-9z3jD3EWh4PSZTUN9i6KUxpdgCEQPhdxQwtL2ob4VpnYnUs2_wowhaQuxB0G_ypaCp-mW361H1ghELsKn3hVzQnFH3hL3nLOfAjq7gXEkBjLwPkuiYu3vk9AUyTlVEgaJsjyI4aMkfPN1C2QE111FmjbiaTxbBLRZjgRhWBSonY0qA1meTFp7zNCgsumjoFmTb95paEj9JlIG2nTL9ulww8Id8-ZiEAGZwCrI39gmKpp3RrxPv3YKPCnAYzvk-RdH3'
        },
        {
            title: 'Performance Boost',
            description: 'Optimized settings and texture handling for smoother mobile gameplay.',
            icon: 'speed',
            image: 'https://lh3.googleusercontent.com/aida-public/AB6AXuChpvq7R5CGxa08xBgFXj-v5adrQ8L2fn9XZ999-LbAIP-E2QUqXd7B6tYhWhR4AUdBqoyGMZ_RVC_bJ-z7PeHpoXjr2aLkhYrhKfNGDrFyHwJd4pxxzKf9qoBxvzgL6qVVpd6XQQCPJcmdcdYIcNdmQqFqwxMg73BTf17JF5DEjDcum8tVOusiGxi1DJjljYbf-049Q-5QUd7PBLJsD7pjQJgp4MpfItN9EN4PUjoYpEBf8VFlYUzyDuH8ZJpvRB4c9csXuRIF3lpC'
        }
    ];

    function handleStart() {
        // Redireciona para a página principal
        window.location.href = '/';
    }
</script>

<div class="bg-background-light dark:bg-background-dark font-display text-gray-900 dark:text-white antialiased overflow-x-hidden min-h-screen">
    <div class="relative flex h-auto min-h-screen w-full flex-col group/design-root">
        <div class="sticky top-0 z-50 flex items-center bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-sm p-4 pb-2 justify-between border-b border-gray-200 dark:border-white/10">
            <div class="text-gray-900 dark:text-white flex size-12 shrink-0 items-center justify-start cursor-pointer hover:opacity-70 transition-opacity">
                <span class="material-symbols-outlined text-[28px]">settings</span>
            </div>
            <div class="flex items-center gap-2">
                <span class="material-symbols-outlined text-primary-onboarding">swords</span>
                <h2 class="text-gray-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] text-center">Modloader</h2>
            </div>
            <div class="flex w-12 items-center justify-end cursor-pointer hover:opacity-70 transition-opacity">
                <p class="text-gray-500 dark:text-[#ab9db9] text-base font-bold leading-normal tracking-[0.015em] shrink-0">Help</p>
            </div>
        </div>
        
        <div class="p-0 sm:p-4">
            <div class="relative flex min-h-[460px] flex-col gap-6 bg-cover bg-center bg-no-repeat sm:rounded-xl items-center justify-end p-6 pb-10 overflow-hidden shadow-lg" 
                 style='background-image: linear-gradient(to top, rgba(25, 16, 34, 1) 0%, rgba(25, 16, 34, 0.4) 50%, rgba(25, 16, 34, 0.1) 100%), url("https://lh3.googleusercontent.com/aida-public/AB6AXuAIgqZyFmyrByJuAS8-Z4vSi4d2mbK_m2HNl7zpk6iSTeR8DAbkpo6enm5KWOFAW4c79DOv4LrelwJjomFV1VYasIBsKPfTdIMBWMulfEBds1gfenUfM2Gx7G2UhQgKo7h_JNEC2ec8sGH5D_3w7Nbu1ITut-jsszY7KoVnCPDl97S8YJRH_wH1u0O6OfeC_W7ecmKa0AxAdlUYfc4ZXlE0jF8NR2Ehmu_rpDEijP8TgT28PkeApUA-R-Jx9w-kcBzUd2e_XPFnXEvu");'>
                <div class="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-background-dark/90"></div>
                <div class="relative z-10 flex flex-col gap-4 text-center items-center w-full max-w-md">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-amber-500/20 border border-amber-500/30 backdrop-blur-md">
                        <span class="material-symbols-outlined text-amber-500 text-[18px]">warning</span>
                        <span class="text-amber-500 text-xs font-bold uppercase tracking-wider">Setup Required</span>
                    </div>
                    <h1 class="text-white text-4xl font-black leading-tight tracking-[-0.033em] sm:text-5xl drop-shadow-md">
                        Welcome to Modloader
                    </h1>
                    <h2 class="text-gray-200 text-sm font-medium leading-relaxed max-w-xs sm:text-base">
                        The complete modding environment for Open Morrowind on Android. Seamlessly install mods, organize plugins, and manage saves directly on your device.
                    </h2>
                </div>
                <button 
                    onclick={handleStart}
                    class="relative z-10 flex w-full max-w-[320px] cursor-pointer items-center justify-center overflow-hidden rounded-xl h-12 px-5 bg-primary-onboarding hover:opacity-90 active:scale-95 transition-all shadow-[0_0_20px_rgba(250,198,56,0.3)] text-background-dark text-base font-bold leading-normal tracking-[0.015em]">
                    <span class="mr-2">Start Modding</span>
                    <span class="material-symbols-outlined text-[20px]">arrow_forward</span>
                </button>
                <p class="relative z-10 text-xs text-white/40 mt-2">v1.0.2 • Open Source Port</p>
            </div>
        </div>

        <div class="px-4 pt-6">
            <div class="rounded-xl bg-card-light dark:bg-card-dark p-5 border border-gray-100 dark:border-white/5 shadow-sm">
                <h3 class="text-gray-900 dark:text-white text-base font-bold mb-2 flex items-center gap-2">
                    <span class="material-symbols-outlined text-primary-onboarding text-[20px]">info</span>
                    App Overview
                </h3>
                <p class="text-gray-600 dark:text-gray-300 text-sm leading-relaxed">
                    Modloader bridges the gap between desktop flexibility and mobile convenience. Designed specifically for the Android port of Open Morrowind, it allows you to import mods from zip files, automatically handle data paths, and sort your plugin load order with a touch-friendly interface.
                </p>
            </div>
        </div>

        <div class="px-4 pb-2 pt-6">
            <h2 class="text-gray-900 dark:text-white tracking-tight text-[24px] font-bold leading-tight text-left">Key Features</h2>
        </div>

        <div class="flex flex-col gap-4 p-4 pt-2">
            {#each features as feature}
                <div class="flex flex-col sm:flex-row items-stretch justify-between gap-4 rounded-xl bg-card-light dark:bg-card-dark p-4 shadow-sm border border-gray-100 dark:border-white/5">
                    <div class="flex flex-col gap-2 flex-[2_2_0px] justify-center">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center justify-center size-8 rounded-lg bg-primary-onboarding/20 text-primary-onboarding">
                                <span class="material-symbols-outlined text-[20px]">{feature.icon}</span>
                            </div>
                            <p class="text-gray-900 dark:text-white text-base font-bold leading-tight">{feature.title}</p>
                        </div>
                        <p class="text-gray-500 dark:text-[#ab9db9] text-sm font-normal leading-normal">{feature.description}</p>
                    </div>
                    <div class="w-full sm:w-auto sm:max-w-[120px] bg-center bg-no-repeat aspect-video bg-cover rounded-lg flex-1 shadow-inner opacity-90" 
                         style='background-image: url("{feature.image}");'></div>
                </div>
            {/each}
        </div>
        
        <div class="h-10"></div>
    </div>
</div>
