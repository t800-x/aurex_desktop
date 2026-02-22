<script lang="ts">
    import type { LineLyrics } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";
    import { commands } from "$lib/bindings";

    let {
        lyrics,
        index
    } : {
        lyrics: LineLyrics;
        index: number
    } = $props();

    let active = $state(false);
    let isInTimeFrame = $derived(audioPlayer.position >= lyrics.start_time && audioPlayer.position <= lyrics.end_time!);
    let idxNull = $derived(audioPlayer.lyricsLineIndex === null);
    let isIdx = $derived(audioPlayer.lyricsLineIndex === index);

    $effect(() => {
        if (lyrics.end_time !== null) {
            if (isInTimeFrame) {
                active = true;
                if (idxNull) {
                    audioPlayer.lyricsLineIndex = index;
                }
            } else {
                active = false;
                if (isIdx) {
                    audioPlayer.lyricsLineIndex = null;
                }
            }
        }
    });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div onclick={() => commands.seek(lyrics.start_time)} class:active={active} class="tile">
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

        transition: transform 0.25s ease-in-out, color 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    .tile:hover {
        background-color: var(--color-hover);
    }

    .active {
        transform: scale(0.9);
        color: rgba(255, 255, 255, 9.3);
    }
</style>

