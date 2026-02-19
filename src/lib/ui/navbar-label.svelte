<script lang="ts">
    let {
        text
    }: {
        text: string;
    } = $props();
</script>

<span class="label">
    {text}
</span>

<style>
    .label {
        padding-left: 10px;
        color: rgb(176, 176, 177);
        font-size: 12px;
        font-weight: 500;
        align-self: flex-start;
    }
</style>