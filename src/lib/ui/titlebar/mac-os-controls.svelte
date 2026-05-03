<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const appWindow = getCurrentWindow();

  let isMaximized = $state(false);
  let isFocused = $state(true);
  let isGroupHovered = $state(false);

  $effect(() => {
    const cleanups: Array<() => void> = [];

    appWindow.isMaximized().then((m) => (isMaximized = m));
    appWindow.isFocused().then((f) => (isFocused = f));

    appWindow
      .onResized(async () => {
        isMaximized = await appWindow.isMaximized();
      })
      .then((fn) => cleanups.push(fn));

    appWindow
      .onFocusChanged(({ payload: focused }) => {
        isFocused = focused;
      })
      .then((fn) => cleanups.push(fn));

    return () => cleanups.forEach((fn) => fn());
  });
</script>

<!--
    macOS Sequoia traffic light spec:
    - Button diameter:  12px
    - Gap between:       8px  (center-to-center: 20px)
    - Left inset:       12px  (to center of first button: 18px)
    - Order L→R:        close · minimize · fullscreen
    - Unfocused:        #9B9B9B (all three)
    - Close hover icon: ✕  color rgba(75,0,0,0.7)
    - Min   hover icon: −  color rgba(75,50,0,0.7)
    - Full  hover icon: ⤢  color rgba(0,55,0,0.7)
-->
<div
  class="traffic-lights"
  class:unfocused={!isFocused}
  class:show-icons={isGroupHovered}
  onmouseenter={() => (isGroupHovered = true)}
  onmouseleave={() => (isGroupHovered = false)}
  role="group"
  aria-label="Window controls"
>
  <button
    class="tl-btn tl-close"
    onclick={() => appWindow.close()}
    aria-label="Close"
    title="Close"
  >
    <svg class="tl-icon" viewBox="0 0 8 8" xmlns="http://www.w3.org/2000/svg">
      <line
        x1="1.5"
        y1="1.5"
        x2="6.5"
        y2="6.5"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
      />
      <line
        x1="6.5"
        y1="1.5"
        x2="1.5"
        y2="6.5"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
      />
    </svg>
  </button>

  <button
    class="tl-btn tl-minimize"
    onclick={() => appWindow.minimize()}
    aria-label="Minimize"
    title="Minimize"
  >
    <svg class="tl-icon" viewBox="0 0 8 8" xmlns="http://www.w3.org/2000/svg">
      <line
        x1="1.5"
        y1="4"
        x2="6.5"
        y2="4"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
      />
    </svg>
  </button>

  <button
    class="tl-btn tl-fullscreen"
    onclick={() => appWindow.toggleMaximize()}
    aria-label={isMaximized ? "Exit Full Screen" : "Enter Full Screen"}
    title={isMaximized ? "Exit Full Screen" : "Enter Full Screen"}
  >
    {#if isMaximized}
      <!-- Two inward arrows -->
      <svg class="tl-icon" viewBox="0 0 8 8" xmlns="http://www.w3.org/2000/svg">
        <polyline
          points="5,1 7,1 7,3"
          fill="none"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
        <polyline
          points="3,7 1,7 1,5"
          fill="none"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
        <line
          x1="4.5"
          y1="3.5"
          x2="6.5"
          y2="1.5"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
        <line
          x1="1.5"
          y1="6.5"
          x2="3.5"
          y2="4.5"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
      </svg>
    {:else}
      <!-- Two outward arrows -->
      <svg class="tl-icon" viewBox="0 0 8 8" xmlns="http://www.w3.org/2000/svg">
        <polyline
          points="4.5,1 7,1 7,3.5"
          fill="none"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
        <polyline
          points="3.5,7 1,7 1,4.5"
          fill="none"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
        <line
          x1="4.5"
          y1="3.5"
          x2="7"
          y2="1"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
        <line
          x1="1"
          y1="7"
          x2="3.5"
          y2="4.5"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
      </svg>
    {/if}
  </button>
</div>

<style>
  .traffic-lights {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px 0 12px;
    height: 100%;
    /* keep it out of the drag region */
    -webkit-app-region: no-drag;
  }

  .tl-btn {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: default;
    flex-shrink: 0;
    position: relative;
    transition: filter 0.08s ease;
    /* subtle inner shadow macOS uses */
    box-shadow: inset 0 0 0 0.5px rgba(0, 0, 0, 0.12);
  }

  /* ── colors (focused) ── */
  .tl-close {
    background-color: #ff5f57;
  }
  .tl-minimize {
    background-color: #febc2e;
  }
  .tl-fullscreen {
    background-color: #28c840;
  }

  /* ── unfocused: all go gray ── */
  .unfocused .tl-btn {
    background-color: #9b9b9b;
    box-shadow: inset 0 0 0 0.5px rgba(0, 0, 0, 0.1);
  }

  /* ── icons: hidden until group hover ── */
  .tl-icon {
    width: 8px;
    height: 8px;
    opacity: 0;
    transition: opacity 0.08s ease;
    fill: none;
  }

  .tl-close .tl-icon {
    color: rgba(75, 0, 0, 0.65);
  }
  .tl-minimize .tl-icon {
    color: rgba(75, 50, 0, 0.65);
  }
  .tl-fullscreen .tl-icon {
    color: rgba(0, 55, 0, 0.65);
  }

  /* unfocused icons don't show */
  .show-icons:not(.unfocused) .tl-icon {
    opacity: 1;
  }

  /* ── press ── */
  .tl-btn:active {
    filter: brightness(0.78);
  }
</style>
