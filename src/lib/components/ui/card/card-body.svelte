<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn, type WithElementRef } from "$lib/utils.js";

    let {
        class: className,
        children,
        ...restProps
    }: HTMLAttributes<HTMLDivElement> = $props();
</script>

<div class={cn("card-body", className)} {...restProps}>
    {@render children?.()}
</div>
