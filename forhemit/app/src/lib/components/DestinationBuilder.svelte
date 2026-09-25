<script lang="ts">
  // The 13-screen Destination Builder walk. Presentation only: per-screen
  // state lives here, assembly + validation in screens.ts, domain rules in
  // the destination engine behind the commands.
  import { api } from "../api";
  import {
    CHANGE_REASONS,
    INCOME_BANDS,
    INCOME_DURATIONS,
    PROCEEDS_BANDS,
    PREFERENCE_OPTIONS,
    SCREENS,
    buildContent,
    draftFromContent,
    freshDraft,
    type DraftState,
  } from "../screens";
  import type { Completeness, Destination, DestinationContent } from "../types";
  import { PREFERENCE_OPTIONS as PREFS } from "../screens";
  import LayerChip from "./LayerChip.svelte";
  import ProgressBar from "./ProgressBar.svelte";
  import NonnegotiableRow from "./NonnegotiableRow.svelte";
  import ProsConsColumn from "./ProsConsColumn.svelte";
  import DecisionHistory from "./DecisionHistory.svelte";
  import ReviewList from "./ReviewList.svelte";
  import { builderHistory, selectedScreenChoice } from "./builderHistory";

  let { onDone, onHome, seed = null }:
    { onDone: (destination: Destination) => void; onHome: () => void; seed?: DestinationContent | null } =
    $props();

  // Screen 1 is the welcome decision; index into SCREENS by position.
  let position = $state(0);
  let draft = $state<DraftState>(freshDraft());
  let completeness = $state<Completeness | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let editMode = $state(false);
  let editReason = $state<string>("learned_something_new");
  let editExplanation = $state<string>("");

  const screen = $derived(SCREENS[position]);
  const isReview = $derived(screen.interaction === "review");
  const progressDone = $derived(position);
  const progressTotal = $derived(SCREENS.length - 1);
  const historyNodes = $derived(builderHistory(draft, position));
  const selectedChoice = $derived(selectedScreenChoice(screen, draft));

  // Editing an existing destination: pre-fill the draft from the stored
  // content and mark the save as a new version.
  if (seed) {
    draft = draftFromContent(seed);
    editMode = true;
  }

  function next() {
    error = null;
    if (position < SCREENS.length - 1) position += 1;
  }

  function back() {
    error = null;
    if (position > 0) position -= 1;
  }

  /** The pros/cons moment: hold the screen ~600ms after a single-select
   *  pick so the tradeoff column registers, then auto-advance. Any other
   *  navigation off this screen (manual Continue, Back, a rail jump)
   *  invalidates the pending advance — the guard makes late timers no-ops. */
  function settleThenNext() {
    const at = position;
    setTimeout(() => {
      if (position === at) next();
    }, 600);
  }

  /** Jump-to-any-step over the 13 screens: the draft is never touched, so
   *  every answer given so far survives the jump. Nothing persists until
   *  the final wholesale submit. */
  function jumpToScreen(screenId: string) {
    const index = SCREENS.findIndex((s) => s.id === screenId);
    if (index === -1) return;
    error = null;
    position = index;
  }

  // Toggle a multi-select participant value.
  function toggleParticipant(value: string) {
    const set = new Set(draft.participants.selected);
    if (set.has(value)) set.delete(value);
    else set.add(value);
    draft.participants.selected = [...set];
  }

  function toggleGoal(value: string) {
    const set = new Set(draft.preservation.selected);
    if (set.has(value)) {
      set.delete(value);
      delete draft.preservation.ranks[value];
      delete draft.preservation.prefs[value];
    } else {
      set.add(value);
      draft.preservation.ranks[value] = set.size;
      draft.preservation.prefs[value] = "unspecified";
    }
    draft.preservation.selected = [...set];
  }

  function toggleAvoidance(value: string) {
    const set = new Set(draft.avoidances.selected);
    if (set.has(value)) {
      set.delete(value);
      delete draft.avoidances.prefs[value];
    } else {
      set.add(value);
      draft.avoidances.prefs[value] = "unspecified";
    }
    draft.avoidances.selected = [...set];
  }

  function addShare() {
    draft.allocation.shares = [
      ...draft.allocation.shares,
      { group: "employees", label: "", percent: 0 },
    ];
  }

  function removeShare(index: number) {
    draft.allocation.shares = draft.allocation.shares.filter((_, i) => i !== index);
  }

  function shareTotal(): number {
    return draft.allocation.shares.reduce((sum, s) => sum + (Number(s.percent) || 0), 0);
  }

  async function refreshCompleteness() {
    // The engine owns the completeness rule; the review screen only
    // renders its six binary areas + overall label.
    const built = buildContent(draft);
    if ("error" in built) {
      completeness = null;
      return;
    }
    try {
      completeness = await api.destinationCompleteness(built.content);
    } catch {
      completeness = null;
    }
  }

  $effect(() => {
    if (isReview) void refreshCompleteness();
  });

  async function finish(mode: "confirm" | "working") {
    error = null;
    busy = true;
    try {
      const built = buildContent(draft);
      if ("error" in built) {
        error = built.error;
        return;
      }
      const destination = editMode
        ? await api.destinationEdit(
            built.content,
            editReason as never,
            editExplanation.trim() ? editExplanation.trim() : null,
          )
        : await api.destinationCreate(built.content);
      const final = mode === "confirm" ? await api.destinationConfirm() : destination;
      onDone(mode === "confirm" ? final : destination);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<div class="builder">
  <ProgressBar done={progressDone} total={progressTotal} label={`Screen ${screen.number} of ${SCREENS.length}`} />

  <DecisionHistory answered={historyNodes} skipped={[]} revisingNodeId={null} onSelect={jumpToScreen} />

  {#if editMode}
    <div class="edit-banner">
      <strong>Editing your destination</strong> — a new version will be created; the original is
      retained and never overwritten.
      <LayerChip kind="system" label="Recorded by Forhemit — system keeps every version" />
      <label class="field">
        Why are you changing this?
        <select bind:value={editReason}>
          {#each CHANGE_REASONS as reason}
            <option value={reason.value}>{reason.label}</option>
          {/each}
        </select>
      </label>
      <label class="field">
        Anything to add? (optional)
        <textarea bind:value={editExplanation} rows="2" placeholder="Optional explanation" />
      </label>
    </div>
  {/if}

  <h2>
    <span class="screen-number">{screen.number}</span> {screen.title}
  </h2>
  <p class="prompt">{screen.prompt}</p>
  <details class="why-disclosure">
    <summary>Why this screen</summary>
    <p>{screen.why}</p>
  </details>

  {#if screen.interaction === "decision"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.welcome === choice.value}
          onclick={() => {
            draft.welcome = choice.value;
            settleThenNext();
          }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
    </div>
  {:else if screen.interaction === "single"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.financial.choice === choice.value}
          onclick={() => {
            draft.financial.choice = choice.value as never;
            settleThenNext();
          }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
      <button type="button" class="choice subtle" class:selected={draft.financial.choice === null && draft.welcome !== ""}
        onclick={() => { draft.financial.choice = null; settleThenNext(); }}>
        <span class="choice-label">I'm not sure</span>
        <span class="choice-detail">A first-class answer — never forced certainty.</span>
      </button>
    </div>
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.financial.pref} />
  {:else if screen.interaction === "proceeds"}
    <div class="choices">
      {#each PROCEEDS_BANDS as band}
        <button
          type="button"
          class="choice"
          class:selected={draft.proceeds.mode === "band" && draft.proceeds.band === band.value}
          onclick={() => {
            draft.proceeds.mode = "band";
            draft.proceeds.band = band.value;
          }}
        >
          <span class="choice-label">{band.label}</span>
        </button>
      {/each}
      <button
        type="button"
        class="choice"
        class:selected={draft.proceeds.mode === "custom"}
        onclick={() => { draft.proceeds.mode = "custom"; }}
      >
        <span class="choice-label">Enter my own range</span>
        {#if draft.proceeds.mode === "custom"}
          <span class="range-inputs">
            <label>Ideally at least <input type="number" min="0" bind:value={draft.proceeds.min} placeholder="no minimum" /></label>
            <label>at most <input type="number" min="0" bind:value={draft.proceeds.max} placeholder="no maximum" /></label>
          </span>
        {/if}
      </button>
      <button
        type="button"
        class="choice subtle"
        class:selected={draft.proceeds.mode === null}
        onclick={() => { draft.proceeds.mode = null; draft.proceeds.band = null; }}
      >
        <span class="choice-label">I don't know yet</span>
        <span class="choice-detail">Never blocked because you don't know your business's value.</span>
      </button>
    </div>
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.proceeds.pref} />
  {:else if screen.interaction === "income"}
    <div class="choices">
      {#each ["yes", "maybe", "no"] as option}
        <button
          type="button"
          class="choice"
          class:selected={draft.income.interest === option}
          onclick={() => { draft.income.interest = option as never; }}
        >
          <span class="choice-label">{option === "yes" ? "Yes" : option === "maybe" ? "Maybe" : "No"}</span>
        </button>
      {/each}
    </div>
    {#if draft.income.interest && draft.income.interest !== "no"}
      <div class="sub-block">
        <label class="field">
          About how much annual income would you ideally like?
          <select bind:value={draft.income.amount}>
            <option value={null}>I don't know yet</option>
            {#each INCOME_BANDS as band}
              <option value={band.value}>{band.label}</option>
            {/each}
          </select>
        </label>
        <label class="field">
          For approximately how long?
          <select bind:value={draft.income.duration}>
            <option value={null}>I don't know yet</option>
            {#each INCOME_DURATIONS as duration}
              <option value={duration.value}>{duration.label}</option>
            {/each}
          </select>
        </label>
      </div>
    {/if}
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.income.pref} />
  {:else if screen.interaction === "participants"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.participants.selected.includes(choice.value)}
          onclick={() => toggleParticipant(choice.value)}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
    </div>
    {#if draft.participants.selected.includes("specific_group")}
      <label class="field">Which group? <input type="text" bind:value={draft.participants.specificGroup} placeholder="e.g. service technicians" /></label>
    {/if}
    {#if draft.participants.selected.includes("other")}
      <label class="field">Describe <input type="text" bind:value={draft.participants.other} placeholder="describe the group" /></label>
    {/if}
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.participants.pref} />
  {:else if screen.interaction === "shape"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.shape.choice === choice.value}
          onclick={() => { draft.shape.choice = choice.value as never; }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
      <button type="button" class="choice subtle" class:selected={draft.shape.choice === null}
        onclick={() => { draft.shape.choice = null; }}>
        <span class="choice-label">I'm not sure</span>
      </button>
    </div>
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.shape.pref} />
  {:else if screen.interaction === "allocation"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.allocation.mode === choice.value}
          onclick={() => {
            draft.allocation.mode = choice.value as never;
            if (choice.value === "percentages" && draft.allocation.shares.length === 0) addShare();
          }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
      <button type="button" class="choice subtle" class:selected={draft.allocation.mode === null}
        onclick={() => { draft.allocation.mode = null; }}>
        <span class="choice-label">I'm not sure</span>
      </button>
    </div>
    {#if draft.allocation.mode === "percentages"}
      <div class="sub-block">
        <p class="note">Percentages describe your desired outcome — they are not a proposed legal ownership structure. They must total exactly 100%.</p>
        {#each draft.allocation.shares as share, index}
          <div class="share-row">
            <select bind:value={share.group}>
              <option value="employees">Employees</option>
              <option value="management">Management</option>
              <option value="other">Other</option>
            </select>
            {#if share.group === "other"}
              <input type="text" placeholder="group name" bind:value={share.label} />
            {/if}
            <input class="percent" type="number" min="0" max="100" bind:value={share.percent} />
            <button type="button" class="link" onclick={() => removeShare(index)}>Remove</button>
          </div>
        {/each}
        <button type="button" class="link" onclick={addShare}>+ Add group</button>
        <p class="note" class:over={shareTotal() !== 100}>Total: {shareTotal()}%</p>
      </div>
    {/if}
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.allocation.pref} />
  {:else if screen.interaction === "role"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.role.choice === choice.value}
          onclick={() => { draft.role.choice = choice.value as never; }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
      <button type="button" class="choice subtle" class:selected={draft.role.choice === null}
        onclick={() => { draft.role.choice = null; }}>
        <span class="choice-label">I'm not sure</span>
      </button>
    </div>
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.role.pref} />
  {:else if screen.interaction === "timing"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.timing.choice === choice.value}
          onclick={() => { draft.timing.choice = choice.value as never; }}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
    </div>
    <LayerChip kind="owner" />
    <NonnegotiableRow bind:pref={draft.timing.pref} />
  {:else if screen.interaction === "preservation"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.preservation.selected.includes(choice.value)}
          onclick={() => toggleGoal(choice.value)}
        >
          <span class="choice-label">{choice.label}</span>
          {#if draft.preservation.selected.includes(choice.value)}
            <span class="rank-badge">#{draft.preservation.ranks[choice.value] ?? "—"}</span>
          {/if}
        </button>
      {/each}
    </div>
    {#if draft.preservation.selected.includes("other")}
      <label class="field">Describe <input type="text" bind:value={draft.preservation.other} placeholder="describe the goal" /></label>
    {/if}
    <p class="note">Selected goals are ranked in the order you picked them; “matters most” is #1. Use 🔄 on the review screen to adjust.</p>
    <LayerChip kind="owner" />
    <p class="note">Each selected goal gets its own preference strength (below, on the review screen before you save).</p>
  {:else if screen.interaction === "avoidances"}
    <div class="choices">
      {#each screen.choices as choice}
        <button
          type="button"
          class="choice"
          class:selected={draft.avoidances.selected.includes(choice.value)}
          onclick={() => toggleAvoidance(choice.value)}
        >
          <span class="choice-label">{choice.label}</span>
        </button>
      {/each}
    </div>
    {#if draft.avoidances.selected.includes("other")}
      <label class="field">Describe <input type="text" bind:value={draft.avoidances.other} placeholder="describe the avoidance" /></label>
    {/if}
    <LayerChip kind="owner" />
  {:else if screen.interaction === "notes"}
    <label class="field">
      Anything else to record?
      <textarea bind:value={draft.notes} rows="4" placeholder="Optional — kept verbatim, recorded as owner-stated context." />
    </label>
    <LayerChip kind="owner" />
  {:else if isReview}
    <ReviewList {draft} {completeness} />
    {#each draft.preservation.selected as goalValue}
      <div class="pref-mini">
        <span>{SCREENS.find((s) => s.id === "preservation_goals")?.choices.find((c) => c.value === goalValue)?.label}</span>
        <select bind:value={draft.preservation.prefs[goalValue]}>
          {#each PREFS as option}
            <option value={option.value}>{option.dot} {option.label}</option>
          {/each}
        </select>
      </div>
    {/each}
  {/if}

  {#if selectedChoice}
    <ProsConsColumn pros={selectedChoice.pros} cons={selectedChoice.cons} />
  {/if}

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <div class="nav">
    <button type="button" class="secondary" onclick={back} disabled={position === 0 || busy}>
      ← Back
    </button>
    {#if isReview}
      <button type="button" class="primary" disabled={busy} onclick={() => void finish("confirm")}>
        {editMode ? "Save new version" : "Confirm — this is the outcome I'm trying to create"}
      </button>
      <button type="button" class="secondary" disabled={busy} onclick={() => void finish("working")}>
        {editMode ? "Save as working draft" : "I'm not sure yet — keep as working draft"}
      </button>
    {:else}
      <button type="button" class="primary" onclick={next} disabled={busy}>
        {screen.optional ? "Continue" : "Continue"} →
      </button>
    {/if}
  </div>
  <button type="button" class="link home-link" onclick={onHome}>Back to home (nothing is lost — versions are kept)</button>
</div>

<style>
  .builder { max-width: 760px; margin: 0 auto; }
  .edit-banner {
    background: var(--surface-2, #f4f4f5); border: 1px solid var(--border, #ddd);
    border-radius: 10px; padding: 12px 16px; margin-bottom: 16px; font-size: 0.9rem;
    display: flex; flex-direction: column; gap: 8px;
  }
  h2 { display: flex; align-items: center; gap: 12px; margin: 8px 0 4px; }
  .screen-number {
    display: inline-flex; align-items: center; justify-content: center;
    width: 30px; height: 30px; border-radius: 50%;
    background: var(--accent, #2f6f4f); color: white; font-size: 0.85rem;
  }
  .prompt { color: var(--text-2, #555); margin: 0 0 8px; }
  .why-disclosure { margin: 0 0 16px; font-size: 0.85rem; }
  .why-disclosure summary { cursor: pointer; color: var(--text-3, #777); width: fit-content; }
  .why-disclosure p { margin: 8px 0 0; color: var(--text-2, #555); }
  .choices { display: flex; flex-direction: column; gap: 8px; margin: 12px 0; }
  .choice {
    text-align: left; padding: 12px 16px; border-radius: 10px;
    border: 1px solid var(--border, #ccc); background: var(--surface, #fff);
    cursor: pointer; display: flex; flex-direction: column; gap: 4px;
  }
  .choice:hover { border-color: var(--accent, #2f6f4f); }
  .choice.selected { border-color: var(--accent, #2f6f4f); box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent, #2f6f4f) 25%, transparent); }
  .choice.subtle { border-style: dashed; color: var(--text-2, #555); }
  .choice-label { font-weight: 500; }
  .choice-detail { font-size: 0.82rem; color: var(--text-3, #777); }
  .sub-block { margin: 12px 0 12px 16px; display: flex; flex-direction: column; gap: 10px; }
  .field { display: flex; flex-direction: column; gap: 4px; margin: 10px 0; font-size: 0.9rem; }
  .field input, .field select, .field textarea {
    padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); font: inherit;
  }
  .range-inputs { display: flex; gap: 12px; flex-wrap: wrap; }
  .range-inputs label { display: flex; gap: 6px; align-items: center; font-size: 0.85rem; }
  .share-row { display: flex; gap: 8px; align-items: center; }
  .share-row .percent { width: 90px; }
  .note { font-size: 0.82rem; color: var(--text-3, #777); margin: 4px 0; }
  .note.over { color: #b3261e; font-weight: 600; }
  .rank-badge { font-size: 0.75rem; background: var(--surface-2, #eee); padding: 2px 8px; border-radius: 999px; }
  .pref-mini { display: flex; justify-content: space-between; align-items: center; gap: 12px; padding: 6px 0; font-size: 0.88rem; }
  .error { color: #b3261e; font-weight: 500; }
  .nav {
    display: flex; align-items: center; gap: 12px; margin-top: 20px; flex-wrap: wrap;
  }
  button.primary, button.secondary {
    padding: 10px 18px; border-radius: 10px; font: inherit; cursor: pointer; border: 1px solid transparent;
  }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  button.link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 4px; }
  .home-link { margin-top: 24px; font-size: 0.85rem; }
</style>
