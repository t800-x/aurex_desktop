<script lang="ts">
  import "../app.css";
  import "../colors.css";
  import "@fontsource-variable/inter";
  import Navbar from "$lib/ui/navbar.svelte";
  import SongsPage from "./songs/songs-page.svelte";
  import AlbumsPage from "./albums/albums-page.svelte";
  import ArtistsPage from "./artists/artists-page.svelte";
  import SearchPage from "./search/search-page.svelte";
  import NowPlaying from "$lib/ui/now-playing.svelte";
  import RightPane from "$lib/ui/right-pane.svelte";
  import PlaylistPage from "./playlist/playlist_page.svelte";
  import { router } from "$lib/router.svelte";
  import CreatePlaylistDialog from "$lib/ui/create-playlist-dialog.svelte";
  import DeletePlaylistDialog from "$lib/ui/delete-playlist-dialog.svelte";
  import RecentlyAddedPage from "./recently-added/recently-added-page.svelte";
    import { onMount } from "svelte";
    import { commands } from "$lib/bindings";

  let blocked = $derived(
    router.rightPaneContent !== null && router.rightPaneOverlaying,
  );

  onMount(() => commands.index());
</script>

<div class="appRoot dark">
  <Navbar />
  <div class="contentWrapper">
    <main class="mainContent" class:blocked>
      <RecentlyAddedPage />
      <SongsPage />
      <AlbumsPage />
      <ArtistsPage />
      <SearchPage />
      <NowPlaying />
      <PlaylistPage />
    </main>

    <RightPane />
  </div>

  <CreatePlaylistDialog />
  <DeletePlaylistDialog />
</div>

<style>
  .appRoot {
    background-color: #202020;
    height: 100vh;
    overflow: hidden;
    isolation: isolate;
    min-height: 100vh;
    color: var(--foreground);
    display: flex;
    flex-direction: row;
  }

  .contentWrapper {
    display: flex;
    flex: 1;
    overflow: hidden;
    height: 100%;
    position: relative;
  }

  .mainContent {
    display: flex;
    flex: 1;
    overflow: hidden;
    height: 100%;
    position: relative;
  }

  .mainContent.blocked {
    pointer-events: none;
    user-select: none;
  }

  :global(body) {
    font-family: "Inter Variable", sans-serif;
  }
</style>
