<script lang="ts">
    import { commands } from "$lib/bindings";
    import { router } from "$lib/router.svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import PlusIcon from "$lib/icons/plus-icon.svelte";
    import CrossIcon from "$lib/icons/cross-icon.svelte";

    let dirs = $state<string[]>([]);
    let canStart = $derived(dirs.length > 0);

    async function pickFolder() {
        const selected = await open({ directory: true, multiple: false });
        if (selected && typeof selected === "string") {
            await commands.addDirectory(selected);
            dirs = await commands.getDirectories();
        }
    }

    async function finish() {
        await commands.index();
        router.closeOnboarding();
    }
</script>

{#if router.showOnboarding}
    <div class="overlay">
        <div class="card">
            <div class="top">
                <div class="logo">♫</div>
                <h1>Welcome to Aurex</h1>
                <p class="subtitle">Add your music folders to get started.</p>
            </div>

            <div class="folderSection">
                {#if dirs.length === 0}
                    <div class="emptyFolders">No folders added yet</div>
                {:else}
                    <div class="folderList">
                        {#each dirs as dir}
                            <div class="folderRow">
                                <span class="folderPath">{dir}</span>
                                <button
                                    class="removeBtn"
                                    onclick={async () => {
                                        await commands.removeDirectory(dir);
                                        dirs = await commands.getDirectories();
                                    }}
                                >
                                    <CrossIcon size={12} />
                                </button>
                            </div>
                        {/each}
                    </div>
                {/if}

                <button class="addBtn" onclick={pickFolder}>
                    <PlusIcon size={16} />
                    <span>Add Music Folder</span>
                </button>
            </div>

            <div class="actions">
                <button
                    class="startBtn"
                    class:disabled={!canStart}
                    disabled={!canStart}
                    onclick={finish}
                >
                    Get Started
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        z-index: 1000;
        background: rgba(0, 0, 0, 0.85);
        backdrop-filter: blur(20px);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .card {
        background: rgb(31, 31, 31);
        border: 1px solid var(--color-divider);
        border-radius: 16px;
        width: 480px;
        box-shadow: 0 30px 60px rgba(0, 0, 0, 0.6);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .top {
        padding: 40px 40px 28px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
        border-bottom: 1px solid var(--color-divider);
    }

    .logo {
        font-size: 48px;
        line-height: 1;
        color: var(--color-accent);
        margin-bottom: 4px;
    }

    h1 {
        font-size: 22px;
        font-weight: 700;
        margin: 0;
        text-align: center;
    }

    .subtitle {
        font-size: 14px;
        color: var(--color-navbar-label);
        margin: 0;
        text-align: center;
    }

    .folderSection {
        padding: 24px 32px;
        display: flex;
        flex-direction: column;
        gap: 12px;
        min-height: 120px;
    }

    .emptyFolders {
        font-size: 13px;
        color: var(--color-navbar-label);
        text-align: center;
        padding: 16px 0;
    }

    .folderList {
        display: flex;
        flex-direction: column;
        gap: 6px;
        max-height: 180px;
        overflow-y: auto;
    }

    .folderRow {
        display: flex;
        align-items: center;
        gap: 8px;
        background: var(--color-hover);
        border: 1px solid var(--color-divider);
        border-radius: 8px;
        padding: 8px 10px;
    }

    .folderPath {
        flex: 1;
        font-size: 12px;
        color: rgba(255, 255, 255, 0.8);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        font-family: monospace;
    }

    .removeBtn {
        flex-shrink: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 22px;
        height: 22px;
        border-radius: 50%;
        color: var(--color-navbar-label);
        transition: background-color 0.15s ease, color 0.15s ease;
    }

    .removeBtn:hover {
        background: rgba(250, 46, 73, 0.2);
        color: var(--color-accent);
    }

    .addBtn {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 10px;
        border: 1px dashed rgba(255, 255, 255, 0.2);
        border-radius: 8px;
        font-size: 14px;
        color: rgba(255, 255, 255, 0.6);
        transition: border-color 0.15s ease, color 0.15s ease, background-color 0.15s ease;
    }

    .addBtn:hover {
        border-color: var(--color-accent);
        color: var(--color-accent);
        background: rgba(250, 46, 73, 0.05);
    }

    .actions {
        padding: 20px 32px 28px;
        display: flex;
        justify-content: center;
        border-top: 1px solid var(--color-divider);
    }

    .startBtn {
        padding: 10px 48px;
        background: var(--color-accent);
        border-radius: 8px;
        font-size: 15px;
        font-weight: 600;
        color: white;
        position: relative;
        overflow: hidden;
        isolation: isolate;
        transition: opacity 0.15s ease;
    }

    .startBtn::after {
        content: "";
        position: absolute;
        inset: 0;
        background: rgba(255, 255, 255, 0.2);
        opacity: 0;
        transition: opacity 0.15s ease;
    }

    .startBtn:hover:not(.disabled)::after {
        opacity: 1;
    }

    .startBtn.disabled {
        opacity: 0.35;
        pointer-events: none;
    }
</style>