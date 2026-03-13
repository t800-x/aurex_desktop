use std::{
    collections::VecDeque,
    sync::{Arc, OnceLock},
    time::Duration,
};

use crate::{library_service::library_service, models::{FullTrack, Track}, traits::Shuffle};
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

            let player = state.get().await;

            if player.queue.is_empty() {
                drop(player);
                _ = clear(state).await;
                return;
            }

            _ = next(state.clone()).await;
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
pub enum LoopType {
    LoopOnce,
    LoopOver,
    Off
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct AudioPlayer {
    currently_playing: Option<FullTrack>,
    shuffle: bool,
    state: PlayerState,
    history: VecDeque<FullTrack>,
    real_queue: VecDeque<FullTrack>,
    queue: VecDeque<FullTrack>, // <- This is a proxy
    position: f64,
    looping: LoopType
}

impl Default for AudioPlayer {
    fn default() -> Self {
        AudioPlayer {
            currently_playing: None,
            shuffle: false,
            state: PlayerState::Empty,
            history: VecDeque::new(),
            real_queue: VecDeque::new(),
            queue: VecDeque::new(),
            position: 0.0,
            looping: LoopType::Off
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
            player.real_queue.clear();
            player.queue.clear();

            state.update_queue(&player);
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
        player.real_queue.clear();

        while index < list.len()  {
            player.real_queue.push_back(list[index].clone());
            index += 1;
        }

        player.queue = player.real_queue.clone();
        state.update_queue(&player);

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
            player.real_queue.push_front(track.clone());
            player.queue.push_front(track);
            state.update_queue(&player);
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
            player.real_queue.push_back(track.clone());
            player.queue.push_back(track);
            state.update_queue(&player);
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

    state.update(|player| {
        if let Some(current) = player.currently_playing.clone() {
            player.history.push_back(current);
        }

        track = player.queue.pop_front();

        if let Some(ref t) = track {
            player.real_queue.retain(|m| m.track.id.unwrap_or(-1) != t.track.id.unwrap_or(-2));
        }

        state.update_queue(&player);
        state.update_history(&player);
    }).await;

    if let Some(t) = track {
        _ = load(state.clone(), t).await;
        _ = play(state.clone()).await;
    }

    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn previous(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    let mut track: Option<FullTrack> = None;

    state.update(|player| {
        track = player.history.pop_back();

        if track.is_some() {
            if let Some(current) = player.currently_playing.clone() {
                player.queue.push_front(current.clone());
                player.real_queue.push_front(current);
            }

            state.update_queue(&player);
            state.update_history(&player);
        }
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
                player.queue.insert(new_index, track.clone());

                if !player.shuffle {
                    player.real_queue.insert(new_index, track);
                }
            }

            state.update_queue(&player);
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

#[tauri::command]
#[specta::specta]
pub async fn shuffle(state: tauri::State<'_, ManagedPlayer>) -> Result<AudioPlayer, String> {
    state.update(|player| {
        player.queue.clone_from(&player.real_queue);
        if !player.shuffle {
            player.queue.shuffle();
        }
        player.shuffle = !player.shuffle;
        state.update_queue(&player);
    }).await;

    Ok(state.get().await)
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
        
        let new_audio_player = AudioPlayer {
            real_queue: VecDeque::new(),
            queue: VecDeque::new(),
            history: VecDeque::new(),
            
            currently_playing: audio_player.currently_playing.clone(),
            shuffle: audio_player.shuffle.clone(),
            state: audio_player.state.clone(),
            position: audio_player.position.clone(),
            looping: audio_player.looping.clone()
        };

        //sending the payload without the queue data for now cause it can get big and cause slowdowns
        let _ = self.app.emit("player-changed", new_audio_player);
    }

    pub async fn get(&self) -> AudioPlayer {
        self.player.lock().await.clone()
    }

    //function to manually emit signals specifically for queue
    pub fn update_queue(&self, player: &AudioPlayer) {
        let _ = self.app.emit("queue-changed", player.queue.clone());
    }

    pub fn update_history(&self, player: &AudioPlayer) {
        let _ = self.app.emit("history-changed", player.history.clone());
    }
}
