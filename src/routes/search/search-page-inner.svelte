<script lang="ts">
  import SearchInput from "$lib/components/ui/input/search-input.svelte";
  import { commands, type Playlist } from "$lib/bindings";
  import type { Album, Artist, TrackResult } from "$lib/bindings";
  import SearchTracksGrid from "./search-tracks-grid.svelte";
  import SearchAlbumsRow from "./search-albums-row.svelte";
  import SearchArtistsRow from "./search-artists-row.svelte";
  import SearchPlaylistsList from "./search-playlists-list.svelte";

  let searchTerm = $state("");
  let tracks = $state<TrackResult[]>([]);
  let albums = $state<Album[]>([]);
  let artists = $state<Artist[]>([]);
  let playlists = $state<Playlist[]>([]);
  let loading = $state(false);

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Live debounced search — 220 ms after the user stops typing
  $effect(() => {
    const term = searchTerm; // snapshot for closure

    if (debounceTimer) clearTimeout(debounceTimer);

    if (term.trim().length === 0) {
      tracks = [];
      albums = [];
      artists = [];
      playlists = [];
      loading = false;
      return;
    }

    loading = true;
    debounceTimer = setTimeout(async () => {
      const result = await commands.search(term);
      // Discard stale results if term changed while request was in flight
      if (searchTerm !== term) return;
      tracks = result.tracks;
      albums = result.albums;
      artists = result.artists;
      playlists = result.playlists;
      loading = false;
    }, 220);
  });

  let hasResults = $derived(
    tracks.length > 0 ||
      albums.length > 0 ||
      artists.length > 0 ||
      playlists.length > 0,
  );
  let showEmpty = $derived(
    searchTerm.trim().length > 0 && !loading && !hasResults,
  );

  function playFromTrackIndex(index: number) {
    commands.playList(
      tracks.map((r) => r.track),
      index,
    );
  }

  async function playPlaylist(playlist: Playlist) {
    const pl = await commands.getPlaylistTracks(Number(playlist.id));
    if (pl.length > 0) commands.playList(pl, 0);
  }
</script>

<div class="searchPage">
  <div class="inputWrap">
    <SearchInput
      bind:value={searchTerm}
      placeholder="Search songs, albums, artists, playlists…"
    />
  </div>

  <div class="results">
    {#if loading}
      <div class="shimmerWrap">
        {#each Array(6) as _}
          <div class="shimmerRow"></div>
        {/each}
      </div>
    {/if}

    {#if showEmpty}
      <div class="emptyState">
        <span class="emptyIcon">𝄞</span>
        <p>Nothing found for <strong>"{searchTerm}"</strong></p>
        <p class="hint">Try a different title, artist, or lyric.</p>
      </div>
    {/if}

    {#if !loading && hasResults}
      {#if tracks.length > 0}
        <SearchTracksGrid {tracks} onPlayFromIndex={playFromTrackIndex} />
      {/if}

      {#if albums.length > 0}
        <SearchAlbumsRow {albums} />
      {/if}

      {#if artists.length > 0}
        <SearchArtistsRow {artists} />
      {/if}

      {#if playlists.length > 0}
        <SearchPlaylistsList {playlists} onPlay={playPlaylist} />
      {/if}

      <div style="height: 80px;"></div>
    {/if}
  </div>
</div>

<style>
  .searchPage {
    display: flex;
    height: 100%;
    width: 100%;
    flex-direction: column;
    overflow: hidden;
  }

  .inputWrap {
    padding: 18px 20px 12px;
    flex-shrink: 0;
  }

  .results {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0 16px;
  }

  /* Loading shimmer */
  .shimmerWrap {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 0;
  }

  .shimmerRow {
    height: 44px;
    border-radius: 7px;
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.04) 25%,
      rgba(255, 255, 255, 0.08) 50%,
      rgba(255, 255, 255, 0.04) 75%
    );
    background-size: 200% 100%;
    animation: shimmer 1.4s infinite;
  }

  @keyframes shimmer {
    0% {
      background-position: 200% center;
    }
    100% {
      background-position: -200% center;
    }
  }

  /* Empty state */
  .emptyState {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: var(--color-navbar-label);
    text-align: center;
    gap: 6px;
  }

  .emptyIcon {
    font-size: 48px;
    opacity: 0.2;
    line-height: 1;
    margin-bottom: 8px;
  }

  .emptyState p {
    font-size: 15px;
    font-weight: 500;
  }

  .hint {
    font-size: 13px !important;
    font-weight: 400 !important;
    opacity: 0.6;
  }
</style>
