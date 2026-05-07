<script lang="ts">
    import { audioPlayer } from "$lib/player.svelte";
    import { RightPaneContent, router } from "$lib/router.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import Backdrop from "./backdrop.svelte";
    import ShrinkIcon from "$lib/icons/shrink-icon.svelte";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import ExpandIcon from "$lib/icons/expand-icon.svelte";
    import Seekbar from "$lib/ui/now_playing/seekbar.svelte";
    import PlaybackControls from "../now_playing/playback-controls.svelte";
    import Lyrics from "../right_pane/lyrics.svelte";
    import PaneControls from "../now_playing/pane-controls.svelte";
    import Queue from "../right_pane/queue.svelte";

    let isQueue = $derived(router.rightPaneContent === RightPaneContent.queue);
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

<div class="fullScreenPlayer" style:display={router.fullscreen ? 'flex' : 'none'}>
    <Backdrop />

    

    <div class="mainContent">
        <div class="trackInfo">
            {#if isCurrentlyPlaying}
                <img class="albumCover" src={imgSrc} alt="" />
            {:else}
                <div class="albumCover">
                <DoubleNoteIcon />
                </div>
            {/if}

            {#if audioPlayer.currentlyPlaying !== null}
                <div class="title">
                    <div class="trackTitle">{trackTitle}</div>
                    <div class="albumInfo">{albumInfo}</div>
                    <div style="height: 20px;"></div>
                    <Seekbar />
                    <div style="height: 20px;"></div>
                    <PlaybackControls big={true} />
                    <div style="height: 20px;"></div>
                    <PaneControls big={true} />
                </div>
            {:else}
                <div class="trackTitle" style="margin-right: auto;">Not Playing</div>
            {/if}
        </div>

        <div class="additionalPane">
            <div style="height: 60%; width: 85%;">
                <div style:display={isQueue ? "contents" : "none"}>
                        <Queue />
                    </div>
                    <div style:display={isQueue ? "none" : "contents"}>
                        <Lyrics />
                    </div>
                </div>
            </div>
        </div>

    <div class="closeButtonWrapper">
        <button onclick={() => {router.fullscreen = false;}} class="closeButton">
            <ShrinkIcon />
        </button>
    </div>
</div>

<style>
    .fullScreenPlayer {
        height: 100%;
        width: 100%;
        background-color: black;
        position: absolute;
        z-index: 950;
        flex-direction: column;
        overflow: hidden;

        isolation: isolate;
    }

    .trackInfo {
        flex: 50;
        display: flex;
        overflow: visible;
        justify-content: center;
        align-items: center;
        min-width: 0;
        flex-direction: column;
    }

    .title {
        width: 50%;
        height: auto;
        margin-top: 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .trackTitle {
        font-size: 16px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        width: 100%;
        text-align: center;
    }

    .albumInfo {
        font-size: 15px;
        font-weight: 500;
        color: var(--color-navbar-label);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        width: 100%;
        text-align: center;
    }

    .additionalPane {
        flex: 50;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .albumCover {
        height: auto;
        width: 50%;
        border-radius: 8px;
        border-color: var(--color-divider);
        border-width: 1px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1c1c1e;
        color: #3d3c41;
        aspect-ratio: 1 / 1;
    }

    .mainContent {
        height: 100%;
        width: 100%;
        position: absolute;
        z-index: 23;
        display: flex;
    }

    .closeButtonWrapper {
        margin-top: 10px;
        width: 100%;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        padding: 25px;
        z-index: 99;
    }

    .closeButton {
        height: 40px;
        width: 40px;
        padding: 5px;
    }

    .closeButton:hover {
        background-color: var(--color-hover);
        border-radius: 8px;
        border-width: 1px;
        border-color: var(--color-divider);
    }
</style>