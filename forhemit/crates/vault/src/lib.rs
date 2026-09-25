//! The Local Vault engine — ForhemitDYI's encrypted local document store
//! (`Local Vault - Workspace Engine.md`: §56 items 1–8 and 11–12 for the
//! minimum viable vault, §9 for the key hierarchy, §47 for the engine
//! contract).
//!
//! Layering, top to bottom:
//!
//! - [`VaultEngine`] — the engine facade: validates input, mints
//!   identifiers, maintains the in-memory search index, and emits one
//!   audit event per material change through the shared
//!   [`enginekit::AuditSink`] contract. Nothing here performs I/O beyond
//!   the vault database and the backup file the owner names — no
//!   network, ever (v1 is local-only per the implementation spec).
//! - [`VaultStore`] — the SQLite document store; ciphertext only.
//! - [`RecoveryEscrow`] / backup — Argon2id recovery and `age` backup
//!   export.
//!
//! Audit payloads deliberately carry identifiers, hashes, and byte
//! counts — never filenames or content: the audit store is a separate
//! plaintext database, and document metadata belongs inside the
//! encrypted boundary.
//!
//! The event vocabulary lives in `forhemit-contracts`; this crate
//! depends on contracts and enginekit only, never on another engine's
//! internals.

mod backup;
mod crypto;
mod error;
mod keys;
mod recovery;
mod search;
mod store;

pub use backup::{export_backup as export_backup_bytes, import_backup as import_backup_bytes};
pub use error::VaultError;
pub use keys::{MemoryKeyStore, OsKeyRing, VaultKey, VaultKeyStore};
pub use recovery::RecoveryEscrow;
pub use search::{SearchHit, SearchIndex};
pub use store::{IntegrityFailure, StoredVersion, StoredVersionContent, VaultStore};

