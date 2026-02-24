<script lang="ts">
    import type { ClassNameValue } from "tailwind-merge";


    let {
        class: className = '',
        Icon,
        onclick
    } : {
        class?: string;
        Icon: any;
        onclick: (e: MouseEvent) => void;
    } = $props();


    function handleClick(e: MouseEvent) {
        e.stopPropagation(); 
        onclick(e);
    }
</script>

<button class="btn {className}" onclick={handleClick}>
    <Icon />
</button>

<style>
    .btn {
        display: none;
        justify-content: center;
        align-items: center;
        background-color: rgba(255, 255, 255, 0.2);
        backdrop-filter: blur(12px);
        border-radius: 50%;
        min-height: 35px;
        min-width: 35px;
        position: absolute;
        bottom: 0;
        z-index: 100;
        margin: 10px;

        transition: background-color 0.15s ease;
    }

    .btn:hover {
        background-color: var(--color-accent);
    }
</style>