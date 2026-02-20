<script lang="ts">

    import { audioPlayer } from "$lib/player.svelte";
    import { VList } from "virtua/svelte";
    import QueueTile from "./queue-tile.svelte";
    import type { FullTrack } from "$lib/bindings";


</script>

<div class="queueContainer">
    Queue

    <VList data={audioPlayer.queue} style='height: 100%' getKey={(track: FullTrack) => track.track.id}>
        {#snippet children(track: FullTrack, index: number)}

                <QueueTile track={track} index={index}/>

                
        {/snippet}
    </VList>

</div>

<style>
    .queueContainer {
        flex: 1;
        overflow: hidden;
        height: 100%;
        width: 100%;
    }
</style>