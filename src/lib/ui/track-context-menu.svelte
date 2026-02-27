<script lang="ts">
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import { loadAndPlay } from "$lib/helpers";
    import { commands, type FullTrack } from "$lib/bindings";

    let {
        track,
        playList
    } : {
        track: FullTrack;
        playList: () => void;
    } = $props();
</script>

<ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
    <ContextMenu.Item onclick={async () => {
        await loadAndPlay(track);
    }}>
        Play
    </ContextMenu.Item>
    
    <ContextMenu.Item
        onclick={() => playList()}
    >
        Play from here
    </ContextMenu.Item>

    <ContextMenu.Item onclick={async () => commands.playNext(track)}>Play Next</ContextMenu.Item>
    <ContextMenu.Item onclick={async () => commands.addToQueue(track)}>Add to Queue</ContextMenu.Item>
</ContextMenu.Content>