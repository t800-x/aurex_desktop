<script lang="ts">
    import LyricsIcon from "$lib/icons/lyrics-icon.svelte";
    import QueueIcon from "$lib/icons/queue-icon.svelte";
    import { RightPaneContent } from "$lib/router.svelte";
    import PaneControlButton from "./pane-control-button.svelte";


</script>

<div class="paneControls">
    <PaneControlButton Icon={LyricsIcon} content={RightPaneContent.lyrics}/>
    <PaneControlButton Icon={QueueIcon} content={RightPaneContent.queue}/>
</div>

<style>

    .paneControls {
        display: flex;
        flex: 20;
        align-items: center;
        justify-content: center;
        gap: 2px;
    }

</style>