use std::collections::HashMap;
use tauri::{AppHandle, Emitter};

use crate::{library_service::library_service, models::{Album, Artist, FullTrack, MatchReason, Playlist, SearchResults, Track, TrackResult}};

#[tauri::command]
#[specta::specta]
pub async fn get_recently_added() -> Vec<Album> {

    if let Ok(library) = library_service().lock() {
        if let Ok(recently_added) = library.get_recently_added_albums() {
            return recently_added;
        }
    }

    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_pl_id_by_name(name: String) -> Option<i32> {
    if let Ok(library) = library_service().lock() {
        if let Ok(result) = library.get_playlist_id_by_name(name.as_str()) {
            if let Some(id) = result {
                return Some(id as i32);
            }
        }
    }

    None
}

#[tauri::command]
#[specta::specta]
pub async fn remove_from_playlist(track_id: i32, position: i32, playlist_id: i32, app_handle: AppHandle) {
    if let Ok(library) = library_service().lock() {
        _ = library.remove_track_from_playlist(playlist_id.into(), track_id.into(), position.into());
        _ = app_handle.emit("playlist-updated", playlist_id);
    }
}

#[tauri::command]
#[specta::specta]
pub async fn add_to_playlist(track_id: Option<i32>, playlist_id: Option<i32>, target_playlist_id: i32, app_handle: AppHandle) {
    if let Some(id) = track_id {
        if let Ok(library) = library_service().lock() {
            _ = library.add_track_to_playlist(target_playlist_id.into(), id.into());
        }
    }

    if let Some(p_id) = playlist_id {
        if let Ok(library) = library_service().lock() {
            if let Ok(tracks) = library.get_tracks_in_playlist(p_id.into()) {
                for track in tracks {
                    _ = library.add_track_to_playlist(target_playlist_id.into(), track.track.id.unwrap());
                }
            }
        }
    }

    _ = app_handle.emit("playlist-updated", target_playlist_id);
}

#[tauri::command]
#[specta::specta]
pub async fn delete_playlist(app_handle: AppHandle, id: i32) {
    let _ = library_service()
        .lock()
        .map(|library| {
            _ = library.delete_playlist(id as i64);

            _ = app_handle.emit("playlists-changed", ());
        }); 
}

#[tauri::command]
#[specta::specta]
pub async fn create_playlist(app_handle: AppHandle,name: String) {
    let _ = library_service()
        .lock()
        .map(|library| {
            _ = library.create_playlist(name.as_str(), None);
            _ = app_handle.emit("playlists-changed", ());
        }); 
}

fn score_track(track: &FullTrack, term: &str) -> u32 {
    let title  = track.track.title.to_lowercase();
    let artist = track.artist_name.to_lowercase();
    let album  = track.album_title.to_lowercase();
    let lyrics = track.track.lyrics.as_deref().unwrap_or("").to_lowercase();

    let mut score = 0u32;

    // Title — highest weight
    if title == term                { score += 400; }
    else if title.starts_with(term) { score += 300; }
    else if title.contains(term)    { score += 200; }

    // Album
    if album == term                { score += 120; }
    else if album.starts_with(term) { score += 90;  }
    else if album.contains(term)    { score += 60;  }

    // Artist
    if artist == term                { score += 80; }
    else if artist.starts_with(term) { score += 60; }
    else if artist.contains(term)    { score += 40; }

    // Lyrics — lowest priority
    if lyrics.contains(term) { score += 15; }

    score
}

#[tauri::command]
#[specta::specta]
pub async fn search(term: String) -> SearchResults {
    let trimmed = term.trim().to_string();
    if trimmed.is_empty() {
        return SearchResults::default();
    }

    let term_lower = trimmed.to_lowercase();

    let Ok(library) = library_service().lock() else {
        return SearchResults::default();
    };

    let tracks    = library.get_all_tracks().unwrap_or_default();
    let albums    = library.get_all_albums().unwrap_or_default();
    let artists   = library.get_all_artists().unwrap_or_default();
    let playlists = library.get_all_playlists().unwrap_or_default();

    drop(library);

    let artist_map: HashMap<i64, &str> = artists
        .iter()
        .filter_map(|a| a.id.map(|id| (id, a.name.as_str())))
        .collect();

    // Set of artist IDs that own at least one album — built from the albums
    let artist_ids_with_albums: std::collections::HashSet<i64> = albums
        .iter()
        .map(|a| a.artist_id)
        .collect();

    // --- Tracks: filter + tag + score + sort ---
    let mut scored: Vec<(TrackResult, u32)> = tracks
        .into_iter()
        .filter_map(|track| {
            let title  = track.track.title.to_lowercase();
            let artist = track.artist_name.to_lowercase();
            let album  = track.album_title.to_lowercase();
            let lyrics = track.track.lyrics.as_deref().unwrap_or("").to_lowercase();

            let mut reasons = Vec::new();
            if title.contains(&term_lower)  { reasons.push(MatchReason::Title);  }
            if album.contains(&term_lower)  { reasons.push(MatchReason::Album);  }
            if artist.contains(&term_lower) { reasons.push(MatchReason::Artist); }
            if lyrics.contains(&term_lower) { reasons.push(MatchReason::Lyrics); }

            if reasons.is_empty() { return None; }

            let score = score_track(&track, &term_lower);
            Some((TrackResult { track, reasons }, score))
        })
        .collect();

    // Highest score first; alphabetical title as tiebreaker
    scored.sort_by(|(a, sa), (b, sb)| {
        sb.cmp(sa).then_with(|| a.track.track.title.cmp(&b.track.track.title))
    });

    let ranked_tracks = scored.into_iter().map(|(r, _)| r).collect();

    // --- Albums ---
    let filtered_albums: Vec<Album> = albums
        .into_iter()
        .filter(|album| {
            let artist_name = artist_map.get(&album.artist_id).copied().unwrap_or("");
            album.title.to_lowercase().contains(&term_lower)
                || artist_name.to_lowercase().contains(&term_lower)
        })
        .collect();

    // --- Artists: name matches AND has at least one album ---
    let filtered_artists: Vec<Artist> = artists
        .into_iter()
        .filter(|a| {
            let has_album = a.id.map_or(false, |id| artist_ids_with_albums.contains(&id));
            has_album && a.name.to_lowercase().contains(&term_lower)
        })
        .collect();

    // --- Playlists ---
    let filtered_playlists: Vec<Playlist> = playlists
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains(&term_lower))
        .collect();

    SearchResults {
        tracks: ranked_tracks,
        albums: filtered_albums,
        artists: filtered_artists,
        playlists: filtered_playlists,
    }
}

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
            let mut final_artists: Vec<Artist> = Vec::new();
            for artist in artists {
                if let Ok(albums) = library.get_albums_by_artist(artist.id.unwrap()) {
                    if !albums.is_empty() {
                        final_artists.push(artist);
                    }
                }
            }
            return final_artists;
        }
    }
    Vec::new()
}

