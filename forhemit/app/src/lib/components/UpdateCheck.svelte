<script lang="ts">
  import { api } from "../api";
  import type { UpdateOutcome } from "../types";

  // The updater exists only in the desktop shell: the dev harness has no
  // updater plugin, and the browser build must not grow a network path.
  const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

  let checking = $state(false);
  let installing = $state(false);
  let outcome = $state<UpdateOutcome | null>(null);
  let installNote = $state<string | null>(null);

  const refusalText: Record<string, string> = {
    malformed: "the update information is malformed",
    not_newer: "the offered update is not newer than this app",
    unsigned: "the offered update is unsigned — refusing before any download",
    bad_signature: "the update signature is not valid",
    missing_platform: "no update is offered for this platform",
    bad_pub_date: "the update's publish date is invalid",
  };

  async function check(): Promise<void> {
    checking = true;
    outcome = null;
    installNote = null;
    try {
      outcome = await api.updateCheck();
    } catch (error) {
      outcome = { outcome: "unavailable", detail: String(error) };
    } finally {
      checking = false;
    }
  }

  async function install(): Promise<void> {
    if (outcome?.outcome !== "available") return;
    installing = true;
    try {
      await api.updateInstall();
      // On Windows the installer exits the app during install; elsewhere
      // the owner restarts when convenient.
      installNote = "update installed — restart Forhemit to apply it";
    } catch (error) {
      installNote = `update install failed: ${String(error)}`;
    } finally {
      installing = false;
    }
  }
</script>

{#if inTauri}
  <span class="updater">
    {#if checking}
      <span class="muted">checking for updates…</span>
    {:else}
      <button type="button" class="link" onclick={check}>Check for updates</button>
    {/if}
    {#if outcome}
      {#if outcome.outcome === "up_to_date"}
        <span class="muted">up to date (v{outcome.current_version})</span>
      {:else if outcome.outcome === "available"}
        <span class="muted">
          v{outcome.version} available
          {#if installing}
            <span class="muted">installing…</span>
          {:else}
            <button type="button" class="link" onclick={install}>install</button>
          {/if}
        </span>
      {:else if outcome.outcome === "refused"}
        <span class="refused">
          update refused — {refusalText[outcome.refusal.reason] ?? outcome.refusal.reason}
        </span>
      {:else}
        <span class="muted">no update information — {outcome.detail}</span>
      {/if}
    {/if}
    {#if installNote}
      <span class="muted">{installNote}</span>
    {/if}
  </span>
{/if}

<style>
  .updater {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    font-size: 0.8rem;
  }
  .muted {
    color: var(--text-3, #888);
  }
  .refused {
    color: #b3261e;
  }
  .link {
    border: none;
    background: none;
    font: inherit;
    font-size: 0.8rem;
    padding: 0;
    cursor: pointer;
    color: var(--accent, #2f6f4f);
    text-decoration: underline;
  }
  .link:disabled {
    cursor: default;
    color: var(--text-3, #888);
  }
</style>
