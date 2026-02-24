mod app_state;
mod audio_player;
mod constants;
mod error;
mod library_service;
mod media_lib_cmd;
mod metadata;
mod models;
mod lyrics;

use app_state::ManagedState;
use std::fs::File;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

use crate::{
    audio_player::{init_audio_player, track_progress, ManagedPlayer},
    constants::ensure_paths_created,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    ensure_paths_created();

    if !constants::dir_file().exists() {
        _ = File::create(constants::dir_file()).expect("Failed to create directories file");
    }

    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        app_state::get_state,
        app_state::increment_click,
        app_state::reset_clicks,

        metadata::index,

        media_lib_cmd::get_all_tracks,
        media_lib_cmd::get_all_albums,
        media_lib_cmd::get_artist_by_id,
        media_lib_cmd::get_album_tracks,

        audio_player::get_player,
        audio_player::play,
        audio_player::load,
        audio_player::change_queue_index,
        audio_player::next,
        audio_player::play_next,
        audio_player::play_list,
        audio_player::add_to_queue,
        audio_player::clear,
        audio_player::pause,
        audio_player::seek,
        audio_player::play_tracks,
        audio_player::add_list_to_queue,
        audio_player::play_list_next,

        library_service::fulltrack_from_id,

        lyrics::get_line_lyrics
    ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::BigInt),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let managed_state = ManagedState::new(app.handle().clone());
            app.manage(managed_state);

            init_audio_player(app.handle().clone());
            track_progress(app.handle().clone());

            app.manage(ManagedPlayer::new(app.handle().clone()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
