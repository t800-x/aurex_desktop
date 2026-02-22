<script lang="ts">
    import type { LineLyrics } from "$lib/bindings";
    import { commands } from "$lib/bindings";

    let {
        lyrics,
        active = false,
    } : {
        lyrics: LineLyrics;
        active?: boolean;
    } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    onclick={() => commands.seek(lyrics.start_time)}
    class:active
    class="tile"
>
    {lyrics.line}
</div>

<style>
    .tile {
        padding: 10px;
        font-weight: 700;
        font-size: 25px;
        color: rgba(255, 255, 255, 0.3);
        text-align: left;
        width: 100%;
        transform: scale(0.8);
        transform-origin: left center;
        border-radius: 8px;
        cursor: default;
        /* active -> inactive: slow */
        transition: transform 0.4s ease-in-out, color 0.4s ease-in-out;
    }

    .tile:hover {
        background-color: var(--color-hover);
    }

    .active {
        transform: scale(0.9);
        color: rgba(255, 255, 255, 0.93);
        /* inactive -> active: fast */
        transition: transform 0.15s ease-out, color 0.15s ease-out;
    }
</style>