use std::{
    collections::VecDeque,
    sync::{Arc, OnceLock},
    time::Duration,
};

use crate::{library_service::library_service, models::{FullTrack, Track}};
use libaurex::{aurex::Player, enums::EngineSignal};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

fn player_callback(event: EngineSignal, app_handle: AppHandle) {
    tokio::spawn(async move {
        let state = app_handle.state::<ManagedPlayer>();
        if event == EngineSignal::MediaEnd {
            println!("Media ended");

            let mut player = state.get().await;

            if player.queue.is_empty() {
                drop(player);
                _ = clear(state).await;
                return;
            }

            let audio_engine = audio_player().lock().await;

            let mut first_track: Option<FullTrack> = None;
            state.update(|player| {
                first_track = player.queue.pop_front();
            }).await;

            if let Some(track) = first_track {
                _ = audio_engine.clone().load(&track.track.file_path).await;
                _ = audio_engine.play().await;

                state
                    .update(move |player| {
                        player.currently_playing = Some(track);
                        player.state = PlayerState::Playing;
                    })
                    .await;
            }
        }
    });
}

static INSTANCE: OnceLock<Mutex<Arc<Player>>> = OnceLock::new();

pub fn init_audio_player(app_handle: AppHandle) {
    INSTANCE.get_or_init(|| {
        Mutex::new(
            Player::new(
                Some(libaurex::enums::ResamplingQuality::VeryHigh),
                Box::new(move |event, _player| {
                    player_callback(event, app_handle.clone());
                }),
            )
            .expect("Failed to setup audio engine"),
        )
    });
}

pub fn audio_player() -> &'static Mutex<Arc<Player>> {
    INSTANCE.get().expect("Audio engine not initialized")
}

pub fn track_progress(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {

        loop {
            let audio_engine = audio_player().lock().await;

            let progress = audio_engine.get_progress().await.unwrap();

            _ = app_handle.emit("progress-changed", progress);

            _ = tokio::time::sleep(Duration::from_millis(20)).await;
        }
    });
}

