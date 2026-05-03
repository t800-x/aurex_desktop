<script lang="ts">
  import { onMount } from "svelte";
  import Header from "../albums/header.svelte";
  import { commands, type Album } from "$lib/bindings";
  import { VList } from "virtua/svelte";
  import AlbumCard from "../albums/album-card.svelte";
  import AlbumView from "../albums/album-view.svelte";
  import { getContext } from "svelte";
  import type { StackContext } from "$lib/ui/views/stack-view.svelte";
  import { listen } from "@tauri-apps/api/event";

  const { push, pop, canPop } = getContext<StackContext>("stack");

  let albums: Album[] = $state([]);
  let proxy: Album[] = $state([]);

  onMount(async () => {
    albums = await commands.getAllAlbums();
    proxy = [...albums];

    await listen<void>("indexing-done", async (event) => {
      albums = await commands.getAllAlbums();
      proxy = [...albums];
    });
  });

  async function onFilterTermChanged(term: string) {
    const lowerTerm = term.toLowerCase();
    proxy =
      term.length === 0
        ? [...albums]
        : albums.filter((a) => a.title.toLowerCase().includes(lowerTerm));
  }

  const MIN_ITEM_WIDTH = 205;
  const GAP = 8;
  const HORIZONTAL_PADDING = 90;

  let containerWidth = $state(0);

  // Prevent negative widths when container hasn't mounted yet
  let availableWidth = $derived(
    Math.max(0, containerWidth - HORIZONTAL_PADDING),
  );

  // Default to 1 column if unmeasured to stop virtua from crashing
  let cols = $derived(
    containerWidth === 0
      ? 1
      : Math.max(
          1,
          Math.floor((availableWidth + GAP) / (MIN_ITEM_WIDTH + GAP)),
        ),
  );

  let rows = $derived(
    Array.from({ length: Math.ceil(proxy.length / cols) }, (_, i) =>
      proxy.slice(i * cols, i * cols + cols),
    ),
  );
</script>

<div class="albums">
  <div class="albumList">
    <Header title={"Albums"} onchanged={onFilterTermChanged} />

    <div bind:clientWidth={containerWidth} style="height: 100%; width: 100%;">
      <VList
        data={rows}
        style="height: 100%;"
        getKey={(row: any) => row.map((a: { id: any }) => a.id).join("-")}
      >
        {#snippet children(row: any, index: number)}
          {#if index === 0}
            <div class="h-[95px]"></div>
          {/if}

          <div
            class="listContainer"
            style="display: grid; grid-template-columns: repeat({cols}, minmax(0, 1fr)); gap: {GAP}px;"
          >
            {#each row as album}
              <div style="min-width: 0; width: 100%;">
                <AlbumCard {album} onclick={() => push(AlbumView, { album })} />
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
  .albums {
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
