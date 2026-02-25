<script lang="ts">

    import Header from "../albums/header.svelte";
    import { commands } from "$lib/bindings";
    import type { Artist } from "$lib/bindings";
    import { onMount } from "svelte";
    import { VList } from "virtua/svelte";
    import PersonIcon from "$lib/icons/person-icon.svelte";
    import ArtistView from "./artist-view.svelte";
    import { router, Section } from "$lib/router.svelte";

    const section = Section.artists;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');

    let artists: Artist[] = $state([]);
    let proxy: Artist[] = $state([]);
    let selected: number | null = $state(null);

    onMount(async () => {
        artists = await commands.getAllArtists();
        selected = Number(artists[0].id);
        proxy = [...artists];
    });

    async function searchChanged(term: string) {
        let lower = term.toLowerCase();
        if (lower.length === 0) {
            proxy = [...artists];
        } else {
            proxy = artists.filter((artist) => artist.name.toLowerCase().includes(lower))
        }
    }

</script>

<div class:hidden = {hidden} style:display={displayMode} class="page artistsPage">

    <Header title={"Artists"} onchanged={searchChanged}/>

    <div class="splitView">
        <div class="artistsList">
            <VList style={"height: 100%"} data={proxy} getKey={(artist: any, _: any) => artist.id}>
                {#snippet children(artist: Artist, index: number)}
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div onclick={() => {selected = Number(artist.id)}} class="listTile" class:selected = {selected === artist.id}>
                        <div style="
                            border-radius: 50%;
                        ">
                            <PersonIcon size={25} />
                        </div>
                        <div style="width: 5px;"></div>
                        <span>{artist.name}</span>
                    </div>
                {/snippet}
            </VList>
        </div>

        <div style="height: 100%; width: 1px; background-color: var(--color-divider);"></div>

        <div class="artistViewWrapper">
            <ArtistView selected={selected} />
        </div>
    </div>
</div>

<style>

    .artistsPage {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        isolation: isolate;
        position: relative;
        align-items: center; 
        justify-content: flex-start;
    }

    .splitView {
        flex: 1;
        min-height: 0;
        width: 100%;
        display: flex;
        overflow: hidden;
    }

    .artistsList {
        width: 240px;
        margin-top: 50px;
        flex-shrink: 0;
        overflow: hidden;
    }

    .listTile {
        margin: 5px;
        width: calc(100% - 10px);
        border-radius: 5px;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding: 5px;
        cursor: default;

        transition: background-color 0.05s ease;
    }

    .listTile span {
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        min-width: 0;
    }

    .listTile:hover {
        background-color: var(--color-hover);
    }

    .selected {
        background-color: var(--color-hover);
    }

    .selected:hover {
        background-color: var(--color-double-hover);
    }

    .artistViewWrapper {
        flex: 1;
        min-width: 0;
        width: 100%;
        height: 100%;
        overflow: hidden;
    }

</style>