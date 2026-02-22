<script lang="ts">
  let { class: className = '' } = $props();
</script>

<div class="header {className}">
    <div class="title">
        Albums
    </div>

</div>

<style>
    .header {
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
        display: flex;
        justify-content: center;
        align-items: center;
        font-weight: 700;
        font-size: 20px;
    }

</style>