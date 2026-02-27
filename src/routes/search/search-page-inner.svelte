<script lang="ts">
    import SearchInput from "$lib/components/ui/input/search-input.svelte";
    import type { Album, Artist, FullTrack, Playlist } from "$lib/bindings";
    import { commands } from "$lib/bindings";
    import TrackTile from "./track-tile.svelte";

    
    let searchTerm: string = $state("");

    let songs: FullTrack[] = $state([]);
    let albums: Album[] = $state([]);

    async function search() {
        if (searchTerm.length !== 0) {
            let result = await commands.search(searchTerm);
            songs = result.tracks;
            albums = result.albums;
        } else {
            songs = [];
            albums = [];
        }
    }
</script>

<div  class="searchPage">
    <div style="width: 100%; padding: 20px">
        <SearchInput bind:value={searchTerm} placeholder={"Search your library"} onEnter={search} />
    </div>

    {#if songs.length !== 0}
        <div class="trackList">
            
        </div>
    {/if}

    {#if albums.length !== 0}
        <!-- albums -->
    {/if}


</div>

<style>
    .searchPage {
        display: flex;
        height: 100%;
        width: 100%;
        flex-direction: column;
        justify-content: flex-start;
        gap: 20px;
        align-items: center;
        padding: 10px;
    }

    .trackList {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: scroll;
    }
</style>