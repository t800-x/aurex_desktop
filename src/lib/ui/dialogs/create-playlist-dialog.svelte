<script lang="ts">
  import { commands } from "$lib/bindings";
  import Input from "$lib/components/ui/input/input.svelte";
  import { router } from "$lib/router.svelte";
  let dialog: HTMLDialogElement;

  $effect(() => {
    if (router.isCreatePlaylistDialogOpen) dialog.showModal();
    else dialog.close();
  });

  async function addToPlaylist() {
    const trackContext = router.trackToAddAfterCreating;
    if (trackContext === null || trackContext.track === null) return;

    let p_id = await commands.getPlIdByName(name);
    if (p_id !== null) {
      await commands.addToPlaylist(Number(trackContext.track.id), null, p_id);
    }

    router.trackToAddAfterCreating = null;
  }

  let name = $state("");
</script>

<dialog
  bind:this={dialog}
  onclose={() => router.closeCreatePlaylistDialog()}
  onclick={(e) => {
    if (e.target === dialog) router.closeCreatePlaylistDialog();
  }}
>
  <div
    style="
        padding: 24px;
    "
  >
    <span
      style="
                font-weight: bold;
                font-size: 20px;
            ">Create Playlist</span
    >

    <div
      style="
            margin-top: 12px;
            width: 100%
        "
    >
      <Input type={"text"} placeholder={"Playlist Name"} bind:value={name} />
    </div>
  </div>

  <div
    style="
        padding: 20px;
        background-color: rgba(0, 0, 0, 0.2);
        width: 100%;

        display: flex;
        gap: 10px;
    "
  >
    <div class="bottomButtonContainer">
      <button
        class:disabledBtn={name.length === 0}
        class="bottomButton createBtn"
        onclick={async () => {
          await commands.createPlaylist(name);
          await addToPlaylist().catch(console.error);
          router.closeCreatePlaylistDialog();
        }}>Create</button
      >
    </div>

    <div class="bottomButtonContainer">
      <button
        class="bottomButton"
        onclick={() => router.closeCreatePlaylistDialog()}>Cancel</button
      >
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
    padding-left: 20px;
    padding-right: 20px;

    transition: background-color 0.15s ease;
  }

  .bottomButton:hover {
    background-color: var(--color-double-hover);
  }

  .createBtn {
    background-color: var(--color-accent);
    position: relative;
    overflow: hidden;
    isolation: isolate;
  }

  .createBtn::after {
    content: "";
    position: absolute;
    inset: 0;
    background-color: rgba(255, 255, 255, 0.22);
    opacity: 0;
    transition: opacity 0.15s ease;
    pointer-events: none;
  }

  .createBtn:hover {
    background-color: var(--color-accent);
  }

  .createBtn:hover::after {
    opacity: 1;
  }

  .disabledBtn {
    background-color: color-mix(in srgb, var(--color-accent), black 40%);
    pointer-events: none;
  }
</style>
