<script lang="ts">
  import { onMount } from 'svelte';
  import { router } from '$lib/router.svelte';
  let isOverlay = $state(false);
  function checkWidth() {
    isOverlay = window.innerWidth < 1200;
  }
  let rightPaneOpen = $derived(router.rightPaneContent !== null);
  onMount(() => {
    checkWidth();
    window.addEventListener('resize', checkWidth);
    return () => window.removeEventListener('resize', checkWidth);
  });
  function closePane() {
    router.setRightPaneContent(null);
  }
</script>
{#if isOverlay && rightPaneOpen}
  <div class="backdrop" onclick={closePane} aria-hidden="true"></div>
{/if}
<aside
  class="rightPane"
  class:open={rightPaneOpen}
  class:overlay={isOverlay}
>
  <div class="rightPaneInner">
    <slot />
  </div>
</aside>
<style>
  .backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.2);
    z-index: 10;
    cursor: default;
  }
  .rightPane {
    width: 0;
    overflow: hidden;
    height: 100%;
    background-color: #2a2a2a;
    border-left: 1px solid rgba(255, 255, 255, 0.08);
    transition: width 0.3s ease;
    flex-shrink: 0;
  }
  .rightPane.open {
    width: 320px;
  }
  .rightPane.overlay {
    position: absolute;
    right: 0;
    top: 0;
    z-index: 20;
    border-left: 1px solid rgba(255, 255, 255, 0.12);
  }
  .rightPane.overlay.open {
    width: min(320px, 85vw);
  }
  .rightPaneInner {
    width: 320px;
    height: 100%;
    padding: 1rem;
    overflow-y: auto;
  }
</style>