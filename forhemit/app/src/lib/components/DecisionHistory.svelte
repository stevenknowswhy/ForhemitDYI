<script lang="ts">
  // The decision history rail: every decision made, in order, grouped by
  // journey stage. Clicking a past decision reopens it for change — the
  // host routes that through the engine's audited revision path; this
  // component only reports which node was clicked. Presentational: props
  // in, one callback out, no API calls.
  import type { SkippedNodeView } from "../types";
  import { groupDecisions, type AnsweredNode } from "./decisionHistory";

  let {
    answered,
    skipped = [],
    revisingNodeId = null,
    onSelect,
  }: {
    answered: AnsweredNode[];
    skipped?: (string | SkippedNodeView)[];
    revisingNodeId?: string | null;
    onSelect: (nodeId: string) => void;
  } = $props();

  // Below this width the rail collapses to a compact pill (matches the
  // 640px breakpoint the walk layouts and ProsConsColumn use).
  const NARROW_PX = 640;

  let narrow = $state(false);
  let expanded = $state(false);

  function measure() {
    narrow = window.innerWidth < NARROW_PX;
  }

  $effect(() => {
    measure();
    window.addEventListener("resize", measure);
    return () => window.removeEventListener("resize", measure);
  });

  // A revision re-asks a past question in place — make sure its rail node
  // is actually visible when the host highlights it.
  $effect(() => {
    if (revisingNodeId) expanded = true;
  });

  const groups = $derived(groupDecisions(answered, skipped));
  const decidedCount = $derived(answered.length);
  const skippedCount = $derived(skipped.length);
  const totalCount = $derived(decidedCount + skippedCount);

  const countLabel = $derived(
    `${decidedCount} ${decidedCount === 1 ? "decision" : "decisions"} made`,
  );
</script>

{#if totalCount > 0}
  {#if narrow && !expanded}
    <button type="button" class="pill" onclick={() => (expanded = true)}>
      {countLabel}{skippedCount > 0 ? ` · ${skippedCount} skipped` : ""}
    </button>
  {:else}
    <aside class="rail" aria-label="Your decisions">
      <header>
        <p class="rail-title">Your decisions</p>
        <p class="rail-hint">{countLabel} — click any to revisit</p>
      </header>
      {#each groups as group}
        <p class="stage">{group.label}</p>
        <ul>
          {#each group.entries as entry}
            <li>
              {#if entry.skipped}
                <span class="node skipped">
                  <span class="node-title">{entry.title}</span>
                  <span class="tag">skipped</span>
                </span>
              {:else}
                <button type="button" class="node" class:revising={entry.nodeId === revisingNodeId}
                  onclick={() => onSelect(entry.nodeId)}>
                  <span class="node-head">
                    <span class="node-title">{entry.title}</span>
                    {#if entry.versions > 1}
                      <span class="tag">{entry.versions} versions</span>
                    {/if}
                    {#if entry.nodeId === revisingNodeId}
                      <span class="tag revising-tag">← revising</span>
                    {/if}
                  </span>
                  <span class="node-summary">{entry.summary}</span>
                </button>
              {/if}
            </li>
          {/each}
        </ul>
      {/each}
      {#if narrow}
        <button type="button" class="hide" onclick={() => (expanded = false)}>Hide history</button>
      {/if}
    </aside>
  {/if}
{/if}

<style>
  .pill {
    display: block;
    width: 100%;
    text-align: left;
    padding: 10px 16px;
    border-radius: 999px;
    border: 1px solid var(--border, #ccc);
    background: var(--surface, #fff);
    color: var(--text-2, #555);
    font: inherit;
    font-size: 0.85rem;
    cursor: pointer;
    margin: 12px 0;
  }
  .pill:hover {
    border-color: var(--accent, #2f6f4f);
  }
  .rail {
    margin: 12px 0;
    padding: 12px 16px;
    background: var(--surface-2, #f6f6f7);
    border: 1px solid var(--border, #e2e2e4);
    border-radius: 10px;
  }
  .rail-title {
    margin: 0;
    font-weight: 600;
  }
  .rail-hint {
    margin: 2px 0 10px;
    font-size: 0.78rem;
    color: var(--text-3, #888);
  }
  .stage {
    margin: 10px 0 4px;
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--text-3, #888);
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .node {
    display: block;
    width: 100%;
    text-align: left;
    padding: 8px 12px;
    border-radius: 10px;
    border: 1px solid var(--border, #ccc);
    background: var(--surface, #fff);
    cursor: pointer;
    font: inherit;
  }
  .node:hover {
    border-color: var(--accent, #2f6f4f);
  }
  .node.revising {
    border-color: var(--accent, #2f6f4f);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent, #2f6f4f) 25%, transparent);
  }
  .node.skipped {
    cursor: default;
    border-style: dashed;
    color: var(--text-3, #999);
  }
  .node-head {
    display: flex;
    align-items: baseline;
    gap: 8px;
    flex-wrap: wrap;
  }
  .node-title {
    font-weight: 500;
  }
  .node-summary {
    display: block;
    font-size: 0.8rem;
    color: var(--text-2, #555);
  }
  .tag {
    font-size: 0.72rem;
    color: var(--text-3, #888);
    border: 1px solid var(--border, #ddd);
    border-radius: 999px;
    padding: 1px 8px;
    white-space: nowrap;
  }
  .revising-tag {
    color: var(--accent, #2f6f4f);
    border-color: var(--accent, #2f6f4f);
  }
  .hide {
    margin-top: 10px;
    background: none;
    border: none;
    color: var(--text-3, #888);
    font: inherit;
    font-size: 0.8rem;
    cursor: pointer;
    padding: 4px;
  }
</style>
