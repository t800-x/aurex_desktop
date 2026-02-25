<script lang="ts">
    import SongsIcon from "$lib/icons/songs-icon.svelte";
    import AlbumsIcon from "$lib/icons/albums-icon.svelte";
    import ArtistsIcon from "$lib/icons/artists-icon.svelte";
    import NavbarItem from "./navbar-item.svelte";
    import NavbarLabel from "./navbar-label.svelte";
    import { RightPaneContent, router, Section } from "$lib/router.svelte";
    import { commands, type Playlist } from "$lib/bindings";
    import { onMount } from "svelte";
    import PlaylistIcon from "$lib/icons/playlist-icon.svelte";


    const width = 268;

    let playlists: Playlist[] = $state([]);
    let playlistsEmpty = $derived(playlists.length === 0);

    onMount(async () => playlists = await commands.getAllPlaylists());

    // Menu items.
    const items = [
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
    <NavbarLabel text={'Library'} />
    {#each items as item}
        <NavbarItem Icon={item.icon} section={item.section} text={item.title}/>
    {/each}

    <div style="height: 20px"></div>

    {#if !playlistsEmpty}
        <NavbarLabel text={"Playlists"} />

        {#each playlists as playlist}
            <NavbarItem Icon={PlaylistIcon} section={`pl-${playlist.id}` as unknown as Section} text={playlist.name} />
        {/each}
    {/if}
</div>
 

<style> 

    .navbar {
        width: 268px;
        height: 100vh;
        display: flex;
        flex-direction: column;
        background-color: transparent;
        align-items: center;
        justify-content: start;
        padding: 10px;
        gap: 5px;
        padding-top: 20px;
        overflow-x: hidden;
        overflow-y: auto;
    }

</style>