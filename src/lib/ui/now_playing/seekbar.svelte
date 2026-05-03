<script lang="ts">
  import { audioPlayer } from '$lib/player.svelte';
  import { commands } from '$lib/bindings';
  import { formatDuration } from '$lib/helpers';

  let dragging = $state(false);
  let hovering = $state(false);
  let trackEl = $state<HTMLDivElement | null>(null);
  let pendingValue = $state<number | null>(null);

  function getDuration(): number {
    const d = audioPlayer.currentlyPlaying?.track.duration;
    return d ? Number(d) : 0;
  }

  let percent = $derived(() => {
    const pos = pendingValue ?? audioPlayer.position;
    const dur = getDuration();
    if (!dur) return 0;
    return Math.min(1, Math.max(0, pos / dur));
  });

  function getPositionFromEvent(e: MouseEvent): number {
    if (!trackEl) return audioPlayer.position;
    const rect = trackEl.getBoundingClientRect();
    const ratio = Math.min(1, Math.max(0, (e.clientX - rect.left) / rect.width));
    return ratio * getDuration();
  }

  function onMouseDown(e: MouseEvent): void {
    dragging = true;
    pendingValue = getPositionFromEvent(e);
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }

  function onMouseMove(e: MouseEvent): void {
    if (dragging) pendingValue = getPositionFromEvent(e);
  }

  async function onMouseUp(e: MouseEvent): Promise<void> {
    const target = getPositionFromEvent(e);
    dragging = false;
    await commands.seek(target);
    pendingValue = null;
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  }

  function onKeyDown(e: KeyboardEvent): void {
    const dur = getDuration();
    const step = dur * 0.01;
    if (e.key === 'ArrowRight') commands.seek(Math.min(dur, audioPlayer.position + step));
    if (e.key === 'ArrowLeft') commands.seek(Math.max(0, audioPlayer.position - step));
  }

  let active = $derived(() => dragging || hovering);
</script>

<div class="seekbar-wrap">
  <span class="time">{formatDuration(pendingValue ?? audioPlayer.position)}</span>

  <div
    class="track"
    class:active={active()}
    bind:this={trackEl}
    onmousedown={onMouseDown}
    onmouseenter={() => hovering = true}
    onmouseleave={() => hovering = false}
    onkeydown={onKeyDown}
    role="slider"
    tabindex="0"
    aria-valuenow={audioPlayer.position}
    aria-valuemin={0}
    aria-valuemax={getDuration()}
  >
    <div class="track-bg"></div>
    <div class="track-fill" style="width: {percent() * 100}%">
      <div class="handle" class:active={active()}></div>
    </div>
  </div>

  <span class="time">{formatDuration(getDuration())}</span>
</div>

<style>
  .seekbar-wrap {
    flex: 1;
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .time {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.3);
    white-space: nowrap;
    pointer-events: none;
    flex-shrink: 0;
  }

  .track {
    flex: 1;
    min-width: 0;
    height: 12px;
    display: flex;
    align-items: center;
    cursor: pointer;
    outline: none;
    user-select: none;
    position: relative;
  }

  .track-bg {
    position: absolute;
    left: 0;
    right: 0;
    height: 2px;
    border-radius: 99px;
    background: rgba(255, 255, 255, 0.12);
    transition: height 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .track.active .track-bg {
    height: 4px;
  }

  .track-fill {
    position: absolute;
    left: 0;
    height: 2px;
    border-radius: 99px;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 0 6px 1px rgba(255, 255, 255, 0.35);
    transition:
      height 0.2s cubic-bezier(0.34, 1.56, 0.64, 1),
      box-shadow 0.2s ease;
    display: flex;
    align-items: center;
    overflow: visible;
  }

  .track.active .track-fill {
    height: 4px;
    box-shadow: 0 0 10px 2px rgba(255, 255, 255, 0.5);
  }

  .handle {
    position: absolute;
    right: 0;
    top: 50%;
    transform: translate(50%, -50%) scale(0);
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 0 8px 2px rgba(255, 255, 255, 0.6);
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
    pointer-events: none;
  }

  .handle.active {
    transform: translate(50%, -50%) scale(1);
  }


</style>