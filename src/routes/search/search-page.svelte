<script lang="ts">
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import StackView from "$lib/ui/stack-view.svelte";
    import SearchPageInner from "./search-page-inner.svelte";

    const section = Section.search;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? 'none' : 'flex');
</script>

<div style:display={displayMode} class:hidden = {hidden} class="page searchPage">
    <StackView initialComponent={SearchPageInner} />
</div>

<style>
    .searchPage {
        height: 100%;
        width: 100%;
    }
</style>