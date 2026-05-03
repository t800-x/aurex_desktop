<script lang="ts">
  import { router } from "$lib/router.svelte";

  let {
    text,
    Icon,
    iconSize = 20
  }: {
    text: string;
    Icon: any;
    iconSize?: number;
  } = $props();
  

</script>


<button
    class="nav-item"
    onclick={() => {
      router.openCreatePlaylistDialog(null);
      console.log("create playlist")
    }}
    >
    <Icon className="nav-icon" size={iconSize}/>
    <span>{text}</span>
</button>

<style>
  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding-left: 10px;
    margin: 0 20px;
    height: 28px;
    width: 100%;
    border: none;
    border-radius: 8px;
    background: transparent;
    transition: all 125ms ease;
    box-shadow: none;
    overflow: visible;
  }
  
  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .nav-item:active {
    background: rgba(255, 255, 255, 0.2);
  }
  
  .nav-item span {
    color: white;
    font-size: 15px;
    font-weight: 500;
  }
  
  .nav-item :global(svg),
  .nav-item :global(i) {
    color: rgb(250, 88, 106);
    flex-shrink: 0;
    min-width: 25px;
  }
</style>