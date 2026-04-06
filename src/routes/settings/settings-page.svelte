<script lang="ts">
    import { commands } from "$lib/bindings";
    import { router, Section } from "$lib/router.svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import PlusIcon from "$lib/icons/plus-icon.svelte";
    import CrossIcon from "$lib/icons/cross-icon.svelte";

    const section = Section.settings;
    let hidden = $derived(router.current !== section);
    let displayMode = $derived(hidden ? "none" : "flex");

    let dirs = $state<string[]>([]);
    let indexing = $state(false);

    onMount(async () => {
        dirs = await commands.getDirectories();

        await listen<void>("directories-changed", async () => {
            dirs = await commands.getDirectories();
        });

        await listen<void>("indexing-done", () => {
            indexing = false;
        });
    });

    async function pickFolder() {
        const selected = await open({ directory: true, multiple: false });
        if (selected && typeof selected === "string") {
            await commands.addDirectory(selected);
        }
    }

    async function rescan() {
        indexing = true;
        await commands.index();
    }
</script>

<div style:display={displayMode} class:hidden class="page settingsPage">
    <div class="inner">
        <h1 class="pageTitle">Settings</h1>

        <section class="section">
            <div class="sectionHeader">
                <div class="sectionMeta">
                    <span class="sectionTitle">Music Library</span>
                    <span class="sectionDesc">Aurex scans these folders for audio files.</span>
                </div>
                <div class="sectionActions">
                    <button class="scanBtn" class:spinning={indexing} onclick={rescan} disabled={indexing}>
                        {indexing ? "Scanning…" : "Rescan Library"}
                    </button>
                </div>
            </div>

            <div class="dirList">
                {#each dirs as dir}
                    <div class="dirRow">
                        <div class="dirIcon">♫</div>
                        <span class="dirPath">{dir}</span>
                        <button
                            class="removeBtn"
                            onclick={() => commands.removeDirectory(dir)}
                            title="Remove folder"
                        >
                            <CrossIcon size={13} />
                        </button>
                    </div>
                {:else}
                    <div class="emptyState">
                        No music folders configured.
                    </div>
                {/each}

                <button class="addRow" onclick={pickFolder}>
                    <div class="addIcon">
                        <PlusIcon size={16} />
                    </div>
                    <span>Add Music Folder</span>
                </button>
            </div>
        </section>
    </div>
</div>

<style>
    .settingsPage {
        height: 100%;
        width: 100%;
        overflow-y: auto;
        flex-direction: column;
        align-items: center;
    }

    .inner {
        width: 100%;
        max-width: 700px;
        padding: 40px 40px 100px;
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .pageTitle {
        font-size: 28px;
        font-weight: 700;
        margin: 0;
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .sectionHeader {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
    }

    .sectionMeta {
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    .sectionTitle {
        font-size: 13px;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        color: var(--color-navbar-label);
    }

    .sectionDesc {
        font-size: 13px;
        color: rgba(255, 255, 255, 0.35);
    }

    .sectionActions {
        display: flex;
        gap: 8px;
    }

    .scanBtn {
        font-size: 13px;
        font-weight: 500;
        padding: 6px 14px;
        border-radius: 7px;
        border: 1px solid var(--color-divider);
        background: var(--color-hover);
        color: rgba(255, 255, 255, 0.7);
        transition: background-color 0.15s ease, color 0.15s ease;
    }

    .scanBtn:hover:not(:disabled) {
        background: var(--color-double-hover);
        color: white;
    }

    .scanBtn:disabled {
        opacity: 0.5;
        pointer-events: none;
    }

    .dirList {
        border: 1px solid var(--color-divider);
        border-radius: 10px;
        overflow: hidden;
        background: rgba(255, 255, 255, 0.02);
    }

    .dirRow {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px 14px;
        border-bottom: 1px solid var(--color-divider);
        transition: background-color 0.15s ease;
    }

    .dirRow:hover {
        background: var(--color-hover);
    }

    .dirIcon {
        width: 32px;
        height: 32px;
        border-radius: 7px;
        background: rgba(250, 46, 73, 0.12);
        color: var(--color-accent);
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
        flex-shrink: 0;
    }

    .dirPath {
        flex: 1;
        font-size: 13px;
        font-family: monospace;
        color: rgba(255, 255, 255, 0.8);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .removeBtn {
        flex-shrink: 0;
        width: 26px;
        height: 26px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--color-navbar-label);
        opacity: 0;
        transition: background-color 0.15s ease, color 0.15s ease, opacity 0.15s ease;
    }

    .dirRow:hover .removeBtn {
        opacity: 1;
    }

    .removeBtn:hover {
        background: rgba(250, 46, 73, 0.2);
        color: var(--color-accent);
    }

    .emptyState {
        padding: 20px;
        text-align: center;
        font-size: 13px;
        color: var(--color-navbar-label);
        border-bottom: 1px solid var(--color-divider);
    }

    .addRow {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 11px 14px;
        width: 100%;
        font-size: 14px;
        color: rgba(255, 255, 255, 0.5);
        transition: background-color 0.15s ease, color 0.15s ease;
    }

    .addRow:hover {
        background: var(--color-hover);
        color: rgba(255, 255, 255, 0.9);
    }

    .addIcon {
        width: 32px;
        height: 32px;
        border-radius: 7px;
        border: 1px dashed rgba(255, 255, 255, 0.2);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition: border-color 0.15s ease, color 0.15s ease;
    }

    .addRow:hover .addIcon {
        border-color: var(--color-accent);
        color: var(--color-accent);
    }
</style>