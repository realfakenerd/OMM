<script>
    import BottomNavbar from "$lib/components/BottomNavbar.svelte";
    import "@fontsource-variable/space-grotesk";
    import "../app.css";
    import { themeChange } from "theme-change";

    let { children } = $props();

    $effect(() => {
        themeChange(false);
    });
</script>

{@render children()}

<BottomNavbar />
