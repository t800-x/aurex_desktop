<script lang="ts">
    import type { Playlist } from "$lib/bindings";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import PlaylistIcon from "$lib/icons/playlist-icon.svelte";
    import PlayIcon from "$lib/icons/play-icon.svelte";
    import SearchSectionHeader from "./search-section-header.svelte";
    import { VList } from "virtua/svelte";

    let {
        playlists,
        onPlay,
    }: {
        playlists: Playlist[];
        onPlay: (p: Playlist) => void;
    } = $props();

    const ITEM_HEIGHT = 50; // 36px art + 6px padding * 2 + 2px gap
    const MAX_VISIBLE = 8;

    let listHeight = $derived(Math.min(playlists.length, MAX_VISIBLE) * ITEM_HEIGHT);
</script>

<section class="section">
    <SearchSectionHeader title="Playlists" count={playlists.length} />

    <VList
        data={playlists}
        style="height: {listHeight}px;"
        getKey={(p: Playlist) => p.id}
    >
        {#snippet children(playlist: Playlist)}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div class="tile" onclick={() => onPlay(playlist)}>
                {#if playlist.cover_path}
                    <img
                        class="art"
                        src={convertFileSrc(playlist.cover_path)}
                        alt=""
                    />
                {:else}
                    <div class="art noArt">
                        <PlaylistIcon size={18} />
                    </div>
                {/if}

                <span class="name">{playlist.name}</span>
                <div class="playIcon">
                    <PlayIcon size={14} />
                </div>
            </div>
        {/snippet}
    </VList>
</section>

<style>
    .section {
        margin-bottom: 28px;
    }

    .tile {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 6px 8px;
        border-radius: 7px;
        cursor: default;
        transition: background-color 0.15s ease;
        box-sizing: border-box;
    }

    .tile:hover {
        background: var(--color-hover);
    }

    .tile:hover .playIcon {
        opacity: 1;
    }

    .art {
        width: 36px;
        height: 36px;
        border-radius: 5px;
        border: 1px solid var(--color-divider);
        box-shadow: 0 3px 8px rgba(0,0,0,0.3);
        object-fit: cover;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .art.noArt {
        background: #1c1c1e;
        color: var(--color-accent);
    }

    .name {
        flex: 1;
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .playIcon {
        opacity: 0;
        color: var(--color-navbar-label);
        transition: opacity 0.15s ease;
        flex-shrink: 0;
        display: flex;
        align-items: center;
    }
</style>