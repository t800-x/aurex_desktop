<script lang="ts">
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import { onMount } from "svelte";
    import Header from "./header.svelte";
    import { commands, type Album } from "$lib/bindings";

    const section = Section.albums;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
    import { VList } from "virtua/svelte";
    import AlbumCard from "./album-card.svelte";

    let albums: Album[] = $state([]);

    onMount(async () => {
        albums = await commands.getAllAlbums();
    });

    // chunk albums into rows
    const ITEM_WIDTH = 205; 
    let containerWidth = $state(0);
    let cols = $derived(Math.max(1, Math.floor(containerWidth / ITEM_WIDTH)));
    let rows = $derived(
        Array.from({ length: Math.ceil(albums.length / cols) }, (_, i) =>
            albums.slice(i * cols, i * cols + cols)
        )
    );

</script>

<div style:display={displayMode} class:hidden = {hidden} class="page albumsPage">
    
    <div class="albumList">
        <Header />


        <div bind:clientWidth={containerWidth} style="height: 100%;" >
            <VList data={rows} style="height: 100%;" getKey={(_, i) => i}>
                {#snippet children(row, index)}

                    {#if index === 0}
                        <div class="h-[95px]"></div>

                        <div class="listContainer" style="display: grid; grid-template-columns: repeat({cols}, 1fr); gap: 8px;">
                        {#each row as album}
                            <AlbumCard {album} />
                        {/each}

                    </div>

                    {:else if index === albums.length - 1}
                        <div class="listContainer" style="display: grid; grid-template-columns: repeat({cols}, 1fr); gap: 8px;">
                            {#each row as album}
                                <AlbumCard {album} />
                            {/each}
                        </div>

                        <div class="h-[80px]"></div>

                    {:else}

                        <div class="listContainer" style="display: grid; grid-template-columns: repeat({cols}, 1fr); gap: 8px;">
                        {#each row as album}
                            <AlbumCard {album} />
                        {/each}

                    </div>
                    {/if}

                    
                {/snippet}
            </VList>
        </div>
    </div>

</div>

<style>

    .albumsPage {
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 100%;
        overflow: hidden;
        isolation: isolate;
    }

    .albumList {
        flex: 1;
        overflow: hidden;
        height: 100%;
        width: 100%;
        isolation: isolate;
        position: relative;
    }

    .listContainer {
        padding-left: 45px;
        padding-right: 45px;
        padding-bottom: 30px;
    }

</style>