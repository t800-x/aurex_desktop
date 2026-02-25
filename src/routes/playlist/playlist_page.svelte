<script lang="ts">
    import { commands, type FullTrack, type Playlist } from "$lib/bindings";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import PlayButtonRed from "$lib/ui/play-button-red.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";
    import ShuffleIcon from "$lib/icons/shuffle-icon.svelte";
    import { router } from "$lib/router.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import Tile from "../albums/tile.svelte";

    let hidden = $derived(!router.current.toString().startsWith("pl-"));
    let playlist_id = $derived(Number(router.current.toString().match(/pl-(\d+)/)?.[1]));
    let isP_id = $derived(!Number.isNaN(playlist_id));
    let displayMode = $derived(hidden ? 'none' : 'flex');
    let playlist: Playlist | null = $state(null);
    let tracks: FullTrack[] = $state([]);

    function getCover(p: Playlist | null): string | null {
        if (!p || !p.cover_path) return null;
        return convertFileSrc(p.cover_path);
    }

    $effect(() => {
        (async () => {
            if (isP_id) {
                playlist = await commands.getPlaylist(playlist_id);
                tracks = await commands.getPlaylistTracks(playlist_id); 
            }
        })();
    });

    let imgSrc = $derived(getCover(playlist));

    function formatDuration(seconds: number): string {
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        if (h > 0) return `${h} hr ${m} min`;
        return `${m} min`;
    }

    let totalDuration = $derived(
        tracks.reduce((acc, ft) => acc + Number(ft.track.duration), 0)
    );

    function shuffleAndPlay() {
        if (tracks.length === 0) return;
        const idx = Math.floor(Math.random() * tracks.length);
        commands.playList(tracks, idx);
    }
</script>

<div style:display={displayMode} class:hidden={hidden} class="playlistPage page">

    <div class="topInfo">
        {#if imgSrc !== null}
            <div class="glowWrapper">
                <img class="glow" src={imgSrc} alt="" />
                <img class="playlistCover" src={imgSrc} alt="" />
            </div>
        {:else}
            <div class="playlistCover noArt">
                <DoubleNoteIcon size={150} />
            </div>
        {/if}

        <div class="playlistInfo">
            <div class="title">
                {playlist?.name ?? "Playlist"}
            </div>

            <div class="extraInfo">
                {tracks.length} {tracks.length === 1 ? 'song' : 'songs'} · {formatDuration(totalDuration)}
            </div>

            <div style="flex: 1"></div>

            <div class="play">
                <PlayButtonRed onclick={() => commands.playList(tracks, 0)}>
                    <PlayIcon size={20} />
                    <span>Play</span>
                </PlayButtonRed>

                <PlayButtonRed onclick={shuffleAndPlay}>
                    <ShuffleIcon size={20} />
                    <span>Shuffle</span>
                </PlayButtonRed>
            </div>
        </div>
    </div>

    <div class="songs">
        {#each tracks as fulltrack, index}
            <Tile
                track={fulltrack.track}
                index={index}
                playList={() => commands.playList(tracks, index)}
            />

            {#if index === tracks.length - 1}
                <div style="height: 80px;"></div>
            {/if}
        {/each}
    </div>

</div>

<style>
    .playlistPage {
        height: 100%;
        width: 100%;
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
        padding-top: 20px;
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
        transform: scale(1.001);
        z-index: 0;
    }

    .playlistCover {
        position: relative;
        z-index: 1;
        height: 270px;
        width: 270px;
        border-radius: 6px;
        border-color: var(--color-divider);
        border-width: 1px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        object-fit: cover;
    }

    .noArt {
        background-color: #1c1c1e;
        color: #3d3c41;
    }

    .playlistInfo {
        display: flex;
        flex-direction: column;
        padding-left: 45px;
        justify-content: flex-start;
        align-self: stretch;
        min-height: 270px;
    }

    .title {
        font-size: 25px;
        font-weight: bold;
    }

    .extraInfo {
        font-size: 12px;
        font-weight: 700;
        color: var(--color-navbar-label);
        margin-top: 4px;
    }

    .play {
        width: 100%;
        display: flex;
        gap: 10px;
        font-size: 15px;
    }

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
</style>