<script lang="ts">
    import type { Album } from "$lib/bindings";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { getContext, onMount } from "svelte";
    import type { StackContext } from "$lib/ui/stack-view.svelte";
    import LeftHalfArrow from "$lib/icons/left-half-arrow.svelte";
    import TransparentButton from "$lib/ui/transparent-button.svelte";
    import { commands } from "$lib/bindings";
    import PlayButtonRed from "$lib/ui/play-button-red.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";
    import ShuffleIcon from "$lib/icons/shuffle-icon.svelte";
    import type { Track } from "$lib/bindings";
    import Tile from "./tile.svelte";

    const { push, pop, canPop } = getContext<StackContext>('stack');

    let {
        album
    } : {
        album: Album;
    } = $props();

    let artistName = $state("Unknown Artist");
    let tracks: Track[] = $state([]);

    onMount(async () => {
        let result = await commands.getArtistById(Number(album.artist_id));
        if (result !== null) {
            artistName = result.name;
        }

        tracks = await commands.getAlbumTracks(Number(album.id));
    });

    // svelte-ignore state_referenced_locally
        let imgSrc = convertFileSrc(album.album_art ?? "");

</script>

<div class="albumView">
    <div style="margin: 10px; align-self: flex-start;">
        <TransparentButton onclick={() => pop()}>
            <LeftHalfArrow size={16} />
            Albums
        </TransparentButton>
    </div>

    <div class="topInfo">
        {#if album.album_art !== null}
            <div class="glowWrapper">
                <img class="glow" src={imgSrc} alt="">
                <img class="albumCover" src={imgSrc} alt="">
            </div>
        {:else}
            <div class="albumCover">
                <DoubleNoteIcon />
            </div>
        {/if}

        <div class="albumInfo">
            <div class="title" style="
                font-size: 25px;
                font-weight: bold;
            ">
                {album.title}
            </div>
            <div class="artist" style="
                font-size: 25px;
                color: var(--color-accent);
            ">
                {artistName}
            </div>
            <div class="extraInfo" style="
                font-size: 12px;
                font-weight: 700;
                color: var(--color-navbar-label)
            ">
                {album.genre} · {album.year}
            </div>

            <div style="flex: 1"></div>

            <div class="play">
                <PlayButtonRed onclick={() => {commands.playTracks(tracks, 0)}}>
                    <PlayIcon size={20} />
                    <span>Play</span>
                </PlayButtonRed>

                <PlayButtonRed onclick={() => {}}>
                    <ShuffleIcon size={20} />
                    <span>Shuffle</span>
                </PlayButtonRed>
            </div>
        </div>
    </div>

    <div class="songs">
        {#each tracks as track, index}
            <Tile track={track} index={index} playList={() => {commands.playTracks(tracks, index)}}  />

            {#if index === tracks.length - 1}
                <div style="height: 80px;"></div>
            {/if}
        {/each}
    </div>

</div>

<style>
    .songs {
        flex-shrink: 0;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 3px;
        padding-top: 45px;
        padding-left: 5px;
        padding-right: 5px;
    }

    .play {
        width: 100%;
        display: flex;
        gap: 10px;
        font-size: 15px;
    }

    .albumView {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        overflow-y: auto;
    }

    .topInfo {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding-left: 45px;
        width: 100%;
    }

    .glowWrapper {
        position: relative;
        width: 270px;
        height: 270px;
        overflow: visible;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-shrink: 0;
    }

    .glow {
        position: absolute;
        width: 90%;
        height: 90%;
        filter: blur(20px);
        opacity: 0.7;
        transform: scale(1.001); /* let it bleed past the edges */
        z-index: 0;
    }

    .albumInfo {
        display: flex;
        flex-direction: column;
        padding-left: 45px;
        justify-content: flex-start;
        height: 100%;
        align-self: stretch;
        min-height: 270px;
    }

    .albumCover {
        position: relative;
        z-index: 1;
        height: 100%;
        width: 100%;
        border-radius: 6px;
        border-color: var(--color-divider);
        border-width: 1px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1c1c1e;
        color: #3d3c41;
    }
</style>