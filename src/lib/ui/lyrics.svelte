<script lang="ts">
    import { commands, type FullTrack, type Lyrics, type LineLyrics, type SyllableLine } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";
    import LyricLine from "./lyric-line.svelte";
    import LyricGap from "./lyric-gap.svelte";

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
                const prev = lyrics!.syllable_lyrics![i - 1];
                // Only dedup if BOTH lines are normal vocals -- bg+main pairs from the same <p>
                // legitimately share a start_time and should not be touched
                if (prev?.start_time === line.start_time && !line.is_background && !prev.is_background) {
                    return { ...line, start_time: line.end_time };
                }
                return line;
            })
    );

    let activeIndices = $derived(
        (() => {
            const items = lyrics?.lyricstype === "Syllable" ? syllableItems : lineItems;
            return items.reduce<number[]>((acc, line, i) => {
                const nextStart = items[i + 1]?.start_time;
                
                let effectiveEnd;
                if (nextStart !== undefined) {
                    // If there's a next line, extend over the gap (nextStart) 
                    // OR keep alive during the overlap (line.end_time).
                    effectiveEnd = line.end_time != null 
                        ? Math.max(line.end_time, nextStart) 
                        : nextStart;
                } else {
                    // Fallback for the very last line of the song
                    effectiveEnd = line.end_time ?? Infinity;
                }
                
                if (audioPlayer.position >= line.start_time && audioPlayer.position < effectiveEnd) {
                    acc.push(i);
                }
                return acc;
            }, []);
        })()
    );

    let scrollTargetIndex = $derived(activeIndices[0] ?? -1);

    let prevScrollTarget = $state(-1);

    function updateScroll() {
        const activeEl = itemEls[scrollTargetIndex];
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
                const delay = Math.abs(i - scrollTargetIndex) * 30;
                el.style.transition = `transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94) ${delay}ms`;
                el.style.transform = 'translateY(0)';
            }
        });
    }

    $effect(() => {
        if (scrollTargetIndex !== prevScrollTarget) {
            prevScrollTarget = scrollTargetIndex;
            if (scrollTargetIndex !== -1) updateScroll();
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
                <!-- {#if syllableItems[0] && syllableItems[0].start_time > 3}
                    <LyricGap gapStart={0} gapEnd={syllableItems[0].start_time} />
                {/if} -->
                {#each syllableItems as syllableLine, index (`${index}-${syllableLine.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={null} hasMultipleSpeakers={lyrics?.multiple_speakers} syllableLyrics={syllableLine} active={activeIndices.includes(index)} />
                    </div>
                    <!-- {#if index < syllableItems.length - 1}
                        {@const gapEnd = syllableItems[index + 1].start_time}
                        {@const gapStart = syllableLine.end_time ?? syllableLine.start_time}
                        {#if gapEnd - gapStart > 3}
                            <LyricGap {gapStart} {gapEnd} />
                        {/if}
                    {/if} -->
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
                <!-- {#if lineItems[0] && lineItems[0].start_time > 3}
                    <LyricGap gapStart={0} gapEnd={lineItems[0].start_time} />
                {/if} -->
                {#each lineItems as line, index (`${index}-${line.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={line} syllableLyrics={null} active={activeIndices.includes(index)} />
                    </div>
                    <!-- {#if index < lineItems.length - 1}
                        {@const gapEnd = lineItems[index + 1].start_time}
                        {@const gapStart = line.end_time ?? line.start_time}
                        {#if gapEnd - gapStart > 3}
                            <LyricGap {gapStart} {gapEnd} />
                        {/if}
                    {/if} -->
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