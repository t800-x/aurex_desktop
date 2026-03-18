<script lang="ts">
    import { audioPlayer } from "$lib/player.svelte";

    let { gapStart, gapEnd }: { gapStart: number; gapEnd: number } = $props();

    let progress = $derived(
        audioPlayer.position < gapStart ? 0 :
        audioPlayer.position >= gapEnd ? 1 :
        (audioPlayer.position - gapStart) / (gapEnd - gapStart)
    );

    // Keep bouncing until we are completely past the gap
    let past = $derived(audioPlayer.position >= gapEnd);
    
    // Trigger the "extra bounce" when we are in the last 15% of the gap
    let almostDone = $derived(progress > 0.85 && progress < 1);

    function dotOpacity(i: number): number {
        // Sequential fade in thresholds
        const lo = i / 3;
        const hi = (i + 1) / 3;
        return Math.min(1, Math.max(0, (progress - lo) / (hi - lo)));
    }
</script>

<div class="gap-dots" 
    class:bouncing={!past && !almostDone}
    class:hurry={almostDone} >
            
    {#each [0, 1, 2] as i}
        {@const opacity = dotOpacity(i)}
        <div
            class="dot"
            style="--fill-opacity: {opacity};"
        >
            <div class="fill"></div>
        </div>
    {/each}
</div>

<style>
    .gap-dots {
        display: flex;
        gap: 10px;
        padding: 14px 10px;
        align-items: center;
    }

    .dot {
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.15); /* Base idle state */
        position: relative;
        /* Removed overflow: hidden so the extra scale/bounce doesn't clip */
    }

    /* Standard bounce leading up to and during the gap */
    .gap-dots.bouncing {
        animation: bounce 1.1s ease-in-out infinite;
    }

    /* Extra bounce right before the gap ends */
    .gap-dots.hurry {
        animation: bounce-high 0.5s ease-in-out infinite;
    }

    /* The new opacity-based fill */
    .fill {
        position: absolute;
        inset: 0; /* Stretches to cover the whole dot immediately */
        background: rgba(255, 255, 255, 0.9);
        border-radius: 50%;
        opacity: var(--fill-opacity);
        transition: opacity 0.08s linear;
    }

    /* We removed opacity from the keyframes so it doesn't fight with the fill or break on rewind */
    @keyframes bounce {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-5px); }
    }

    @keyframes bounce-high {
        0%, 100% { transform: translateY(0) scale(1); }
        50% { transform: translateY(-8px) scale(1.2); }
    }
</style>