use forhemit_contracts::{
    ActorRecord, AnalysisId, AuditEventDraft, AuditEventType, CorrelationId, DocumentId,
    DocumentVersionId, EngineId, ObjectId, VaultId, WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use std::sync::{Arc, Mutex};

/// A fresh identifier for a created object (ULID), as the docs prescribe
/// for engine identifiers.
fn mint_id() -> String {
    ulid::Ulid::new().to_string()
}

/// The Local Vault engine: import, version, search, analyze, back up,
/// and restore business documents — fully offline.
///
/// Every constructor ends with the vault unlocked or the attempt failed:
/// the key comes from the OS keychain ([`VaultKeyStore`]), or from the
/// recovery escrow when the keychain holds none. When the keychain has
/// no key the engine refuses to start rather than degrade — "vault
/// locked" is a state the UI must surface, not work around (Vault doc
/// §50).
pub struct VaultEngine {
    vault_id: VaultId,
    workspace_id: WorkspaceId,
    store: VaultStore,
    keystore: Arc<dyn VaultKeyStore>,
    sink: Arc<dyn AuditSink<AuditEventDraft>>,
    clock: Arc<dyn Clock>,
    /// In-memory FTS5 index over decrypted content; rebuilt from the
    /// encrypted documents whenever the vault opens (Vault doc §50) and
    /// updated on every version write.
    index: Mutex<SearchIndex>,
}

impl VaultEngine {
    /// Creates a new vault: generates the vault key, escrows it under an
    /// Argon2id-derived recovery key, records the vault's identity rows,
    /// and binds the key into the key store. The recovery passphrase is
    /// the owner's offline credential — it is never stored, only its
    /// derived key wraps the vault key.
    pub fn create(
        store: VaultStore,
        keystore: Arc<dyn VaultKeyStore>,
        sink: Arc<dyn AuditSink<AuditEventDraft>>,
        clock: Arc<dyn Clock>,
        workspace_id: WorkspaceId,
        recovery_passphrase: &str,
    ) -> Result<Self, VaultError> {
        if store.read_workspace_id()?.is_some() {
            return Err(VaultError::VaultAlreadyExists);
        }
        // One identity: the store was opened with the caller's vault id —
        // the escrow row carries that same id, and recovery re-derives the
        // AAD binding from it. Minting a second id here would seal the
        // escrow under an identity the store never records, making
        // recovery structurally impossible.
        let vault_id = store.vault_id().clone();
        let vault_key = VaultKey::from_bytes(crypto::generate_key());
        let escrow = RecoveryEscrow::create(&vault_id, &vault_key, recovery_passphrase)?;
        let created_at = clock.now().unix_timestamp_nanos();
        store.write_vault_record(&workspace_id, created_at)?;
        store.write_escrow(&escrow)?;
        keystore.store(&vault_id, &vault_key)?;
        Ok(Self {
            vault_id,
            workspace_id,
            store,
            keystore,
            sink,
            clock,
            index: Mutex::new(SearchIndex::new()?),
        })
    }

    /// Opens an existing vault with the key from the key store. A missing
    /// key is [`VaultError::VaultKeyUnavailable`] — the vault-locked
    /// state; recovery goes through [`VaultEngine::recover`].
    pub fn open(
        store: VaultStore,
        keystore: Arc<dyn VaultKeyStore>,
        sink: Arc<dyn AuditSink<AuditEventDraft>>,
        clock: Arc<dyn Clock>,
        workspace_id: WorkspaceId,
    ) -> Result<Self, VaultError> {
        let engine = Self {
            vault_id: store.vault_id().clone(),
            workspace_id,
            store,
            keystore,
            sink,
            clock,
            index: Mutex::new(SearchIndex::new()?),
        };
        if engine.store.read_workspace_id()?.as_ref() != Some(&engine.workspace_id) {
            return Err(VaultError::VaultNotFound);
        }
        // Fail before building the index if the keychain holds no key —
        // no point decrypting anything into memory for a locked vault.
        engine.load_key()?;
        engine.rebuild_search_index()?;
        Ok(engine)
    }

    /// Opens a vault whose keychain entry is gone, using the recovery
    /// passphrase to unwrap the escrowed vault key, then re-binds the
    /// key into the key store (Vault doc §9 recovery arm). A wrong
    /// passphrase is [`VaultError::RecoveryFailed`] — nothing else about
    /// the failure is exposed.
    pub fn recover(
        store: VaultStore,
        keystore: Arc<dyn VaultKeyStore>,
        sink: Arc<dyn AuditSink<AuditEventDraft>>,
        clock: Arc<dyn Clock>,
        workspace_id: WorkspaceId,
        recovery_passphrase: &str,
    ) -> Result<Self, VaultError> {
        let engine = Self {
            vault_id: store.vault_id().clone(),
            workspace_id,
            store,
            keystore,
            sink,
            clock,
            index: Mutex::new(SearchIndex::new()?),
        };
        if engine.store.read_workspace_id()?.as_ref() != Some(&engine.workspace_id) {
            return Err(VaultError::VaultNotFound);
        }
        let escrow = engine
            .store
            .read_escrow()?
            .ok_or(VaultError::VaultNotFound)?;
        let vault_key = escrow.open(&engine.vault_id, recovery_passphrase)?;
        engine.keystore.store(&engine.vault_id, &vault_key)?;
        engine.rebuild_search_index()?;
        Ok(engine)
    }

    /// Restores a vault from an exported backup file: decrypts the `age`
    /// envelope with the recovery passphrase, writes the database to
    /// `db_path` (refusing to clobber an existing file), re-derives the
    /// vault key from the recovery escrow inside the restored database,
    /// and re-binds it into the key store.
    #[allow(clippy::too_many_arguments)] // restore threads every dependency through explicitly
    pub fn restore_from_backup(
        backup_path: &std::path::Path,
        recovery_passphrase: &str,
        db_path: &std::path::Path,
        keystore: Arc<dyn VaultKeyStore>,
        sink: Arc<dyn AuditSink<AuditEventDraft>>,
        clock: Arc<dyn Clock>,
        workspace_id: WorkspaceId,
    ) -> Result<Self, VaultError> {
        if db_path.exists() {
            return Err(VaultError::InvalidInput(format!(
                "refusing to overwrite an existing vault database at {}",
                db_path.display()
            )));
        }
        let database_bytes = backup::import_backup(backup_path, recovery_passphrase)?;
        std::fs::write(db_path, &database_bytes)
            .map_err(|error| VaultError::Backup(format!("write restored database: {error}")))?;
        let store = VaultStore::open_discovering(db_path)?;
        Self::recover(
            store,
            keystore,
            sink,
            clock,
            workspace_id,
            recovery_passphrase,
        )
    }

    /// The vault's identifier.
    pub fn vault_id(&self) -> &VaultId {
        &self.vault_id
    }

    /// The workspace the vault belongs to.
    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    /// Imports a document as its first version: encrypted at rest with a
    /// fresh per-version key wrapped by the vault key, indexed for local
    /// search, and recorded in the audit trail (Vault doc §56 item 3;
    /// "Importing a document does not upload it").
    pub fn import_document(
        &self,
        actor: &ActorRecord,
        filename: &str,
        content: &[u8],
        note: Option<&str>,
    ) -> Result<StoredVersionContent, VaultError> {
        if filename.trim().is_empty() {
            return Err(VaultError::InvalidInput(
                "filename must not be empty".into(),
            ));
        }
        let key = self.load_key()?;
        let document_id = DocumentId::new(mint_id())?;
        let version_id = DocumentVersionId::new(mint_id())?;
        let created_at = self.clock.now().unix_timestamp_nanos();
        let sequence = self.store.next_sequence(&document_id)?;
        let content_hash = self.store.insert_version(
            &key,
            &document_id,
            &version_id,
            sequence,
            filename,
            content,
            note,
            created_at,
            None,
            None,
        )?;
        self.index_version(&document_id, &version_id, filename, content)?;
        self.emit(
            AuditEventType::DocumentImported,
            document_id.as_str(),
            actor,
            serde_json::json!({
                "document_id": document_id.as_str(),
                "version_id": version_id.as_str(),
                "content_hash": content_hash.as_str(),
                "byte_count": content.len(),
            }),
        )?;
        self.store.read_version_with_content(&key, &version_id)
    }

    /// Adds a new version to an existing document (Vault doc §14:
    /// "Versioning is mandatory"). The new version supersedes the
    /// document's current head; history is retained, never rewritten.
    pub fn add_version(
        &self,
        actor: &ActorRecord,
        document_id: &DocumentId,
        filename: &str,
        content: &[u8],
        note: Option<&str>,
    ) -> Result<StoredVersionContent, VaultError> {
        if filename.trim().is_empty() {
            return Err(VaultError::InvalidInput(
                "filename must not be empty".into(),
            ));
        }
        let key = self.load_key()?;
        let previous = self
            .store
            .latest_version(&key, document_id)?
            .ok_or_else(|| VaultError::DocumentNotFound {
                document_id: document_id.as_str().to_owned(),
            })?;
        let version_id = DocumentVersionId::new(mint_id())?;
        let created_at = self.clock.now().unix_timestamp_nanos();
        let sequence = self.store.next_sequence(document_id)?;
        let content_hash = self.store.insert_version(
            &key,
            document_id,
            &version_id,
            sequence,
            filename,
            content,
            note,
            created_at,
            None,
            Some(&previous.version_id),
        )?;
        self.index_version(document_id, &version_id, filename, content)?;
        self.emit(
            AuditEventType::DocumentVersionCreated,
            document_id.as_str(),
            actor,
            serde_json::json!({
                "document_id": document_id.as_str(),
                "version_id": version_id.as_str(),
                "supersedes_version_id": previous.version_id.as_str(),
                "content_hash": content_hash.as_str(),
                "byte_count": content.len(),
            }),
        )?;
        self.store.read_version_with_content(&key, &version_id)
    }

    /// Restores a previous version as a new version at the head of the
    /// document's chain — history is never rewritten (Vault doc §14:
    /// "Restore a previous version"; §53.2). Restoring the current head
    /// is refused: it would duplicate the version, not restore anything.
    pub fn restore_version(
        &self,
        actor: &ActorRecord,
        source_version_id: &DocumentVersionId,
        note: Option<&str>,
    ) -> Result<StoredVersionContent, VaultError> {
        let key = self.load_key()?;
        let source = self
            .store
            .read_version_with_content(&key, source_version_id)?;
        let document_id = source.version.document_id.clone();
        let previous = self
            .store
            .latest_version(&key, &document_id)?
            .ok_or_else(|| VaultError::DocumentNotFound {
                document_id: document_id.as_str().to_owned(),
            })?;
        if &previous.version_id == source_version_id {
            return Err(VaultError::InvalidInput(
                "version is already the document's current head".to_owned(),
            ));
        }
        let version_id = DocumentVersionId::new(mint_id())?;
        let created_at = self.clock.now().unix_timestamp_nanos();
        let sequence = self.store.next_sequence(&document_id)?;
        self.store.insert_version(
            &key,
            &document_id,
            &version_id,
            sequence,
            &source.version.filename,
            &source.content,
            note,
            created_at,
            Some(source_version_id),
            Some(&previous.version_id),
        )?;
        self.index_version(
            &document_id,
            &version_id,
            &source.version.filename,
            &source.content,
        )?;
        self.emit(
            AuditEventType::DocumentRestored,
            document_id.as_str(),
            actor,
            serde_json::json!({
                "document_id": document_id.as_str(),
                "restored_version_id": source_version_id.as_str(),
                "new_version_id": version_id.as_str(),
                "supersedes_version_id": previous.version_id.as_str(),
            }),
        )?;
        self.store.read_version_with_content(&key, &version_id)
    }

    /// Reads the latest version of a document with its decrypted content
    /// — the `getDocument()` engine contract (Vault doc §47).
    pub fn read_document(
        &self,
        document_id: &DocumentId,
    ) -> Result<StoredVersionContent, VaultError> {
        let key = self.load_key()?;
        self.store
            .latest_version_with_content(&key, document_id)?
            .ok_or_else(|| VaultError::DocumentNotFound {
                document_id: document_id.as_str().to_owned(),
            })
    }

    /// Reads one specific version with its decrypted content.
    pub fn read_version(
        &self,
        version_id: &DocumentVersionId,
    ) -> Result<StoredVersionContent, VaultError> {
        let key = self.load_key()?;
        self.store.read_version_with_content(&key, version_id)
    }

    /// Full version history of a document, oldest first (Vault doc §53.2:
    /// "version history retained for audit and restoration").
    pub fn version_history(
        &self,
        document_id: &DocumentId,
    ) -> Result<Vec<StoredVersion>, VaultError> {
        let key = self.load_key()?;
        let history = self.store.version_history(&key, document_id)?;
        if history.is_empty() {
            return Err(VaultError::DocumentNotFound {
                document_id: document_id.as_str().to_owned(),
            });
        }
        Ok(history)
    }

    /// Every document in the vault (metadata-only list view).
    pub fn list_documents(&self) -> Result<Vec<DocumentId>, VaultError> {
        self.store.list_documents()
    }

    /// Records a local-analysis run over a document's latest version
    /// (Vault doc §47 `LocalAnalysis`; §45 "Document analyzed"). The
    /// summary is the analysis tool's own output — expected to be
    /// non-content statistics; raw document content never enters the
    /// audit trail.
    pub fn record_analysis(
        &self,
        actor: &ActorRecord,
        document_id: &DocumentId,
        tool: &str,
        summary: &str,
    ) -> Result<AnalysisId, VaultError> {
        if tool.trim().is_empty() {
            return Err(VaultError::InvalidInput("tool must not be empty".into()));
        }
        let key = self.load_key()?;
        let latest = self
            .store
            .latest_version(&key, document_id)?
            .ok_or_else(|| VaultError::DocumentNotFound {
                document_id: document_id.as_str().to_owned(),
            })?;
        let analysis_id = AnalysisId::new(mint_id())?;
        let created_at = self.clock.now().unix_timestamp_nanos();
        self.store.record_analysis(
            &analysis_id,
            document_id,
            &latest.version_id,
            tool,
            summary,
            created_at,
        )?;
        self.emit(
            AuditEventType::DocumentAnalyzed,
            document_id.as_str(),
            actor,
            serde_json::json!({
                "analysis_id": analysis_id.as_str(),
                "document_id": document_id.as_str(),
                "version_id": latest.version_id.as_str(),
                "tool": tool,
                "used_external_services": false,
            }),
        )?;
        Ok(analysis_id)
    }

    /// Full-text search over indexed filenames and document content
    /// (Vault doc §56 item 11). The index lives only in memory; a query
    /// never reaches the database's encrypted pages.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, VaultError> {
        let index = self
            .index
            .lock()
            .map_err(|_| VaultError::Internal("search index lock poisoned".to_owned()))?;
        index.search(query, limit)
    }

    /// Rebuilds the search index from the encrypted documents — the §50
    /// recovery path. Returns the number of indexed versions (the latest
    /// version of each document; superseded versions stay retrievable
    /// but out of the index).
    pub fn rebuild_search_index(&self) -> Result<usize, VaultError> {
        let key = self.load_key()?;
        let index = self
            .index
            .lock()
            .map_err(|_| VaultError::Internal("search index lock poisoned".to_owned()))?;
        index.clear()?;
        let mut indexed = 0usize;
        for document_id in self.store.list_documents()? {
            let latest = self.store.latest_version_with_content(&key, &document_id)?;
            if let Some(full) = latest {
                index.index_version(
                    &full.version.document_id,
                    &full.version.version_id,
                    &full.version.filename,
                    &full.content,
                )?;
                indexed += 1;
            }
        }
        Ok(indexed)
    }

    /// Integrity sweep across every stored version (Vault doc §56 item
    /// 12's verification duty): authenticated decryption plus hash
    /// recomputation. An empty result is an intact vault.
    pub fn verify_integrity(&self) -> Result<Vec<IntegrityFailure>, VaultError> {
        let key = self.load_key()?;
        self.store.verify_all_hashes(&key)
    }

    /// Exports the whole vault as an `age`-encrypted backup file
    /// (Vault doc §35: "The Vault should support encrypted backups").
    /// The database snapshot is ciphertext already; the backup envelope
    /// adds a second, passphrase-held layer so a stolen backup file
    /// alone reveals nothing.
    pub fn export_backup(
        &self,
        actor: &ActorRecord,
        passphrase: &str,
        output_path: &std::path::Path,
    ) -> Result<(), VaultError> {
        let snapshot = self.store.snapshot_bytes()?;
        backup::export_backup(&snapshot, passphrase, output_path)?;
        self.emit(
            AuditEventType::BackupCompleted,
            self.vault_id.as_str(),
            actor,
            serde_json::json!({
                "vault_id": self.vault_id.as_str(),
                "byte_count": snapshot.len(),
            }),
        )?;
        Ok(())
    }

    /// The vault key from the key store — `VaultKeyUnavailable` when the
    /// vault is locked (Vault doc §50).
    fn load_key(&self) -> Result<VaultKey, VaultError> {
        self.keystore
            .load(&self.vault_id)?
            .ok_or(VaultError::VaultKeyUnavailable)
    }

    /// Indexes one version in the in-memory search index.
    fn index_version(
        &self,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        filename: &str,
        content: &[u8],
    ) -> Result<(), VaultError> {
        let index = self
            .index
            .lock()
            .map_err(|_| VaultError::Internal("search index lock poisoned".to_owned()))?;
        index.index_version(document_id, version_id, filename, content)
    }

    /// Emits one audit event for a material change. Emission failures
    /// fail the operation — an unrecorded change is not a completed
    /// change (audit-first, Implementation Roadmap Phase 1).
    fn emit(
        &self,
        event_type: AuditEventType,
        object_id: &str,
        actor: &ActorRecord,
        payload: serde_json::Value,
    ) -> Result<(), VaultError> {
        let draft = AuditEventDraft {
            event_type,
            source_engine: EngineId::Vault,
            source_object: ObjectId::new(object_id)?,
            actor: actor.clone(),
            workspace_id: self.workspace_id.clone(),
            transaction_id: None,
            correlation_id: CorrelationId::new(mint_id())?,
            causation_id: None,
            payload,
        };
        self.sink
            .emit(draft)
            .map_err(|error| VaultError::Audit(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use forhemit_enginekit::{AuditError, SystemClock};
    use std::sync::Mutex as StdMutex;

    #[derive(Default)]
    struct SilentSink {
        events: StdMutex<Vec<AuditEventDraft>>,
    }

    impl AuditSink<AuditEventDraft> for SilentSink {
        fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
            self.events.lock().unwrap().push(event);
            Ok(())
        }
    }

    fn workspace() -> WorkspaceId {
        WorkspaceId::new("local-workspace").unwrap()
    }

    /// Regression: the shell mints the vault id, opens the store with it,
    /// and only then calls create. Recovery must still work across a
    /// restart — the escrow's AAD binding and the store's identity have to
    /// agree, or the owner is permanently locked out.
    #[test]
    fn recovery_works_when_the_store_carries_a_caller_minted_id() {
        let dir = std::env::temp_dir().join(format!(
            "forhemit-vault-identity-{}-{}",
            std::process::id(),
            mint_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let db_path = dir.join("vault.sqlite3");

        let caller_minted = VaultId::new(format!("vlt_{}", mint_id())).unwrap();
        let store = VaultStore::open(caller_minted.clone(), &db_path).unwrap();
        let engine = VaultEngine::create(
            store,
            Arc::new(MemoryKeyStore::default()),
            Arc::new(SilentSink::default()),
            Arc::new(SystemClock),
            workspace(),
            "correct horse battery staple",
        )
        .unwrap();
        let original_key = engine.vault_id().clone();
        assert_eq!(
            original_key, caller_minted,
            "engine must adopt the store's identity"
        );
        drop(engine);

        // Restart: fresh key store (the keychain entry is gone), the id is
        // re-discovered from the escrow row.
        let store = VaultStore::open_discovering(&db_path).unwrap();
        assert_eq!(store.vault_id(), &caller_minted);
        let recovered = VaultEngine::recover(
            store,
            Arc::new(MemoryKeyStore::default()),
            Arc::new(SilentSink::default()),
            Arc::new(SystemClock),
            workspace(),
            "correct horse battery staple",
        )
        .expect("recovery with the right passphrase must open the escrow");
        assert_eq!(recovered.vault_id(), &caller_minted);

        // And a wrong passphrase still fails honestly, without exposing why.
        let store = VaultStore::open_discovering(&db_path).unwrap();
        let wrong = VaultEngine::recover(
            store,
            Arc::new(MemoryKeyStore::default()),
            Arc::new(SilentSink::default()),
            Arc::new(SystemClock),
            workspace(),
            "not the passphrase",
        );
        assert!(matches!(&wrong, Err(VaultError::RecoveryFailed)));

        std::fs::remove_dir_all(&dir).ok();
    }
}
