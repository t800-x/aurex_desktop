<script lang="ts">
    import type { Playlist } from "$lib/bindings";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import { router } from "$lib/router.svelte";

    let {
        playlist,
    } : {
        playlist: Playlist;
    } = $props();
</script>

<ContextMenu.Content onInteractOutside={(e) => e.stopPropagation()}>
    <ContextMenu.Item variant="destructive" onclick={() => router.openDeletePlaylistDialog(playlist)}>Delete Playlist</ContextMenu.Item>
</ContextMenu.Content>