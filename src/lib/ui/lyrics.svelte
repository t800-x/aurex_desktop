<script lang="ts">
    import { commands, type FullTrack, type LineLyrics } from "$lib/bindings";
    import { audioPlayer } from "$lib/player.svelte";
    import { onMount } from "svelte";
    import LineLyric from "./line-lyric.svelte";

    let line_lyrics: LineLyrics[] = $state([]);
    let isSomethingPlaying = $derived(audioPlayer.currentlyPlaying !== null);
    let itemEls: HTMLElement[] = [];
    let listEl = $state<HTMLDivElement>();
    let localTrack = $state<FullTrack | null>(null);
    let tracksMatched = $derived(JSON.stringify(localTrack) === JSON.stringify(audioPlayer.currentlyPlaying));
    let localLineIdx = $state<number | null>(null);
    let localLineIdxNull = $derived(localLineIdx === null);
    let matchLineIdx = $derived(localLineIdx === audioPlayer.lyricsLineIndex);

    function spawn(fn: () => void) {
        fn();
    }

    function scrollToLine(index: number) {
        const el = itemEls[index]; 
        if (!el || !listEl) return; 

        // Use scrollTo for a native smooth animation 
        listEl.scrollTo({
            top: el.offsetTop - 100, // Adjusted offset to keep the active line more centered
            behavior: 'smooth'
        });
    }

    $effect(() => {
        spawn(async () => {
            if (!matchLineIdx) {
                localLineIdx = audioPlayer.lyricsLineIndex;
                
                if (!localLineIdxNull) {
                    scrollToLine(localLineIdx!);
                }
            }

            if (!tracksMatched) {
                localTrack = audioPlayer.currentlyPlaying;
                line_lyrics = await commands.getLineLyrics(audioPlayer.currentlyPlaying!);
            }  
        });
    });
</script>

<div class="lyricsContainer">
    Lyrics

    <div bind:this={listEl} class="lyricsDisplay">
    {#each line_lyrics as lyrics, index}
        <div bind:this={itemEls[index]}>
            <LineLyric lyrics={lyrics} index={index}/>
        </div>
    {/each}
</div>
</div>

<style>
    .lyricsContainer {
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        overflow: hidden;
        height: 100%;
        width: 100%;
    }

    .lyricsDisplay {
        padding: 20px;
        flex: 1;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start; 
        overflow-y: scroll;
        height: 100%;
        width: 100%;
        scroll-behavior: smooth;
    }
</style>