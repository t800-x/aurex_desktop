<script lang="ts">
  import type { FullTrack, Playlist } from "$lib/bindings";
  import { formatDuration, loadAndPlay } from "$lib/helpers";
  import PlayIcon from "$lib/icons/play-icon.svelte";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import PlaylistTrackContextMenu from "$lib/ui/context_menus/playlist-track-context-menu.svelte";

  let {
    track,
    playList,
    playlist,
    index
  } : {
    track: FullTrack;
    playList: () => void;
    playlist: Playlist | null;
    index: number;
  } = $props();

  let hovered = $state(false);

  let artSrc = $derived(track.album_art ? convertFileSrc(track.album_art) : null);
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger>
    <div
      role="listitem"
      class:odd={index % 2 !== 0}
      class="tile"
      onmouseenter={() => hovered = true}
      onmouseleave={() => hovered = false}
      ondblclick={() => loadAndPlay(track)}
    >
      <!-- leading: index or play button -->
      <div class="leading" aria-hidden="true">
        {#if hovered}
          <button class="playBtn" aria-label="Play track" onclick={() => playList()}>
            <PlayIcon size={15} />
          </button>
        {:else}
          <span class="index">{index + 1}</span>
        {/if}
      </div>

      <!-- album art -->
      <div class="art">
        {#if artSrc}
          <img src={artSrc} alt="" class="artImg" />
        {:else}
          <div class="artPlaceholder"></div>
        {/if}
      </div>

      <!-- title -->
      <div class="col title-col" title={track.track.title}>
        <span class="title-text">{track.track.title}</span>
      </div>

      <!-- artist -->
      <div class="col artist-col" title={track.artist_name}>
        <span class="secondary">{track.artist_name}</span>
      </div>

      <!-- album -->
      <div class="col album-col" title={track.album_title}>
        <span class="secondary">{track.album_title}</span>
      </div>

      <!-- duration -->
      <div class="trailing">
        <span class="duration">{formatDuration(track.track.duration)}</span>
      </div>
    </div>
  </ContextMenu.Trigger>
  <PlaylistTrackContextMenu track={track} playList={playList} playlist={playlist} />
</ContextMenu.Root>

<style>
  .tile {
    display: flex;
    align-items: center;
    height: 44px;
    width: 100%;
    border-radius: 5px;
    background-color: var(--color-hover);
    padding-right: 16px;
    font-size: 14px;
    box-sizing: border-box;
    gap: 8px;
  }
  .tile:hover {
    background-color: var(--color-double-hover);
  }
  .odd {
    background-color: transparent;
  }

  .leading {
    width: 28px;
    min-width: 28px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 12px;
    box-sizing: border-box;
    flex-shrink: 0;
  }
  .index {
    width: 100%;
    display: inline-block;
    text-align: right;
    color: var(--color-navbar-label);
    padding-right: 6px;
  }
  .playBtn {
    width: 100%;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding: 4px 8px 4px 4px;
    border-radius: 6px;
    color: var(--color-accent);
    border: none;
    cursor: default;
    box-sizing: border-box;
    transition: background-color 0.15s ease;
  }
  .playBtn:hover {
    background-color: var(--color-hover);
  }

  .art {
    width: 32px;
    min-width: 32px;
    height: 32px;
    border-radius: 3px;
    overflow: hidden;
    flex-shrink: 0;
  }
  .artImg {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .artPlaceholder {
    width: 100%;
    height: 100%;
    background-color: var(--color-double-hover);
  }

  .col {
    flex: 1;
    min-width: 0;
    overflow: hidden;
  }
  .title-col {
    flex: 2;
  }
  .title-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
  }
  .secondary {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
    color: var(--color-navbar-label);
    font-weight: 500;
    font-size: 13px;
  }

  .trailing {
    margin-left: auto;
    padding-left: 12px;
    display: flex;
    align-items: center;
    min-width: 48px;
    justify-content: flex-end;
    flex-shrink: 0;
  }
  .duration {
    color: var(--color-navbar-label);
    font-weight: 600;
    font-size: 13px;
  }
</style>