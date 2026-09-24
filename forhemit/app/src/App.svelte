<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greeting = $state("");

  async function greet(): Promise<void> {
    greeting = await invoke<string>("greet", { name });
  }

  function handleSubmit(event: SubmitEvent): void {
    event.preventDefault();
    void greet();
  }
</script>

<main>
  <h1>Forhemit</h1>
  <p>
    Local-first workspace for the employee-ownership journey. This scaffold
    runs entirely on your machine — nothing leaves it.
  </p>

  <form onsubmit={handleSubmit}>
    <input
      type="text"
      placeholder="Your name"
      bind:value={name}
      aria-label="Your name"
    />
    <button type="submit">Greet</button>
  </form>

  {#if greeting}
    <p role="status">{greeting}</p>
  {/if}
</main>

<style>
  :root {
    font-family: system-ui, sans-serif;
    color-scheme: light dark;
  }
  main {
    max-width: 40rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }
  form {
    display: flex;
    gap: 0.5rem;
    margin: 1rem 0;
  }
  input {
    flex: 1;
    padding: 0.5rem 0.75rem;
  }
  button {
    padding: 0.5rem 1rem;
    cursor: pointer;
  }
</style>
