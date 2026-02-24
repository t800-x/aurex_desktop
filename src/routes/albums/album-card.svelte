<script lang="ts">
    import { commands, type Album } from "$lib/bindings";
    import type { Artist, Track } from "$lib/bindings";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import { onMount } from "svelte";
    import CardButton from "./card-button.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";

    let {
        album
    } : {
        album: Album;
    } = $props();

    // svelte-ignore state_referenced_locally
        let imgSrc = convertFileSrc(album.album_art ?? "");

    let artist: Artist | null = $state(null);
    let tracks: Track[] | null = $state(null);

    onMount(async () => {
        artist = await commands.getArtistById(Number(album.artist_id));
        tracks = await commands.getAlbumTracks(Number(album.id));
    });

</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <div class="card">

            <div class="coverContainer">

                {#if album.album_art !== null}
                    <img class="albumCover" src={imgSrc} alt="">
                {:else}
                    <div class="albumCover">
                        <DoubleNoteIcon />
                    </div>
                {/if}

                <CardButton class="cardButton" Icon={PlayIcon} onclick={async () => {
                    commands.playTracks(tracks!, 0)
                }}/>
            </div>

            <div class="label title">
                {album.title}
            </div>

            <div class="label artist">
                {artist?.name}
            </div>
        
        </div>
    </ContextMenu.Trigger>

    <ContextMenu.Content>
        <ContextMenu.Item onclick={async () => commands.playTracks(tracks!, 0)}>Play</ContextMenu.Item>
        <ContextMenu.Item onclick={async () => commands.playListNext(null, tracks)}>Play Next</ContextMenu.Item>
        <ContextMenu.Item onclick={async () => commands.addListToQueue(null, tracks)}>Add to Queue</ContextMenu.Item>
    </ContextMenu.Content>
</ContextMenu.Root>

<style>

    .card {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: center;
        width: 205px;
        min-width: 0;
        cursor: default;
        

        transition: transform 0.15s ease-in-out;
    }

    .coverContainer {
        isolation: isolate;
        position: relative;
    }

    .card:hover {
        transform: scale(1.1);
    }

    .card:hover :global(.cardButton) {
        display: flex !important;
    }

    .albumCover {
        height: 205px;
        width: 205px;
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

    .label {
        min-width: 0px;
        width: 100%;
        text-align: left;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .title {
        font-size: 15px;
    }

    .artist {
        color: var(--color-navbar-label);
        font-size: 13px;
    }
</style>