<script lang="ts">
  import type { MatchReason, TrackResult } from "$lib/bindings";
  import { commands } from "$lib/bindings";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
  import TrackContextMenu from "$lib/ui/context_menus/track-context-menu.svelte";
  import { formatDuration } from "$lib/helpers";
  import SearchSectionHeader from "./search-section-header.svelte";
  import { VList } from "virtua/svelte";

  let {
    tracks,
    onPlayFromIndex,
  }: {
    tracks: TrackResult[];
    onPlayFromIndex: (index: number) => void;
  } = $props();

  const ROWS = 3;
  const COL_WIDTH = 280;
  const TILE_HEIGHT = 44; // 34px art + 10px padding
  const GAP = 2;

  // Pre-chunk into columns of ROWS so VList gets one item per column.
  let columns = $derived(
    tracks.reduce<TrackResult[][]>((cols, track, i) => {
      const col = Math.floor(i / ROWS);
      if (!cols[col]) cols[col] = [];
      cols[col].push(track);
      return cols;
    }, []),
  );

  // Height of the VList container — fits exactly ROWS tiles + gaps.
  const listHeight = ROWS * (TILE_HEIGHT + GAP) + (ROWS + 5) * GAP;

  const reasonLabel: Record<MatchReason, string> = {
    Title: "Title",
    Artist: "Artist",
    Album: "Album",
    Lyrics: "Lyrics",
  };
  const reasonStyle: Record<MatchReason, string> = {
    Title: "background:rgba(250,46,73,0.15);  color:#fa2e49;",
    Artist: "background:rgba(59,130,246,0.15); color:#60a5fa;",
    Album: "background:rgba(168,85,247,0.15); color:#c084fc;",
    Lyrics: "background:rgba(245,158,11,0.15); color:#fbbf24;",
  };

  async function playTrack(result: TrackResult) {
    await commands.clear();
    await commands.load(result.track);
    commands.play();
  }
</script>

<section class="section">
  <SearchSectionHeader title="Songs" count={tracks.length} />

  <VList
    data={columns}
    horizontal
    style="height: {listHeight}px;"
    getKey={(_col: any, i: number) => i}
  >
    {#snippet children(col: TrackResult[], colIndex: number)}
      <div class="column" style="width: {COL_WIDTH}px;">
        {#each col as result, rowIndex}
          {@const globalIndex = colIndex * ROWS + rowIndex}
          <ContextMenu.Root>
            <ContextMenu.Trigger>
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                class="tile"
                class:odd={globalIndex % 2 !== 0}
                onclick={() => playTrack(result)}
              >
                {#if result.track.album_art}
                  <img
                    class="art"
                    src={convertFileSrc(result.track.album_art)}
                    alt=""
                  />
                {:else}
                  <div class="art noArt">
                    <DoubleNoteIcon size={14} />
                  </div>
                {/if}

                <div class="meta">
                  <span class="title">{result.track.track.title}</span>
                  <span class="sub">
                    {result.track.artist_name} · {result.track.album_title}
                  </span>
                </div>

                <div class="right">
                  <span class="duration">
                    {formatDuration(result.track.track.duration)}
                  </span>
                  <div class="pills">
                    {#each result.reasons as reason}
                      <span class="pill" style={reasonStyle[reason]}>
                        {reasonLabel[reason]}
                      </span>
                    {/each}
                  </div>
                </div>
              </div>
            </ContextMenu.Trigger>

            <TrackContextMenu
              track={result.track}
              playList={() => onPlayFromIndex(globalIndex)}
            />
          </ContextMenu.Root>
        {/each}
      </div>
    {/snippet}
  </VList>
</section>

<style>
  .section {
    margin-bottom: 28px;
  }

  .column {
    display: flex;
    flex-direction: column;
    gap: 2px;
    margin-right: 8px;
  }

  .tile {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 8px;
    border-radius: 7px;
    background: var(--color-hover);
    cursor: default;
    width: 280px;
    min-width: 0;
    height: 44px;
    box-sizing: border-box;
    transition: background-color 0.15s ease;
  }

  .tile.odd {
    background: transparent;
  }
  .tile:hover {
    background: var(--color-double-hover) !important;
  }

  .art {
    width: 34px;
    height: 34px;
    border-radius: 5px;
    border: 1px solid var(--color-divider);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.35);
    object-fit: cover;
    flex-shrink: 0;
  }

  .art.noArt {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1c1c1e;
    color: #3d3c41;
  }

  .meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    font-size: 13px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sub {
    font-size: 11px;
    color: var(--color-navbar-label);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 3px;
    flex-shrink: 0;
  }

  .duration {
    font-size: 11px;
    font-weight: 600;
    color: var(--color-navbar-label);
  }

  .pills {
    display: flex;
    gap: 3px;
  }

  .pill {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.03em;
    padding: 1px 5px;
    border-radius: 99px;
    white-space: nowrap;
  }
</style>
