<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import WindowIcons from '$lib/icons/window-icons.svelte';

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
    Windows 11 caption button spec (WinUI 3 / XAML reference):
    - Button size:          46×32px
    - Icon size:            10×10px  (Segoe Fluent icon subset)
    - Min/Max hover bg:     rgba(255,255,255,0.0605)  — yes, that specific
    - Min/Max active bg:    rgba(255,255,255,0.0419)
    - Close hover bg:       #C42B1C  (NOT #E81123 — that's Win10)
    - Close active bg:      rgba(196,43,28,0.90)
    - Close hover icon:     #ffffff
    - No border-radius on any button
    - No transition on close (intentional — snappy)
-->
<div class="win-controls">
    <button
        class="win-btn"
        onclick={() => appWindow.minimize()}
        aria-label="Minimize"
        title="Minimize"
    >
        <WindowIcons icon="minimizeWin" />
    </button>

    <button
        class="win-btn"
        onclick={() => appWindow.toggleMaximize()}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
        title={isMaximized ? 'Restore' : 'Maximize'}
    >
        <WindowIcons icon={isMaximized ? 'maximizeRestoreWin' : 'maximizeWin'} />
    </button>

    <button
        class="win-btn win-close"
        onclick={() => appWindow.close()}
        aria-label="Close"
        title="Close"
    >
        <WindowIcons icon="closeWin" />
    </button>
</div>

<style>
    .win-controls {
        display: flex;
        height: 32px;
        -webkit-app-region: no-drag;
    }

    .win-btn {
        width: 46px;
        height: 32px;
        border: none;
        background: transparent;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: default;
        color: var(--titlebar-icon-color, #ffffff);
        /* Win11 uses a fast 167ms ease for min/max */
        transition: background-color 0.167s ease;
    }

    .win-btn:hover {
        background-color: rgba(255, 255, 255, 0.0605);
    }

    .win-btn:active {
        background-color: rgba(255, 255, 255, 0.0419);
    }

    /* Win11 close is intentionally instant — no transition */
    .win-close {
        transition: none;
    }

    .win-close:hover {
        background-color: #C42B1C;
        color: #ffffff;
    }

    .win-close:active {
        background-color: rgba(196, 43, 28, 0.90);
        color: #ffffff;
    }

    :global(.win-btn svg) {
        width: 10px;
        height: 10px;
        fill: currentColor;
    }

	:global(.win-btn svg) {
		width: 10px;
		height: 10px;
		fill: currentColor;
		shape-rendering: crispEdges;
		image-rendering: pixelated;
	}

	.win-controls {
		display: flex;
		height: 32px;
		-webkit-app-region: no-drag;
		transform: translateZ(0);   /* own compositing layer */
		will-change: transform;     /* prevents fractional subpixel positioning */
	}
</style>
