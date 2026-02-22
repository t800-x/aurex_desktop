<script lang="ts">

    import type { FullTrack } from "$lib/bindings";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import { commands } from "$lib/bindings";
    import { formatDuration } from "$lib/helpers";
    import { loadAndPlay } from "$lib/helpers";

    let {
        track,
        index,
        playList
    } : {
        track: FullTrack;
        index: number;
        playList: (e: MouseEvent) => void;
    } = $props();

    
    
</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div onclick={async () => await loadAndPlay(track)} class="tile" class:even = {index % 2 === 0}>
            <span class="infoLabel" style:flex={4}>{track.track.title}</span>
            <span class="infoLabel" style:flex={1}>{formatDuration(track.track.duration)}</span>
            <span class="infoLabel" style:flex={3}>{track.artist_name}</span>
            <span class="infoLabel" style:flex={3}>{track.album_title}</span>
        </div>
    </ContextMenu.Trigger>

    <ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
        <ContextMenu.Item onclick={async () => {
            await loadAndPlay(track);
        }}>
            Play
        </ContextMenu.Item>
        
        <ContextMenu.Item
            onclick={(e) => playList(e)}
        >
            Play from here
        </ContextMenu.Item>

        <ContextMenu.Item onclick={async () => commands.playNext(track)}>Play Next</ContextMenu.Item>
        <ContextMenu.Item onclick={async () => commands.addToQueue(track)}>Add to Queue</ContextMenu.Item>
    </ContextMenu.Content>

</ContextMenu.Root>

<style>
    .tile {
        padding: 4px;
        width: calc(100% - 10px);
        margin-left: 5px;
        margin-right: 5px;
        display: flex;
        padding-left: 10px;
        padding-right: 10px;
        justify-content: center;
        border-radius: 6px;
        align-items: center;
        background-color: var(--color-hover);
        cursor: default;
    }

    .tile:hover {
        background-color: var(--color-double-hover);
    }

    .tile.even {
        background-color: transparent;
    }

    .tile.even:hover {
        background-color: var(--color-hover);
    }

    .infoLabel {
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>