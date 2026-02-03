<script lang="ts">
    import { cn } from "$lib/utils";
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLLIElement> {
        children: Snippet;
    }

    let { children, class: className, ...restProps }: Props = $props();
</script>

<li class={cn("list-row", className)} {...restProps}>
    {@render children()}
</li>