#[derive(Clone, Serialize, Deserialize, Debug, Type, PartialEq)]
pub enum PlayerState {
    Paused,
    Playing,
    Empty,
    Stopped,
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct AudioPlayer {
    currently_playing: Option<FullTrack>,
    state: PlayerState,
    queue: VecDeque<FullTrack>,
    position: f64,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        AudioPlayer {
            currently_playing: None,
            state: PlayerState::Empty,
            queue: VecDeque::new(),
            position: 0.0,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn play_tracks(state: tauri::State<'_, ManagedPlayer>, tracks: Vec<Track>, index: i32) -> Result<AudioPlayer, String> {
    let mut full_tracks: Vec<FullTrack> = Vec::new();
    
    if let Ok(service) = library_service().lock() {
        //Convert tracks to fulltrack
        for track in tracks {
            if let Ok(ft_option) = service.get_full_track_by_id(track.id.unwrap()) {
                if let Some(full_track) = ft_option {
                    full_tracks.push(full_track);
                }
            }
        }
    }

    if !full_tracks.is_empty() {
        _ = play_list(state.clone(), full_tracks, index).await;
    }

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn get_player(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn load(
    state: tauri::State<'_, ManagedPlayer>,
    track: FullTrack,
) -> Result<AudioPlayer, String> {
    let audio_engine = audio_player().lock().await;
    _ = audio_engine.clone().load(&track.track.file_path).await;

    state
        .update(|player| {
            player.currently_playing = Some(track);
            player.state = PlayerState::Paused;
        })
        .await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn play(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    let playerstate = state.get().await.state;

    if (playerstate != PlayerState::Playing) && (playerstate != PlayerState::Empty) {
        state.update(|s| {
                s.state = PlayerState::Playing;
            })
            .await;

        let player = audio_player().lock().await;
        _ = player.play().await;

        
    }

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn pause(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    let player = state.get().await;

    if player.state != PlayerState::Paused {
        state.update(|s| {
                s.state = PlayerState::Paused;
            })
            .await;

        let audio_engine = audio_player().lock().await;
        _ = audio_engine.pause().await;
  
    }

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn clear(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    state
        .update(|player| {
            player.currently_playing = None;
            player.state = PlayerState::Empty;
            player.queue.clear();
        })
        .await;

    let audio_engine = audio_player().lock().await;
    _ = audio_engine.clear().await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn play_list(
    state: tauri::State<'_, ManagedPlayer>,
    mut list: Vec<FullTrack>,
    idx: i32,
) -> Result<AudioPlayer, String> {

    let mut index = idx as usize;

    let first_track = list.remove(index);
    _ = load(state.clone(), first_track).await;
    _ = play(state.clone()).await;
    
    state.update(|player| {
        player.queue.clear();

        while index < list.len()  {
            player.queue.push_back(list[index].clone());
            index += 1;
        }
    }).await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn play_next(
    state: tauri::State<'_, ManagedPlayer>,
    track: FullTrack,
) -> Result<AudioPlayer, String> {
    state
        .update(|player| {
            player.queue.push_front(track);
        })
        .await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn add_to_queue(
    state: tauri::State<'_, ManagedPlayer>,
    track: FullTrack,
) -> Result<AudioPlayer, String> {
    state
        .update(|player| {
            player.queue.push_back(track);
        })
        .await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn add_list_to_queue(state: tauri::State<'_, ManagedPlayer>, fulltracks: Option<Vec<FullTrack>>, tracks: Option<Vec<Track>>) -> Result<AudioPlayer, String> {

    if let Some(fulltracks) = fulltracks {
        for track in fulltracks {
            _ = add_to_queue(state.clone(), track).await;
        }
    }

    if let Some(tracks) = tracks {
        for track in tracks {
            let mut temp_track: Option<FullTrack> = None;

            if let Ok(library) = library_service().lock() {
                if let Ok(res) = library.get_full_track_by_id(track.id.unwrap()) {
                    if let Some(fulltrack) = res {
                        temp_track = Some(fulltrack);
                    }
                }
            } 

           if let Some(track) = temp_track {
                _ = add_to_queue(state.clone(), track).await;
           }
        }
    } 

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn play_list_next(state: tauri::State<'_, ManagedPlayer>, fulltracks: Option<Vec<FullTrack>>, tracks: Option<Vec<Track>>) -> Result<AudioPlayer, String> {

    if let Some(fulltracks) = fulltracks {
        for track in fulltracks {
            _ = play_next(state.clone(), track).await;
        }
    }

    if let Some(tracks) = tracks {
        for track in tracks {
            let mut temp_track: Option<FullTrack> = None;

            if let Ok(library) = library_service().lock() {
                if let Ok(res) = library.get_full_track_by_id(track.id.unwrap()) {
                    if let Some(fulltrack) = res {
                        temp_track = Some(fulltrack);
                    }
                }
            } 

           if let Some(track) = temp_track {
                _ = play_next(state.clone(), track).await;
           }
        }
    } 

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn next(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    let mut track: Option<FullTrack> = None;

    state.update(|player|  {
        track = player.queue.pop_front();
    }).await;

    if let Some(t) = track {
        _ = load(state.clone(), t).await;
        _ = play(state.clone()).await;
    }

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn change_queue_index(
    state: tauri::State<'_, ManagedPlayer>,
    old_idx: i32,
    new_idx: i32,
) -> Result<AudioPlayer, String> {
    let old_index = old_idx as usize;
    let mut new_index = new_idx as usize;

    state
        .update(|player| {
            if old_index < new_index {
                new_index -= 1;
            }

            if let Some(track) = player.queue.remove(old_index) {
                player.queue.insert(new_index, track);
            }
        })
        .await;

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn seek(time: f64) {
    let audio_engine = audio_player().lock().await;
    _ = audio_engine.seek(time).await;
}

pub struct ManagedPlayer {
    pub player: Arc<Mutex<AudioPlayer>>,
    pub app: AppHandle,
}

impl ManagedPlayer {
    pub fn new(app: AppHandle) -> Self {
        Self {
            player: Arc::new(Mutex::new(AudioPlayer::default())),
            app,
        }
    }

    pub async fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut AudioPlayer),
    {
        let mut audio_player = self.player.lock().await;
        updater(&mut *&mut audio_player);

        let _ = self.app.emit("player-changed", audio_player.clone());
    }

    pub async fn get(&self) -> AudioPlayer {
        self.player.lock().await.clone()
    }
}
