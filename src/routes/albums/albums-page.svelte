<script lang="ts">
    import { router, Section } from "$lib/router.svelte";
    import StackView from "$lib/ui/stack-view.svelte";
    import AlbumsPageInner from "./albums-page-inner.svelte";

    const section = Section.albums;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page albumsPage">

    <StackView initialComponent={AlbumsPageInner}></StackView>

</div>

<style>
    .albumsPage {
        height: 100%;
        width: 100%;
    }
</style>