<script lang="ts">
    import WindowControls from "./window-controls.svelte";
    import icon from "../../../assets/icons/icon.png";

</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="titlebar vibranceBlur" ondblclick={() => appWindow.toggleMaximize()}>
    <img class="appIcon" src={icon} alt=""/>
    <div class="spacer"></div>
    <WindowControls />
</div>

<style>
    .spacer {
        flex: 1;
    }

    .appIcon {
        margin: 10px;
        margin-top: 7px;
        height: 20px;
        width: 20px;
        border-width: 1px;
        border-color: var(--color-divider);
        border-radius: 20%;
    }

    .titlebar {
        height: 30px;
        width: 100%;
        display: flex;
        position: absolute;
        top: 0;
        -webkit-app-region: drag;
        app-region: drag;
        z-index: 1000;

        justify-content: flex-end;
    }
</style>