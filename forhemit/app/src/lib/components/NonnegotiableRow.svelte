<script lang="ts">
  // The three-state preference strength + "no designation" default.
  // NONNEGOTIABLE doc §12/§16: the system defaults to Unspecified and
  // never guesses importance.
  import { PREFERENCE_OPTIONS } from "../screens";
  import type { NonnegotiableState } from "../types";

  let { pref = $bindable("unspecified") }: { pref?: NonnegotiableState } = $props();
</script>

<div class="pref-row">
  <span class="pref-title">How strongly do you hold this?</span>
  <div class="pref-options" role="radiogroup" aria-label="Preference strength">
    {#each PREFERENCE_OPTIONS as option}
      <button
        type="button"
        role="radio"
        aria-checked={pref === option.value}
        class="pref-option"
        class:active={pref === option.value}
        onclick={() => (pref = option.value)}
      >
        <span aria-hidden="true">{option.dot}</span> {option.label}
      </button>
    {/each}
  </div>
  <span class="pref-detail">
    {PREFERENCE_OPTIONS.find((option) => option.value === pref)?.detail}
  </span>
</div>

<style>
  .pref-row { display: flex; flex-direction: column; gap: 6px; margin: 14px 0; font-size: 0.85rem; }
  .pref-title { font-weight: 600; font-size: 0.82rem; }
  .pref-options { display: flex; gap: 6px; flex-wrap: wrap; }
  .pref-option {
    padding: 5px 12px; border-radius: 999px; border: 1px solid var(--border, #bbb);
    background: var(--surface, #fff); cursor: pointer; font: inherit; font-size: 0.8rem;
  }
  .pref-option.active { background: var(--accent-soft, #e3efe7); border-color: var(--accent, #2f6f4f); font-weight: 600; }
  .pref-detail { color: var(--text-3, #777); font-size: 0.78rem; }
</style>
