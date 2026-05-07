<script lang="ts">
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
  import { audioPlayer } from "$lib/player.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";

  let isCurrentlyPlaying = $derived(audioPlayer.currentlyPlaying !== null);
  let imgPath = $derived(audioPlayer.currentlyPlaying?.album_art ?? "");
  let imgSrc = $derived(convertFileSrc(imgPath));
  
</script>

<div class="backdrop">
  {#if isCurrentlyPlaying && (audioPlayer.currentlyPlaying?.album_art !== null)}
    <img src={imgSrc} alt="" class="albumCover"/>
  {:else}
    <div class="albumCover">
      <DoubleNoteIcon size={100} />
    </div>
  {/if}

  <div class="tint"></div>
</div>

<style>
  .backdrop {
    height: 100%;
    width: 100%;
    position: absolute;
    z-index: 0;
    overflow: hidden;
    isolation: isolate;
  }

  .tint {
    z-index: 30;
    height: 100%;
    width: 100%;
    position: absolute;
    background-color: rgba(0, 0, 0, 0.35);
  }

  .albumCover {
    position: absolute;
    top: 50%;
    left: 50%;
    translate: -50% -50%;
    width: 150vmax; 
    height: 150vmax;
    max-width: none;
    object-fit: cover; 
    filter: blur(80px);
    animation: spin 120s linear infinite;
  }

  @keyframes spin {
    from {
      rotate: 0deg;
    }
    to {
      rotate: 360deg;
    }
  }
</style>