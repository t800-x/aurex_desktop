<script lang="ts">
    import { router, type RightPaneContent } from "$lib/router.svelte";

    
    let {
        Icon,
        content
    } : {
        Icon: any,
        content: RightPaneContent
    } = $props();

    let active = $derived(router.rightPaneContent === content);
</script>

<button onclick={() => router.setRightPaneContent(content)} class="paneButton" class:active={active}>
    <Icon size={20}/>
</button>

<style>
    .paneButton {
        display: flex;
        justify-content: center;
        align-items: center;
        padding-top: 3px;
        padding-bottom: 3px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 6px;

        transition: background-color 0.2s ease 0s;
        transition: box-shadow 0.2s ease 0s;
    }

    .paneButton:hover {
        background-color: var(--color-hover);
    }

    .paneButton.active{
        background-color: var(--color-accent);
        box-shadow: 0 5px 10px 2px rgba(0, 0, 0, 0.3);
    }
</style>