<script lang="ts">
  // Vault screens — import (file picker → encrypt → store), document list
  // with versions, FTS5 search, and backups, with the honest v1 sharing
  // wording: v1 keeps everything on this device; the owner exports the
  // package file themselves. Locked and not-set-up states are first-class
  // screens, never silent failures.
  import { api } from "../api";
  import { timeLabel } from "./bits";
  import LayerChip from "./LayerChip.svelte";
  import type {
    VaultDocumentView,
    VaultSearchHitView,
    VaultStatusView,
    VaultVersionContentView,
    VaultVersionView,
  } from "../types";

  const SHARING_WORDING =
    "v1 keeps everything on this device. There is no cloud copy and no sharing " +
    "link — when information needs to leave, you export the package file yourself.";

  let status = $state<VaultStatusView | null>(null);
  let documents = $state<VaultDocumentView[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);

  let passphrase = $state("");
  let confirmPassphrase = $state("");

  let importFile = $state<File | null>(null);
  let importNote = $state("");
  let importing = $state(false);

  let searchQuery = $state("");
  let searchHits = $state<VaultSearchHitView[] | null>(null);
  let searching = $state(false);

  let openDocumentId = $state<string | null>(null);
  let documentVersions = $state<VaultVersionView[]>([]);
  let versionContent = $state<VaultVersionContentView | null>(null);

  async function loadStatus() {
    loading = true;
    error = null;
    try {
      status = await api.vaultStatus();
      if (status.state === "ready") {
        documents = await api.vaultDocuments();
      } else {
        documents = [];
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void loadStatus();
  });

  async function setup() {
    error = null;
    notice = null;
    if (passphrase.length < 8) {
      error = "The recovery passphrase must be at least 8 characters.";
      return;
    }
    if (passphrase !== confirmPassphrase) {
      error = "The two passphrases do not match.";
      return;
    }
    try {
      status = await api.vaultSetup(passphrase);
      documents = await api.vaultDocuments();
      notice =
        "Vault created. The recovery passphrase is the only way back in if the key is lost — store it somewhere safe.";
      passphrase = "";
      confirmPassphrase = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function recover() {
    error = null;
    notice = null;
    try {
      status = await api.vaultRecover(passphrase);
      documents = status.state === "ready" ? await api.vaultDocuments() : [];
      notice = "Vault opened with the recovery passphrase.";
      passphrase = "";
      confirmPassphrase = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function pickImport(event: Event) {
    const files = (event.currentTarget as HTMLInputElement).files;
    importFile = files && files.length > 0 ? files[0] : null;
  }

  // The file never leaves this device: it is read here and handed to the
  // shell, which encrypts it before it touches disk.
  async function importDocument() {
    if (!importFile) return;
    importing = true;
    error = null;
    notice = null;
    try {
      const bytes = new Uint8Array(await importFile.arrayBuffer());
      let binary = "";
      const chunk = 0x8000;
      for (let offset = 0; offset < bytes.length; offset += chunk) {
        binary += String.fromCharCode(...bytes.subarray(offset, offset + chunk));
      }
      const stored = await api.vaultImport(importFile.name, btoa(binary), importNote.trim() || null);
      importFile = null;
      importNote = "";
      notice = `Imported “${stored.filename}” — encrypted at rest on this device.`;
      documents = await api.vaultDocuments();
      status = await api.vaultStatus();
    } catch (e) {
      error = String(e);
    } finally {
      importing = false;
    }
  }

  async function runSearch() {
    if (!searchQuery.trim()) return;
    searching = true;
    error = null;
    try {
      searchHits = await api.vaultSearch(searchQuery.trim());
    } catch (e) {
      error = String(e);
    } finally {
      searching = false;
    }
  }

  function clearSearch() {
    searchQuery = "";
    searchHits = null;
  }

  async function openDocument(document: VaultDocumentView) {
    error = null;
    versionContent = null;
    if (openDocumentId === document.document_id) {
      openDocumentId = null;
      documentVersions = [];
      return;
    }
    try {
      openDocumentId = document.document_id;
      const history = await api.vaultDocumentHistory(document.document_id);
      documentVersions = history.versions;
    } catch (e) {
      error = String(e);
    }
  }

  async function openVersion(versionId: string) {
    error = null;
    try {
      versionContent = await api.vaultDocumentContent(versionId);
    } catch (e) {
      error = String(e);
    }
  }

  function closeVersion() {
    versionContent = null;
  }

  async function backup() {
    error = null;
    notice = null;
    if (passphrase.length < 8) {
      error = "Enter the recovery passphrase (at least 8 characters) to encrypt the backup.";
      return;
    }
    try {
      const file = await api.vaultBackup(passphrase);
      notice = `Backup saved as “${file.file_name}” in ${file.saved_path} — passphrase-encrypted, restorable only with that passphrase.`;
      passphrase = "";
      confirmPassphrase = "";
    } catch (e) {
      error = String(e);
    }
  }

  function byteLabel(count: number): string {
    if (count < 1024) return `${count} B`;
    if (count < 1024 * 1024) return `${(count / 1024).toFixed(1)} KB`;
    return `${(count / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<section class="vault">
  <header class="vault-head">
    <h1>Document vault</h1>
    <p class="lede">
      Imported documents are encrypted before they touch the disk and stay on
      this device. Search runs locally — nothing is uploaded to anything.
    </p>
    <LayerChip kind="system" label="Importing a document does not upload it — v1 has no upload" />
  </header>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}
  {#if notice}
    <p class="notice">{notice}</p>
  {/if}

  {#if loading}
    <p class="loading">Checking the vault…</p>
  {:else if status === null}
    <p class="error" role="alert">The vault status could not be read.</p>
  {:else if status.state === "not_set_up"}
    <section class="state-panel">
      <h2>Set up your vault</h2>
      <p class="hint">
        The vault encrypts every document with its own key, protected by a
        vault key held in this machine's keychain. Choose a recovery
        passphrase: it is the only way back in if the key is lost. It is never
        sent anywhere — it stays on this device.
      </p>
      <div class="form">
        <input type="password" placeholder="Recovery passphrase (8+ characters)" bind:value={passphrase} />
        <input type="password" placeholder="Repeat the passphrase" bind:value={confirmPassphrase} />
        <button type="button" class="primary" onclick={setup}>Create vault</button>
      </div>
      <p class="sharing">{SHARING_WORDING}</p>
    </section>
  {:else if status.state === "locked"}
    <section class="state-panel">
      <h2>The vault is locked</h2>
      <p class="hint">
        Your documents are safe on disk but unreadable this session — no data
        is lost. Unlock with your recovery passphrase.
      </p>
      <div class="form">
        <input type="password" placeholder="Recovery passphrase" bind:value={passphrase} />
        <button type="button" class="primary" onclick={recover}>Unlock vault</button>
      </div>
    </section>
  {:else}
    <section class="panel">
      <h2>Import a document</h2>
      <p class="hint">
        Files are encrypted the moment they are stored. Nothing is sent
        anywhere. {SHARING_WORDING}
      </p>
      <div class="form">
        <input type="file" onchange={pickImport} />
        <input type="text" placeholder="Note (optional) — what this document is" bind:value={importNote} />
        <button type="button" class="primary" onclick={importDocument} disabled={!importFile || importing}>
          {importing ? "Encrypting and storing…" : "Import (encrypt → store)"}
        </button>
      </div>
    </section>

    <section class="panel">
      <h2>Search your documents</h2>
      <p class="hint">Full-text search over imported content — runs entirely on this device.</p>
      <div class="form">
        <div class="row">
          <input
            type="search"
            placeholder="Search imported content…"
            bind:value={searchQuery}
            onkeydown={(event) => {
              if (event.key === "Enter") void runSearch();
            }}
          />
          <button type="button" class="primary" onclick={runSearch} disabled={!searchQuery.trim() || searching}>
            {searching ? "Searching…" : "Search"}
          </button>
          {#if searchHits !== null}
            <button type="button" class="secondary" onclick={clearSearch}>Clear</button>
          {/if}
        </div>
      </div>
      {#if searchHits !== null}
        {#if searchHits.length === 0}
          <p class="empty">No documents match “{searchQuery}”.</p>
        {:else}
          <ul class="hits">
            {#each searchHits as hit (hit.version_id)}
              <li>
                <button type="button" class="link" onclick={() => void openVersion(hit.version_id)}>
                  {hit.filename}
                </button>
                <p class="snippet">{hit.snippet}</p>
              </li>
            {/each}
          </ul>
        {/if}
      {/if}
    </section>

    <section class="panel">
      <h2>Documents ({documents.length})</h2>
      {#if documents.length === 0}
        <p class="empty">Nothing imported yet — the vault is ready when you are.</p>
      {:else}
        <ul class="docs">
          {#each documents as document (document.document_id)}
            <li>
              <div class="doc-head">
                <button type="button" class="link" onclick={() => openDocument(document)}>
                  {openDocumentId === document.document_id ? "▾" : "▸"}
                  {document.filename}
                </button>
                <span class="meta">
                  {byteLabel(document.byte_count)} · {document.version_count}
                  {document.version_count === 1 ? "version" : "versions"} ·
                  imported {timeLabel(document.latest_created_at)}
                </span>
              </div>
              {#if document.note}
                <p class="meta">“{document.note}”</p>
              {/if}
              {#if openDocumentId === document.document_id}
                <ul class="versions">
                  {#each documentVersions as version (version.version_id)}
                    <li>
                      <button type="button" class="link" onclick={() => openVersion(version.version_id)}>
                        Version {version.version_number}
                      </button>
                      <span class="meta">
                        {byteLabel(version.byte_count)} · {timeLabel(version.created_at)} ·
                        hash {version.content_hash.slice(0, 12)}…
                        {#if version.restored_from}
                          · restored
                        {/if}
                      </span>
                    </li>
                  {/each}
                </ul>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <section class="panel">
      <h2>Encrypted backup</h2>
      <p class="hint">
        Writes a passphrase-encrypted backup file of every vault document. The
        file is restorable only with your passphrase — on this device or
        another copy of the app.
      </p>
      <div class="form">
        <input type="password" placeholder="Recovery passphrase" bind:value={passphrase} />
        <button type="button" class="primary" onclick={backup}>Create backup file</button>
      </div>
    </section>

    {#if versionContent}
      <section class="version-view">
        <h3>{versionContent.filename}</h3>
        <p class="meta">
          {byteLabel(versionContent.byte_count)} · decrypted for this view only — still stored encrypted
        </p>
        {#if versionContent.content_text !== null}
          <pre class="content">{versionContent.content_text}</pre>
        {:else}
          <p class="hint">
            This file type has no extractable text — its content stays
            encrypted at rest; search indexes whatever could be read on import.
          </p>
        {/if}
        <button type="button" class="secondary" onclick={closeVersion}>Close</button>
      </section>
    {/if}
  {/if}
</section>

<style>
  .vault { display: flex; flex-direction: column; gap: 18px; }
  .vault-head h1 { margin-bottom: 4px; }
  .lede { color: var(--text-2, #555); max-width: 640px; }
  .state-panel, .panel, .version-view { border: 1px solid var(--border, #ddd); border-radius: 10px; padding: 14px 18px; }
  .state-panel h2, .panel h2 { margin-top: 0; }
  .form { display: flex; flex-direction: column; gap: 8px; max-width: 560px; }
  .form input { font: inherit; padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border, #ccc); }
  .row { display: flex; gap: 8px; }
  .row > input[type="search"] { flex: 1; }
  button.primary, button.secondary { padding: 8px 16px; border-radius: 8px; font: inherit; cursor: pointer; border: 1px solid transparent; }
  button.primary { background: var(--accent, #2f6f4f); color: white; }
  button.secondary { background: transparent; border-color: var(--border, #bbb); }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  .link { background: none; border: none; color: var(--accent, #2f6f4f); cursor: pointer; font: inherit; padding: 2px 4px; text-align: left; }
  .meta { color: var(--text-3, #888); font-size: 0.8rem; }
  .hint { color: var(--text-2, #555); font-size: 0.9rem; }
  .sharing { color: var(--text-2, #555); font-size: 0.85rem; border-left: 3px solid var(--accent, #2f6f4f); padding-left: 10px; }
  .docs, .versions, .hits { list-style: none; padding: 6px 0 0; display: flex; flex-direction: column; gap: 10px; }
  .doc-head { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
  .versions { margin-left: 22px; }
  .snippet { margin: 2px 0 0; font-size: 0.85rem; color: var(--text-2, #555); }
  .content { background: var(--surface-2, #f7f7f8); border-radius: 8px; padding: 12px; white-space: pre-wrap; word-break: break-word; max-height: 320px; overflow: auto; }
  .error { color: #b3261e; font-weight: 500; }
  .notice { color: #1d6b3c; }
  .loading, .empty { color: var(--text-3, #888); }
</style>
