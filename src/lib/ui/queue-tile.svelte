<script lang="ts" >
    import type { FullTrack } from "$lib/bindings";
    import DoubleNoteIcon from "$lib/icons/double-note-icon.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { draggable, dropTargetForElements } from '@atlaskit/pragmatic-drag-and-drop/element/adapter';
    import { onMount } from 'svelte';
    import { commands } from "$lib/bindings";

    let {
        track,
        index
    }: {
        track: FullTrack;
        index: number;
    } = $props();

    let el!: HTMLElement;

    let isDraggedOver = $state(false);
    let isDragging = $state(false);

    // svelte-ignore state_referenced_locally
        let coverSrc = convertFileSrc(track.album_art ?? "");

    // svelte-ignore state_referenced_locally
        let trackTitle = track.track.title ?? "Unknown";

    // svelte-ignore state_referenced_locally
        let albumInfo = `${track.artist_name} - ${track.album_title}`;

    // svelte-ignore state_referenced_locally
        let currentIndex = index;

    $effect(() => {
        currentIndex = index;
    });

    onMount(() => {
        const cleanup = [
            draggable({
                element: el,
                getInitialData: () => ({ index: currentIndex }),
                onDragStart: () => isDragging = true,
                onDrop: () => isDragging = false,
            }),
            dropTargetForElements({
                element: el,
                getData: () => ({ index: currentIndex }),
                onDragEnter: () => isDraggedOver = true,
                onDragLeave: () => isDraggedOver = false,
                onDrop({ source }) {
                    isDraggedOver = false;
                    const from = source.data.index as number;
                    const to = currentIndex;
                    if (from !== to) {
                        commands.changeQueueIndex(from, to);
                    }
                },
            }),
        ];
        return () => cleanup.forEach(fn => fn());
    });
</script>

<div bind:this={el} class="tile" class:dragging={isDragging} class:draggedOver={isDraggedOver}>

    {#if track.album_art !== null}
        <img class="albumCover" src={coverSrc} alt="">
    {:else}
        <div class="albumCover">
            <DoubleNoteIcon />
        </div>
    {/if}

    <div class="title">
        <div class="trackTitle">{trackTitle}</div>
        <div class="albumInfo">{albumInfo}</div>
    </div>
    
</div>

<style>
    .tile {
        width: 100%;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding: 5px;
        overflow: visible;
        cursor: default;
        border-radius: 7px;

        transition: background-color 0.2s ease 0s;
    }

    .tile:hover {
        background-color: var(--color-hover);
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
        font-weight: w500;
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

    .dragging {
        opacity: 0.4;
    }

    .draggedOver {
        transform: translateY(4px);
        border-top: 2px solid var(--color-accent);
    }
</style>