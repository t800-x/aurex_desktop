<script lang="ts">
  import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
  import ExpandIcon from "$lib/icons/expand-icon.svelte";
  import { audioPlayer } from "$lib/player.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import Seekbar from "./seekbar.svelte";
  import { router } from "$lib/router.svelte";

  let isCurrentlyPlaying = $derived(audioPlayer.currentlyPlaying !== null);
  let imgPath = $derived(audioPlayer.currentlyPlaying?.album_art ?? "");
  let imgSrc = $derived(convertFileSrc(imgPath));
  let trackTitle = $derived(
    audioPlayer.currentlyPlaying?.track.title ??
      audioPlayer.currentlyPlaying?.track.file_path ??
      "Unknown",
  );
  let albumInfo = $derived(
    `${audioPlayer.currentlyPlaying?.artist_name ?? "Unknown Artist"} - ${audioPlayer.currentlyPlaying?.album_title ?? "Unknown Album"}`,
  );
</script>

<div class="trackInfo">
  {#if isCurrentlyPlaying}
    <div class="albumCoverWrapper">
      <img class="albumCover" src={imgSrc} alt="" />
      <button class="albumCoverOverlay" onclick={() => {router.fullscreen = true;}}>
        <ExpandIcon />
      </button>
    </div>
  {:else}
    <div class="albumCover">
      <DoubleNoteIcon />
    </div>
  {/if}
  {#if audioPlayer.currentlyPlaying !== null}
    <div class="title">
      <div class="trackTitle">{trackTitle}</div>
      <div class="albumInfo">{albumInfo}</div>
      <Seekbar />
    </div>
  {:else}
    <div class="trackTitle" style="margin-right: auto;">Not Playing</div>
  {/if}
</div>

<style>
  .trackInfo {
    flex: 50;
    display: flex;
    overflow: visible;
    justify-content: center;
    align-items: center;
    min-width: 0;
  }

  .albumCoverWrapper {
    position: relative;
    height: 45px;
    width: 45px;
    flex-shrink: 0;
    cursor: pointer;
  }

  .albumCoverWrapper:hover {
    cursor: pointer;
  }

  .albumCover {
    height: 45px;
    width: 45px;
    border-radius: 8px;
    border-color: var(--color-divider);
    border-width: 1px;
    box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #1c1c1e;
    color: #3d3c41;
    transition: filter 0.15s ease;
  }

  .albumCoverWrapper:hover .albumCover {
    filter: brightness(0.55);
  }

  .albumCoverOverlay {
    position: absolute;
    inset: 0;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.15s ease;
    color: white;
  }

  .albumCoverWrapper:hover .albumCoverOverlay {
    opacity: 1;
  }

  .title {
    margin-left: 10px;
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    width: 100%;
  }

  .trackTitle {
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
    text-align: center;
  }

  .albumInfo {
    font-size: 12px;
    color: var(--color-navbar-label);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
    text-align: center;
  }
</style>