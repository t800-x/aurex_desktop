<script lang="ts">
  import {
    commands,
    type FullTrack,
    type Lyrics,
    type LineLyrics,
    type SyllableLine,
  } from "$lib/bindings";
  import { audioPlayer } from "$lib/player.svelte";
  import LyricLine from "./lyric-line.svelte";

  let lyrics = $state<Lyrics | null>(null);
  let localTrack = $state<FullTrack | null>(null);

  let itemEls: Array<HTMLElement | null> = $state([]);
  let listEl: HTMLDivElement | null = $state(null);
  let innerEl: HTMLDivElement | null = $state(null);
  let scrollOffset = 0;

  let isUserScrolling = $state(false);
  let userScrollTimeout: ReturnType<typeof setTimeout> | null = null;

  let activeIndex = $state(-1);
  let nextTimer: ReturnType<typeof setTimeout> | null = null;

  let lineItems = $derived<LineLyrics[]>(
    !lyrics || lyrics.lyricstype !== "Line" || !lyrics.line_lyrics
      ? []
      : lyrics.line_lyrics.map((line, i) => {
          if (
            lyrics!.line_lyrics![i - 1]?.start_time === line.start_time &&
            line.end_time != null
          ) {
            return { ...line, start_time: line.end_time };
          }
          return line;
        }),
  );

  let syllableItems = $derived<SyllableLine[]>(
    !lyrics || lyrics.lyricstype !== "Syllable" || !lyrics.syllable_lyrics
      ? []
      : lyrics.syllable_lyrics.map((line, i) => {
          const prev = lyrics!.syllable_lyrics![i - 1];
          if (
            prev?.start_time === line.start_time &&
            !line.is_background &&
            !prev.is_background
          ) {
            return { ...line, start_time: line.end_time };
          }
          return line;
        }),
  );

  function getItems() {
    return lyrics?.lyricstype === "Syllable" ? syllableItems : lineItems;
  }

  function computeEffectiveEnd(
    items: LineLyrics[] | SyllableLine[],
    i: number,
  ): number {
    const item = items[i];
    const nextStart = items[i + 1]?.start_time;
    if (nextStart !== undefined) {
      return item.end_time != null
        ? Math.max(item.end_time, nextStart)
        : nextStart;
    }
    return item.end_time ?? Infinity;
  }

  function scheduleActiveIndex() {
    if (nextTimer) {
      clearTimeout(nextTimer);
      nextTimer = null;
    }

    const items = getItems();
    if (!items.length) {
      activeIndex = -1;
      return;
    }

    // read position once, not as a reactive dependency
    const now = audioPlayer.position;

    // binary search for the last item whose start_time <= now
    let lo = 0,
      hi = items.length - 1,
      found = -1;
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      if (items[mid].start_time <= now) {
        found = mid;
        lo = mid + 1;
      } else {
        hi = mid - 1;
      }
    }

    // make sure it hasn't already ended
    if (found !== -1) {
      const effectiveEnd = computeEffectiveEnd(items, found);
      if (now >= effectiveEnd) found = -1;
    }

    activeIndex = found;

    // schedule next wake-up at the next line boundary
    let nextBoundary: number | undefined;
    for (let i = 0; i < items.length; i++) {
      if (items[i].start_time > now) {
        nextBoundary = items[i].start_time;
        break;
      }
    }

    // also wake up at the effective end of the current line if it
    // ends before the next line starts (gap between lines)
    if (found !== -1) {
      const effectiveEnd = computeEffectiveEnd(items, found);
      if (isFinite(effectiveEnd)) {
        nextBoundary =
          nextBoundary !== undefined
            ? Math.min(nextBoundary, effectiveEnd)
            : effectiveEnd;
      }
    }

    if (nextBoundary !== undefined) {
      const delay = Math.max(0, (nextBoundary - now) * 1000);
      nextTimer = setTimeout(scheduleActiveIndex, delay);
    }
  }

  // re-schedule only when lyrics content changes, not on every position tick
  $effect(() => {
    // explicitly depend on syllableItems/lineItems so this fires after
    // they're recomputed from a new track load
    syllableItems;
    lineItems;
    scheduleActiveIndex();
    return () => {
      if (nextTimer) clearTimeout(nextTimer);
    };
  });


  let scrollTargetIndex = $derived(activeIndex);
  let prevScrollTarget = $state(-1);

  function clearItemTransforms() {
    for (const el of itemEls) {
      if (!el) continue;
      el.style.transition = "none";
      el.style.transform = "translateY(0)";
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
      el.style.transition = "none";
      el.style.transform = `translateY(${delta}px)`;
      el.style.willChange = "transform";
    }

    innerEl.style.transition = "";
    innerEl.style.transform = `translateY(-${scrollOffset}px)`;

    requestAnimationFrame(() => {
      for (let i = 0; i < itemEls.length; i++) {
        const el = itemEls[i];
        if (!el) continue;
        const delay = Math.abs(i - scrollTargetIndex) * 30;
        el.style.transition = `transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94) ${delay}ms`;
        el.style.transform = "translateY(0)";
      }

      setTimeout(() => {
          for (const el of itemEls) {
              if (el) el.style.willChange = "auto";
          }
      }, 600); 
    });
  }

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
    const isOutOfView =
      elVisibleTop < 0 || elVisibleTop > containerHeight * 0.75;

    const targetOffset = Math.max(
      0,
      activeEl.offsetTop - containerHeight * 0.1,
    );

    clearItemTransforms();
    scrollOffset = targetOffset;

    if (isOutOfView) {
      innerEl.style.transition =
        "transform 0.65s cubic-bezier(0.25, 0.46, 0.45, 0.94)";
      innerEl.style.transform = `translateY(-${scrollOffset}px)`;

      innerEl.addEventListener(
        "transitionend",
        () => {
          if (!innerEl) return;
          innerEl.style.transition = "";
          isUserScrolling = false;
          prevScrollTarget = -1;
        },
        { once: true },
      );
    } else {
      innerEl.style.transition = "";
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
    innerEl.style.transition = "none";
    innerEl.style.transform = `translateY(-${scrollOffset}px)`;

    isUserScrolling = true;
    if (userScrollTimeout) clearTimeout(userScrollTimeout);
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
    const tracksMatched =
      localTrack?.track.id === audioPlayer.currentlyPlaying?.track.id;
    if (tracksMatched) return;

    localTrack = audioPlayer.currentlyPlaying;
    lyrics = null;
    activeIndex = -1;
    scrollOffset = 0;
    isUserScrolling = false;
    if (userScrollTimeout) {
      clearTimeout(userScrollTimeout);
      userScrollTimeout = null;
    }
    if (nextTimer) {
      clearTimeout(nextTimer);
      nextTimer = null;
    }
    if (innerEl) {
      innerEl.style.transition = "";
      innerEl.style.transform = "translateY(0)";
    }

    if (!audioPlayer.currentlyPlaying) {
      itemEls = [];
      return;
    }

    commands.getLyrics(audioPlayer.currentlyPlaying).then((l) => {
      lyrics = l;
      const len =
        l.lyricstype === "Syllable"
          ? (l.syllable_lyrics?.length ?? 0)
          : (l.line_lyrics?.length ?? 0);
      itemEls = new Array(len).fill(null);
    });
  });
