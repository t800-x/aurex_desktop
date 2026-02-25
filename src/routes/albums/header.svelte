<script lang="ts">
    import { Input } from "$lib/components/ui/input";

    let {
        onchanged,
        title,
    } : {
        onchanged: (searchterm: string) => void;
        title: string
    } = $props();

  let searchTerms = $state("");
  $effect(() => {
    console.log(searchTerms);
    onchanged(searchTerms);
  });
</script>

<div class="header">
    <div class="title">
        <span style="grid-column: 2; justify-self: center;">{title}</span>

        <div style="display: inline-block; grid-column: 3; justify-self: end; margin-right: 4px;">
            <Input type="text" bind:value={searchTerms} placeholder={'Filter'} />
        </div>
    </div>

</div>

<style>
    .header {
        align-self: flex-start;
        height: 50px;
        flex-shrink: 0;
        width: calc(100% - 8px);
        border-bottom: 1px solid var(--color-divider);
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        background-color: var(--color-header);
        position: absolute;
        top: 0;
        z-index: 100;
        backdrop-filter: blur(12px);
    }

    .title {
        height: 100%;
        width: 100%;
        font-weight: 700;
        font-size: 20px;
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
    }

</style>