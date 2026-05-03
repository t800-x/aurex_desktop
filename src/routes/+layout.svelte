<script lang="ts">
  import "../app.css";
  import "../colors.css";
  import "@fontsource-variable/inter";
  import Navbar from "$lib/ui/navbar/navbar.svelte";
  import SongsPage from "./songs/songs-page.svelte";
  import AlbumsPage from "./albums/albums-page.svelte";
  import ArtistsPage from "./artists/artists-page.svelte";
  import SearchPage from "./search/search-page.svelte";
  import NowPlaying from "$lib/ui/now_playing/now-playing.svelte";
  import RightPane from "$lib/ui/right_pane/right-pane.svelte";
  import PlaylistPage from "./playlist/playlist_page.svelte";
  import { router } from "$lib/router.svelte";
  import CreatePlaylistDialog from "$lib/ui/dialogs/create-playlist-dialog.svelte";
  import DeletePlaylistDialog from "$lib/ui/dialogs/delete-playlist-dialog.svelte";
  import RecentlyAddedPage from "./recently-added/recently-added-page.svelte";
  import { onMount } from "svelte";
  import { commands } from "$lib/bindings";
  import SettingsPage from "./settings/settings-page.svelte";
  import OnboardingDialog from "$lib/ui/dialogs/onboarding-dialog.svelte";
  import Titlebar from "$lib/ui/titlebar/titlebar.svelte";

  let { children } = $props();

  let blocked = $derived(
    router.rightPaneContent !== null && router.rightPaneOverlaying,
  );

  onMount(async () => {
    const dirs = await commands.getDirectories();
    if (dirs.length === 0) {
      router.openOnboarding();
    } else {
      commands.index();
    }
  });
</script>

<div class="appRoot dark">
  <Titlebar />
  <div class="contentWrapper">
    <Navbar />
    <main class="mainContent" class:blocked>
      <RecentlyAddedPage />
      <SongsPage />
      <AlbumsPage />
      <ArtistsPage />
      <SearchPage />
      <NowPlaying />
      <PlaylistPage />
      <SettingsPage />
    </main>

    <RightPane />
  </div>

  <CreatePlaylistDialog />
  <DeletePlaylistDialog />
  <OnboardingDialog />

  <div style="display: none;">{@render children()}</div>
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
    padding-left: 10px;
  }

  .mainContent.blocked {
    pointer-events: none;
    user-select: none;
  }

  :global(body) {
    font-family: "Inter Variable", sans-serif;
  }
</style>
