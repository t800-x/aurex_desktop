<script lang="ts">

    let {
        title,
        artist,
        time,
        album,
        index,
        onclick
    } : {
        title: string;
        artist: string;
        time: string;
        album: string;
        index: number;
        onclick?: (e: MouseEvent) => void;
    } = $props();

    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";

</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div onclick={onclick} class="tile" class:even = {index % 2 === 0}>
            <span class="infoLabel" style:flex={4}>{title}</span>
            <span class="infoLabel" style:flex={1}>{time}</span>
            <span class="infoLabel" style:flex={3}>{artist}</span>
            <span class="infoLabel" style:flex={3}>{album}</span>
        </div>
    </ContextMenu.Trigger>

    <ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
        <ContextMenu.Item>Play</ContextMenu.Item>
        <ContextMenu.Item>Play from here</ContextMenu.Item>
        <ContextMenu.Item>Play Next</ContextMenu.Item>
        <ContextMenu.Item>Add to Queue</ContextMenu.Item>
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