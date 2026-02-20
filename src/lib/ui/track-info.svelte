<script lang="ts">
    import DoubleNoteIcon from '$lib/icons/double-note-icon.svelte';
    import { audioPlayer } from '$lib/player.svelte';
    import { convertFileSrc } from '@tauri-apps/api/core';
    import Seekbar from './seekbar.svelte';

    let isCurrentlyPlaying = $derived(audioPlayer.currentlyPlaying !== null);
    let imgPath = $derived(audioPlayer.currentlyPlaying?.album_art ?? "");
    let imgSrc = $derived(convertFileSrc(imgPath));

    let trackTitle = $derived(audioPlayer.currentlyPlaying?.track.title ?? audioPlayer.currentlyPlaying?.track.file_path ?? "Unknown");

    let albumInfo = $derived(`${audioPlayer.currentlyPlaying?.artist_name ?? "Unknown Artist"} - ${audioPlayer.currentlyPlaying?.album_title ?? "Unknown Album"}`);
</script>

<div class="trackInfo">

    {#if isCurrentlyPlaying}
        <img class="albumCover" src={imgSrc} alt="">
    {:else}
        <div class="albumCover">
            <DoubleNoteIcon />
        </div>
    {/if}

    <div class="title">
        <div class="trackTitle">{trackTitle}</div>
        <div class="albumInfo">{albumInfo}</div>
        <Seekbar />
    </div>
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

    .albumCover {
        padding-left: 5px;
        height: 45px;
        width: 45px;
        border-radius: 10px;
        border-color: var(--color-divider);
        border-width: 1px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1c1c1e;
        color: #3d3c41;
    }

    .title{
        margin-left: 10px;
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        width: 100%;
    }

    .trackTitle{
        font-size: 14px;
        font-weight: w500;
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