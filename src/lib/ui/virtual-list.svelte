<script lang="ts">
  /**
   * VirtualList.svelte - Svelte 5 virtualised list, zero deps.
   *
   * Props:
   *   items        - the full array of data
   *   itemHeight   - fixed px height of each row (required for the math)
   *   height       - height of the scroll container (px or css string)
   *   overscan     - extra rows rendered above/below viewport (default 3)
   *
   * Usage:
   *   <VirtualList {items} itemHeight={48} height={600}>
   *     {#snippet row(item, index)}
   *       <div>{item.name}</div>
   *     {/snippet}
   *   </VirtualList>
   */

  import { type Snippet } from 'svelte';

  type Item = unknown;

  interface Props {
    items: Item[];
    itemHeight: number;
    height?: number | string;
    overscan?: number;
    row: Snippet<[Item, number]>;
  }

  let {
    items,
    itemHeight,
    height = 500,
    overscan = 3,
    row,
  }: Props = $props();

  // ── state ──────────────────────────────────────────────────────────────────
  let scrollTop = $state(0);
  let containerEl = $state<HTMLDivElement | null>(null);
  let containerHeight = $state(typeof height === 'number' ? height : 500);

  // ── derived geometry ───────────────────────────────────────────────────────
  const totalHeight = $derived(items.length * itemHeight);

  const startIndex = $derived(
    Math.max(0, Math.floor(scrollTop / itemHeight) - overscan)
  );

  const endIndex = $derived(
    Math.min(
      items.length - 1,
      Math.ceil((scrollTop + containerHeight) / itemHeight) + overscan
    )
  );

  const visibleItems = $derived(
    items.slice(startIndex, endIndex + 1).map((item, i) => ({
      item,
      index: startIndex + i,
    }))
  );

  const offsetY = $derived(startIndex * itemHeight);

  // ── scroll handler ─────────────────────────────────────────────────────────
  function onScroll(e: Event) {
    scrollTop = (e.currentTarget as HTMLDivElement).scrollTop;
  }

  // ── sync container height when using a CSS string value ───────────────────
  $effect(() => {
    if (!containerEl) return;
    const ro = new ResizeObserver(([entry]) => {
      containerHeight = entry.contentRect.height;
    });
    ro.observe(containerEl);
    return () => ro.disconnect();
  });
</script>

<!-- 
  Scroll container: fixed height, overflow-y scroll.
  position:relative so we can absolutely position the inner runway.
-->
<div
  bind:this={containerEl}
  class="vl-container"
  style:height={typeof height === 'number' ? `${height}px` : height}
  onscroll={onScroll}
  role="list"
>
  <!-- Full-height runway — tricks the scrollbar into thinking all rows exist -->
  <div class="vl-runway" style:height="{totalHeight}px">
    <!-- Translated slab containing only visible rows -->
    <div class="vl-slab" style:transform="translateY({offsetY}px)">
      {#each visibleItems as { item, index } (index)}
        <div
          class="vl-row"
          style:height="{itemHeight}px"
          role="listitem"
        >
          {@render row(item, index)}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .vl-container {
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
    /* GPU-accelerate the scroll surface */
    will-change: scroll-position;
    /* Momentum scrolling on iOS */
    -webkit-overflow-scrolling: touch;
  }

  .vl-runway {
    position: relative;
    width: 100%;
    /* No actual content here — just a height placeholder */
  }

  .vl-slab {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    will-change: transform;
  }

  .vl-row {
    /* Rows must not overflow their declared itemHeight */
    overflow: hidden;
    box-sizing: border-box;
  }
</style>