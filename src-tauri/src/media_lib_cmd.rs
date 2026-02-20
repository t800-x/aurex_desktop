use crate::{library_service::library_service, models::FullTrack};

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
