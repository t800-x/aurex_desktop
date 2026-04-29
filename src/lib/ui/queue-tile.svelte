<script lang="ts">
    import { commands, type FullTrack } from "$lib/bindings";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { draggable, dropTargetForElements } from '@atlaskit/pragmatic-drag-and-drop/element/adapter';
    import { onMount } from 'svelte';
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import QueueContextMenu from "./queue-context-menu.svelte";

    let {
        track,
        index,
        lastIndex,
        draggedFrom = $bindable(),
        draggedTo = $bindable(),
    }: {
        track: FullTrack;
        index: number;
        lastIndex: number;
        draggedFrom: number | null;
        draggedTo: number | null;
    } = $props();

    let el!: HTMLElement;
    // svelte-ignore non_reactive_update
        // svelte-ignore state_referenced_locally
                let currentIndex = index;

    $effect(() => { currentIndex = index; });

    // compute how much this tile should shift
    let shift = $derived(() => {
        if (draggedFrom === null || draggedTo === null) return 0;
        if (currentIndex === draggedFrom) return 0; // the dragged item itself

        const tileHeight = 45; // match your tile height + padding

        if (draggedFrom < draggedTo) {
            // dragging downward: items between from+1 and to shift up
            if (currentIndex > draggedFrom && currentIndex <= draggedTo) return -tileHeight;
        } else {
            // dragging upward: items between to and from-1 shift down
            if (currentIndex >= draggedTo && currentIndex < draggedFrom) return tileHeight;
        }
        return 0;
    });

    // svelte-ignore state_referenced_locally
        let coverSrc = convertFileSrc(track.album_art ?? "");
    // svelte-ignore state_referenced_locally
        let trackTitle = track.track.title ?? "Unknown";
    // svelte-ignore state_referenced_locally
        let albumInfo = `${track.artist_name} - ${track.album_title}`;

    onMount(() => {
        const cleanup = [
            draggable({
                element: el,
                getInitialData: () => ({ index: currentIndex }),
                onDragStart: () => { draggedFrom = currentIndex; draggedTo = currentIndex; },
                onDrop: () => { draggedFrom = null; draggedTo = null; },
            }),
            dropTargetForElements({
                element: el,
                getData: () => ({ index: currentIndex }),
                onDragEnter: () => { draggedTo = currentIndex; },
            }),
        ];
        return () => cleanup.forEach(fn => fn());
    });
</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <div
            role="listitem"
            bind:this={el}
            class="tile"
            class:dragging={draggedFrom === index}
            style:transform="translateY({shift()}px)"
            onpointerenter={() => { if (draggedFrom !== null) draggedTo = currentIndex; }}
            data-index={currentIndex}
        >
            {#if track.album_art !== null}
                <img class="albumCover" src={coverSrc} alt="">
            {:else}
                <div class="albumCover"><DoubleNoteIcon /></div>
            {/if}
            <div class="title">
                <div class="trackTitle">{trackTitle}</div>
                <div class="albumInfo">{albumInfo}</div>
            </div>

            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button class="removeBtn" onclick={(e) => commands.removeFromQueue(index)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>

        
    </ContextMenu.Trigger>

    <QueueContextMenu track={track} index={index} lastIndex={lastIndex} />
</ContextMenu.Root>



<style>
    .tile {
        width: 100%;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding: 5px;
        cursor: grab;
        border-radius: 7px;
        transition: transform 0.15s ease, opacity 0.15s ease, background-color 0.2s ease;
        isolation: isolate;
        position: relative;
    }
    .tile:hover {
        background-color: var(--color-hover);
    }
    .dragging {
        opacity: 0.3;
        cursor: grabbing;
    }

    .removeBtn {
        opacity: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 22px;
        height: 22px;
        border-radius: 50%;
        border: none;
        background-color: rgba(255, 255, 255, 0.1);
        color: var(--color-navbar-label);
        margin-right: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
        flex-shrink: 0; 
    }

    .tile:hover .removeBtn {
        opacity: 1;
    }

    .removeBtn:hover {
        background-color: #ff453a; 
        color: white;
        transform: scale(1.1);
    }

    .removeBtn:active {
        transform: scale(0.95);
    }

    .albumCover {
        height: 35px;
        width: 35px;
        border-radius: 5px;
        border-color: var(--color-divider);
        border-width: 1px;
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1c1c1e;
        color: #3d3c41;
    }

    .title{
        margin-left: 10px;
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        width: 100%;
    }

    .trackTitle{
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        width: 100%;
        text-align: left;
    }

    .albumInfo {
        font-size: 12px;
        color: var(--color-navbar-label);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        width: 100%;
        text-align: left;
    }
    
</style>