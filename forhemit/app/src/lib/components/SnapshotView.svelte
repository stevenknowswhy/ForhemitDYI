<script lang="ts">
  // Business Snapshot entry — owner-reported facts at the owner-reported
  // provenance level. Recording a fact records its FIRST version; a
  // revision creates a new version and never edits the old one.
  import { api } from "../api";
  import { answerSummary, reasonLabel, timeLabel } from "./bits";
  import LayerChip from "./LayerChip.svelte";
  import type { FactKind, FactValue, FactVersion } from "../types";

  let { onAudit }: { onAudit: () => void } = $props();

  let facts = $state<FactVersion[]>([]);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let kind = $state<FactKind>("industry");
  let textValue = $state("");
  // Number inputs bind numbers in Svelte 5 (or "" when empty) — the state type
  // must say so or arithmetic/trim on it throws inside the click handler.
  let numberValue = $state<number | "">("");
  let lowerValue = $state<number | "">("");
  let upperValue = $state<number | "">("");
  let definition = $state("");
  let revisingFactId = $state<string | null>(null);
  let reviseReason = $state("learned_something_new");

  const KINDS: { value: FactKind; label: string; shape: "text" | "number" | "range" }[] = [
    { value: "industry", label: "What the business does", shape: "text" },
    { value: "years_operating", label: "How long the business has operated", shape: "number" },
    { value: "revenue", label: "Annual revenue", shape: "range" },
    { value: "operating_cash_flow", label: "Annual operating cash flow (EBITDA)", shape: "range" },
    { value: "debt", label: "Outstanding debt", shape: "range" },
    { value: "employee_count", label: "Number of employees", shape: "number" },
    { value: "ownership_structure", label: "Ownership structure", shape: "text" },
  ];

  const currentKinds = $derived(new Set(facts.map((fact) => fact.kind)));

  async function load() {
    try {
      facts = await api.snapshotCurrent();
    } catch (e) {
      error = String(e);
    }
  }

  $effect(() => {
    void load();
  });

  function resetForm() {
    textValue = "";
    numberValue = "";
    lowerValue = "";
    upperValue = "";
    definition = "";
  }

  /** A number input's binding: a number once filled, "" when empty (Svelte 5
   *  may also hand us null on clear — treat every non-finite as empty). */
  function boundNumber(value: number | "" | null | undefined): number | null {
    return typeof value === "number" && Number.isFinite(value) ? value : null;
  }

  function buildValue(): FactValue | null {
    const shape = KINDS.find((entry) => entry.value === kind)?.shape;
    if (shape === "text") {
      return textValue.trim() ? { text: textValue.trim() } : null;
    }
    if (shape === "number") {
      const parsed = boundNumber(numberValue);
      return parsed !== null ? { number: parsed } : null;
    }
    const lower = boundNumber(lowerValue);
    const upper = boundNumber(upperValue);
    if (lower === null && upper === null) return null;
    return { range: { lower, upper } };
  }

  async function record() {
    error = null;
    const value = buildValue();
    if (value === null) {
      error = "Enter a value that fits the kind (text, a number, or a range with at least one bound).";
      return;
    }
    busy = true;
    try {
      if (revisingFactId) {
        facts = await api.snapshotRevise(revisingFactId, value, definition.trim() || null, reviseReason);
        revisingFactId = null;
      } else {
        facts = await api.snapshotRecord(kind, value, definition.trim() || null);
      }
      resetForm();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function valueLabel(fact: FactVersion): string {
    if ("text" in fact.value) return fact.value.text;
    if ("number" in fact.value) {
      const kindShape = KINDS.find((entry) => entry.value === fact.kind);
      return kindShape?.value === "revenue" || kindShape?.value === "debt" ||
          kindShape?.value === "operating_cash_flow" || fact.kind === "employee_count"
        ? Number(fact.value.number).toLocaleString("en-US")
        : String(fact.value.number);
    }
    const lower = fact.value.range.lower !== null ? `$${fact.value.range.lower.toLocaleString("en-US")}` : "up to";
    const upper = fact.value.range.upper !== null ? `$${fact.value.range.upper.toLocaleString("en-US")}` : "and up";
    return `${lower} – ${upper}`;
  }

  const selectedShape = $derived(KINDS.find((entry) => entry.value === kind)?.shape ?? "text");
</script>

<div class="snapshot">
  <h2>Business Snapshot</h2>
  <p class="intro">
    Facts about your business as <strong>you</strong> state them. Nothing here is verified in v1 —
    verification levels activate later with vault intelligence. Revising a fact keeps every version.
  </p>
  <LayerChip kind="owner" label="You said this — owner-reported, not yet verified" />

  {#if error}<p class="error" role="alert">{error}</p>{/if}

  <div class="form">
    {#if revisingFactId}
      <div class="revise-note">
        Revising a fact — a new version is created; the old one is kept.
        <label class="field">
          Why are you changing it?
          <select bind:value={reviseReason}>
            <option value="learned_something_new">I learned something new</option>
            <option value="priorities_changed">My priorities changed</option>
            <option value="professional_suggested_another_approach">My professional suggested another approach</option>
            <option value="business_changed">The business changed</option>
            <option value="exploring_different_outcome">I want to explore a different outcome</option>
            <option value="other">Other</option>
          </select>
        </label>
      </div>
    {:else}
      <label class="field">
        What are you recording?
        <select bind:value={kind}>
          {#each KINDS as entry}
            <option value={entry.value} disabled={currentKinds.has(entry.value) && !revisingFactId}>
              {entry.label}{currentKinds.has(entry.value) ? " (already recorded — revise below)" : ""}
            </option>
          {/each}
        </select>
      </label>
    {/if}

    {#if selectedShape === "text"}
      <label class="field">Your answer <input type="text" bind:value={textValue} placeholder="in your own words" /></label>
    {:else if selectedShape === "number"}
      <label class="field">Number <input type="number" min="0" bind:value={numberValue} /></label>
    {:else}
      <div class="range-row">
        <label>Lower bound ($) <input type="number" min="0" bind:value={lowerValue} placeholder="open if blank" /></label>
        <label>Upper bound ($) <input type="number" min="0" bind:value={upperValue} placeholder="open if blank" /></label>
      </div>
    {/if}
    <label class="field">Your own definition label (optional — “what this figure means to you”) <input type="text" bind:value={definition} placeholder="e.g. cash basis revenue" /></label>
    <button type="button" class="primary" onclick={record} disabled={busy}>
      {revisingFactId ? "Record new fact version" : "Record fact"}
    </button>
    {#if revisingFactId}
      <button type="button" class="secondary" onclick={() => { revisingFactId = null; resetForm(); }}>Cancel revision</button>
    {/if}
  </div>

  <div class="facts">
    <h3>Current snapshot</h3>
    {#if facts.length === 0}
      <p class="note">No facts recorded yet — that is fine. The journey works with whatever you choose to state.</p>
    {:else}
      <ul>
        {#each facts as fact}
          <li>
            <div class="fact-head">
              <strong>{KINDS.find((entry) => entry.value === fact.kind)?.label ?? fact.kind}</strong>
              <span class="fact-value">{valueLabel(fact)}</span>
              <button type="button" class="link" onclick={() => {
                revisingFactId = fact.fact_id;
                kind = fact.kind;
                if ("text" in fact.value) textValue = fact.value.text;
                if ("number" in fact.value) numberValue = fact.value.number;
                if ("range" in fact.value) {
                  lowerValue = fact.value.range.lower ?? "";
                  upperValue = fact.value.range.upper ?? "";
                }
              }}>Revise</button>
            </div>
            <span class="fact-meta">
              You said this · unverified · recorded {timeLabel(fact.recorded_at)}
              {#if fact.definition}· your definition: “{fact.definition}”{/if}
              {#if fact.supersedes}· supersedes a previous version (kept in history){/if}
            </span>
          </li>
        {/each}
      </ul>
      <LayerChip kind="system" label="Version history and provenance stamps — recorded by Forhemit" />
    {/if}
  </div>

  <button type="button" class="secondary" onclick={onAudit}>Open the audit trail →</button>
</div>

<style>
  .snapshot { max-width: 760px; margin: 0 auto; }
  .intro { color: var(--text-2, #555); }
  .error { color: #b3261e; }
  .form { background: var(--surface-2, #f7f7f8); border-radius: 10px; padding: 14px 16px; margin: 14px 0; display: flex; flex-direction: column; gap: 4px; }
  .revise-note { font-size: 0.88rem; background: #fdf3e7; border: 1px solid #e5c9a3; border-radius: 8px; padding: 10px 12px; display: flex; flex-direction: column; gap: 8px; }
  .field { display: flex; flex-direction: column; gap: 4px; font-size: 0.88rem; }
  .field input, .field select { padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); font: inherit; }
  .range-row { display: flex; gap: 12px; flex-wrap: wrap; }
  .range-row label { display: flex; flex-direction: column; gap: 4px; font-size: 0.88rem; }
  .facts ul { list-style: none; padding: 0; display: flex; flex-direction: column; gap: 10px; }
  .facts li { background: var(--surface-2, #f7f7f8); border-radius: 8px; padding: 10px 14px; }
  .fact-head { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
  .fact-value { color: var(--text-2, #333); font-weight: 500; }
  .fact-meta { display: block; font-size: 0.78rem; color: var(--text-3, #888); margin-top: 2px; }
  .note { color: var(--text-3, #888); font-size: 0.85rem; }
  button.primary, button.secondary { padding: 10px 18px; border-radius: 10px; font: inherit; cursor: pointer; border: 1px solid transparent; }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  button.link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 4px; }
</style>
