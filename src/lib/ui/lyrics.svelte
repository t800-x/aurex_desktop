<script lang="ts">
    import { commands, type FullTrack, type LineLyrics } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";
    import LineLyric from "./line-lyric.svelte";

    let line_lyrics: LineLyrics[] = $state([]);
    let itemEls: Array<HTMLElement | null> = [];
    let listEl: HTMLDivElement | null = null;
    let innerEl: HTMLDivElement | null = null;
    let localTrack = $state<FullTrack | null>(null);
    let scrollOffset = 0;

    let activeIndex = $derived(
        line_lyrics.reduce<number>((acc, line, i) => {
            const effectiveEnd = line_lyrics[i + 1]?.start_time ?? line.end_time ?? Infinity;
            if (audioPlayer.position >= line.start_time && audioPlayer.position < effectiveEnd) {
                return i;
            }
            return acc;
        }, -1)
    );

    let prevActiveIndex = $state(-1);

    function updateScroll() {
        const activeEl = itemEls[activeIndex];
        if (!activeEl || !listEl || !innerEl) return;

        const containerHeight = listEl.getBoundingClientRect().height;
        const newOffset = Math.max(0, activeEl.offsetTop - containerHeight * 0.1);
        const delta = newOffset - scrollOffset;
        scrollOffset = newOffset;

        // Step 1: instantly displace all items by the delta so visually nothing has moved yet
        for (let i = 0; i < itemEls.length; i++) {
            const el = itemEls[i];
            if (!el) continue;
            el.style.transition = 'none';
            el.style.transform = `translateY(${delta}px)`;
        }

        // Move the container to its new position simultaneously
        innerEl.style.transform = `translateY(-${scrollOffset}px)`;

        // Step 2: next frame, animate each item back to 0 with staggered delay
        requestAnimationFrame(() => {
            for (let i = 0; i < itemEls.length; i++) {
                const el = itemEls[i];
                if (!el) continue;
                const delay = Math.abs(i - activeIndex) * 30;
                el.style.transition = `transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94) ${delay}ms`;
                el.style.transform = 'translateY(0)';
            }
        });
    }

    $effect(() => {
        if (activeIndex !== prevActiveIndex) {
            prevActiveIndex = activeIndex;
            if (activeIndex !== -1) updateScroll();
        }
    });

    $effect(() => {
        const tracksMatched = JSON.stringify(localTrack) === JSON.stringify(audioPlayer.currentlyPlaying);
        if (!tracksMatched) {
            localTrack = audioPlayer.currentlyPlaying;
            if (audioPlayer.currentlyPlaying) {
                commands.getLineLyrics(audioPlayer.currentlyPlaying).then((l) => {
                    line_lyrics = l ?? [];
                    itemEls = new Array(line_lyrics.length).fill(null);
                    scrollOffset = 0;
                    if (innerEl) innerEl.style.transform = 'translateY(0)';
                });
            } else {
                line_lyrics = [];
                itemEls = [];
            }
        }
    });
</script>

<div class="lyricsContainer">
    <div bind:this={listEl} class="lyricsDisplay">
        <div bind:this={innerEl} class="lyricsInner">
            {#each line_lyrics as lyrics, index (lyrics.start_time)}
                <div class="item-wrap" bind:this={itemEls[index]}>
                    <LineLyric {lyrics} active={index === activeIndex} />
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .lyricsContainer {
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        overflow: hidden;
        height: 100%;
        width: 100%;
    }

    .lyricsDisplay {
        padding: 20px;
        flex: 1;
        overflow: hidden; /* no real scroll - items move themselves */
        height: 100%;
        width: 100%;
        position: relative;
    }

    .lyricsInner {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        width: 100%;
        /* transition on the container itself is instant - string effect is per-item */
    }

    .item-wrap {
        display: block;
        width: 100%;
        will-change: transform;
    }
</style>