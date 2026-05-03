<script lang="ts">

    import type { Snippet } from 'svelte';

    let { 
        children, 
        onclick
    }: { 
        children: Snippet;
        onclick: () => void;
    } = $props();

</script>

<button class="btn" onclick={onclick}>
    {@render children()}
</button>

<style>
    .btn {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        gap: 6px;
        border-radius: 7px;
        padding: 5px;
        border-width: 1px;

        transition: background-color 0.15s ease;
    }

    .btn:hover {
        background-color: var(--color-hover);
        
        border-color: var(--color-divider);
    }
</style>