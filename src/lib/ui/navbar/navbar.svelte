<script lang="ts">
    import SongsIcon from "$lib/icons/songs-icon.svelte";
    import AlbumsIcon from "$lib/icons/albums-icon.svelte";
    import ArtistsIcon from "$lib/icons/artists-icon.svelte";
    import NavbarItem from "./navbar-item.svelte";
    import NavbarLabel from "./navbar-label.svelte";
    import { Section } from "$lib/router.svelte";
    import { commands, type Playlist } from "$lib/bindings";
    import { onMount } from "svelte";
    import PlaylistIcon from "$lib/icons/playlist-icon.svelte";
    import SearchIcon from "$lib/icons/search-icon.svelte";
    import NewPlaylistButton from "./new-playlist-button.svelte";
    import PlusIcon from "$lib/icons/plus-icon.svelte";
    import { listen } from "@tauri-apps/api/event";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    import PlaylistContextMenu from "../context_menus/playlist-context-menu.svelte";
    import RecentlyAddedIcon from "$lib/icons/recently-added-icon.svelte";
    import SettingsIcon from "$lib/icons/settings-icon.svelte";

    const width = 268;

    let playlists: Playlist[] = $state([]);
    let playlistsEmpty = $derived(playlists.length === 0);

    onMount(async () => {
        playlists = await commands.getAllPlaylists();
        await listen<void>('playlists-changed', async (event) => {
            playlists = await commands.getAllPlaylists();
        });

        await listen<void>('indexing-done', async (event) => {
            playlists = await commands.getAllPlaylists();
        });
    });

    // Menu items.
    const items = [
        {
            title: "Recently Added",
            section: Section.recentlyAdded,
            icon: RecentlyAddedIcon
        },
        {
            title: "Songs",
            section: Section.songs,
            icon: SongsIcon,
        },
        {
            title: "Albums",
            section: Section.albums,
            icon: AlbumsIcon,
        },
        {
            title: "Artists",
            section: Section.artists,
            icon: ArtistsIcon,
        },
    ];
</script>

<div class="navbar">
<NavbarItem text="Search" Icon={SearchIcon} section={Section.search} iconSize={22}/>

    <div style="height: 20px"></div>

    <NavbarLabel text={'Library'} />
    {#each items as item}
        {#if item.section === Section.recentlyAdded}
            <NavbarItem Icon={item.icon} section={item.section} text={item.title} iconSize={22}/>
        {:else}
            <NavbarItem Icon={item.icon} section={item.section} text={item.title}/>
        {/if}
    {/each}

    <div style="height: 20px"></div>

    <NavbarLabel text={"Playlists"} />
    <NewPlaylistButton text="Create Playlist" Icon={PlusIcon} />

    <div class="playlists">
        {#if !playlistsEmpty}
            {#each playlists as playlist}
                <ContextMenu.Root>
                    <ContextMenu.Trigger style="display: block; width: 100%;">
                        <NavbarItem Icon={PlaylistIcon} section={`pl-${playlist.id}` as unknown as Section} text={playlist.name} />
                    </ContextMenu.Trigger>

                    <PlaylistContextMenu playlist={playlist} />
                </ContextMenu.Root>
            {/each}
        {/if}
    </div>

    <div style="height: 1px; width: calc(100% - 20px); background: var(--color-divider); margin: 4px 0;"></div>
    <NavbarItem text="Settings" Icon={SettingsIcon} section={Section.settings} />
</div>
 

<style> 

    .navbar {
        width: 268px;
        height: 100%;
        display: flex;
        flex-direction: column;
        background-color: transparent;
        align-items: center;
        justify-content: start;
        padding: 10px;
        gap: 5px;
        padding-top: 50px;
        overflow-x: hidden;
        overflow-y: auto;
        flex-shrink: 0;
    }

    .playlists {
        flex: 1;
        display: flex;
        flex-direction: column;
        width: 100%;
        overflow-y: scroll;
        gap: 5px;
        align-items: center;
        justify-content: start;
    }
</style>