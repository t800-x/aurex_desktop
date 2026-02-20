<script lang="ts">
    import { scale } from 'svelte/transition';
</script>

<button aria-label="" onclick={() => console.log("clicked")} class="playbackControlButton playPauseButton">
    <span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
    </span>
</button>

<style>
    .playbackControlButton {
        padding: 5px;
        border-radius: 40px;
        cursor: default;
    }

    .playbackControlButton:hover {
        background-color: var(--color-hover);
    }

    .playPauseButton {
        min-width: 35px;
        min-height: 35px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        transition: transform 0.15s ease, background-color 0.15s ease;
    }

    .playPauseButton:hover {
        background-color: var(--color-hover);
        transform: scale(1.1);
    }

    .playPauseButton:active {
        transform: scale(0.92);
    }

    .icon {
        display: flex;
        align-items: center;
        justify-content: center;
        position: absolute;
    }
</style>