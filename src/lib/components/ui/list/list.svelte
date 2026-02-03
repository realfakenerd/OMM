<script lang="ts">
    import { cn } from "$lib/utils";
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLUListElement> {
        children: Snippet;
    }

    let { children, class: className, ...restProps }: Props = $props();
</script>

<ul class={cn("list", className)} {...restProps}>
    {@render children()}
</ul>
