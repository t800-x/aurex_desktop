<script lang="ts">
    import { scale } from 'svelte/transition';
    import { audioPlayer } from "$lib/player.svelte";
    import { commands } from "$lib/bindings";
    import { PlayerState } from "$lib/player.svelte";
    import ShuffleIcon from "$lib/icons/shuffle-icon.svelte";
    import PreviousIcon from "$lib/icons/previous-icon.svelte";
    import PauseIcon from "$lib/icons/pause-icon.svelte";
    import NextIcon from "$lib/icons/next-icon.svelte";
    import LoopIcon from "$lib/icons/loop-icon.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";

    let isPlaying = $derived(audioPlayer.state == PlayerState.Playing);
</script>

<div class="playbackControls">
    <button class="playbackControlButton secondaryControl">
        <ShuffleIcon size={18}/>
    </button>

    <button class="playbackControlButton primaryControl">
        <span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
            <PreviousIcon />
        </span>
    </button>

    <button 
    onclick={async () => isPlaying ? commands.pause() : commands.play()} 
    class="playbackControlButton primaryControl"
    >
        {#if isPlaying}
            <span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
                <PauseIcon />
            </span>
        {:else}
            <span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
            <PlayIcon />
            </span>
        {/if}
    </button>

    <button class="playbackControlButton primaryControl">
        <span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
            <NextIcon />
        </span>
    </button>

    <button class="playbackControlButton secondaryControl">
        <LoopIcon size={18}/>
    </button>
</div>

<style>
    .playbackControls {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .playbackControlButton {
        padding: 5px;
        border-radius: 40px;
        cursor: default;
    }

    .playbackControlButton:hover {
        background-color: var(--color-hover);
    }

    .secondaryControl {
        color: var(--color-navbar-label);
    }

    .primaryControl {
        min-width: 35px;
        min-height: 35px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        transition: transform 0.15s ease, background-color 0.15s ease;
    }

    .primaryControl:hover {
        background-color: var(--color-hover);
        transform: scale(1.1);
    }

    .primaryControl:active {
        transform: scale(0.92);
    }


    .icon {
        display: flex;
        align-items: center;
        justify-content: center;
        position: absolute;
    }
</style>