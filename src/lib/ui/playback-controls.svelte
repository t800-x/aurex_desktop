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
    import PoppingButton from './popping-button.svelte';

    let isPlaying = $derived(audioPlayer.state == PlayerState.Playing);
</script>

<div class="playbackControls">
    <button class="playbackControlButton secondaryControl shuffleBtn" class:enabledControl={audioPlayer.shuffle} onclick={() => commands.shuffle()}>
        <ShuffleIcon size={18} />
        {#if audioPlayer.shuffle}
            <span class="shuffleDot" transition:scale></span>
        {/if}
    </button>

    <button class="playbackControlButton primaryControl" onclick={() => commands.previous()} class:disabledControl={audioPlayer.history.length === 0}>
        <PoppingButton>
            <PreviousIcon />
        </PoppingButton>
    </button>

    <button 
    onclick={async () => isPlaying ? commands.pause() : commands.play()} 
    class="playbackControlButton primaryControl"
    >
        {#if isPlaying}
            <PoppingButton>
                <PauseIcon />
            </PoppingButton>
        {:else}
            <PoppingButton>
                <PlayIcon />
            </PoppingButton>
        {/if}
    </button>

    <button class="playbackControlButton primaryControl" onclick={async () => commands.next()} class:disabledControl={audioPlayer.queue.length === 0}>
        <PoppingButton>
            <NextIcon />
        </PoppingButton>
    </button>

    <button class="playbackControlButton secondaryControl">
        <LoopIcon size={18}/>
    </button>
</div>

<style>
    .disabledControl {
        pointer-events: none;
    }

    .disabledControl :global(i) {
        color: var(--color-navbar-label);
    }

    .enabledControl :global(i) {
        color: var(--color-accent);
    }

    .playbackControls {
        flex: 25;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .playbackControlButton {
        padding: 5px;
        border-radius: 40px;
        cursor: default;

        transition: color 0.15s ease, transform 0.15s ease, background-color 0.15s ease;
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
    }

    .primaryControl:hover {
        background-color: var(--color-hover);
    }

    .primaryControl:active {
        transform: scale(0.92);
    }

    .shuffleBtn {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .shuffleDot {
        width: 4px;
        height: 4px;
        border-radius: 50%;
        background-color: var(--color-accent);
        position: absolute;
        bottom: 3px;
    }
</style>