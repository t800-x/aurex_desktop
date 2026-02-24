<script lang="ts">
  import type { Track } from "$lib/bindings";
  import { formatDuration, loadAndPlay } from "$lib/helpers";
  import PlayIcon from "$lib/icons/play-icon.svelte";
  import { commands } from "$lib/bindings";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";

  let {
    track,
    playList,
    index
  } : {
    track: Track;
    playList: () => void;
    index: number;
  } = $props();

  let hovered = $state(false);
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger>
    <div
      role="listitem"
      class:odd={index % 2 !== 0}
      class="tile"
      onmouseenter={() => hovered = true}
      onmouseleave={() => hovered = false}
    >
      <div class="title">
        <div class="leading" aria-hidden="true">
          {#if hovered}
            <button class="playBtn" aria-label="Play track" onclick={() => playList()}>
              <PlayIcon size={15} />
            </button>
          {:else}
            <span class="index">{index + 1}</span>
          {/if}
        </div>

        <div class="title-text" title={track.title}>
          {track.title}
        </div>
      </div>

      <div class="trailing">
        <span class="duration">{formatDuration(track.duration)}</span>
      </div>
    </div>
  </ContextMenu.Trigger>

  <ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
        <ContextMenu.Item onclick={async () => {
            let fulltrack = await commands.fulltrackFromId(Number(track.id));
            if (fulltrack !== null) {
              loadAndPlay(fulltrack);
            }
        }}>
            Play
        </ContextMenu.Item>
        
        <ContextMenu.Item
            onclick={() => playList()}
        >
            Play from here
        </ContextMenu.Item>

        <ContextMenu.Item onclick={async () => {
            let fulltrack = await commands.fulltrackFromId(Number(track.id));
            if (fulltrack !== null) {
              commands.playNext(fulltrack)
            }
          }}>
          Play Next
        </ContextMenu.Item>
        <ContextMenu.Item onclick={async () => {
          let fulltrack = await commands.fulltrackFromId(Number(track.id));
            if (fulltrack !== null) {
              commands.addToQueue(fulltrack)
            }
        }}>Add to Queue</ContextMenu.Item>
    </ContextMenu.Content>

</ContextMenu.Root>

<style>

  .tile {
    display: flex;
    align-items: center;
    height: 40px;
    width: 100%;
    border-radius: 5px;
    background-color: var(--color-hover);
    padding-right: 16px;
    font-size: 15px;
    box-sizing: border-box;
  }

  .tile:hover {
    background-color: var(--color-double-hover);
  }

  .odd {
    background-color: transparent;
  }

  
  .leading {
    width: 28px;              
    min-width: 28px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 12px;       
    margin-right: 12px;        
    box-sizing: border-box;
  }


  .index {
    width: 100%;
    display: inline-block;
    text-align: right;         
    color: var(--color-navbar-label);
    padding-right: 6px;        
  }

  .playBtn {
    width: 100%;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding: 4px 8px 4px 4px; 
    border-radius: 6px;
    color: var(--color-accent);
    border: none;
    cursor: default;
    box-sizing: border-box;
    
    transition: background-color 0.15s ease;
  }

  .playBtn:hover {
    background-color: var(--color-hover);
  }

  .title {
    display: flex;
    align-items: center;
    flex: 1;                   
    overflow: hidden;
  }

  .title-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .trailing {
    margin-left: auto;
    padding-left: 12px;
    display: flex;
    align-items: center;
    min-width: 64px;
    justify-content: flex-end;
  }

  .duration {
    color: var(--color-navbar-label);
    font-weight: 600;
    font-size: 13px;
  }
</style>