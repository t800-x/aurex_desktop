<script lang="ts">
    import type { Artist } from "$lib/bindings";
    import PersonIcon from "$lib/icons/person-icon.svelte";
    import SearchSectionHeader from "./search-section-header.svelte";
    import { router } from "$lib/router.svelte";
    import { VList } from "virtua/svelte";

    let { artists }: { artists: Artist[] } = $props();

    // avatar 64px + gap 7px + name ~16px + padding 10px top/bottom = ~107px
    const listHeight = 107;
    const ITEM_WIDTH = 80 + 16; // chip width + gap
</script>

<section class="section">
    <SearchSectionHeader title="Artists" count={artists.length} />

    <VList
        data={artists}
        horizontal
        style="height: {listHeight}px; padding: 4px 4px 6px;"
        getKey={(artist: Artist) => artist.id}
    >
        {#snippet children(artist: Artist)}
            <div class="item" style="width: {ITEM_WIDTH}px;">
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="chip" onclick={() => router.goToArtist(Number(artist.id))}>
                    <div class="avatar">
                        <PersonIcon size={28} />
                    </div>
                    <span class="name">{artist.name}</span>
                </div>
            </div>
        {/snippet}
    </VList>
</section>

<style>
    .section {
        margin-bottom: 28px;
    }

    .item {
        display: flex;
        box-sizing: border-box;
    }

    .chip {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 7px;
        width: 80px;
        flex-shrink: 0;
        cursor: default;
    }

    .avatar {
        width: 64px;
        height: 64px;
        border-radius: 50%;
        background: rgba(255,255,255,0.06);
        border: 1px solid rgba(255,255,255,0.1);
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--color-navbar-label);
        transition: background-color 0.15s ease, transform 0.15s ease;
    }

    .chip:hover .avatar {
        background: rgba(255,255,255,0.12);
        transform: scale(1.05);
    }

    .name {
        font-size: 12px;
        font-weight: 500;
        text-align: center;
        width: 100%;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>