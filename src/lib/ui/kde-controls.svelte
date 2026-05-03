<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    let isMaximized = $state(false);

    $effect(() => {
        let unlisten: () => void;
        appWindow.onResized(async () => {
            isMaximized = await appWindow.isMaximized();
        }).then((fn) => (unlisten = fn));
        appWindow.isMaximized().then((m) => (isMaximized = m));
        return () => unlisten?.();
    });
</script>

<!--
    KDE Plasma 6 Breeze decoration spec:
    - Titlebar height:   30px (system default)
    - Button region:     right side, order: [minimize][maximize][close]
    - Button hitbox:     30×30px (fills titlebar height)
    - Icon size:         16×16px, stroke 1.1px, round caps/joins
    - Hover background:  rounded rect, 18×18px, rgba(255,255,255,0.15) dark / rgba(0,0,0,0.08) light
    - Close hover bg:    #DA4453 (Breeze red)
    - Close active bg:   #BF3040
    - Pressed shift:     icon translates 1px down
    - No border/outline on any state
-->
<div class="breeze-controls">
    <button
        class="breeze-btn"
        onclick={() => appWindow.minimize()}
        aria-label="Minimize"
        title="Minimize"
    >
        <!-- Breeze minimize: single horizontal line, vertically centered-low -->
        <svg class="breeze-icon" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <line x1="4" y1="12" x2="12" y2="12" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>
        </svg>
    </button>

    <button
        class="breeze-btn"
        onclick={() => appWindow.toggleMaximize()}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
        title={isMaximized ? 'Restore' : 'Maximize'}
    >
        {#if isMaximized}
            <!-- Restore: two overlapping rectangles -->
            <svg class="breeze-icon" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
                <rect x="5.5" y="3.5" width="7" height="7" rx="0.5" fill="none" stroke="currentColor" stroke-width="1.1"/>
                <polyline points="3.5,7 3.5,12.5 9,12.5 9,10.5" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        {:else}
            <!-- Maximize: single rectangle -->
            <svg class="breeze-icon" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
                <rect x="4" y="4" width="8" height="8" rx="0.5" fill="none" stroke="currentColor" stroke-width="1.1"/>
            </svg>
        {/if}
    </button>

    <button
        class="breeze-btn breeze-close"
        onclick={() => appWindow.close()}
        aria-label="Close"
        title="Close"
    >
        <!-- Breeze X: two lines, no serif, round caps -->
        <svg class="breeze-icon" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <line x1="4.5" y1="4.5" x2="11.5" y2="11.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="11.5" y1="4.5" x2="4.5" y2="11.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>
        </svg>
    </button>
</div>

<style>
    .breeze-controls {
        display: flex;
        align-items: center;
        height: 30px;
        padding: 0 2px;
        -webkit-app-region: no-drag;
    }

    .breeze-btn {
        width: 30px;
        height: 30px;
        border: none;
        background: transparent;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: default;
        position: relative;
        color: var(--titlebar-icon-color, rgba(255, 255, 255, 0.85));
    }

    /* hover pill — Breeze uses a rounded square behind the icon, not a full button fill */
    .breeze-btn::before {
        content: '';
        position: absolute;
        inset: 6px;               /* 18×18px pill centered in 30px button */
        border-radius: 3px;
        background: transparent;
        transition: background-color 0.12s ease;
    }

    .breeze-btn:hover::before {
        background-color: rgba(255, 255, 255, 0.14);
    }

    .breeze-btn:active::before {
        background-color: rgba(255, 255, 255, 0.08);
    }

    /* icon sits above the ::before pill */
    .breeze-icon {
        width: 16px;
        height: 16px;
        position: relative;
        z-index: 1;
        transition: transform 0.06s ease;
    }

    .breeze-btn:active .breeze-icon {
        transform: translateY(1px);
    }

    /* ── close button overrides ── */
    .breeze-close:hover::before {
        background-color: #DA4453;
    }

    .breeze-close:active::before {
        background-color: #BF3040;
    }

    /* close icon goes white on its red bg */
    .breeze-close:hover {
        color: #ffffff;
    }
</style>
