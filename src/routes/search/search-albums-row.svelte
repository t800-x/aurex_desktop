<script lang="ts">
  import type { Album } from "$lib/bindings";
  import AlbumCard from "../../routes/albums/album-card.svelte";
  import SearchSectionHeader from "./search-section-header.svelte";
  import { router } from "$lib/router.svelte";
  import { VList } from "virtua/svelte";

  let { albums }: { albums: Album[] } = $props();

  // Card is 205px, scale(1.1) adds ~20px each side vertically.
  // VList clips overflow, so we bake that breathing room into the
  // container height and use matching padding on each item.
  const CARD_SIZE = 205;
  const LABEL_HEIGHT = 40; // two text lines beneath the cover
  const VPAD = 20; // breathing room for scale(1.1)
  const ITEM_WIDTH = CARD_SIZE + 14; // card + gap
  const listHeight = CARD_SIZE + LABEL_HEIGHT + VPAD * 2;
</script>

<section class="section">
  <SearchSectionHeader title="Albums" count={albums.length} />

  <VList
    data={albums}
    horizontal
    style="height: {listHeight}px; padding: {VPAD}px 4px;"
    getKey={(album: Album) => album.id}
  >
    {#snippet children(album: Album, index: number)}
      {#if index === 0}
        <div style="width: 15px;"></div>
      {/if}

      <div class="item" style="width: {ITEM_WIDTH}px;">
        <AlbumCard {album} onclick={() => router.goToAlbum(album)} />
      </div>
    {/snippet}
  </VList>
</section>

<style>
  .section {
    margin-bottom: 28px;
  }

  .item {
    display: flex;
    align-items: flex-start;
    box-sizing: border-box;
  }
</style>
