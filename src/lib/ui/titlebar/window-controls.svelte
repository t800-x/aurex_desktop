<script lang="ts">
  import { type } from "@tauri-apps/plugin-os";
  import MacOsControls from "./mac-os-controls.svelte";
  import WindowsControls from "./windows-controls.svelte";
  import KdeControls from "./kde-controls.svelte";

  const osType = type();
</script>

<div class="titlebar-controls">
  {#if osType === "macos"}
    <MacOsControls />
  {:else if osType === "linux"}
    <KdeControls />
  {:else}
    <WindowsControls />
  {/if}
</div>

<style>
  .titlebar-controls {
    display: flex;
    align-items: center;
    -webkit-app-region: no-drag;
  }
</style>
