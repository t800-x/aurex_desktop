mod app_state;
mod library_service;
mod metadata;
mod constants;
mod error;
mod models;
mod media_lib_cmd;
mod audio_player;

use std::fs::File;
use app_state::{
    ManagedState
};
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

use crate::{audio_player::{ManagedPlayer, init_audio_player, track_progress}, constants::ensure_paths_created};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    ensure_paths_created();

    if !constants::dir_file().exists() {
        _ = File::create(constants::dir_file()).expect("Failed to create directories file");
    }

    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            app_state::get_state,
            app_state::increment_click,
            app_state::reset_clicks,

            metadata::index,
            media_lib_cmd::get_all_tracks,

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
            audio_player::seek
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