</script>

<div class="lyricsContainer">
  {#if !lyrics}
    <!-- loading / no track -->
  {:else if lyrics.lyricstype === "Syllable" && syllableItems.length > 0}
    <div bind:this={listEl} class="lyricsDisplay" onwheel={handleWheel}>
      <div bind:this={innerEl} class="lyricsInner">
        {#each syllableItems as syllableLine, index (`${index}-${syllableLine.start_time}`)}
          <div class="item-wrap" bind:this={itemEls[index]}>
            <LyricLine
              lineLyrics={null}
              hasMultipleSpeakers={lyrics?.multiple_speakers}
              syllableLyrics={syllableLine}
              active={activeIndex === index}
            />
          </div>
        {/each}
      </div>
    </div>
  {:else if lyrics.lyricstype === "Unsynced" && lyrics.unsynced}
    <div class="unsyncedDisplay">
      {#each lyrics.unsynced.split("\n") as para}
        <p class="unsyncedLine" class:empty={para.trim() === ""}>
          {para || "\u00A0"}
        </p>
      {/each}
    </div>
  {:else if lyrics.lyricstype === "Line" && lineItems.length > 0}
    <div bind:this={listEl} class="lyricsDisplay" onwheel={handleWheel}>
      <div bind:this={innerEl} class="lyricsInner">
        {#each lineItems as line, index (`${index}-${line.start_time}`)}
          <div class="item-wrap" bind:this={itemEls[index]}>
            <LyricLine
              lineLyrics={line}
              syllableLyrics={null}
              active={activeIndex === index}
            />
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
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
    mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
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
  }

  .unsyncedDisplay {
    padding: 24px 20px;
    overflow-y: auto;
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 10%, black 90%, transparent 100%);
    mask-image: linear-gradient(to bottom, transparent 0%, black 10%, black 90%, transparent 100%);
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