<script>
    import { router } from "$lib/router.svelte";
    import { Section } from "$lib/router.svelte";
    import { onMount } from "svelte";

    const section = Section.albums;
    let hidden = $derived(router.current !== section);
    onMount(() => console.log("Hidden: " + hidden));
    $effect(() => console.log("Hidden: " + hidden));
</script>

<div class:hidden = {hidden} class="page">
    Albums
</div>