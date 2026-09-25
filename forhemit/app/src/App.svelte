<script lang="ts">
  // App router — the shell around the walks. Views: home (destination
  // summary + version history), builder (13 screens), journey, snapshot,
  // audit. All domain logic lives in the engine crates; every state change
  // here happened through a command adapter, which emitted audit events.
  import { api } from "./lib/api";
  import { describeValue, draftFromContent, SCREENS } from "./lib/screens";
  import { reasonLabel, statusClass, statusLabel, timeLabel } from "./lib/components/bits";
  import DestinationBuilder from "./lib/components/DestinationBuilder.svelte";
  import JourneyWalk from "./lib/components/JourneyWalk.svelte";
  import SnapshotView from "./lib/components/SnapshotView.svelte";
  import AuditView from "./lib/components/AuditView.svelte";
  import LayerChip from "./lib/components/LayerChip.svelte";
  import type { Destination, DestinationVersion } from "./lib/types";

  type View = "home" | "builder" | "journey" | "snapshot" | "audit";

  let view = $state<View>("home");
  let destination = $state<Destination | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let editing = $state(false);
  let browsingVersionId = $state<string | null>(null);

  const headVersion = $derived(destination ? destination.versions[destination.versions.length - 1] : null);
  const browsingVersion = $derived(
    destination?.versions.find((version) => version.version_id === browsingVersionId) ?? null,
  );

  async function loadDestination() {
    loading = true;
    error = null;
    try {
      destination = await api.destinationGet();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void loadDestination();
  });

  function openBuilderForEdit(version: DestinationVersion) {
    editing = true;
    browsingVersionId = null;
    builderSeed = version.content;
    view = "builder";
  }

  let builderSeed = $state<import("./lib/types").DestinationContent | null>(null);

  function openBuilderFresh() {
    editing = false;
    builderSeed = null;
    view = "builder";
  }

  async function confirmCurrent() {
    if (!destination) return;
    error = null;
    try {
      destination = await api.destinationConfirm();
    } catch (e) {
      error = String(e);
    }
  }

  async function markWorking() {
    if (!destination) return;
    error = null;
    try {
      destination = await api.destinationMarkWorking();
    } catch (e) {
      error = String(e);
    }
  }

  async function archive() {
    if (!destination) return;
    error = null;
    try {
      destination = await api.destinationArchive();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<header class="topbar">
  <span class="brand">Forhemit</span>
  <span class="tagline">Explore selling to your employees — offline, on your machine</span>
  <nav>
    <button type="button" class:active={view === "home"} onclick={() => { view = "home"; void loadDestination(); }}>Home</button>
    {#if destination}
      <button type="button" class:active={view === "journey"} onclick={() => (view = "journey")}>Journey</button>
      <button type="button" class:active={view === "snapshot"} onclick={() => (view = "snapshot")}>Business Snapshot</button>
    {/if}
    <button type="button" class:active={view === "audit"} onclick={() => (view = "audit")}>Audit</button>
  </nav>
</header>

<main>
  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  {#if view === "builder"}
    <DestinationBuilder
      seed={builderSeed}
      onDone={(updated) => { destination = updated; editing = false; view = "home"; }}
      onHome={() => { view = "home"; void loadDestination(); }}
    />
  {:else if view === "journey"}
    <JourneyWalk onSnapshot={() => (view = "snapshot")} />
  {:else if view === "snapshot"}
    <SnapshotView onAudit={() => (view = "audit")} />
  {:else if view === "audit"}
    <AuditView onHome={() => { view = "home"; void loadDestination(); }} />
  {:else if loading}
    <p class="loading">Opening your workspace…</p>
  {:else if destination === null}
    <section class="welcome">
      <h1>Start where every journey starts — your destination</h1>
      <p>
        You'll walk a short series of screens about the outcome you want to create. Nothing is
        sent anywhere: everything stays on this machine, and every version you create is kept.
      </p>
      <button type="button" class="primary big" onclick={openBuilderFresh}>
        Begin the Destination Builder →
      </button>
    </section>
  {:else if headVersion}
    <section class="summary">
      <div class="head-row">
        <h1>Your destination</h1>
        <span class="status {statusClass(destination.status)}">{statusLabel(destination.status)}</span>
      </div>
      <p class="head-content">{describeValue("financial_objective", headVersion.content)}</p>

      <div class="actions">
        <button type="button" class="primary" onclick={() => (view = "journey")}>Continue the journey →</button>
        <button type="button" class="secondary" onclick={() => openBuilderForEdit(headVersion)}>
          Edit destination (creates a new version)
        </button>
        {#if destination.status !== "confirmed"}
          <button type="button" class="secondary" onclick={confirmCurrent}>Mark confirmed</button>
        {/if}
        {#if destination.status !== "working"}
          <button type="button" class="secondary" onclick={markWorking}>Mark working draft</button>
        {/if}
        {#if destination.status !== "archived"}
          <button type="button" class="secondary" onclick={archive}>Archive</button>
        {/if}
      </div>

      <h2>Version history — browseable, never destructive</h2>
      <LayerChip kind="system" label="Recorded by Forhemit — every version retained; edits create versions, never rewrites" />
      <ul class="history">
        {#each destination.versions as version}
          <li>
            <div class="version-head">
              <strong>Version {version.version_number}</strong>
              {#if version.change}
                <span class="change-meta">
                  changed — {reasonLabel(version.change.reason)}
                  {#if version.change.explanation}· “{version.change.explanation}”{/if}
                  {#if version.change.changed_fields.length > 0}
                    · fields: {version.change.changed_fields.join(", ")}
                  {/if}
                </span>
              {:else}
                <span class="change-meta">initial version</span>
              {/if}
              <span class="change-meta">{timeLabel(version.created_at)}</span>
              <button type="button" class="link" onclick={() => (browsingVersionId = version.version_id)}>
                View
              </button>
            </div>
          </li>
        {/each}
      </ul>

      {#if browsingVersion}
        <div class="version-view">
          <h3>Version {browsingVersion.version_number} — read-only view</h3>
          <ul>
            {#each SCREENS.filter((screen) => !["welcome", "review"].includes(screen.id)) as screen}
              <li><strong>{screen.title}:</strong> {describeValue(screen.id, browsingVersion.content)}</li>
            {/each}
          </ul>
          <button type="button" class="secondary" onclick={() => (browsingVersionId = null)}>Close</button>
        </div>
      {/if}
    </section>
  {/if}
</main>

<footer class="footer">
  <LayerChip kind="system" label="Whose information am I looking at? Owner-stated vs system-recorded is labeled everywhere" />
</footer>

<style>
  .topbar {
    display: flex; align-items: center; gap: 16px; padding: 10px 20px;
    border-bottom: 1px solid var(--border, #e2e2e4); flex-wrap: wrap;
  }
  .brand { font-weight: 700; letter-spacing: 0.01em; }
  .tagline { color: var(--text-3, #888); font-size: 0.8rem; flex: 1; }
  nav { display: flex; gap: 6px; }
  nav button {
    border: none; background: none; font: inherit; padding: 6px 12px;
    border-radius: 8px; cursor: pointer; color: var(--text-2, #555);
  }
  nav button.active { background: var(--accent-soft, #e3efe7); color: var(--accent, #2f6f4f); font-weight: 600; }
  main { padding: 24px 20px; max-width: 980px; margin: 0 auto; }
  .welcome { text-align: center; margin-top: 8vh; display: flex; flex-direction: column; gap: 16px; align-items: center; }
  .welcome p { max-width: 540px; color: var(--text-2, #555); }
  button.primary, button.secondary {
    padding: 10px 18px; border-radius: 10px; font: inherit; cursor: pointer; border: 1px solid transparent;
  }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.big { padding: 14px 26px; font-size: 1.05rem; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  .link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 4px; }
  .summary h1 { margin-bottom: 0; }
  .head-row { display: flex; align-items: center; gap: 14px; }
  .status { font-size: 0.78rem; padding: 3px 12px; border-radius: 999px; border: 1px solid var(--border, #ccc); }
  .status-confirmed { background: #e3efe7; border-color: #9dc3aa; color: #1d6b3c; font-weight: 600; }
  .status-working { background: #fdf3e7; border-color: #e5c9a3; color: #8a5a12; }
  .status-archived { background: #f0f0f1; color: #777; }
  .status-draft { background: #eef2f7; border-color: #b9c8dd; color: #33557e; }
  .head-content { color: var(--text-2, #555); }
  .actions { display: flex; gap: 10px; flex-wrap: wrap; margin: 16px 0 28px; }
  .history { list-style: none; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  .history li { background: var(--surface-2, #f7f7f8); border-radius: 8px; padding: 10px 14px; }
  .version-head { display: flex; gap: 12px; align-items: baseline; flex-wrap: wrap; }
  .change-meta { font-size: 0.8rem; color: var(--text-3, #888); }
  .version-view { border: 1px solid var(--border, #ddd); border-radius: 10px; padding: 14px 18px; margin-top: 14px; }
  .version-view ul { list-style: none; padding: 0; font-size: 0.88rem; display: flex; flex-direction: column; gap: 6px; }
  .error { color: #b3261e; font-weight: 500; }
  .loading { color: var(--text-3, #888); }
  .footer { padding: 14px 20px 28px; display: flex; justify-content: center; }
</style>
