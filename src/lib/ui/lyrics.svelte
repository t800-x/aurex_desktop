<script lang="ts">
    import { commands, type FullTrack, type Lyrics, type LineLyrics, type SyllableLine } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";
    import LyricLine from "./lyric-line.svelte";

    let lyrics = $state<Lyrics | null>(null);
    let localTrack = $state<FullTrack | null>(null);

    let itemEls: Array<HTMLElement | null> = $state([]);
    let listEl: HTMLDivElement | null = $state(null);
    let innerEl: HTMLDivElement | null = $state(null);
    let scrollOffset = 0;

    let lineItems = $derived<LineLyrics[]>(
        !lyrics || lyrics.lyricstype !== "Line" || !lyrics.line_lyrics
            ? []
            : lyrics.line_lyrics.map((line, i) => {
                if (lyrics!.line_lyrics![i - 1]?.start_time === line.start_time && line.end_time != null) {
                    return { ...line, start_time: line.end_time };
                }
                return line;
            })
    );

    let syllableItems = $derived<SyllableLine[]>(
        !lyrics || lyrics.lyricstype !== "Syllable" || !lyrics.syllable_lyrics
            ? []
            : lyrics.syllable_lyrics.map((line, i) => {
                if (lyrics!.syllable_lyrics![i - 1]?.start_time === line.start_time) {
                    return { ...line, start_time: line.end_time };
                }
                return line;
            })
    );

    let activeIndex = $derived(
        (() => {
            const items = lyrics?.lyricstype === "Syllable" ? syllableItems : lineItems;
            return items.reduce<number>((acc, line, i) => {
                const effectiveEnd = items[i + 1]?.start_time ?? line.end_time ?? Infinity;
                if (audioPlayer.position >= line.start_time && audioPlayer.position < effectiveEnd) {
                    return i;
                }
                return acc;
            }, -1);
        })()
    );

    let prevActiveIndex = $state(-1);

    function updateScroll() {
        const activeEl = itemEls[activeIndex];
        if (!activeEl || !listEl || !innerEl) return;

        const containerHeight = listEl.getBoundingClientRect().height;
        const newOffset = Math.max(0, activeEl.offsetTop - containerHeight * 0.1);
        const delta = newOffset - scrollOffset;
        scrollOffset = newOffset;

        for (let i = 0; i < itemEls.length; i++) {
            const el = itemEls[i];
            if (!el) continue;
            el.style.transition = 'none';
            el.style.transform = `translateY(${delta}px)`;
        }

        innerEl.style.transform = `translateY(-${scrollOffset}px)`;

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
        if (tracksMatched) return;

        localTrack = audioPlayer.currentlyPlaying;
        lyrics = null;
        scrollOffset = 0;
        if (innerEl) innerEl.style.transform = 'translateY(0)';

        if (!audioPlayer.currentlyPlaying) {
            itemEls = [];
            return;
        }

        commands.getLyrics(audioPlayer.currentlyPlaying).then((l) => {
            lyrics = l;
            const len = l.lyricstype === "Syllable"
                ? (l.syllable_lyrics?.length ?? 0)
                : (l.line_lyrics?.length ?? 0);
            itemEls = new Array(len).fill(null);
        });
    });
</script>

<div class="lyricsContainer">
    {#if !lyrics}
        <!-- loading / no track, show nothing -->
    {:else if lyrics.lyricstype === "Syllable" && syllableItems.length > 0}
        <div bind:this={listEl} class="lyricsDisplay">
            <div bind:this={innerEl} class="lyricsInner">
                {#each syllableItems as syllableLine, index (`${index}-${syllableLine.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={null} syllableLyrics={syllableLine} active={index === activeIndex} />
                    </div>
                {/each}
            </div>
        </div>
    {:else if lyrics.lyricstype === "Unsynced" && lyrics.unsynced}
        <div class="unsyncedDisplay">
            {#each lyrics.unsynced.split('\n') as para}
                <p class="unsyncedLine" class:empty={para.trim() === ''}>{para || '\u00A0'}</p>
            {/each}
        </div>
    {:else if lyrics.lyricstype === "Line" && lineItems.length > 0}
        <div bind:this={listEl} class="lyricsDisplay">
            <div bind:this={innerEl} class="lyricsInner">
                {#each lineItems as line, index (`${index}-${line.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={line} syllableLyrics={null} active={index === activeIndex} />
                    </div>
                {/each}
            </div>
        </div>
    {/if}
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
        overflow: hidden;
        height: 100%;
        width: 100%;
        position: relative;
    }

    .lyricsInner {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        width: 100%;
    }

    .item-wrap {
        display: block;
        width: 100%;
        will-change: transform;
    }

    .unsyncedDisplay {
        padding: 24px 20px;
        overflow-y: auto;
        height: 100%;
        width: 100%;
        box-sizing: border-box;
    }

    .unsyncedLine {
        margin: 0;
        line-height: 1.7;
        font-size: 1rem;
        opacity: 0.85;
    }

    .unsyncedLine.empty {
        height: 1em;
    }
</style>