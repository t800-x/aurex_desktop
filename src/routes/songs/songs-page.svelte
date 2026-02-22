<script lang="ts">
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import Header from "./header.svelte";
    import ListTile from "./list-tile.svelte";
    import {commands} from "$lib/bindings";
    import { onMount } from "svelte";
    import type { FullTrack } from '$lib/bindings';
    import { formatDuration } from "$lib/helpers";
    import { VList } from "virtua/svelte";

    const section = Section.songs;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
    let tracks: FullTrack[] = $state([]);

    onMount(async () => {
        tracks = await commands.getAllTracks();
    });

    async function playList(index: number): Promise<void> {
        commands.playList(tracks, index);
    }
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page songsPage">

    <div class="tracklist">

        <Header />

        <VList data={tracks} style='height: 100%' getKey={(_, i) => i}>
            {#snippet children(track, i)}
                {#if i === 0}
                    <div class="h-[100px]">
                    </div>

                    <ListTile 
                        track={track}
                        index={i}
                        playList={async () =>  {
                            playList(i);
                        }}
                    />
                    <div class="h-[2px]"></div>

                {:else if i === tracks.length -1 }
                    <ListTile 
                        track={track}
                        index={i-1}
                        playList={async () =>  {
                            playList(i);
                        }}
                    />

                    <div class="h-[80px]"></div>
                {:else}
                    <ListTile 
                        track={track}
                        index={i}
                        playList={async () =>  {
                            playList(i);
                        }}
                    />

                    <div class="h-[2px]"></div>
                {/if}

                
            {/snippet}
        </VList>

    </div>
</div>

<style>
    .songsPage {
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 100%;
        overflow: hidden;
        isolation: isolate;
    }

    .tracklist {
        flex: 1;
        overflow: hidden;
        height: 100%;
        width: 100%;
        isolation: isolate;
        position: relative;
    }

</style>