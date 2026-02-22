<script lang="ts">
  import { onMount } from 'svelte';
  import { RightPaneContent, router } from '$lib/router.svelte';
    import Queue from './queue.svelte';
    import Lyrics from './lyrics.svelte';

  function checkWidth() {
    router.rightPaneOverlaying = window.innerWidth < 1500;
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

  let isQueue = $derived(router.rightPaneContent === RightPaneContent.queue);
</script>


{#if router.rightPaneOverlaying && rightPaneOpen}
  <div class="backdrop" onclick={closePane} aria-hidden="true"></div>
{/if}
<aside
  class="rightPane"
  class:open={rightPaneOpen}
  class:overlay={router.rightPaneOverlaying}
>


  <div class="rightPaneInner">
      <div style:display={isQueue ? 'contents' : 'none'}>
          <Queue />
      </div>
      <div style:display={isQueue ? 'none' : 'contents'}>
          <Lyrics />
      </div>
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
    background-color: var(--color-header);
    border-left: 1px solid rgba(255, 255, 255, 0.08);
    transition: width 0.3s ease;
    flex-shrink: 0;
    backdrop-filter: blur(18px);
    box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
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
    overflow: hidden;
  }
</style>