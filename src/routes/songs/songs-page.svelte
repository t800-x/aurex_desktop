<script lang="ts">
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import Header from "./header.svelte";
    import ListTile from "./list-tile.svelte";
    import {commands} from "$lib/bindings";
    import { onMount } from "svelte";
    import type { FullTrack } from '$lib/bindings';
    import { formatDuration } from "$lib/helpers";

    const section = Section.songs;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
    let tracks: FullTrack[] = $state([]);

    onMount(async () => {
        tracks = await commands.getAllTracks();
        console.log(tracks);
    });
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page songsPage">

    <div class="tracklist">
        <Header class="header"/>

        {#each tracks as track, i}
            <ListTile 
                album={track.album_title}
                artist={track.artist_name}
                title={track.track.title}
                time={formatDuration(track.track.duration)}
                index={i}
                onclick={async () =>  {
                    await commands.load(track);
                    commands.play();
                }}
            />
        {/each}

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
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 2px;
    }

</style>