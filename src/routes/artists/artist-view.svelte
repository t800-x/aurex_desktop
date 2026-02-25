<script lang="ts">
    import { commands, type Album } from "$lib/bindings";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { VList } from "virtua/svelte";
    import Tile from "../albums/tile.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";
    import PoppingButton from "$lib/ui/popping-button.svelte";

    let {
        selected,
    } : {
        selected: number | null;
    } = $props();

    let isThereSomethingToDisplay = $derived(selected !== null);
    let albums: Album[] = $state([]);
    let containerWidth = $state(0);

    $effect(() => {
        (async () => {
            if (isThereSomethingToDisplay) {
                albums = await commands.getArtistAlbums(selected!);
            } else {
                albums = [];
            }
        })();
    });

</script>


<div class="artistView">

    {#if isThereSomethingToDisplay}
            <VList data={albums} style="height: 100%; width: 100%;" getKey={(album: Album, _: any) => album.id}>
                {#snippet children(album: Album, index: number)}
                    {#if index === 0}
                        <div style="height: 50px;"></div>
                    {/if}
                
                    <div class="album">
                        {#if album.album_art !== null}
                            <img class="albumCover" src={convertFileSrc(album.album_art)} alt="">
                        {:else}
                            <div class="albumCover">
                                <DoubleNoteIcon />
                            </div>
                        {/if}

                        <div class="tracklist">
                            <div class="title" style=" margin: 20px; width: calc(100% - 40px); display: flex;">
                                
                                <div>
                                    <div style="font-weight: 500;">{album.title}</div>
                                    <div style="font-weight: 500; font-size: 13px; color: var(--color-navbar-label);">{album.genre ?? ""} · {album.year ?? ""}</div>
                                </div>

                                <div class="playBtn">
                                    <PoppingButton>
                                        <button onclick={async () => commands.playTracks(await commands.getAlbumTracks(Number(album.id)), 0)}><PlayIcon size={25} /></button>
                                    </PoppingButton>
                                </div>
                            </div>

                            <div class="tracks" style="padding: 20px; width: 100%;">
                                <!-- svelte-ignore block_empty -->
                                {#await commands.getAlbumTracks(Number(album.id))}
                                    
                                {:then tracks} 
                                    {#each tracks as track, index}
                                        <Tile track={track} index={index} playList={() => {commands.playTracks(tracks, 0)}} />
                                        <div style="height: 3px;"></div>
                                    {/each}
                                {/await}
                            </div>
                        </div>
                    </div>
                {/snippet}
            </VList>
    {/if}

</div>

<style>
    .artistView {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .album {
        padding: 40px;
        box-sizing: border-box;
        display: flex;
        justify-content: flex-start;
        align-items: flex-start;
        gap: 20px;
        width: 100%;
        min-width: 0;
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

    .tracklist {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: center;
        margin: 10px;
        flex: 1;
        min-width: 0;
    }

    .playBtn {
        padding: 5px;
        border-radius: 5px;
        color: var(--color-accent);
        margin-left: auto;
        margin-right: 15px;
        width: 40px;
        display: flex;
        justify-content: center;
        align-items: center;
        
        transition: background-color 0.15s ease, transform 0.15s ease;
    }

    .playBtn:hover {
        background-color: var(--color-hover);
    }

    .playBtn:active {
        transform: scale(0.92);
    }
</style>