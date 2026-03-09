<script lang="ts">
    import { commands, type Playlist } from "$lib/bindings";
    import { router } from "$lib/router.svelte";

    let dialog: HTMLDialogElement;

    $effect(() => {
        if (router.isDeletePlaylistDialogOpen) dialog.showModal();
        else dialog.close();
    });
</script>

<dialog
    bind:this={dialog}
    onclose={() => router.closeDeletePlaylistDialog()}
    onclick={(e) => { if (e.target === dialog) router.closeDeletePlaylistDialog() }}
>
    <div style="padding: 24px;">
        <span style="font-weight: bold; font-size: 20px;">Delete Playlist</span>
        <p style="margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.6);">
            Are you sure you want to delete <strong style="color: white;">"{router.pendingDeletePlaylist?.name}"</strong>? This can't be undone.
        </p>
    </div>

    <div style="
        padding: 20px;
        background-color: rgba(0, 0, 0, 0.2);
        width: 100%;
        display: flex;
        gap: 10px;
    ">
        <div class="bottomButtonContainer">
            <button class="bottomButton deleteBtn" onclick={async () => {
                await commands.deletePlaylist(Number(router.pendingDeletePlaylist!.id));
                router.closeDeletePlaylistDialog();
            }}>Delete</button>
        </div>
        <div class="bottomButtonContainer">
            <button class="bottomButton" onclick={() => router.closeDeletePlaylistDialog()}>Cancel</button>
        </div>
    </div>
</dialog>

<style>
    dialog {
        border-radius: 8px;
        border-width: 1px;
        border-color: var(--color-divider);
        background: var(--color-header);
        color: white;
        position: fixed;
        backdrop-filter: blur(10px);
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        box-shadow: var(--shadow-floating);
        min-width: 320px;
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, 0.4);
        backdrop-filter: blur(5px);
    }
    .bottomButtonContainer {
        flex: 2;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .bottomButton {
        border-color: var(--color-divider);
        padding: 10px;
        background-color: var(--color-hover);
        color: white;
        font-size: 14px;
        border-width: 1px;
        border-radius: 6px;
        padding-left: 30px;
        padding-right: 30px;
        transition: background-color 0.15s ease;
    }
    .bottomButton:hover {
        background-color: var(--color-double-hover);
    }
    .deleteBtn {
        background-color: var(--color-accent);
        position: relative;
        overflow: hidden;
        isolation: isolate;
    }
    .deleteBtn::after {
        content: "";
        position: absolute;
        inset: 0;
        background-color: rgba(255, 255, 255, 0.22);
        opacity: 0;
        transition: opacity 0.15s ease;
        pointer-events: none;
    }
    .deleteBtn:hover {
        background-color: var(--color-accent);
    }
    .deleteBtn:hover::after {
        opacity: 1;
    }
</style>