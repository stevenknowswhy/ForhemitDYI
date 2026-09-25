<script lang="ts">
  // "Whose information am I looking at?" — THREE DECISION LAYERS.md §14–15
  // requires this distinction on every screen.
  import { OWNER_CHIP, SYSTEM_CHIP } from "./bits";

  let { kind = "owner", label = undefined }: { kind?: "owner" | "system" | "professional"; label?: string } = $props();
</script>

<span class="chip chip-{kind}">
  <span class="dot" aria-hidden="true">{kind === "owner" ? "👤" : kind === "professional" ? "🧑‍⚖️" : "⚙️"}</span>
  {label ?? (kind === "owner" ? OWNER_CHIP : SYSTEM_CHIP)}
</span>

<style>
  .chip {
    display: inline-flex; align-items: center; gap: 6px;
    font-size: 0.72rem; padding: 3px 10px; border-radius: 999px;
    border: 1px solid var(--border, #ccc); color: var(--text-2, #555);
    width: fit-content;
  }
  .chip-owner { background: #eef7ef; border-color: #bcd9c2; }
  .chip-system { background: #eef2f7; border-color: #b9c8dd; }
  .chip-professional { background: #f7f2ee; border-color: #ddc9b9; }
</style>
