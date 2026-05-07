<script lang="ts">
  import { RightPaneContent, router } from "$lib/router.svelte";


  let {
    Icon,
    content,
    big = false,
  }: {
    Icon: any;
    content: RightPaneContent;
    big?: boolean; 
  } = $props();

  let active = $derived(router.rightPaneContent === content);

  //Edge case for fullscreen player
  $effect(() => {
    if (big && (router.rightPaneContent === null) && (content === RightPaneContent.lyrics)) {
      active = true;
    }
  });
</script>

<button
  onclick={() => router.setRightPaneContent(content)}
  class="paneButton"
  class:active
  class:bigBtn={big}
>
  <Icon size={20} />
</button>

<style>
  .paneButton {
    display: flex;
    justify-content: center;
    align-items: center;
    padding-top: 3px;
    padding-bottom: 3px;
    padding-left: 5px;
    padding-right: 5px;
    border-radius: 6px;

    transition: background-color 0.2s ease 0s;
    transition: box-shadow 0.2s ease 0s;
  }

  .paneButton:hover {
    background-color: var(--color-hover);
  }

  .paneButton.active {
    background-color: var(--color-accent);
    box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
  }

  .bigBtn {
    width: 50%;
    padding-top: 6px;
    padding-bottom: 6px;
  }

  .bigBtn.active{
    background-color: var(--color-hover-w2);
    box-shadow: 0 3px 10px 2px rgba(0, 0, 0, 0.1);
  }
</style>
