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
    let proxy: FullTrack[] = $state([]);

    onMount(async () => {
        tracks = await commands.getAllTracks();
        proxy = [...tracks];
    });

    async function playList(index: number): Promise<void> {
        commands.playList(proxy, index);
    }

    async function onFilterTermChanged(term: string) {
        let lowerTerm = term.toLowerCase();
        if (term.length === 0) {
            proxy = [...tracks];
        } else {
            proxy = tracks.filter(t => {
                return (
                    t.album_title?.toLowerCase().includes(lowerTerm) ||
                    t.artist_name?.toLowerCase().includes(lowerTerm) ||
                    t.track?.title?.toLowerCase().includes(lowerTerm)
                );
            });
        }
    }
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page songsPage">

    <div class="tracklist">

        <Header onchanged={onFilterTermChanged} />

        <VList data={proxy} style='height: 100%' getKey={(_: any, i: any) => i}>
            {#snippet children(track: any, i: number)}
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