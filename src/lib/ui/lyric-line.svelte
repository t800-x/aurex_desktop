<script lang="ts">
    import type { LineLyrics, SyllableLine, SyllableWord } from "$lib/bindings";
    import { commands } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";

    let {
        lineLyrics,
        syllableLyrics,
        active = false,
    }: {
        lineLyrics: LineLyrics | null;
        syllableLyrics: SyllableLine | null;
        active?: boolean;
    } = $props();

    function wordFill(word: SyllableWord, pos: number): number {
        if (pos <= word.start_time) return 0;
        if (pos >= word.end_time) return 100;
        return ((pos - word.start_time) / (word.end_time - word.start_time)) * 100;
    }

    function lineFill(line: SyllableLine, pos: number): number {
        if (pos <= line.start_time) return 0;
        if (pos >= line.end_time) return 100;

        const totalChars = line.words.reduce((a, w) => a + w.text.length, 0);
        let filledChars = 0;

        for (const word of line.words) {
            if (pos >= word.end_time) {
                filledChars += word.text.length;
            } else if (pos >= word.start_time) {
                filledChars += ((pos - word.start_time) / (word.end_time - word.start_time)) * word.text.length;
                break;
            } else {
                break;
            }
        }

        return (filledChars / totalChars) * 100;
    }
</script>

{#if syllableLyrics !== null}
    {@const _ = syllableLyrics}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
        onclick={() => commands.seek(_.start_time)}
        class:active
        class="tile"
    >
        {#each _.words as word}
            {@const fill = active ? wordFill(word, audioPlayer.position) : 0}
            <span
                class="word"
                data-text={word.text}
                style="--fill: {fill}%"
            >{word.text}</span>
        {/each}
    </div>
{:else if lineLyrics !== null}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
        onclick={() => commands.seek(lineLyrics.start_time)}
        class:active
        class="tile"
    >
        {lineLyrics.line}
    </div>
{/if}

<style>
    .tile {
        padding: 10px;
        font-size: 0; /* kills inline-block gaps */
        font-weight: 700;
        text-align: left;
        width: 100%;
        transform: scale(0.8);
        transform-origin: left center;
        filter: blur(2px);
        border-radius: 8px;
        cursor: default;
        transition: transform 0.4s ease-in-out;
    }
    .tile:hover {
        background-color: var(--color-hover);
    }
    .active {
        transform: scale(0.9);
        filter: none;
        transition: transform 0.15s ease-out;
    }

    .word {
        display: inline-block;
        font-size: 25px;
        white-space: pre-wrap;
        position: relative;
        color: rgba(255, 255, 255, 0.3);
    }

    .word::after {
        content: attr(data-text);
        position: absolute;
        left: 0;
        top: 0;
        color: rgba(255, 255, 255, 0.93);
        white-space: pre;
        clip-path: inset(0 calc(100% - var(--fill, 0%)) 0 0);
        pointer-events: none;
    }
</style>