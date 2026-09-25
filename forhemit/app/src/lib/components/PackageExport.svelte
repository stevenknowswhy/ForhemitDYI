<script lang="ts">
  // Package export flow — readiness gating shown, provenance visible before
  // export, export → saved file. The preview is the same contract-typed
  // snapshot the engine assembles; the UI adds nothing and hides nothing.
  // Every screen answers "Whose information am I looking at?" via the
  // provenance block, and the package carries the not-advice disclosure.
  import { api } from "../api";
  import LayerChip from "./LayerChip.svelte";
  import { timeLabel } from "./bits";
  import type { PackagePreviewView } from "../types";

  let preview = $state<PackagePreviewView | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let exporting = $state(false);
  let format = $state<"html" | "pdf">("html");

  async function loadPreview() {
    loading = true;
    error = null;
    try {
      preview = await api.packagePreview();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void loadPreview();
  });

  async function exportPackage() {
    exporting = true;
    error = null;
    notice = null;
    try {
      const saved = await api.packageExport(format);
      notice = `Saved “${saved.file_name}” in ${saved.saved_path} — this file is yours to share; nothing was uploaded.`;
      await loadPreview();
    } catch (e) {
      error = String(e);
    } finally {
      exporting = false;
    }
  }
</script>

<section class="package">
  <header class="package-head">
    <h1>Professional Review Package</h1>
    <p class="lede">
      Assembles what you have built — destination, Business Reality, journey,
      scenarios — into one file a professional can review. Readiness gates
      decide what is included; exclusions are shown, never hidden.
    </p>
    <LayerChip kind="system" label="Export saves a file on this device — v1 sends nothing anywhere" />
  </header>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}
  {#if notice}
    <p class="notice">{notice}</p>
  {/if}

  {#if loading}
    <p class="loading">Assembling the snapshot…</p>
  {:else if preview === null}
    <p class="error" role="alert">The package snapshot could not be assembled.</p>
  {:else}
    <section class="panel provenance-panel">
      <h2>Provenance — whose information is this?</h2>
      <dl class="prov-grid">
        <div>
          <dt>Destination</dt>
          <dd>v{preview.provenance.destination_version_number}</dd>
        </div>
        <div>
          <dt>Journey version used</dt>
          <dd>{preview.provenance.journey_version_used}</dd>
        </div>
        <div>
          <dt>Business Reality version</dt>
          <dd class="mono">{preview.provenance.business_reality_version_id.slice(0, 12)}…</dd>
        </div>
        <div>
          <dt>Assembled</dt>
          <dd>{timeLabel(preview.created_at)}</dd>
        </div>
      </dl>
    </section>

    <section class="panel">
      <h2>Included in the package</h2>
      <ul class="sections">
        {#each preview.sections as section (section.number)}
          <li>
            <strong>{section.title}</strong>
            <p class="meta">{section.blocks.length} {section.blocks.length === 1 ? "entry" : "entries"}</p>
          </li>
        {/each}
      </ul>

      <h3>Scenarios — included on readiness, not on merit</h3>
      {#if preview.provenance.included_scenario_version_ids.length === 0}
        <p class="empty">
          No scenario is ready enough to include. Build one to
          READY_FOR_PROFESSIONAL_REVIEW on the Scenarios screen.
        </p>
      {:else}
        <ul class="scenarios">
          {#each preview.provenance.included_scenario_version_ids as scenarioVersionId (scenarioVersionId)}
            <li><span class="mono">{scenarioVersionId.slice(0, 12)}…</span> — ready for professional review</li>
          {/each}
        </ul>
      {/if}
    </section>

    <section class="panel">
      <h2>Excluded — and why</h2>
      <p class="hint">
        Exclusions are recorded, not hidden: a professional sees what was not
        ready and why it was left out.
      </p>
      {#if preview.provenance.excluded_scenarios.length === 0}
        <p class="empty">Nothing excluded.</p>
      {:else}
        <ul class="exclusions">
          {#each preview.provenance.excluded_scenarios as exclusion (exclusion.scenario_version_id)}
            <li><strong>{exclusion.name}</strong> — {exclusion.readiness}</li>
          {/each}
        </ul>
      {/if}
    </section>

    <section class="panel export-panel">
      <h2>Export</h2>
      <p class="hint">
        The export saves one file on this device. Sharing it with a
        professional is your move — v1 sends nothing anywhere.
      </p>
      <div class="export-row">
        <select bind:value={format}>
          <option value="html">HTML (opens in any browser)</option>
          <option value="pdf">PDF (print-ready)</option>
        </select>
        <button type="button" class="primary" onclick={exportPackage} disabled={exporting}>
          {exporting ? "Exporting…" : `Export ${format.toUpperCase()}`}
        </button>
      </div>
      <p class="disclosure">
        This package is prepared information for professional review. It is
        not financial, legal, tax, or valuation advice, and it does not
        recommend any transaction.
      </p>
    </section>
  {/if}
</section>

<style>
  .package { display: flex; flex-direction: column; gap: 18px; }
  .package-head h1 { margin-bottom: 4px; }
  .lede { color: var(--text-2, #555); max-width: 640px; }
  .panel { border: 1px solid var(--border, #ddd); border-radius: 10px; padding: 14px 18px; }
  .panel h2 { margin-top: 0; }
  .prov-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 12px; margin: 0; }
  .prov-grid dt { font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.04em; color: var(--text-3, #888); }
  .prov-grid dd { margin: 2px 0 0; font-weight: 600; }
  .sections, .scenarios, .exclusions { list-style: none; padding: 6px 0 0; display: flex; flex-direction: column; gap: 10px; }
  .sections li { border-left: 3px solid var(--accent, #2f6f4f); padding-left: 10px; }
  .exclusions li { border-left: 3px solid #e5c9a3; padding-left: 10px; }
  .meta { color: var(--text-3, #888); font-size: 0.8rem; margin: 2px 0 0; }
  .mono { font-family: ui-monospace, monospace; font-size: 0.85em; }
  .hint { color: var(--text-2, #555); font-size: 0.9rem; }
  .empty { color: var(--text-3, #888); }
  .export-row { display: flex; gap: 8px; align-items: center; max-width: 560px; }
  .export-row select { flex: 1; font: inherit; padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); }
  button.primary { padding: 8px 16px; border-radius: 8px; font: inherit; cursor: pointer; border: 1px solid transparent; background: var(--accent, #2f6f4f); color: white; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  .disclosure { font-size: 0.85rem; color: var(--text-2, #555); border-left: 3px solid #e5c9a3; padding-left: 10px; margin-bottom: 0; }
  .error { color: #b3261e; font-weight: 500; }
  .notice { color: #1d6b3c; }
  .loading { color: var(--text-3, #888); }
</style>
