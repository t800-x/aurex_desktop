import type { Album, Playlist } from "./bindings";

export enum Section {
    search = 1,
    songs,
    albums,
    artists
}

export enum RightPaneContent {
    queue = 1,
    lyrics
}

class Router {
    current = $state<Section>(Section.songs);
    rightPaneContent = $state<RightPaneContent | null>(null);
    rightPaneOverlaying = $state(false);

    // Navigation targets set by external callers (e.g. search page).
    // Destination pages watch these and clear them once consumed.
    pendingAlbum = $state<Album | null>(null);
    pendingArtistId = $state<number | null>(null);

    isCreatePlaylistDialogOpen = $state(false);

    isDeletePlaylistDialogOpen = $state(false);
    pendingDeletePlaylist = $state<Playlist | null>(null);

    constructor() {}

    openDeletePlaylistDialog(playlist: Playlist) {
        this.pendingDeletePlaylist = playlist;
        this.isDeletePlaylistDialogOpen = true;
    }
    closeDeletePlaylistDialog() {
        this.isDeletePlaylistDialogOpen = false;
        this.pendingDeletePlaylist = null;
    }

    openCreatePlaylistDialog() {
        this.isCreatePlaylistDialogOpen = true;
    }

    closeCreatePlaylistDialog() {
        this.isCreatePlaylistDialogOpen = false;
    }

    go(s: Section): void {
        this.current = s;
    }

    goToAlbum(album: Album): void {
        this.pendingAlbum = album;
        this.current = Section.albums;
    }

    goToArtist(artistId: number): void {
        this.pendingArtistId = artistId;
        this.current = Section.artists;
    }

    setRightPaneContent(content: RightPaneContent | null): void {
        if (content == this.rightPaneContent) {
            this.rightPaneContent = null;
            return;
        }
        this.rightPaneContent = content;
    }
}

export const router = new Router();