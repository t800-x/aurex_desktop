<script lang="ts">
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import { loadAndPlay } from "$lib/helpers";
    import { commands, type FullTrack, type Playlist } from "$lib/bindings";
    import { onMount } from "svelte";
    import type { Snippet } from "svelte";
    import { router } from "$lib/router.svelte";
    import { listen } from "@tauri-apps/api/event";

    let {
        track,
        index,
        lastIndex,
        extra,
    } : {
        track: FullTrack;
        index: number;
        lastIndex: number;
        extra?: Snippet;
    } = $props();

    let playlists = $state<Playlist[]>([]);

    onMount(async () => {
        playlists = await commands.getAllPlaylists();

        await listen<void>('playlists-changed', async (event) => {
            playlists = await commands.getAllPlaylists();
        });
    });
</script>

<ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
    <ContextMenu.Item onclick={async () => await loadAndPlay(track)}>
        Play
    </ContextMenu.Item>
    
    <ContextMenu.Item onclick={() => commands.changeQueueIndex(index, 0)}>Move to Top</ContextMenu.Item>
    <ContextMenu.Item onclick={() => commands.changeQueueIndex(index, lastIndex)}>Move to Bottom</ContextMenu.Item>

    <ContextMenu.Sub>
        <ContextMenu.SubTrigger>Add to Playlist</ContextMenu.SubTrigger>
        <ContextMenu.Portal>
            <ContextMenu.SubContent>
                <ContextMenu.Item onclick={() => router.openCreatePlaylistDialog(track)}>New Playlist</ContextMenu.Item>
                <ContextMenu.Separator />
                {#each playlists as playlist}
                    <ContextMenu.Item onclick={() => commands.addToPlaylist(Number(track.track.id), null, Number(playlist.id))}>
                        {playlist.name}
                    </ContextMenu.Item>
                {/each}
            </ContextMenu.SubContent>
        </ContextMenu.Portal>
    </ContextMenu.Sub>

    <ContextMenu.Separator />

    <ContextMenu.Item onclick={() => commands.removeFromQueue(index)} variant="destructive">Remove from Queue</ContextMenu.Item>
    {@render extra?.()}
</ContextMenu.Content>