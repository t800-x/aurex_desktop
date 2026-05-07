<script lang="ts">
  import LyricsIcon from "$lib/icons/lyrics-icon.svelte";
  import QueueIcon from "$lib/icons/queue-icon.svelte";
  import { RightPaneContent } from "$lib/router.svelte";
  import PaneControlButton from "./pane-control-button.svelte";

  let {
    big = false
  } : {
    big?: boolean;
  } = $props();
</script>

<div class="paneControls" class:bigBtn={big}>
  <PaneControlButton Icon={LyricsIcon} content={RightPaneContent.lyrics} big={big} />
  <PaneControlButton Icon={QueueIcon} content={RightPaneContent.queue} big={big} />
</div>

<style>
  .paneControls {
    display: flex;
    flex: 20;
    align-items: center;
    justify-content: center;
    gap: 2px;
  }

  .bigBtn {
    width: 100%;
  }
</style>
