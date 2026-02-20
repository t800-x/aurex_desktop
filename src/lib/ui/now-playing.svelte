<script lang="ts">
    import { audioPlayer } from "$lib/player.svelte";
    import { commands } from "$lib/bindings";
    import { PlayerState } from "$lib/player.svelte";
    import PlaybackControls from "./playback-controls.svelte";
    import TrackInfo from "./track-info.svelte";
    import PaneControls from "./pane-controls.svelte";

</script>

<div class="nowPlaying">
    <PlaybackControls />
    <div class="w-[10px]"> </div>
    <TrackInfo />
    <PaneControls />
</div>

<style>

    .nowPlaying {
        height: 60px;
        width: 75%;
        border-color: var(--color-divider);
        border-width: 1px;
        border-radius: 50px;
        background-color: var(--color-header);
        position: absolute;
        backdrop-filter: blur(12px);
        bottom: 0;
        left: 0;
        z-index: 100;
        left: 50%;
        transform: translateX(-50%);
        margin-bottom: 20px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        padding-left: 25px;
        padding-right: 25px;
        pointer-events: auto;
    }

</style>