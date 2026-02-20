import { listen } from "@tauri-apps/api/event";
import type { AudioPlayer } from "./bindings";
import type { FullTrack } from "./bindings";
import { commands } from "./bindings";
import { areArraysEqual } from "./helpers";

export const PlayerState = {
  Paused: "Paused",
  Playing: "Playing",
  Empty: "Empty",
  Stopped: "Stopped"
} as const

export type PlayerState = typeof PlayerState[keyof typeof PlayerState]

class PlayerManager {
    currentlyPlaying = $state<FullTrack | null>(null);
    state = $state<PlayerState>(PlayerState.Empty);
    queue = $state<FullTrack[] | null>(null);

    private async init() {
        const result = await commands.getPlayer();

        if (result.status === "ok") {
            const player = result.data;

            this.currentlyPlaying = player.currently_playing;
            this.queue = player.queue;
            this.state = player.state;
        }

        await listen<AudioPlayer>('player-changed', (event) => {
            const newPlayer = event.payload;

            if (this.currentlyPlaying !== newPlayer.currently_playing) {
                this.currentlyPlaying = newPlayer.currently_playing;
            }

            if (this.state !== newPlayer.state) {
                this.state = newPlayer.state;
            }

            if (areArraysEqual(this.queue ?? [], newPlayer.queue ?? [])) {
                this.queue = newPlayer.queue;
            }
        });
    }

    constructor() {
        this.init();
    }
}

export const audioPlayer = new PlayerManager();