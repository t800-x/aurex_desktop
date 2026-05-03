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
    <div class="inner">
        {@render children()}
    </div>
</button>

<style>
    .btn {
        height: 30px;
        width: 115px;
        background-color: var(--color-accent);
        isolation: isolate;
        position: relative;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        border-radius: 5px;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;

        transition: background-color 0.15s ease;
    }

    .inner {
        position: absolute;
        z-index: 1;
        display: flex;
        gap: 5px;
    }

    .btn:hover {
        background-color: #ff5f7a;
    }


</style>