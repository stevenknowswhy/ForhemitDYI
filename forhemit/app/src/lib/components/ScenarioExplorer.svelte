<script lang="ts">
  // Scenario explorer — families and versions browseable, what-if branch
  // creation, conflict surfacing per the invariant: the owner decides;
  // the UI never filters, scores, or auto-relaxes anything. Comparison
  // renders factual statuses only — never a winner (schema doc §33).
  // All domain rules live in the scenario engine; this screen only builds
  // the wire objects the shell validates.
  import { api } from "../api";
  import { timeLabel } from "./bits";
  import LayerChip from "./LayerChip.svelte";
  import {
    ASSUMPTION_CATEGORIES,
    BRANCH_TYPES,
    COMPARISON_OUTCOMES,
    CONFLICT_SEVERITIES,
    CONFLICT_TYPES,
    DIMENSION_TYPES,
    OWNER_DECISIONS,
    PROVENANCES,
    READINESS_LEVELS,
    SCENARIO_TYPES,
    UNKNOWN_IMPORTANCES,
    UNKNOWN_STATUSES,
    VALUE_TYPES,
    VERIFICATIONS,
    lifecycleLabel,
    outcomeLabel,
    readinessLabel,
    vocabLabel,
  } from "../vocab";
  import type {
    ComparisonView,
    DestinationVersion,
    FactVersion,
    ScenarioFamilyView,
    ScenarioVersionView,
    WireValue,
  } from "../types";

  let families = $state<ScenarioFamilyView[]>([]);
  let destinationVersions = $state<DestinationVersion[]>([]);
  let facts = $state<FactVersion[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let selectedVersionId = $state<string | null>(null);
  let selectedVersion = $state<ScenarioVersionView | null>(null);
  let comparison = $state<ComparisonView | null>(null);

  // Create-family form. The engine pins the destination version and the
  // exact Business Reality fact versions (integrity rules 1–2), so the
  // picker lists what the destination and snapshot already hold.
  let createName = $state("");
  let createType = $state("esop");
  let createDescription = $state("");
  let createDestinationVersion = $state("");
  let createFactIds = $state<string[]>([]);

  // Draft-editor forms.
  let assumptionCategory = $state("financial");
  let assumptionName = $state("");
  let assumptionType = $state("currency");
  let assumptionValue = $state("");
  let assumptionCurrency = $state("USD");
  let assumptionProvenance = $state("scenario_assumed");
  let assumptionVerification = $state("unverified");
  let assumptionSource = $state("");

  let unknownDescription = $state("");
  let unknownImportance = $state("important");
  let unknownAction = $state("");

  let conflictType = $state("owner_objective");
  let conflictSeverity = $state("attention");
  let conflictDescription = $state("");
  let conflictDecisions = $state<Record<string, string>>({});

  let branchType = $state("what_if");
  let branchName = $state("");
  let branchReason = $state("");
  let branchOverrides = $state<string[]>([]);

  let comparisonName = $state("");
  let comparisonVersionIds = $state<string[]>([]);
  let comparisonDimension = $state("financial");
  let comparisonDimensionLabel = $state("");
  let comparisonCells = $state<Record<string, string>>({});

  function wireValue(): WireValue {
    return {
      value_type: assumptionType as WireValue["value_type"],
      value: assumptionValue.trim(),
      currency: assumptionType === "currency" ? assumptionCurrency.trim() || "USD" : null,
    };
  }

  async function loadAll() {
    loading = true;
    error = null;
    try {
      const [familyList, destination] = await Promise.all([
        api.scenarioFamilies(),
        api.destinationGet(),
      ]);
      families = familyList;
      destinationVersions = destination?.versions ?? [];
      facts = await api.snapshotCurrent();
      if (destinationVersions.length > 0 && !createDestinationVersion) {
        createDestinationVersion = destinationVersions[destinationVersions.length - 1].version_id;
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void loadAll();
  });

  async function selectVersion(versionId: string) {
    error = null;
    comparison = null;
    try {
      selectedVersion = await api.scenarioVersionView(versionId);
      selectedVersionId = versionId;
      conflictDecisions = {};
      branchOverrides = [];
      branchName = "";
      branchReason = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function refreshVersion(versionId: string) {
    try {
      selectedVersion = await api.scenarioVersionView(versionId);
      await loadAll();
    } catch (e) {
      error = String(e);
    }
  }

  function selectHeadVersion(family: ScenarioFamilyView) {
    const head = family.versions[family.versions.length - 1];
    if (head) void selectVersion(head.scenario_version_id);
  }

  async function createFamily() {
    error = null;
    try {
      const created = await api.scenarioCreate({
        name: createName.trim(),
        scenario_type: createType,
        description: createDescription.trim() || null,
        destination_version_id: createDestinationVersion,
        reality_fact_version_ids: createFactIds,
      });
      createName = "";
      createDescription = "";
      createFactIds = [];
      await loadAll();
      await selectVersion(created.version.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function addAssumption() {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioAddAssumption(selectedVersion.scenario_version_id, {
        category: assumptionCategory,
        name: assumptionName.trim(),
        description: null,
        value: wireValue(),
        provenance: assumptionProvenance,
        verification: assumptionVerification,
        nonnegotiable_objective_id: null,
        source_reference: assumptionSource.trim() || null,
      });
      assumptionName = "";
      assumptionValue = "";
      assumptionSource = "";
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function addUnknown() {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioAddUnknown(selectedVersion.scenario_version_id, {
        category: null,
        description: unknownDescription.trim(),
        importance: unknownImportance,
        required_action: unknownAction.trim() || null,
        source_dependency: null,
      });
      unknownDescription = "";
      unknownAction = "";
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function resolveUnknown(unknownId: string, status: string) {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioResolveUnknown(
        selectedVersion.scenario_version_id,
        unknownId,
        status,
        null,
      );
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function addNonnegotiableTest(objectiveId: string, description: string) {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioAddNonnegotiable(selectedVersion.scenario_version_id, {
        destination_objective_id: objectiveId,
        destination_version_id: selectedVersion.destination_version_id,
        description,
      });
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function recordConflict() {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioRecordConflict(selectedVersion.scenario_version_id, {
        conflict_type: conflictType,
        severity: conflictSeverity,
        description: conflictDescription.trim(),
        source_reference: null,
        affected_object_type: null,
        affected_object_id: null,
      });
      conflictDescription = "";
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  // The owner's decision — recorded verbatim, never applied by the UI.
  // Changing the requirement itself is a destination-engine edit the owner
  // makes on the Destination screen (integrity rule 9).
  async function decideConflict(conflictId: string) {
    if (!selectedVersion) return;
    const decision = conflictDecisions[conflictId];
    if (!decision) return;
    error = null;
    try {
      await api.scenarioResolveConflict(
        selectedVersion.scenario_version_id,
        conflictId,
        decision,
        null,
      );
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function finalize() {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioFinalize(selectedVersion.scenario_version_id);
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function setReadiness(readiness: string) {
    if (!selectedVersion) return;
    error = null;
    try {
      await api.scenarioSetReadiness(selectedVersion.scenario_version_id, readiness, null);
      await refreshVersion(selectedVersion.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  async function createBranch() {
    if (!selectedVersion) return;
    error = null;
    try {
      const created = await api.scenarioWhatIf({
        parent_scenario_version_id: selectedVersion.scenario_version_id,
        branch_type: branchType,
        reason: branchReason.trim(),
        changed_assumptions: branchOverrides,
        name: branchName.trim(),
      });
      branchOverrides = [];
      branchName = "";
      branchReason = "";
      await loadAll();
      await selectVersion(created.version.scenario_version_id);
    } catch (e) {
      error = String(e);
    }
  }

  // One comparison dimension per comparison in this first UI; the engine
  // accepts more, and every cell points at dimension 0.
  // The owner picked versions by name in the "add" dropdown — rows keep
  // that name (ids are plumbing, not reading material).
  function versionLabel(versionId: string): string {
    for (const family of families) {
      for (const version of family.versions) {
        if (version.scenario_version_id === versionId) {
          return `${version.name} v${version.version_number}`;
        }
      }
    }
    return `${versionId.slice(0, 12)}…`;
  }

  async function recordComparison() {
    error = null;
    try {
      const results = comparisonVersionIds.map((versionId) => ({
        scenario_version_id: versionId,
        dimension_index: 0,
        value: comparisonCells[versionId]?.trim()
          ? { value_type: "text" as const, value: comparisonCells[versionId].trim(), currency: null }
          : null,
        outcome: comparisonCells[`outcome:${versionId}`] ?? "insufficient_information",
        source_reference: null,
      }));
      comparison = await api.scenarioComparison({
        name: comparisonName.trim(),
        scenario_version_ids: comparisonVersionIds,
        dimensions: [
          {
            dimension_type: comparisonDimension,
            label:
              comparisonDimensionLabel.trim() || vocabLabel(DIMENSION_TYPES, comparisonDimension),
          },
        ],
        results,
      });
      comparisonName = "";
      comparisonDimensionLabel = "";
      comparisonCells = {};
    } catch (e) {
      error = String(e);
    }
  }

  const headVersion = $derived(
    selectedVersion ??
      (families.length > 0 ? families[0].versions[families[0].versions.length - 1] ?? null : null),
  );

  function severityWord(severity: string): string {
    return vocabLabel(CONFLICT_SEVERITIES, severity).split(" — ")[0];
  }

  function severityClass(severity: string): string {
    return severityWord(severity).toLowerCase();
  }
</script>

<section class="explorer">
  <header class="explorer-head">
    <h1>Scenarios</h1>
    <p class="lede">
      Explore paths side by side. Scenarios are information, never advice —
      nothing here is ranked, scored, or recommended. Comparisons show facts
      on the same dimensions so <em>you</em> can decide.
    </p>
    <LayerChip kind="system" label="Platform scenario layer — recorded by Forhemit, built from your destination and Business Reality" />
  </header>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  {#if loading}
    <p class="loading">Opening the scenario engine…</p>
  {:else}
    <div class="columns">
      <aside class="family-list">
        <h2>Families</h2>
        {#if families.length === 0}
          <p class="empty">No scenarios yet — create the first one below to start exploring.</p>
        {/if}
        {#each families as family (family.scenario_family_id)}
          <div class="family-card">
            <button type="button" class="link" onclick={() => selectHeadVersion(family)}>
              <strong>{family.name}</strong>
            </button>
            <div class="meta">{vocabLabel(SCENARIO_TYPES, family.scenario_type)}</div>
            <ul class="version-links">
              {#each family.versions as version (version.scenario_version_id)}
                <li>
                  <button
                    type="button"
                    class="link"
                    class:current={version.scenario_version_id === selectedVersionId}
                    onclick={() => selectVersion(version.scenario_version_id)}
                  >
                    v{version.version_number} — {version.name}
                  </button>
                  <div class="meta">
                    {lifecycleLabel(version.lifecycle)} · {readinessLabel(version.readiness)}
                  </div>
                </li>
              {/each}
            </ul>
          </div>
        {/each}
      </aside>

      <div class="detail">
        {#if headVersion === null}
          <section class="create-panel">
            <h2>Create your first scenario</h2>
            <p class="hint">
              A scenario explores one path — “what would an ESOP look like for
              this company?” It is pinned to a destination version and the
              Business Reality facts you choose, so later edits never silently
              rewrite what you explored.
            </p>
          </section>
        {:else}
          <article class="version-detail">
            <div class="detail-head">
              <h2>{headVersion.name} <span class="meta">v{headVersion.version_number}</span></h2>
              <span class="chip">{lifecycleLabel(headVersion.lifecycle)}</span>
              <span class="chip">{readinessLabel(headVersion.readiness)}</span>
            </div>
            <p class="meta">
              Pinned to destination v{destinationVersions.find(
                (v) => v.version_id === headVersion.destination_version_id,
              )?.version_number ?? "?"} · Business Reality {headVersion.business_reality_version_id.slice(0, 12)}…
            </p>

            {#if headVersion.is_draft}
              <div class="actions">
                <button type="button" class="primary" onclick={finalize}>
                  Finalize this version (becomes immutable)
                </button>
              </div>
              <div class="actions">
                {#each READINESS_LEVELS.slice(0, 5) as level (level.value)}
                  <button type="button" class="secondary" onclick={() => setReadiness(level.value)}>
                    Readiness: {level.label.split(" — ")[0]}
                  </button>
                {/each}
              </div>
            {:else}
              <p class="meta">
                Finalized {headVersion.finalized_at ? timeLabel(headVersion.finalized_at) : ""} —
                this version is immutable. Explore changes in a branch.
              </p>
            {/if}
          </article>
        {/if}

        {#if headVersion && headVersion.is_draft}
          <details class="panel" open>
            <summary>Assumptions ({headVersion.assumptions.length})</summary>
            <ul class="items">
              {#each headVersion.assumptions as assumption (assumption.assumption_id)}
                <li>
                  <strong>{assumption.name}</strong> — {assumption.value_label}
                  <div class="meta">
                    {vocabLabel(ASSUMPTION_CATEGORIES, assumption.category)} ·
                    {vocabLabel(PROVENANCES, assumption.provenance)} ·
                    {vocabLabel(VERIFICATIONS, assumption.verification)}
                  </div>
                  <label class="override">
                    <input type="checkbox" bind:group={branchOverrides} value={assumption.assumption_id} />
                    override in branch
                  </label>
                </li>
              {/each}
            </ul>
            <div class="form">
              <input type="text" placeholder="What does the scenario assume? (name)" bind:value={assumptionName} />
              <div class="row">
                <select bind:value={assumptionCategory}>
                  {#each ASSUMPTION_CATEGORIES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
                <select bind:value={assumptionType}>
                  {#each VALUE_TYPES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
              </div>
              <div class="row">
                <input type="text" placeholder="Value" bind:value={assumptionValue} />
                {#if assumptionType === "currency"}
                  <input type="text" placeholder="Code (USD)" bind:value={assumptionCurrency} />
                {/if}
              </div>
              <div class="row">
                <select bind:value={assumptionProvenance}>
                  {#each PROVENANCES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
                <select bind:value={assumptionVerification}>
                  {#each VERIFICATIONS as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
              </div>
              <input type="text" placeholder="Where did it come from? (optional)" bind:value={assumptionSource} />
              <button type="button" class="primary" onclick={addAssumption} disabled={!assumptionName.trim() || !assumptionValue.trim()}>
                Add assumption
              </button>
            </div>
          </details>

          <details class="panel" open>
            <summary>Unknowns — first-class, never blank zeros ({headVersion.unknowns.length})</summary>
            <ul class="items">
              {#each headVersion.unknowns as unknown (unknown.unknown_id)}
                <li>
                  <strong>{unknown.description}</strong>
                  <div class="meta">
                    {vocabLabel(UNKNOWN_IMPORTANCES, unknown.importance)} ·
                    {vocabLabel(UNKNOWN_STATUSES, unknown.resolution_status)}
                  </div>
                  {#if unknown.resolution_status === "open" || unknown.resolution_status === "in_progress"}
                    <button type="button" class="link" onclick={() => resolveUnknown(unknown.unknown_id, "resolved")}>
                      Mark resolved
                    </button>
                    <button type="button" class="link" onclick={() => resolveUnknown(unknown.unknown_id, "waived")}>
                      Waive — proceed without it
                    </button>
                  {/if}
                </li>
              {/each}
            </ul>
            <div class="form">
              <input type="text" placeholder="What is missing?" bind:value={unknownDescription} />
              <select bind:value={unknownImportance}>
                {#each UNKNOWN_IMPORTANCES as option (option.value)}
                  <option value={option.value}>{option.label}</option>
                {/each}
              </select>
              <input type="text" placeholder="What to do about it (optional)" bind:value={unknownAction} />
              <button type="button" class="primary" onclick={addUnknown} disabled={!unknownDescription.trim()}>
                Record unknown
              </button>
            </div>
          </details>

          <details class="panel">
            <summary>Nonnegotiables under test ({headVersion.nonnegotiables.length})</summary>
            <ul class="items">
              {#each headVersion.nonnegotiables as nn (nn.nonnegotiable_id)}
                <li>
                  🔴 Tests a destination must-have: {nn.description}
                </li>
              {/each}
            </ul>
            <div class="form">
              <select
                onchange={(event) => {
                  const value = event.currentTarget.value;
                  if (value) void addNonnegotiableTest(value, "Testing this must-have in this scenario");
                  event.currentTarget.value = "";
                }}
              >
                <option value="">Test a destination must-have in this scenario…</option>
                {#each destinationVersions as version (version.version_id)}
                  <optgroup label={`Destination v${version.version_number}`}>
                    <option value={version.content.financial_objective.objective_id}>
                      Financial objective
                    </option>
                    <option value={version.content.owner_role.objective_id}>Owner role</option>
                    <option value={version.content.transition_timing.objective_id}>Timeframe</option>
                  </optgroup>
                {/each}
              </select>
            </div>
          </details>

          <details class="panel" open>
            <summary>Conflicts ({headVersion.conflicts.length}) — you decide, never the system</summary>
            <p class="hint">
              A conflict is surfaced, not filtered. Recording one never blocks
              you silently: you choose what happens, and the choice is
              recorded. A 🔴 nonnegotiable is never silently relaxed.
            </p>
            <ul class="items conflicts">
              {#each headVersion.conflicts as conflict (conflict.conflict_id)}
                <li>
                  <span class="sev {severityClass(conflict.severity)}">{severityWord(conflict.severity)}</span>
                  <strong>{vocabLabel(CONFLICT_TYPES, conflict.conflict_type)}</strong> —
                  {conflict.description}
                  {#if conflict.is_nonnegotiable}
                    <div class="meta">conflicts with a 🔴 destination nonnegotiable</div>
                  {/if}
                  {#if conflict.resolved}
                    <div class="meta">
                      Decided — {conflict.owner_decision ?? conflict.nonnegotiable?.owner_decision ?? "(recorded in the audit trail)"}
                    </div>
                  {:else}
                    <div class="decision-row">
                      <select bind:value={conflictDecisions[conflict.conflict_id]}>
                        <option value="">Your decision…</option>
                        {#each OWNER_DECISIONS as option (option.value)}
                          <option value={option.value}>{option.label}</option>
                        {/each}
                      </select>
                      <button
                        type="button"
                        class="secondary"
                        disabled={!conflictDecisions[conflict.conflict_id]}
                        onclick={() => decideConflict(conflict.conflict_id)}
                      >
                        Record my decision
                      </button>
                    </div>
                  {/if}
                </li>
              {/each}
            </ul>
            <div class="form">
              <div class="row">
                <select bind:value={conflictType}>
                  {#each CONFLICT_TYPES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
                <select bind:value={conflictSeverity}>
                  {#each CONFLICT_SEVERITIES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
              </div>
              <input type="text" placeholder="What is the conflict?" bind:value={conflictDescription} />
              <button type="button" class="primary" onclick={recordConflict} disabled={!conflictDescription.trim()}>
                Record conflict
              </button>
            </div>
          </details>
        {/if}

        {#if headVersion}
        {#key headVersion.scenario_version_id}
          <details class="panel">
            <summary>What-if branch — explore a change without rewriting anything</summary>
            <div class="form">
              <select bind:value={branchType}>
                {#each BRANCH_TYPES as option (option.value)}
                  <option value={option.value}>{option.label}</option>
                {/each}
              </select>
              <input type="text" placeholder={"Name the branch, e.g. “What if revenue dips?”"} bind:value={branchName} />
              <input type="text" placeholder="Why this branch exists" bind:value={branchReason} />
              <p class="meta">
                Assumption overrides selected: {branchOverrides.length === 0
                  ? "none — the branch starts from the same assumptions"
                  : branchOverrides.join(", ")}
              </p>
              <button type="button" class="primary" onclick={createBranch} disabled={!branchReason.trim()}>
                Create branch
              </button>
            </div>
          </details>
        {/key}
        {/if}

        <details class="panel">
          <summary>Compare scenarios — facts on shared dimensions, no winner</summary>
          <p class="hint">
            Outcomes are factual statuses — aligns / does not currently align /
            not enough information to say — never ranks, scores, or
            recommendations.
          </p>
          <div class="form">
            <input type="text" placeholder="Name the comparison" bind:value={comparisonName} />
            <select
              onchange={(event) => {
                const value = event.currentTarget.value;
                if (value && !comparisonVersionIds.includes(value)) {
                  comparisonVersionIds = [...comparisonVersionIds, value];
                }
                event.currentTarget.value = "";
              }}
            >
              <option value="">Add a scenario version…</option>
              {#each families as family (family.scenario_family_id)}
                {#each family.versions as version (version.scenario_version_id)}
                  <option value={version.scenario_version_id}>{family.name} v{version.version_number}</option>
                {/each}
              {/each}
            </select>
            <div class="row">
              <select bind:value={comparisonDimension}>
                {#each DIMENSION_TYPES as option (option.value)}
                  <option value={option.value}>{option.label}</option>
                {/each}
              </select>
              <input type="text" placeholder="Dimension label (optional)" bind:value={comparisonDimensionLabel} />
            </div>
            {#each comparisonVersionIds as versionId (versionId)}
              <div class="cell-row">
                <span class="meta">{versionLabel(versionId)}</span>
                <input type="text" placeholder="Factual reading (optional)" bind:value={comparisonCells[versionId]} />
                <select bind:value={comparisonCells[`outcome:${versionId}`]}>
                  {#each COMPARISON_OUTCOMES as option (option.value)}
                    <option value={option.value}>{option.label}</option>
                  {/each}
                </select>
              </div>
            {/each}
            <button type="button" class="primary" onclick={recordComparison} disabled={comparisonVersionIds.length < 2 || !comparisonName.trim()}>
              Record comparison
            </button>
          </div>

          {#if comparison}
            <div class="comparison-grid">
              <h3>{comparison.name}</h3>
              <table>
                <thead>
                  <tr>
                    <th>Scenario</th>
                    {#each comparison.dimensions as dimension (dimension.index)}
                      <th>{dimension.label}</th>
                    {/each}
                  </tr>
                </thead>
                <tbody>
                  {#each comparison.scenario_version_ids as versionId (versionId)}
                    <tr>
                      <td>{versionLabel(versionId)}</td>
                      {#each comparison.dimensions as dimension (dimension.index)}
                        {@const cell = comparison.cells.find(
                          (candidate) =>
                            candidate.scenario_version_id === versionId &&
                            candidate.dimension_index === dimension.index,
                        )}
                        <td>
                          {#if cell}
                            {outcomeLabel(cell.outcome)}{cell.value_label ? ` — ${cell.value_label}` : ""}
                          {:else}
                            Not enough information to say
                          {/if}
                        </td>
                      {/each}
                    </tr>
                  {/each}
                </tbody>
              </table>
              <LayerChip kind="system" label="Factual statuses on shared dimensions — no scenario is ranked, scored, or recommended" />
            </div>
          {/if}
        </details>
      </div>
    </div>

    <section class="create-panel">
      <h2>Create a scenario</h2>
      <p class="hint">
        Pinned to a destination version and the Business Reality facts you
        choose — later edits never silently rewrite what you explored.
      </p>
      {#if destinationVersions.length === 0}
        <p class="hint">Build your destination first — every scenario starts from it.</p>
      {:else}
        <div class="form">
          <div class="row">
            <select bind:value={createType}>
              {#each SCENARIO_TYPES as option (option.value)}
                <option value={option.value}>{option.label}</option>
              {/each}
            </select>
            <input type="text" placeholder={"Name, e.g. “ESOP path”"} bind:value={createName} />
          </div>
          <select bind:value={createDestinationVersion}>
            {#each destinationVersions as version (version.version_id)}
              <option value={version.version_id}>Pinned to destination v{version.version_number}</option>
            {/each}
          </select>
          <fieldset class="fact-picker">
            <legend>Pin Business Reality facts (the versions this scenario is built on)</legend>
            {#if facts.length === 0}
              <span class="meta">No Business Reality facts recorded yet.</span>
            {/if}
            {#each facts as fact (fact.fact_version_id)}
              <label class="fact-row">
                <input type="checkbox" bind:group={createFactIds} value={fact.fact_version_id} />
                {fact.kind} — {fact.period}
              </label>
            {/each}
          </fieldset>
          <button type="button" class="primary" onclick={createFamily} disabled={!createName.trim()}>
            Create scenario
          </button>
        </div>
      {/if}
    </section>
  {/if}
</section>

<style>
  .explorer { display: flex; flex-direction: column; gap: 18px; }
  .explorer-head h1 { margin-bottom: 4px; }
  .lede { color: var(--text-2, #555); max-width: 640px; }
  .columns { display: grid; grid-template-columns: 280px 1fr; gap: 20px; align-items: start; }
  .family-list { display: flex; flex-direction: column; gap: 12px; }
  .family-card { background: var(--surface-2, #f7f7f8); border-radius: 10px; padding: 12px 14px; }
  .version-links { list-style: none; padding: 6px 0 0; display: flex; flex-direction: column; gap: 6px; }
  .version-links .current { font-weight: 700; }
  .link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 2px 4px; text-align: left; }
  .meta { color: var(--text-3, #888); font-size: 0.8rem; }
  .detail { display: flex; flex-direction: column; gap: 14px; }
  .detail-head { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
  .detail-head h2 { margin: 0; }
  .chip { font-size: 0.75rem; padding: 3px 10px; border-radius: 999px; border: 1px solid var(--border, #ccc); background: var(--surface-2, #f7f7f8); }
  .actions { display: flex; gap: 8px; flex-wrap: wrap; margin: 8px 0; }
  .panel { border: 1px solid var(--border, #ddd); border-radius: 10px; padding: 12px 16px; }
  .panel summary { cursor: pointer; font-weight: 600; }
  .items { list-style: none; padding: 8px 0; display: flex; flex-direction: column; gap: 10px; }
  .items li { border-left: 3px solid var(--border, #e2e2e4); padding-left: 10px; }
  .conflicts li { border-left-color: #e5c9a3; }
  .override { display: block; font-size: 0.8rem; color: var(--text-3, #888); }
  .form { display: flex; flex-direction: column; gap: 8px; padding-top: 8px; max-width: 620px; }
  .form input, .form select { font: inherit; padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); }
  .row { display: flex; gap: 8px; }
  .row > * { flex: 1; }
  .fact-picker { border: 1px solid var(--border, #ddd); border-radius: 8px; display: flex; flex-direction: column; gap: 6px; }
  .fact-row { display: flex; gap: 8px; align-items: center; font-size: 0.9rem; }
  .decision-row { display: flex; gap: 8px; margin-top: 6px; flex-wrap: wrap; }
  .cell-row { display: grid; grid-template-columns: 130px 1fr 220px; gap: 8px; align-items: center; }
  button.primary, button.secondary { padding: 8px 16px; border-radius: 8px; font: inherit; cursor: pointer; border: 1px solid transparent; }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  .sev { display: inline-block; font-size: 0.75rem; padding: 1px 8px; border-radius: 999px; border: 1px solid var(--border, #ccc); margin-right: 6px; }
  .informational { background: #f0f0f1; }
  .attention { background: #fdf3e7; border-color: #e5c9a3; color: #8a5a12; }
  .material { background: #fdeaea; border-color: #dda3a3; color: #8a1a12; }
  .blocking { background: #b3261e; color: white; }
  .comparison-grid table { border-collapse: collapse; margin: 10px 0; width: 100%; }
  .comparison-grid th, .comparison-grid td { border: 1px solid var(--border, #ddd); padding: 8px 10px; text-align: left; font-size: 0.9rem; }
  .comparison-grid th { background: var(--surface-2, #f7f7f8); }
  .error { color: #b3261e; font-weight: 500; }
  .loading, .empty { color: var(--text-3, #888); }
  .create-panel { border: 1px dashed var(--border, #ccc); border-radius: 10px; padding: 14px 18px; }
  .hint { color: var(--text-2, #555); font-size: 0.9rem; }
</style>
