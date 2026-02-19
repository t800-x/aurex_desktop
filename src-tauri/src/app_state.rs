use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};

use specta::Type;

// <------------State------------>
#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct AppState {
    clicks: i32
}

impl Default for AppState {
    fn default() -> Self {
        AppState { 
            clicks: 0 
        }
    }
}




// <------------Commands------------>
#[tauri::command]
#[specta::specta]
pub async fn get_state(state: tauri::State<'_, ManagedState>) -> Result<AppState, String> {
    Ok(state.get().await)
}

#[tauri::command]
#[specta::specta]
pub async fn increment_click(state: tauri::State<'_, ManagedState>) -> Result<(), String> {
    state.update(|s| {
        s.clicks += 1;
    }).await;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn reset_clicks(state: tauri::State<'_, ManagedState>) -> Result<(), String> {
    state.update(|s| {
        s.clicks = 0;
    }).await;

    Ok(())
}




// <------------Manager------------>
pub struct ManagedState {
    pub state: Arc<Mutex<AppState>>,
    pub app: AppHandle
}

impl ManagedState {
    pub fn new(app: AppHandle) -> Self {
        Self { 
            state: Arc::new(Mutex::new(AppState::default())),
            app
        }
    }

    pub async fn update<F>(&self, updater: F)
    where 
        F: FnOnce(&mut AppState),
    {
        let mut state = self.state.lock().await;
        updater(&mut *state);

        let _ = self.app.emit("state-changed", state.clone());
    }

    pub async fn get(&self) -> AppState {
        self.state.lock().await.clone()
    }
}