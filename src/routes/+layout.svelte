<script>
    import BottomNavbar from "$lib/components/BottomNavbar.svelte";
    import "@fontsource-variable/space-grotesk";
    import { ModeWatcher } from "mode-watcher";
    import "../app.css";

    let { children } = $props();
</script>

<ModeWatcher />

{@render children()}

<BottomNavbar />
