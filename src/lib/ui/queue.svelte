<script lang="ts">
    import { audioPlayer } from "$lib/player.svelte";
    import { VList } from "virtua/svelte";
    import QueueTile from "./queue-tile.svelte";
    import type { FullTrack } from "$lib/bindings";
    import { monitorForElements } from '@atlaskit/pragmatic-drag-and-drop/element/adapter';
    import { onMount } from 'svelte';
    import { commands } from "$lib/bindings";

    let draggedFrom = $state<number | null>(null);
    let draggedTo = $state<number | null>(null);

    let empty = $derived(audioPlayer.queue.length === 0);

    let draggedToRef = { current: null as number | null };

    $effect(() => {
        draggedToRef.current = draggedTo;
    });

    onMount(() => {
        const cleanup = monitorForElements({
            onDrop({ source }) {
                const from = source.data.index as number;
                const to = draggedToRef.current;  // read from ref not closure
                
                draggedFrom = null;
                draggedTo = null;
                draggedToRef.current = null;

                if (to === null || from === to) return;

                const newQueue = [...audioPlayer.queue];
                const [moved] = newQueue.splice(from, 1);
                newQueue.splice(to, 0, moved);
                audioPlayer.queue = newQueue;

                commands.changeQueueIndex(from, to);
            },
            onDragStart({ source }) {
                draggedFrom = source.data.index as number;
                draggedTo = source.data.index as number;
                draggedToRef.current = source.data.index as number;
            }
        });
        return cleanup;
    });
</script>

<div role="list" class="queueContainer" onpointermove={(e) => {
    if (draggedFrom === null) return;
    const el = document.elementFromPoint(e.clientX, e.clientY)?.closest('.tile');
    if (!el) return;
    const idx = Number(el.getAttribute('data-index'));
    if (!isNaN(idx)) {
        draggedTo = idx;
        draggedToRef.current = idx;
    }
}}>
    Queue

    <div class="listWrapper">

        <div class="h-[10px]"></div>

        {#if empty}
            <div
                style="
                    flex: 1;
                    color: var(--color-navbar-label);
                    font-weight: 500;
                    font-size: 13px;
                    text-align: center;
                "
            >There's no music in the queue.</div>

        {:else}
            <VList data={audioPlayer.queue} style='height: 100%' getKey={(track: FullTrack) => track.track.id}>
                {#snippet children(track: FullTrack, index: number)}
                    <QueueTile
                        track={track}
                        index={index}
                        bind:draggedFrom
                        bind:draggedTo
                    />
                {/snippet}
            </VList>
        {/if}
    </div>
</div>

<style>
    .queueContainer {
        padding: 15px;
        flex: 1;
        overflow: hidden;
        height: 100%;
        width: 100%;
    }

    .listWrapper {
        height: 100%;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>