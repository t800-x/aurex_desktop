<script lang="ts">
    import { onMount } from "svelte";
    import Header from "./header.svelte";
    import { commands, type Album } from "$lib/bindings";
    import { VList } from "virtua/svelte";
    import AlbumCard from "./album-card.svelte";
    import AlbumView from "./album-view.svelte";

    import { getContext } from "svelte";
    import type { StackContext } from "$lib/ui/stack-view.svelte";

    const { push, pop, canPop } = getContext<StackContext>('stack');

    let albums: Album[] = $state([]);
    let proxy: Album[] = $state([]);

    onMount(async () => {
        albums = await commands.getAllAlbums();
        proxy = [...albums];
    });

    async function onFilterTermChanged(term: string) {
        let lowerTerm = term.toLowerCase();
        if (term.length === 0) {
            proxy = [...albums];
        } else {
            proxy = albums.filter(a => {
                return (
                    a.title.toLowerCase().includes(lowerTerm)
                );
            });
        }
    }

    // chunk albums into rows
    const ITEM_WIDTH = 205; 
    let containerWidth = $state(0);
    let cols = $derived(Math.max(1, Math.floor(containerWidth / ITEM_WIDTH)));
    let rows = $derived(
        Array.from({ length: Math.ceil(proxy.length / cols) }, (_, i) =>
            proxy.slice(i * cols, i * cols + cols)
        )
    );
</script>

<div class="page albumsPage">
    
    <div class="albumList">
        <Header title={"Albums"} onchanged={onFilterTermChanged}/>


        <div bind:clientWidth={containerWidth} style="height: 100%;" >
            <VList data={rows} style="height: 100%;" getKey={(row: any, _: any) => row.map((a: { id: any; }) => a.id).join('-')}>
                {#snippet children(row: any, index: number)}

                    {#if index === 0}
                        <div class="h-[95px]"></div>
                    {/if}

                    <div class="listContainer" style="display: grid; grid-template-columns: repeat({cols}, 1fr); gap: 8px;">
                        {#each row as album}
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <div onclick={() => push(AlbumView, {album: album})}>
                            <AlbumCard {album} />
                        </div>
                        {/each}
                    </div>

                    {#if index === rows.length - 1}
                        <div class="h-[80px]"></div>
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