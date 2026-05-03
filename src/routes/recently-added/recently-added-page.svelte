<script lang="ts">
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import StackView from "$lib/ui/views/stack-view.svelte";
    import RecentlyAddedPageInner from "./recently-added-page-inner.svelte";

    const section = Section.recentlyAdded;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page recentlyAdded">
    <StackView initialComponent={RecentlyAddedPageInner} />
</div>

<style>
    .recentlyAdded {
        height: 100%;
        width: 100%;
    }
</style>