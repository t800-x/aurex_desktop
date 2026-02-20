<script lang="ts">
  let { class: className = '' } = $props();
</script>

<div class="header {className}">
    <div class="title">
        Songs
    </div>

    <div class="listInfo">
        <span class="infoLabel" style:flex={4}>Title</span>
        <span class="infoLabel" style:flex={1}>Time</span>
        <span class="infoLabel" style:flex={3}>Artist</span>
        <span class="infoLabel" style:flex={3}>Album</span>
    </div>
</div>

<style>
    .header {
        height: 100px;
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
        height: 50%;
        width: 100%;
        border-bottom: 1px solid var(--color-divider);
        display: flex;
        justify-content: center;
        align-items: center;
        font-weight: 700;
        font-size: 20px;
    }

    .listInfo {
        display: flex;
        height: 50%;
        width: 100%;
        padding-left: 13px;
        padding-right: 13px;
        justify-content: center;
        align-items: center;
    }

    .infoLabel {
        font-size: 14px;
        font-weight: 500;
    }
</style>