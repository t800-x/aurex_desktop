<script lang="ts">
  import { router } from "$lib/router.svelte";
  import { Section } from "$lib/router.svelte";

  let {
    text,
    Icon,
    section,
    iconSize = 20
  }: {
    text: string;
    Icon: any;
    section: Section;
    iconSize?: number;
  } = $props();
  
  let selected = $derived(router.current === section);

</script>

<button
  class="nav-item"
  class:selected
  onclick={() => router.go(section)}
>
  <Icon className="nav-icon" size={iconSize}/>
  <span>{text}</span>
</button>

<style>
  .nav-item {
    margin: 0; 
    padding-left: 10px;
    padding-right: 10px; 
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    height: 28px;
    border: none;
    border-radius: 8px;
    background: transparent;
    transition: all 125ms ease;
    box-shadow: none;
    overflow: visible;
    flex-shrink: 0;
  }
  
  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
  }
  
  .nav-item.selected {
    background: rgba(255, 255, 255, 0.1);
    box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.15);
  }
  
  .nav-item.selected:hover {
    background: rgba(255, 255, 255, 0.2);
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