#[tauri::command]
#[specta::specta]
pub async fn get_album_tracks(album_id: i32) -> Vec<Track> {
    match library_service().lock() {
        Ok(service) => service.get_tracks_by_album(album_id.into()).unwrap_or_default(),
        Err(e) => { eprintln!("Failed to lock db mutex: {}", e); Vec::new() }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_artist_by_id(id: i32) -> Artist {
    match library_service().lock() {
        Ok(service) => service
            .get_artist_by_id(id.into())
            .ok()
            .flatten()
            .unwrap_or_else(|| Artist { id: None, name: "Unknown Artist".into(), genre: None }),
        Err(e) => {
            eprintln!("Failed to lock library: {}", e);
            Artist { id: None, name: "Unknown Artist".into(), genre: None }
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_albums() -> Vec<Album> {
    match library_service().lock() {
        Ok(service) => service.get_all_albums().unwrap_or_else(|e| { eprintln!("{}", e); Vec::new() }),
        Err(e) => { eprintln!("Failed to lock library: {}", e); Vec::new() }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_tracks() -> Vec<FullTrack> {
    match library_service().lock() {
        Ok(service) => service.get_all_tracks().unwrap_or_else(|e| { eprintln!("{}", e); Vec::new() }),
        Err(e) => { eprintln!("Error locking library: {}", e); Vec::new() }
    }
}