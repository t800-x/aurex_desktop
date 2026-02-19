<script lang="ts">
  import "../app.css";
  import "../colors.css"
  import '@fontsource-variable/inter';
  import Navbar from "$lib/ui/navbar.svelte";
  import SongsPage from "./songs/songs-page.svelte";
  import AlbumsPage from "./albums/albums-page.svelte";
  import ArtistsPage from "./artists/artists-page.svelte";
  import SearchPage from "./search/search-page.svelte";
  
</script>

<div class="appRoot dark min-h-screen text-foreground flex flex-row">
    <Navbar />
    <main class="flex flex-1 overflow-hidden h-full">
      <SongsPage />
      <AlbumsPage />
      <ArtistsPage />
      <SearchPage />
    </main>
</div>

<style>
  .appRoot {
    background-color: #202020;
    height: 100vh;
    overflow: hidden;
  }

  :global(body) {
    font-family: 'Inter Variable', sans-serif;
  }
</style>
