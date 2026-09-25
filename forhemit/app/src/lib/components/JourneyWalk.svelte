<script lang="ts">
  // The EOJ v0.2 walk — a stateful walk, not a form: one question at a
  // time, content screens between nodes, revise creates a new answer
  // version, optional questions can be skipped, "I'm not sure" everywhere.
  // All domain rules live in the journey engine; this renders views.
  import { api } from "../api";
  import { layerLabel, renderAnswerValue } from "../screens";
  import { answerSummary, reasonLabel, timeLabel } from "./bits";
  import LayerChip from "./LayerChip.svelte";
  import ProgressBar from "./ProgressBar.svelte";
  import type { AnsweredView, AnswerValue, JourneyView, QuestionView } from "../types";

  let { onSnapshot }: { onSnapshot: () => void } = $props();

  let view = $state<JourneyView | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let revisingNodeId = $state<string | null>(null);
  let reviseReason = $state<string>("priorities_changed");
  let numericDraft = $state<string>("");

  // Draft selections for the current question.
  let singleDraft = $state<string | null>(null);
  let multiDraft = $state<string[]>([]);
  let rankDraft = $state<string[]>([]);

  function seedDrafts(value: AnswerValue | null) {
    singleDraft = null;
    multiDraft = [];
    rankDraft = [];
    numericDraft = "";
    if (value) {
      if (typeof value === "object" && "single" in value) singleDraft = value.single;
      if (typeof value === "object" && "multi" in value) multiDraft = value.multi;
      if (typeof value === "object" && "ranking" in value) rankDraft = value.ranking;
      if (typeof value === "object" && "amount" in value) numericDraft = String(value.amount);
    }
  }

  function resetDrafts(question: QuestionView | null) {
    seedDrafts(question?.current_value ?? null);
  }

  async function load() {
    error = null;
    busy = true;
    try {
      const state = await api.journeyState();
      view = state ?? (await api.journeyStart());
      resetDrafts(view.current);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  $effect(() => {
    void load();
  });

  async function submit(value: AnswerValue) {
    if (!view?.current) return;
    error = null;
    busy = true;
    try {
      if (revisingNodeId) {
        // Revise the node whose answer is being changed — never the
        // node the walk happens to be sitting on.
        view = await api.journeyRevise(revisingNodeId, value, reviseReason);
        revisingNodeId = null;
      } else {
        view = await api.journeyRecord(view.current.node_id, value);
      }
      resetDrafts(view.current);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function startRevision(answer: AnsweredView) {
    revisingNodeId = answer.node_id;
    seedDrafts(answer.value);
  }

  function cancelRevision() {
    revisingNodeId = null;
    resetDrafts(view?.current ?? null);
  }

  async function skip() {
    if (!view?.current) return;
    error = null;
    busy = true;
    try {
      view = await api.journeySkip(view.current.node_id);
      resetDrafts(view.current);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function submitSingle() {
    if (singleDraft === null) return;
    void submit({ single: singleDraft });
  }

  function submitMulti() {
    if (multiDraft.length === 0) return;
    const interaction = displayed?.interaction;
    void submit(
      interaction === "select_up_to_3"
        ? { multi: multiDraft.slice(0, 3) }
        : { multi: multiDraft },
    );
  }

  // Ranking: clicking a choice appends it to the ordered draft.
  function toggleRank(value: string) {
    const index = rankDraft.indexOf(value);
    if (index >= 0) rankDraft = rankDraft.filter((v) => v !== value);
    else rankDraft = [...rankDraft, value];
  }

  function moveRank(value: string, direction: -1 | 1) {
    const index = rankDraft.indexOf(value);
    const target = index + direction;
    if (index < 0 || target < 0 || target >= rankDraft.length) return;
    const next = [...rankDraft];
    [next[index], next[target]] = [next[target], next[index]];
    rankDraft = next;
  }

  function submitRanking() {
    if (rankDraft.length === 0) return;
    void submit({ ranking: rankDraft });
  }

  function submitAmount() {
    const amount = Number(numericDraft);
    if (!Number.isFinite(amount) || amount < 0) return;
    void submit({ amount: Math.round(amount) });
  }

  function toggleMulti(value: string) {
    if (multiDraft.includes(value)) multiDraft = multiDraft.filter((v) => v !== value);
    else multiDraft = [...multiDraft, value];
  }

  // While revising, the walk re-asks the past question in place, pre-filled
  // with its recorded value; otherwise it shows the current question.
  const revising = $derived(
    revisingNodeId
      ? view?.answered.find((answer) => answer.node_id === revisingNodeId) ?? null
      : null,
  );
  const displayed = $derived<QuestionView | null>(
    revising
      ? {
          node_id: revising.node_id,
          title: revising.title,
          text: revising.text,
          why_we_ask: null,
          interaction: revising.interaction,
          choices: revising.choices,
          required: true,
          decision_layer: revising.decision_layer,
          stage: "revise",
          is_current: false,
          current_value: revising.value,
        }
      : view?.current ?? null,
  );
  const layer = $derived(layerLabel(displayed?.decision_layer ?? null));
</script>

{#if error}
  <p class="error" role="alert">{error}</p>
{/if}

{#if !view}
  <p class="loading">{busy ? "Preparing your journey…" : ""}</p>
{:else if view.status === "completed" && view.outro_screens.length > 0}
  <div class="walk">
    <h2>The journey walk is complete</h2>
    {#each view.outro_screens as screen}
      <div class="content-screen">
        <h3>{screen.title}</h3>
        {#if screen.body}<p>{screen.body}</p>{/if}
      </div>
    {/each}
    <LayerChip kind="system" label="Your answers are stored by their declared scope — recorded by Forhemit" />
    <button type="button" class="primary" onclick={onSnapshot}>Continue to your Business Snapshot →</button>
  </div>
{:else if view.current === null}
  <div class="walk">
    <h2>The journey walk is complete</h2>
    <LayerChip kind="system" />
    <button type="button" class="primary" onclick={onSnapshot}>Continue to your Business Snapshot →</button>
  </div>
{:else}
  <div class="walk">
    <ProgressBar done={view.progress.done} total={view.progress.total}
      label={`Question ${Math.min(view.progress.done + 1, view.progress.total)} of ${view.progress.total}`} />

    {#each view.screens_to_show as screen}
      <div class="content-screen">
        <h3>{screen.title}</h3>
        {#if screen.body}<p>{screen.body}</p>{/if}
      </div>
    {/each}

    {#if revising}
      <div class="revise-banner">
        <strong>Revising “{revising.title}”</strong> — a new answer version is created; the old one is kept.
        <label class="field">
          Why are you changing this answer?
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
    {/if}

    <h2>{displayed.title}</h2>
    <p class="prompt">{displayed.text}</p>
    {#if displayed.why_we_ask}
      <p class="why">Why we ask: {displayed.why_we_ask}</p>
    {/if}

    {#if layer}
      <LayerChip kind={layer.kind === "owner" ? "owner" : layer.kind === "professional" ? "professional" : "system"} label={layer.label} />
    {/if}

    <div class="choices">
      {#if displayed.interaction === "single_select" || displayed.interaction === "yes_no"}
        {#each displayed.choices as choiceItem}
          <button type="button" class="choice" class:selected={singleDraft === choiceItem.value}
            onclick={() => (singleDraft = choiceItem.value)}>
            <span class="choice-label">{choiceItem.label}</span>
          </button>
        {/each}
        <button type="button" class="choice subtle" class:selected={singleDraft === "not_sure"}
          onclick={() => (singleDraft = "not_sure")}>
          <span class="choice-label">I'm not sure</span>
        </button>
      {:else if displayed.interaction === "multi_select" || displayed.interaction === "select_up_to_3"}
        {#each displayed.choices as choiceItem}
          <button type="button" class="choice" class:selected={multiDraft.includes(choiceItem.value)}
            onclick={() => toggleMulti(choiceItem.value)}>
            <span class="choice-label">{choiceItem.label}</span>
          </button>
        {/each}
        <button type="button" class="choice subtle" class:selected={multiDraft.includes("not_sure")}
          onclick={() => toggleMulti("not_sure")}>
          <span class="choice-label">I'm not sure</span>
        </button>
      {:else if displayed.interaction === "ranking"}
        <p class="note">Pick the items that matter, in order — most important first.</p>
        {#each displayed.choices as choiceItem}
          <div class="rank-row">
            <button type="button" class="choice rank-main" class:selected={rankDraft.includes(choiceItem.value)}
              onclick={() => toggleRank(choiceItem.value)}>
              <span class="choice-label">
                {#if rankDraft.includes(choiceItem.value)}#{rankDraft.indexOf(choiceItem.value) + 1} — {/if}
                {choiceItem.label}
              </span>
            </button>
            {#if rankDraft.includes(choiceItem.value)}
              <span class="rank-controls">
                <button type="button" aria-label="Move up" onclick={() => moveRank(choiceItem.value, -1)}>↑</button>
                <button type="button" aria-label="Move down" onclick={() => moveRank(choiceItem.value, 1)}>↓</button>
              </span>
            {/if}
          </div>
        {/each}
      {:else if displayed.interaction === "numeric"}
        <label class="field">
          Your amount (whole dollars)
          <input type="number" min="0" bind:value={numericDraft} placeholder="e.g. 500000" />
        </label>
        <button type="button" class="choice subtle" onclick={() => void submit({ amount: 0 })}>
          <span class="choice-label">I'm not sure — skip the number for now</span>
        </button>
      {:else if displayed.interaction === "confirm"}
        <p class="note">Review your answers below, then confirm to continue.</p>
      {:else}
        <p class="note">This question type ({displayed.interaction}) shows its options when the journey defines them.</p>
      {/if}
    </div>

    <div class="nav">
      {#if !displayed.required && !revisingNodeId}
        <button type="button" class="secondary" onclick={() => void skip()} disabled={busy}>
          Skip this question
        </button>
      {/if}
      {#if displayed.interaction === "confirm"}
        <button type="button" class="primary" onclick={() => void submit("confirmed")} disabled={busy}>
          I confirm — continue
        </button>
      {:else if displayed.interaction === "numeric"}
        <button type="button" class="primary" onclick={submitAmount} disabled={busy || !numericDraft}>
          {revisingNodeId ? "Save new answer version" : "Record answer"} →
        </button>
      {:else if displayed.interaction === "ranking"}
        <button type="button" class="primary" onclick={submitRanking} disabled={busy || rankDraft.length === 0}>
          {revisingNodeId ? "Save new answer version" : "Record answer"} →
        </button>
      {:else if displayed.interaction === "multi_select" || displayed.interaction === "select_up_to_3"}
        <button type="button" class="primary" onclick={submitMulti} disabled={busy || multiDraft.length === 0}>
          {revisingNodeId ? "Save new answer version" : "Record answer"} →
        </button>
      {:else}
        <button type="button" class="primary" onclick={submitSingle} disabled={busy || singleDraft === null}>
          {revisingNodeId ? "Save new answer version" : "Record answer"} →
        </button>
      {/if}
      {#if revisingNodeId}
        <button type="button" class="secondary" onclick={cancelRevision}>
          Cancel revision
        </button>
      {/if}
    </div>

    {#if view.answered.length > 0}
      <details class="answered" open={revisingNodeId !== null}>
        <summary>Your answers so far ({view.answered.length}) — every version kept</summary>
        <ul>
          {#each view.answered as answer}
            <li>
              <div class="answer-head">
                <strong>{answer.title}</strong>
                <span class="answer-value">{renderAnswerValue(answer.value, answer.choices)}</span>
                <button type="button" class="link"
                  onclick={() => startRevision(answer)}>
                  Change answer
                </button>
              </div>
              <span class="versions">
                {answer.versions.length} version{answer.versions.length === 1 ? "" : "s"} — latest:
                {answerSummary(answer.value)} · {timeLabel(answer.versions[answer.versions.length - 1]?.recorded_at ?? "")}
                {#if answer.versions[answer.versions.length - 1]?.change_reason}
                  · because “{reasonLabel(answer.versions[answer.versions.length - 1].change_reason ?? "")}”
                {/if}
              </span>
            </li>
          {/each}
          {#each view.skipped as nodeId}
            <li class="skipped"><em>Skipped: {nodeId}</em></li>
          {/each}
        </ul>
        <LayerChip kind="owner" label="Your answers — owner-stated" />
      </details>
    {/if}

    {#if view.nonnegotiables.length > 0}
      <div class="nonneg">
        <LayerChip kind="system" label="Recorded by Forhemit — derived from your answers" />
        <p><strong>Marked nonnegotiable by your answers:</strong></p>
        <ul>
          {#each view.nonnegotiables as mark}
            <li>🔴 {mark.target}</li>
          {/each}
        </ul>
        <p class="note">A nonnegotiable is never silently relaxed, ignored, or overridden — if a scenario conflicts with one, it is surfaced for your decision.</p>
      </div>
    {/if}
  </div>
{/if}

<style>
  .walk { max-width: 760px; margin: 0 auto; }
  .loading, .error { color: #b3261e; }
  .content-screen { background: var(--surface-2, #f6f6f7); border-radius: 10px; padding: 12px 16px; margin-bottom: 12px; }
  .content-screen h3 { margin: 0 0 4px; font-size: 1rem; }
  .revise-banner { background: #fdf3e7; border: 1px solid #e5c9a3; border-radius: 10px; padding: 12px 16px; margin-bottom: 12px; font-size: 0.88rem; display: flex; flex-direction: column; gap: 8px; }
  h2 { margin: 8px 0 4px; }
  .prompt { color: var(--text-2, #555); margin: 0 0 8px; }
  .why { font-size: 0.8rem; color: var(--text-3, #888); font-style: italic; }
  .choices { display: flex; flex-direction: column; gap: 8px; margin: 12px 0; }
  .rank-row { display: flex; gap: 6px; align-items: center; }
  .rank-row .rank-main { flex: 1; }
  .rank-controls { display: flex; gap: 4px; }
  .rank-controls button { margin-left: 0; cursor: pointer; padding: 6px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); background: var(--surface, #fff); font: inherit; }
  .choice { text-align: left; padding: 12px 16px; border-radius: 10px; border: 1px solid var(--border, #ccc); background: var(--surface, #fff); cursor: pointer; display: flex; justify-content: space-between; gap: 8px; align-items: center; }
  .choice:hover { border-color: var(--accent, #2f6f4f); }
  .choice.selected { border-color: var(--accent, #2f6f4f); box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent, #2f6f4f) 25%, transparent); }
  .choice.subtle { border-style: dashed; color: var(--text-2, #555); }
  .rank-controls button { margin-left: 4px; cursor: pointer; }
  .field { display: flex; flex-direction: column; gap: 4px; margin: 10px 0; font-size: 0.9rem; }
  .field input, .field select { padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); font: inherit; }
  .note { font-size: 0.82rem; color: var(--text-3, #777); }
  .nav { display: flex; align-items: center; gap: 12px; margin-top: 16px; flex-wrap: wrap; }
  button.primary, button.secondary { padding: 10px 18px; border-radius: 10px; font: inherit; cursor: pointer; border: 1px solid transparent; }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  button.link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 4px; }
  .answered { margin-top: 20px; border-top: 1px solid var(--border, #e2e2e4); padding-top: 12px; }
  .answered ul { list-style: none; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  .answer-head { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
  .answer-value { color: var(--text-2, #444); }
  .versions { display: block; font-size: 0.78rem; color: var(--text-3, #888); }
  .skipped { color: var(--text-3, #999); }
  .nonneg { margin-top: 16px; background: #fdeeee; border: 1px solid #e5b8b8; border-radius: 10px; padding: 12px 16px; font-size: 0.88rem; }
  .nonneg ul { margin: 4px 0; }
</style>
