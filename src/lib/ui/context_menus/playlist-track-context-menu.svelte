<script lang="ts">
    import { commands, type FullTrack, type Playlist } from "$lib/bindings";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import TrackContextMenu from "./track-context-menu.svelte";

        let {
        track,
        playList,
        playlist
    } : {
        track: FullTrack;
        playList: () => void;
        playlist: Playlist | null;
    } = $props();

</script>

<TrackContextMenu track={track} playList={playList}>
    {#snippet extra()}
        <ContextMenu.Separator />
        <ContextMenu.Item variant="destructive" onclick={() => commands.removeFromPlaylist(Number(track.track.id), Number(track.playlist_position), Number(playlist?.id))}>
            Remove from Playlist
        </ContextMenu.Item>
    {/snippet}
</TrackContextMenu>