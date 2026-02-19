<script lang="ts">
    import SongsIcon from "$lib/icons/songs-icon.svelte";
    import AlbumsIcon from "$lib/icons/albums-icon.svelte";
    import ArtistsIcon from "$lib/icons/artists-icon.svelte";
    import NavbarItem from "./navbar-item.svelte";
    import NavbarLabel from "./navbar-label.svelte";
    import { Section } from "$lib/router.svelte";


    const width = 268;

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