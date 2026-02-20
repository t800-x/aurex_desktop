<script lang="ts">
  import "../app.css";
  import "../colors.css"
  import '@fontsource-variable/inter';
  import Navbar from "$lib/ui/navbar.svelte";
  import SongsPage from "./songs/songs-page.svelte";
  import AlbumsPage from "./albums/albums-page.svelte";
  import ArtistsPage from "./artists/artists-page.svelte";
  import SearchPage from "./search/search-page.svelte";
  import NowPlaying from "$lib/ui/now-playing.svelte";
  
</script>

<div class="appRoot dark">
    <Navbar />

    <main class="mainContent">
      <SongsPage />
      <AlbumsPage />
      <ArtistsPage />
      <SearchPage />

      <div class="spacer">
        <style>
          .spacer {
            height: 85px
          }
        </style>
      </div>

      <NowPlaying />
    </main>
    


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

  .mainContent {
    display: flex;
    flex: 1;
    overflow: hidden;
    height: 100%;
    isolation: isolate;
    position: relative;
  }

  :global(body) {
    font-family: 'Inter Variable', sans-serif;
  }
</style>
