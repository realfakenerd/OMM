<script lang="ts" module>
    import { cn } from "$lib/utils.js";
    import type {
        HTMLAnchorAttributes,
        HTMLButtonAttributes,
    } from "svelte/elements";
    import { type VariantProps, tv } from "tailwind-variants";

    export const buttonVariants = tv({
        base: "btn",
        variants: {
            variant: {
                default: "",
                destructive: "btn-error",
                outline: "btn-outline",
                secondary:
                    "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-xs",
                ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
                link: "text-primary underline-offset-4 hover:underline",
            },
            size: {
                default: "",
                sx: "btn-sx",
                sm: "btn-sm",
                lg: "btn-lg",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    });

    export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
    export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

    export type ButtonProps = HTMLButtonAttributes &
        HTMLAnchorAttributes & {
            variant?: ButtonVariant;
            size?: ButtonSize;
        };
</script>

<script lang="ts">
    let {
        class: className,
        variant = "default",
        size = "default",
        href = undefined,
        type = "button",
        disabled,
        children,
        ...restProps
    }: ButtonProps = $props();
</script>

{#if href}
    <a
        data-slot="button"
        class={cn(buttonVariants({ variant, size }), className)}
        href={disabled ? undefined : href}
        aria-disabled={disabled}
        role={disabled ? "link" : undefined}
        tabindex={disabled ? -1 : undefined}
        {...restProps}
    >
        {@render children?.()}
    </a>
{:else}
    <button
        data-slot="button"
        class={cn(buttonVariants({ variant, size }), className)}
        {type}
        {disabled}
        {...restProps}
    >
        {@render children?.()}
    </button>
{/if}
