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

    // isUserScrolling stays true through the entire snap-back animation so that
    // line-change effects don't fire updateScroll mid-pan and fight the transition.
    let isUserScrolling = $state(false);
    let userScrollTimeout: ReturnType<typeof setTimeout> | null = null;

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
                    effectiveEnd = line.end_time != null
                        ? Math.max(line.end_time, nextStart)
                        : nextStart;
                } else {
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

    function clearItemTransforms() {
        for (const el of itemEls) {
            if (!el) continue;
            el.style.transition = 'none';
            el.style.transform = 'translateY(0)';
        }
    }

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

        // Always clear any lingering transition from snap-back before moving innerEl,
        // otherwise the first post-snap updateScroll animates with the 0.65s curve
        // instead of being instant (which breaks the stagger illusion).
        innerEl.style.transition = '';
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

    /**
     * Smoothly returns to the active lyric line after the user-scroll cooldown.
     * Keeps isUserScrolling=true through the entire pan so that in-flight line
     * changes don't call updateScroll and fight the animation.
     * Only releases control (isUserScrolling=false, prevScrollTarget=-1) once
     * the transition has landed -- or immediately if the line is already visible.
     */
    function snapBackToActive() {
        if (scrollTargetIndex === -1 || !listEl || !innerEl) {
            isUserScrolling = false;
            prevScrollTarget = -1;
            return;
        }

        const activeEl = itemEls[scrollTargetIndex];
        if (!activeEl) {
            isUserScrolling = false;
            prevScrollTarget = -1;
            return;
        }

        const containerHeight = listEl.getBoundingClientRect().height;
        const elVisibleTop = activeEl.offsetTop - scrollOffset;
        const isOutOfView = elVisibleTop < 0 || elVisibleTop > containerHeight * 0.75;

        const targetOffset = Math.max(0, activeEl.offsetTop - containerHeight * 0.1);

        clearItemTransforms();
        scrollOffset = targetOffset;

        if (isOutOfView) {
            innerEl.style.transition = 'transform 0.65s cubic-bezier(0.25, 0.46, 0.45, 0.94)';
            innerEl.style.transform = `translateY(-${scrollOffset}px)`;

            // Release AFTER the pan lands, not before -- keeps updateScroll from
            // firing mid-transition and leaving innerEl.style.transition dirty.
            innerEl.addEventListener('transitionend', () => {
                if (!innerEl) return;
                innerEl.style.transition = '';
                isUserScrolling = false;
                prevScrollTarget = -1; // $state write -> re-triggers the $effect
            }, { once: true });
        } else {
            innerEl.style.transition = '';
            innerEl.style.transform = `translateY(-${scrollOffset}px)`;
            isUserScrolling = false;
            prevScrollTarget = -1;
        }
    }

    function handleWheel(e: WheelEvent) {
        e.preventDefault();
        if (!listEl || !innerEl) return;

        clearItemTransforms();

        const maxScroll = Math.max(0, innerEl.offsetHeight - listEl.clientHeight);
        scrollOffset = Math.max(0, Math.min(maxScroll, scrollOffset + e.deltaY));
        innerEl.style.transition = 'none';
        innerEl.style.transform = `translateY(-${scrollOffset}px)`;

        isUserScrolling = true;
        if (userScrollTimeout) clearTimeout(userScrollTimeout);
        // snapBackToActive owns the isUserScrolling=false release from here on.
        userScrollTimeout = setTimeout(snapBackToActive, 2000);
    }

    $effect(() => {
        if (isUserScrolling) return;
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
        isUserScrolling = false;
        if (userScrollTimeout) { clearTimeout(userScrollTimeout); userScrollTimeout = null; }
        if (innerEl) { innerEl.style.transition = ''; innerEl.style.transform = 'translateY(0)'; }

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
        <div bind:this={listEl} class="lyricsDisplay" onwheel={handleWheel}>
            <div bind:this={innerEl} class="lyricsInner">
                {#each syllableItems as syllableLine, index (`${index}-${syllableLine.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={null} hasMultipleSpeakers={lyrics?.multiple_speakers} syllableLyrics={syllableLine} active={activeIndices.includes(index)} />
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
        <div bind:this={listEl} class="lyricsDisplay" onwheel={handleWheel}>
            <div bind:this={innerEl} class="lyricsInner">
                {#each lineItems as line, index (`${index}-${line.start_time}`)}
                    <div class="item-wrap" bind:this={itemEls[index]}>
                        <LyricLine lineLyrics={line} syllableLyrics={null} active={activeIndices.includes(index)} />
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