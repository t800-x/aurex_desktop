<script lang="ts">
  import { untrack } from "svelte";
  import type { LineLyrics, SyllableLine } from "$lib/bindings";
  import { commands } from "$lib/bindings";
  import { audioPlayer } from "$lib/player.svelte";

  let {
    lineLyrics,
    syllableLyrics,
    hasMultipleSpeakers = false,
    active = false,
  }: {
    lineLyrics: LineLyrics | null;
    syllableLyrics: SyllableLine | null;
    active?: boolean;
    hasMultipleSpeakers?: boolean;
  } = $props();

  type WordTiming = { duration: number; delay: number };
  let wordTimings = $state<WordTiming[]>([]);

  $effect(() => {
    if (active && syllableLyrics) {
      const now = untrack(() => audioPlayer.position);
      wordTimings = syllableLyrics.words.map((w) => ({
        duration: (w.end_time - w.start_time) * 1000,
        delay: (w.start_time - now) * 1000,
      }));
    } else {
      wordTimings = [];
    }
  });
</script>

{#if syllableLyrics !== null}
  {@const _ = syllableLyrics}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    onclick={() => commands.seek(_.start_time)}
    class:active
    class="tile"
    class:leftSpeaker={hasMultipleSpeakers && _.speaker === 0}
    class:rightSpeaker={hasMultipleSpeakers && _.speaker === 1}
    class:backgroundVocals={_.is_background}
  >
    {#each _.words as word, i}
      {@const timing = wordTimings[i]}
      <span
        class:word={!_.is_background}
        class:backgroundVocalsWord={_.is_background}
        class:animated={!!timing}
        data-text={word.text}
        style={timing
          ? `--wd: ${timing.duration}ms; --wdl: ${timing.delay}ms`
          : ""}
      >{word.text}</span>
    {/each}
  </div>
{:else if lineLyrics !== null}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    onclick={() => commands.seek(lineLyrics.start_time)}
    class:lineActive={active}
    class="tile lineTile"
  >
    {lineLyrics.line}
  </div>
{/if}

<style>
  .tile {
    padding: 10px;
    font-size: 0;
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

  .backgroundVocals {
    max-height: 0;
    overflow: hidden;
    padding-top: 0;
    padding-bottom: 0;
    transition:
      max-height 0.2s ease-in-out,
      padding 0.2s ease-in-out;
  }

  .backgroundVocals.active {
    max-height: 60px;
    padding-top: 10px;
    padding-bottom: 10px;
    transition:
      max-height 0.2s ease-in-out,
      padding 0.2s ease-in-out,
      transform 0.15s ease-out;
  }

  .tile:hover {
    background-color: var(--color-hover);
  }

  .active {
    transform: scale(0.9);
    filter: none;
    transition: transform 0.15s ease-out;
  }

  .leftSpeaker {
    width: fit-content;
    max-width: 90%;
    margin-right: 10%;
  }

  .rightSpeaker {
    width: fit-content;
    max-width: 90%;
    margin-left: auto;
    margin-right: 0;
    text-align: right;
    transform-origin: right center;
  }

  .lineTile {
    font-size: 25px;
    color: rgba(255, 255, 255, 0.3);
  }

  .lineActive {
    transform: scale(0.9);
    color: rgba(255, 255, 255, 0.95);
    filter: none;
    transition: transform 0.15s ease-out;
  }

  /*
   * The gradient shape is FIXED - only mask-position animates.
   * The gradient is 300% wide. The solid-black zone (0-40%) is
   * 120% of the element width, so it can fully cover it. The fade
   * runs from 40% to 52% of gradient space (~36% element width).
   *
   * start (hidden):  mask-position 75% → element sees the transparent tail
   * end (revealed):  mask-position  0% → element sees the solid black head
   *
   * The gradient itself never changes shape, so the browser doesn't
   * need to recompute it every frame - only mask-position moves.
   */
  @keyframes word-reveal {
      from { mask-position: 200% 0; }
      to   { mask-position: 0% 0; }
  }

  .word,
  .backgroundVocalsWord {
    display: inline-block;
    white-space: pre-wrap;
    position: relative;
    vertical-align: top;
    color: rgba(255, 255, 255, 0.3);
  }

  .word                 { font-size: 25px; }
  .backgroundVocalsWord { font-size: 15px; }

  .word::after,
  .backgroundVocalsWord::after {
      content: attr(data-text);
      position: absolute;
      left: 0;
      top: 0;
      white-space: pre;
      pointer-events: none;
      mask-image: linear-gradient(
          to right,
          black            0%,
          black            77%,
          rgba(0,0,0,0.35) 88%,
          transparent      100%
      );
      mask-size: 300% 100%;
      mask-repeat: no-repeat;
      mask-position: 200% 0;  /* fully hidden at start */
  }

  .word::after              { color: rgba(255, 255, 255, 0.95); }
  .backgroundVocalsWord::after { color: rgba(255, 255, 255, 0.5); }

  .word.animated::after,
  .backgroundVocalsWord.animated::after {
    animation: word-reveal var(--wd) linear both;
    animation-delay: var(--wdl);
  }
</style>