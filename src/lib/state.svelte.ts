import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface AppState {
    clicks: number
}

class StateManager {
    clicks = $state<number>(0);

    constructor() {
        this.init();
    }

    private async init() {
        const initial = await invoke<AppState>('get_state');

        this.clicks = initial.clicks;

        await listen<AppState>('state-changed', (event) => {
            const newState = event.payload;

            if (this.clicks != newState.clicks) this.clicks = newState.clicks;
        });
    }
}

export const appState = new StateManager();