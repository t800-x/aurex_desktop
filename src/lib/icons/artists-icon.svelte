<script lang="ts">
  let { className = "", size = 25 }: { className?: string; size?: number } = $props();
</script>

<i class="icon-font icon {className}" style="font-size: {size}px;">{"\uF725"}</i>