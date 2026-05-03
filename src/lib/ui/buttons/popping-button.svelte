<script lang="ts">
  import { scale } from 'svelte/transition';

  let { children } = $props();
</script>

<span class="icon" in:scale={{ duration: 200, start: 0.5 }} out:scale={{ duration: 150, start: 0.5 }}>
  {@render children()}
</span>

<style>
  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
  }
</style>