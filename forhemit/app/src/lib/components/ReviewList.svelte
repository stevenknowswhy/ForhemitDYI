<script lang="ts">
  // Screen 13 — the read-only review of the assembled draft plus the
  // engine-computed completeness indicator (six binary areas, no scores).
  import { buildContent, describeValue, type DraftState } from "../screens";
  import type { Completeness, DestinationContent } from "../types";
  import LayerChip from "./LayerChip.svelte";

  let { draft, completeness }: { draft: DraftState; completeness: Completeness | null } = $props();

  const rows = [
    { id: "financial_objective", title: "2. Financial outcome" },
    { id: "closing_proceeds", title: "3. Cash at closing" },
    { id: "future_income", title: "4. Income after closing" },
    { id: "ownership_participants", title: "5. Who should own" },
    { id: "employee_ownership_shape", title: "6. Ownership shape" },
    { id: "ownership_allocation", title: "7. Allocation" },
    { id: "owner_role", title: "8. Your role" },
    { id: "transition_timing", title: "9. Timeframe" },
  ];

  const areaLabels: Record<string, string> = {
    financial: "Financial (screens 2–4)",
    ownership: "Ownership (screens 5–7)",
    personal: "Personal (screen 8)",
    timing: "Timing (screen 9)",
    preservation: "Preservation (screen 10)",
    avoidances: "Avoidances (screen 11)",
  };

  // The rendered content; an incomplete draft renders as unanswered
  // fields rather than throwing — buildContent only refuses at save time.
  const content: DestinationContent = $derived(
    (() => {
      const built = buildContent(draft);
      if ("error" in built) {
        return built_content_fallback(draft);
      }
      return built.content;
    })(),
  );

  const allEstablished = $derived(
    completeness !== null && Object.values(completeness).every((state) => state === "established"),
  );

  function built_content_fallback(_draft: DraftState): DestinationContent {
    // Unreachable in practice (the walk gates the review screen); render
    // the draft as unanswered instead of lying about content.
    const built = buildContent({ ...draft, welcome: "not_sure_yet" });
    if ("error" in built) {
      // Loud, not silent — a rule the engine would refuse is a bug upstream.
      throw new Error(built.error);
    }
    return built.content;
  }
</script>

<div class="review">
  <LayerChip kind="owner" label="Everything below is what you said — owner-stated" />
  <ul class="rows">
    {#each rows as row}
      <li>
        <span class="row-title">{row.title}</span>
        <span class="row-value">{describeValue(row.id, content)}</span>
      </li>
    {/each}
    <li>
      <span class="row-title">10. What should remain true</span>
      <span class="row-value">{draft.preservation.selected.length > 0 ? draft.preservation.selected.length + " goal(s) selected" : "(none selected)"}</span>
    </li>
    <li>
      <span class="row-title">11. What to avoid</span>
      <span class="row-value">{draft.avoidances.selected.length > 0 ? draft.avoidances.selected.length + " avoidance(s) selected" : "(nothing specific)"}</span>
    </li>
    <li>
      <span class="row-title">12. Additional context</span>
      <span class="row-value">{draft.notes.trim() ? draft.notes.trim() : "(none)"}</span>
    </li>
  </ul>

  <div class="completeness">
    <LayerChip kind="system" label="Recorded by Forhemit — system-computed indicator" />
    {#if completeness}
      <p class="overall">
        {allEstablished
          ? "Destination ready for exploration — all six areas established."
          : "Working draft — at least one area is still open."}
      </p>
      <ul class="areas">
        {#each Object.entries(completeness) as [area, state]}
          <li class:established={state === "established"}>
            {state === "established" ? "✅" : "⬜"} {areaLabels[area] ?? area}
          </li>
        {/each}
      </ul>
    {:else}
      <p class="note">Complete the earlier screens to see the indicator.</p>
    {/if}
    <p class="note">“I'm not sure” answers count as not established — honest unknowns are shown, never filled in.</p>
  </div>
</div>

<style>
  .review { display: flex; flex-direction: column; gap: 10px; }
  .rows { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 6px; }
  .rows li { display: flex; gap: 12px; padding: 8px 12px; background: var(--surface-2, #f7f7f8); border-radius: 8px; font-size: 0.88rem; }
  .row-title { font-weight: 600; min-width: 220px; }
  .row-value { color: var(--text-2, #444); }
  .completeness { border: 1px solid var(--border, #ddd); border-radius: 10px; padding: 12px 16px; margin-top: 8px; }
  .overall { font-weight: 600; margin: 6px 0; }
  .areas { list-style: none; padding: 0; display: grid; grid-template-columns: 1fr 1fr; gap: 4px; font-size: 0.85rem; }
  .areas li:not(.established) { color: var(--text-3, #888); }
  .note { font-size: 0.78rem; color: var(--text-3, #777); }
</style>
