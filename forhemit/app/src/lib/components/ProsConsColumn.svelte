<script lang="ts">
  // The tradeoff column: what the selected choice gains and what it gives
  // up. Presentational — props in, nothing else; renders nothing when the
  // option carries no pros/cons. Copy is balanced and factual by contract —
  // never a ranking or recommendation.
  let { pros, cons }: { pros?: string[] | null; cons?: string[] | null } = $props();

  const hasPros = $derived(!!pros && pros.length > 0);
  const hasCons = $derived(!!cons && cons.length > 0);
</script>

{#if hasPros || hasCons}
  <aside class="pros-cons" aria-label="Tradeoffs of the selected choice">
    {#if hasPros}
      <section class="col">
        <h4>What this choice gains</h4>
        <ul>
          {#each pros as item}
            <li>{item}</li>
          {/each}
        </ul>
      </section>
    {/if}
    {#if hasCons}
      <section class="col">
        <h4>What it gives up</h4>
        <ul>
          {#each cons as item}
            <li>{item}</li>
          {/each}
        </ul>
      </section>
    {/if}
  </aside>
{/if}

<style>
  .pros-cons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    margin: 12px 0;
    border-left: 3px solid var(--accent, #2f6f4f);
    padding-left: 12px;
  }
  .col {
    background: var(--surface, #fff);
    border: 1px solid var(--border, #ccc);
    border-radius: 10px;
    padding: 12px 16px;
  }
  h4 {
    margin: 0 0 6px;
    font-size: 0.78rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--text-3, #777);
  }
  ul {
    margin: 0;
    padding-left: 18px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 0.88rem;
    color: var(--text-2, #555);
  }
  @media (max-width: 640px) {
    .pros-cons {
      grid-template-columns: 1fr;
    }
  }
</style>
