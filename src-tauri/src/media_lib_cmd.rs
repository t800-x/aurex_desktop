use crate::{library_service::library_service, models::{Album, Artist, FullTrack}};

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
