<script lang="ts">
  let {
    title,
    count,
  }: {
    title: string;
    count: number;
  } = $props();
</script>

<div class="sectionHeader">
  <span class="sectionTitle">{title}</span>
  <span class="sectionCount">{count}</span>
</div>

<style>
  .sectionHeader {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 10px;
    padding-left: 4px;
  }

  .sectionTitle {
    font-size: 13px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--color-navbar-label);
  }

  .sectionCount {
    font-size: 12px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.2);
  }
</style>
