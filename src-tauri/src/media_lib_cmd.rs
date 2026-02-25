use crate::{library_service::library_service, models::{Album, Artist, FullTrack, Playlist, Track}};

#[tauri::command]
#[specta::specta]
pub async fn get_playlist(id: i32) -> Option<Playlist> {
    if let Ok(library) = library_service().lock() {
        if let Some(playlist) = library.get_playlist_by_id(id.into()) {
            return Some(playlist);
        }
    }

    None
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_playlists() -> Vec<Playlist> {

    if let Ok(library) = library_service().lock() {
        if let Ok(playlists) = library.get_all_playlists() {
            return playlists;
        }
    }

    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_playlist_tracks(playlist_id: i32) -> Vec<FullTrack> {
    if let Ok(library) = library_service().lock() {
        if let Ok(playlist_tracks) = library.get_tracks_in_playlist(playlist_id.into()) {
            return playlist_tracks;
        }
    }

    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_artist_albums(id: i32) -> Vec<Album> {

    if let Ok(library) = library_service().lock() {
        if let Ok(albums) = library.get_albums_by_artist(id.into()) {
            return albums;
        }
    }

    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_artists() -> Vec<Artist> {
    
    if let Ok(library) = library_service().lock() {
        if let Ok(artists) = library.get_all_artists() {
            return artists;
        }
    }

    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_album_tracks(album_id: i32) -> Vec<Track> {
    let library = library_service().lock();

    match library {
        Ok(service) => {
            if let Ok(tracks) = service.get_tracks_by_album(album_id.into()) {
                return tracks;
            }
        },
        Err(e) => {
            eprintln!("Failed to lock db mutex: {}", e);
            return Vec::new();
        }
    }

    return Vec::new();
}

#[tauri::command]
#[specta::specta]
pub async fn get_artist_by_id(id: i32) -> Artist {
    let library = library_service().lock();

    match library {
        Ok(service) => {
            if let Ok(artist) = service.get_artist_by_id(id.into()) {
                if let Some(m_artist) = artist {
                    return m_artist;
                }
            }
        },

        Err(e) => {
            eprintln!("Failed to lock library: {}", e);

            return Artist {
                id: None,
                name: String::from("Unknown Artist"),
                genre: None
            };
        }
    }

    return Artist {
        id: None,
        name: String::from("Unknown Artist"),
        genre: None
    };
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_albums() -> Vec<Album> {
    let library = library_service().lock();

    match library {
        Ok(service) => match service.get_all_albums() {
            Ok(albums) => albums,
            Err(e) => {
                eprintln!("{}", e);
                Vec::new()
            }
        },

        Err(e) => {
            eprintln!("Failed to lock library: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_tracks() -> Vec<FullTrack> {
    let library = library_service().lock();

    match library {
        Ok(service) => match service.get_all_tracks() {
            Ok(tracks) => tracks,
            Err(e) => {
                eprintln!("Error fetching tracks: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("Error locking library: {}", e);
            Vec::new()
        }
    }
}
