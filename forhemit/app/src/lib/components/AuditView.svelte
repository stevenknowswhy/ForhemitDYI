<script lang="ts">
  // The audit panel: the session event log plus the hash-chain integrity
  // check. Tampering surfaces as a loud error, never a silent repair.
  import { api } from "../api";
  import { timeLabel } from "./bits";
  import LayerChip from "./LayerChip.svelte";
  import type { AuditLogLine, VerifyView } from "../types";

  let { onHome }: { onHome: () => void } = $props();

  let lines = $state<AuditLogLine[]>([]);
  let verify = $state<VerifyView | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      lines = await api.auditRecent();
    } catch (e) {
      error = String(e);
    }
  }

  async function runVerify() {
    error = null;
    busy = true;
    try {
      verify = await api.auditVerify();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  $effect(() => {
    void load();
  });
</script>

<div class="audit">
  <h2>Audit trail</h2>
  <p class="intro">
    Every meaningful change appends an event to a local, hash-chained, append-only store.
    Corrections create additional events — history is never rewritten.
  </p>
  <LayerChip kind="system" label="Recorded by Forhemit — the system's own record of what it did" />

  <button type="button" class="primary" onclick={runVerify} disabled={busy}>Verify hash chain now</button>
  {#if verify}
    {#if verify.verified}
      <p class="ok">✅ {verify.detail} ({verify.event_count} events)</p>
    {:else}
      <p class="error" role="alert">
        ⚠️ TAMPERING DETECTED — {verify.detail} Do not trust historical records; the chain does not verify.
      </p>
    {/if}
  {/if}
  {#if error}<p class="error" role="alert">{error}</p>{/if}

  <table>
    <thead>
      <tr><th>When</th><th>Event</th><th>Engine</th><th>Object</th></tr>
    </thead>
    <tbody>
      {#each lines as line}
        <tr>
          <td>{timeLabel(line.at)}</td>
          <td><code>{line.event_type}</code></td>
          <td>{line.engine}</td>
          <td class="object">{line.object}</td>
        </tr>
      {/each}
      {#if lines.length === 0}
        <tr><td colspan="4">No events yet this session.</td></tr>
      {/if}
    </tbody>
  </table>

  <button type="button" class="secondary" onclick={onHome}>← Back to home</button>
</div>

<style>
  .audit { max-width: 900px; margin: 0 auto; }
  .intro { color: var(--text-2, #555); }
  .ok { color: #1d6b3c; font-weight: 500; }
  .error { color: #b3261e; font-weight: 500; }
  table { width: 100%; border-collapse: collapse; margin: 16px 0; font-size: 0.85rem; }
  th, td { text-align: left; padding: 6px 10px; border-bottom: 1px solid var(--border, #e2e2e4); }
  th { font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.03em; color: var(--text-3, #888); }
  td.object { max-width: 340px; overflow-wrap: anywhere; color: var(--text-2, #555); }
  button.primary, button.secondary { padding: 10px 18px; border-radius: 10px; font: inherit; cursor: pointer; border: 1px solid transparent; }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
</style>
