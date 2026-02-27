<!-- search-input.svelte -->
<script lang="ts">
    import type { HTMLInputAttributes, HTMLInputTypeAttribute } from "svelte/elements";
    import { cn } from "$lib/utils.js";
    import SearchIcon from "$lib/icons/search-icon.svelte";
    import CrossIcon from "$lib/icons/cross-icon.svelte";

    type InputType = Exclude<HTMLInputTypeAttribute, "file">;

    type Props = Omit<HTMLInputAttributes, "type"> & {
        type?: InputType;
        ref?: HTMLInputElement | null;
        onClear?: () => void;
        onEnter?: () => void;
    };

    let {
        ref = $bindable(null),
        value = $bindable(),
        type,
        class: className,
        onClear,
        onEnter,
        ...restProps
    }: Props = $props();

    function handleClear() {
        value = "";
        onClear?.();
    }
</script>

<div class="relative flex items-center w-full">
    <span class="absolute left-3 flex items-center pointer-events-none text-muted-foreground">
        <SearchIcon size={16} />
    </span>

    <input
        onkeydown={(e) => {
            if (e.key === "Enter") onEnter?.();
        }}
        bind:this={ref}
        class={cn(
            "border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground flex h-7.5 w-full min-w-0 rounded-full border px-9 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            "focus-visible:border-ring focus-visible:ring-[var(--color-accent)] focus-visible:ring-[3px]",
            "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
            className
        )}
        {type}
        bind:value
        {...restProps}
    />

    {#if value}
        <button
            type="button"
            onclick={handleClear}
            class="absolute right-3 flex items-center text-muted-foreground hover:text-foreground transition-colors"
            aria-label="Clear input"
        >
            <CrossIcon size={16} />
        </button>
    {/if}
</div>