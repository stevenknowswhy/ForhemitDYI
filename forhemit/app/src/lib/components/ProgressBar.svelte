<script lang="ts">
  // Visible progress — Destination Builder doc requires it on every screen.
  let { done, total, label }: { done: number; total: number; label?: string } = $props();
  const pct = $derived(total > 0 ? Math.round((done / total) * 100) : 0);
</script>

<div class="progress" role="progressbar" aria-valuenow={done} aria-valuemin={0} aria-valuemax={total}>
  <div class="bar"><div class="fill" style="width: {pct}%"></div></div>
  {#if label}<span class="label">{label}</span>{/if}
</div>

<style>
  .progress { display: flex; align-items: center; gap: 12px; margin-bottom: 18px; }
  .bar { flex: 1; height: 8px; background: var(--surface-2, #e8e8ea); border-radius: 999px; overflow: hidden; }
  .fill { height: 100%; background: var(--accent, #2f6f4f); transition: width 0.2s ease; }
  .label { font-size: 0.8rem; color: var(--text-2, #666); white-space: nowrap; }
</style>
