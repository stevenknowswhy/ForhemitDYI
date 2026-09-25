//! The encrypted document store — SQLite at rest, ciphertext only
//! (Vault doc §53: the vault's persistent layer; §13 per-version fields;
//! §56 items 1–8 and 11–12).
//!
//! What never touches disk here: plaintext document content, plaintext
//! filenames, plaintext notes, and the vault key. Filenames, notes, and
//! content are sealed with each version's document key (each under its
//! own AAD namespace); document keys are stored only wrapped by the
//! keychain-held vault key. What is deliberately readable: document and
//! version IDs, timestamps, byte counts, and content hashes (hashes
//! enable duplicate detection and integrity checks without exposing
//! content).
//!
//! This module is deliberately low-level: identifiers are supplied by the
//! caller (the engine facade mints ULIDs), and every write is one
//! transaction. No audit emission happens here — the facade emits.

use crate::crypto;
use crate::error::VaultError;
use crate::keys::VaultKey;
use forhemit_contracts::{DocumentId, DocumentVersionId, Sha256Hex, VaultId};
use rusqlite::Connection;

/// One immutable version of one document, as returned by the store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredVersion {
    /// The document this version belongs to.
    pub document_id: DocumentId,
    /// This version's identifier.
    pub version_id: DocumentVersionId,
    /// The decrypted filename.
    pub filename: String,
    /// SHA-256 over the plaintext content — recorded per Vault doc §13.
    pub content_hash: Sha256Hex,
    /// Plaintext byte count (readable metadata; content is not).
    pub byte_count: i64,
    /// Owner's note recorded with the version, if any.
    pub note: Option<String>,
    /// When the version was created (nanoseconds since the Unix epoch).
    pub created_at_unix_nanos: i128,
    /// For a restored version: the older version its content came from.
    pub restored_from: Option<DocumentVersionId>,
    /// The version this one replaced at the head of the chain (Vault doc
    /// §47 `supersedes_version`).
    pub previous_version_id: Option<DocumentVersionId>,
}

/// One immutable version of one document with its decrypted content —
/// the full `getDocument` result (Vault doc §47).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredVersionContent {
    /// The version's metadata (filename, hashes, lineage).
    pub version: StoredVersion,
    /// The decrypted document bytes.
    pub content: Vec<u8>,
}

/// A document version that failed integrity verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegrityFailure {
    /// The version that failed.
    pub version_id: DocumentVersionId,
    /// How it failed: a content-hash mismatch, or authenticated
    /// decryption itself refused the row (tampered ciphertext or key).
    pub reason: String,
}

pub(crate) const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS documents (
    document_id           TEXT PRIMARY KEY,
    created_at_unix_nanos INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS document_versions (
    version_id            TEXT PRIMARY KEY,
    document_id           TEXT NOT NULL REFERENCES documents(document_id),
    sequence              INTEGER NOT NULL,
    created_at_unix_nanos INTEGER NOT NULL,
    filename_nonce_b64    TEXT NOT NULL,
    filename_ciphertext   BLOB NOT NULL,
    content_nonce_b64     TEXT NOT NULL,
    content_ciphertext    BLOB NOT NULL,
    wrapped_dek_nonce_b64 TEXT NOT NULL,
    wrapped_dek           BLOB NOT NULL,
    content_hash          TEXT NOT NULL,
    byte_count            INTEGER NOT NULL,
    note_nonce_b64        TEXT,
    note_ciphertext       BLOB,
    restored_from_version_id TEXT,
    supersedes_version_id    TEXT,
    UNIQUE(document_id, sequence)
);
CREATE INDEX IF NOT EXISTS idx_versions_document
    ON document_versions(document_id, sequence);
CREATE TABLE IF NOT EXISTS vault_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS local_analyses (
    analysis_id            TEXT PRIMARY KEY,
    document_id            TEXT NOT NULL REFERENCES documents(document_id),
    document_version_id    TEXT NOT NULL,
    tool                   TEXT NOT NULL,
    summary                TEXT NOT NULL,
    used_external_services INTEGER NOT NULL CHECK (used_external_services = 0),
    created_at_unix_nanos  INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS recovery_escrow (
    vault_id              TEXT PRIMARY KEY,
    salt_hex              TEXT NOT NULL,
    wrapped_nonce_hex     TEXT NOT NULL,
    wrapped_vault_key_hex TEXT NOT NULL
);
";

/// AAD namespaces: every sealed buffer declares what it is and which row
/// it belongs to, so a ciphertext cannot be transplanted between fields,
/// versions, or documents — a moved blob fails authentication instead of
/// decrypting into plausible garbage.
fn content_aad(version_id: &DocumentVersionId) -> Vec<u8> {
    format!("forhemit-vault:content:{version_id}").into_bytes()
}

fn filename_aad(version_id: &DocumentVersionId) -> Vec<u8> {
    format!("forhemit-vault:filename:{version_id}").into_bytes()
}

fn note_aad(version_id: &DocumentVersionId) -> Vec<u8> {
    format!("forhemit-vault:note:{version_id}").into_bytes()
}

fn dek_aad(vault_id: &VaultId, document_id: &DocumentId) -> Vec<u8> {
    format!("forhemit-vault:dek:{vault_id}:{document_id}").into_bytes()
}

/// The persistent vault store.
pub struct VaultStore {
    vault_id: VaultId,
    /// Filesystem path of the database, when file-backed (in-memory
    /// stores are for tests and cannot be backed up).
    path: Option<std::path::PathBuf>,
    conn: Connection,
}

impl VaultStore {
    /// Opens (or creates) the vault database at `path`.
    pub fn open(vault_id: VaultId, path: &std::path::Path) -> Result<Self, VaultError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                VaultError::Internal(format!("create vault directory: {error}"))
            })?;
        }
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(Self {
            vault_id,
            path: Some(path.to_path_buf()),
            conn,
        })
    }

    /// Opens an in-memory vault database — tests and fixtures.
    pub fn open_in_memory(vault_id: VaultId) -> Result<Self, VaultError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(Self {
            vault_id,
            path: None,
            conn,
        })
    }

    /// Opens a vault database without knowing its identity up front —
    /// the restore-from-backup path. The vault id is read from the
    /// recovery-escrow row every created vault carries.
    pub fn open_discovering(path: &std::path::Path) -> Result<Self, VaultError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.execute_batch(SCHEMA_SQL)?;
        let vault_id = {
            let mut statement = conn.prepare("SELECT vault_id FROM recovery_escrow LIMIT 1")?;
            let mut rows = statement.query([])?;
            match rows.next()? {
                Some(row) => {
                    let id: String = row.get(0)?;
                    VaultId::new(id)
                        .map_err(|_| VaultError::Internal("escrow vault id was empty".to_owned()))?
                }
                None => {
                    return Err(VaultError::Backup(
                        "restored database carries no recovery escrow — not a Forhemit vault"
                            .to_owned(),
                    ))
                }
            }
        };
        Ok(Self {
            vault_id,
            path: Some(path.to_path_buf()),
            conn,
        })
    }

    /// The vault this store belongs to.
    pub fn vault_id(&self) -> &VaultId {
        &self.vault_id
    }

    /// Raw database snapshot for the backup layer.
    ///
    /// WAL is checkpointed into the main file first, so the snapshot is a
    /// complete, consistent image without readers' side files.
    pub fn snapshot_bytes(&self) -> Result<Vec<u8>, VaultError> {
        self.conn
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()))?;
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| VaultError::Backup("in-memory vaults cannot be backed up".to_owned()))?;
        std::fs::read(path).map_err(|error| VaultError::Backup(format!("read vault file: {error}")))
    }

    /// Nanoseconds fit `i64` until the year 2262 — SQLite stores them as
    /// INTEGER (the column's declared type), and reads must expect that.
    fn sqlite_nanos(created_at_unix_nanos: i128) -> Result<i64, VaultError> {
        i64::try_from(created_at_unix_nanos)
            .map_err(|_| VaultError::Internal("timestamp out of range for SQLite".to_owned()))
    }

    /// Writes the vault-record rows (Vault doc §47 "Vault": `vault_id`,
    /// workspace, creation time) — exactly once, at vault creation.
    pub fn write_vault_record(
        &self,
        workspace_id: &forhemit_contracts::WorkspaceId,
        created_at_unix_nanos: i128,
    ) -> Result<(), VaultError> {
        self.conn.execute(
            "INSERT INTO vault_meta (key, value) VALUES ('workspace_id', ?1)",
            rusqlite::params![workspace_id.as_str()],
        )?;
        self.conn.execute(
            "INSERT INTO vault_meta (key, value) VALUES ('created_at_unix_nanos', ?1)",
            rusqlite::params![created_at_unix_nanos.to_string()],
        )?;
        Ok(())
    }

    /// The workspace the vault belongs to, if recorded.
    pub fn read_workspace_id(&self) -> Result<Option<forhemit_contracts::WorkspaceId>, VaultError> {
        let value: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM vault_meta WHERE key = 'workspace_id'",
                [],
                |row| row.get(0),
            )
            .map(Some)
            .or_else(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(other),
            })?;
        value
            .map(|id| forhemit_contracts::WorkspaceId::new(&id).map_err(VaultError::from))
            .transpose()
    }

    /// Records the recovery escrow row (written exactly once, at vault
    /// creation; rewriting it would silently re-bind recovery).
    pub fn write_escrow(&self, escrow: &crate::recovery::RecoveryEscrow) -> Result<(), VaultError> {
        self.conn.execute(
            "INSERT OR IGNORE INTO recovery_escrow
                (vault_id, salt_hex, wrapped_nonce_hex, wrapped_vault_key_hex)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                self.vault_id.as_str(),
                escrow.salt_hex,
                escrow.wrapped_nonce_hex,
                escrow.wrapped_vault_key_hex,
            ],
        )?;
        Ok(())
    }

    /// Loads the recovery escrow, if the vault has one.
    pub fn read_escrow(&self) -> Result<Option<crate::recovery::RecoveryEscrow>, VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT salt_hex, wrapped_nonce_hex, wrapped_vault_key_hex
             FROM recovery_escrow WHERE vault_id = ?1",
        )?;
        let mut rows = statement.query(rusqlite::params![self.vault_id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(crate::recovery::RecoveryEscrow {
                salt_hex: row.get(0)?,
                wrapped_nonce_hex: row.get(1)?,
                wrapped_vault_key_hex: row.get(2)?,
            }));
        }
        Ok(None)
    }

    /// Inserts one document version — the single write path behind
    /// import, new versions, and restores.
    ///
    /// Seals the filename and the content under a fresh document key,
    /// wraps the key under the vault key, and stores the plaintext hash
    /// (Vault doc §13) alongside the ciphertext. Everything happens in
    /// one transaction: a version row never appears half-written.
    #[allow(clippy::too_many_arguments)] // one write path, all fields required
    pub fn insert_version(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        sequence: i64,
        filename: &str,
        plaintext: &[u8],
        note: Option<&str>,
        created_at_unix_nanos: i128,
        restored_from: Option<&DocumentVersionId>,
        supersedes: Option<&DocumentVersionId>,
    ) -> Result<Sha256Hex, VaultError> {
        if filename.is_empty() {
            return Err(VaultError::InvalidInput(
                "filename must not be empty".into(),
            ));
        }
        let content_hash = crypto::sha256_hex(plaintext);
        let transaction = self.conn.unchecked_transaction()?;
        transaction.execute(
            "INSERT OR IGNORE INTO documents (document_id, created_at_unix_nanos) VALUES (?1, ?2)",
            rusqlite::params![
                document_id.as_str(),
                Self::sqlite_nanos(created_at_unix_nanos)?
            ],
        )?;
        // Fresh DEK for this version, wrapped immediately; the raw DEK
        // never leaves this scope unwrapped, and each version can be
        // re-keyed independently of the vault key.
        let dek = crypto::generate_key();
        let dek_nonce = crypto::random_nonce();
        let wrapped_dek = crypto::seal(
            vault_key.as_bytes(),
            &dek_nonce,
            &dek_aad(&self.vault_id, document_id),
            &dek,
            "wrap document key",
        )?;
        let content_nonce = crypto::random_nonce();
        let content_ciphertext = crypto::seal(
            &dek,
            &content_nonce,
            &content_aad(version_id),
            plaintext,
            "seal document content",
        )?;
        let filename_nonce = crypto::random_nonce();
        let filename_ciphertext = crypto::seal(
            &dek,
            &filename_nonce,
            &filename_aad(version_id),
            filename.as_bytes(),
            "seal document filename",
        )?;
        // The note is owner-authored context ("password sent
        // separately") — sealed exactly like the filename: version DEK,
        // its own nonce and AAD namespace.
        let (note_nonce_b64, note_ciphertext) = match note {
            Some(note_text) => {
                let note_nonce = crypto::random_nonce();
                let note_ciphertext = crypto::seal(
                    &dek,
                    &note_nonce,
                    &note_aad(version_id),
                    note_text.as_bytes(),
                    "seal document note",
                )?;
                (Some(crypto::encode_b64(&note_nonce)), Some(note_ciphertext))
            }
            None => (None, None),
        };
        transaction.execute(
            "INSERT INTO document_versions (
                version_id, document_id, sequence, created_at_unix_nanos,
                filename_nonce_b64, filename_ciphertext,
                content_nonce_b64, content_ciphertext,
                wrapped_dek_nonce_b64, wrapped_dek,
                content_hash, byte_count, note_nonce_b64, note_ciphertext,
                restored_from_version_id, supersedes_version_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            rusqlite::params![
                version_id.as_str(),
                document_id.as_str(),
                sequence,
                Self::sqlite_nanos(created_at_unix_nanos)?,
                crypto::encode_b64(&filename_nonce),
                filename_ciphertext,
                crypto::encode_b64(&content_nonce),
                content_ciphertext,
                crypto::encode_b64(&dek_nonce),
                wrapped_dek,
                content_hash.as_str(),
                plaintext.len() as i64,
                note_nonce_b64,
                note_ciphertext,
                restored_from.map(|id| id.as_str()),
                supersedes.map(|id| id.as_str()),
            ],
        )?;
        transaction.commit()?;
        Ok(content_hash)
    }

    /// Reads and decrypts one version: verifies the AEAD tag (key + AAD
    /// binding) and then the stored content hash, returning the metadata
    /// and plaintext bytes.
    pub fn read_version_with_content(
        &self,
        vault_key: &VaultKey,
        version_id: &DocumentVersionId,
    ) -> Result<StoredVersionContent, VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT document_id, sequence, created_at_unix_nanos,
                    filename_nonce_b64, filename_ciphertext,
                    content_nonce_b64, content_ciphertext,
                    wrapped_dek_nonce_b64, wrapped_dek,
                    content_hash, byte_count, note_nonce_b64, note_ciphertext,
                    restored_from_version_id, supersedes_version_id
             FROM document_versions WHERE version_id = ?1",
        )?;
        let mut rows = statement.query(rusqlite::params![version_id.as_str()])?;
        let Some(row) = rows.next()? else {
            return Err(VaultError::VersionNotFound {
                version_id: version_id.as_str().to_owned(),
            });
        };
        let document_id = DocumentId::new(row.get::<_, String>(0)?)
            .map_err(|_| VaultError::Internal("stored document_id was empty".to_owned()))?;
        let filename = self.decrypt_filename(vault_key, &document_id, version_id, row)?;
        let plaintext = self.decrypt_content(vault_key, &document_id, version_id, row)?;
        Ok(StoredVersionContent {
            version: StoredVersion {
                document_id: document_id.clone(),
                version_id: version_id.clone(),
                filename,
                content_hash: crypto::sha256_hex(&plaintext),
                byte_count: row.get(10)?,
                note: self.decrypt_note(vault_key, &document_id, version_id, row)?,
                created_at_unix_nanos: i128::from(row.get::<_, i64>(2)?),
                restored_from: row
                    .get::<_, Option<String>>(13)?
                    .map(|id| DocumentVersionId::new(&id))
                    .transpose()?,
                previous_version_id: row
                    .get::<_, Option<String>>(14)?
                    .map(|id| DocumentVersionId::new(&id))
                    .transpose()?,
            },
            content: plaintext,
        })
    }

    /// Reads one version's metadata — content is decrypted (read access
    /// verifies integrity) but not returned.
    pub fn read_version(
        &self,
        vault_key: &VaultKey,
        version_id: &DocumentVersionId,
    ) -> Result<StoredVersion, VaultError> {
        self.read_version_with_content(vault_key, version_id)
            .map(|full| full.version)
    }

    /// The latest version of a document, with its filename decrypted.
    pub fn latest_version(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
    ) -> Result<Option<StoredVersion>, VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT version_id FROM document_versions
             WHERE document_id = ?1 ORDER BY sequence DESC LIMIT 1",
        )?;
        let mut rows = statement.query(rusqlite::params![document_id.as_str()])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let version_id = DocumentVersionId::new(row.get::<_, String>(0)?)
            .map_err(|_| VaultError::Internal("stored version_id was empty".to_owned()))?;
        self.read_version(vault_key, &version_id).map(Some)
    }

    /// The latest version of a document with its decrypted content —
    /// the `getDocument` path (Vault doc §47).
    pub fn latest_version_with_content(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
    ) -> Result<Option<StoredVersionContent>, VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT version_id FROM document_versions
             WHERE document_id = ?1 ORDER BY sequence DESC LIMIT 1",
        )?;
        let mut rows = statement.query(rusqlite::params![document_id.as_str()])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let version_id = DocumentVersionId::new(row.get::<_, String>(0)?)
            .map_err(|_| VaultError::Internal("stored version_id was empty".to_owned()))?;
        self.read_version_with_content(vault_key, &version_id)
            .map(Some)
    }

    /// Full version history of a document, oldest first, filenames
    /// decrypted. Every historical version stays readable — "version
    /// history retained for audit and restoration" (Vault doc §53.2).
    pub fn version_history(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
    ) -> Result<Vec<StoredVersion>, VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT version_id FROM document_versions
             WHERE document_id = ?1 ORDER BY sequence ASC",
        )?;
        let ids = statement
            .query_map(rusqlite::params![document_id.as_str()], |row| {
                row.get::<_, String>(0)
            })?
            .collect::<Result<Vec<_>, _>>()?;
        ids.iter()
            .map(|id| {
                let version_id = DocumentVersionId::new(id)
                    .map_err(|_| VaultError::Internal("stored version_id was empty".to_owned()))?;
                self.read_version(vault_key, &version_id)
            })
            .collect()
    }

    /// All documents — the list view (metadata only; no decryption).
    pub fn list_documents(&self) -> Result<Vec<DocumentId>, VaultError> {
        let mut statement = self
            .conn
            .prepare("SELECT document_id FROM documents ORDER BY created_at_unix_nanos, rowid")?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|id| {
                DocumentId::new(&id)
                    .map_err(|_| VaultError::Internal("stored document_id was empty".to_owned()))
            })
            .collect()
    }

    /// Highest sequence number a document has reached (0 = no versions).
    pub fn next_sequence(&self, document_id: &DocumentId) -> Result<i64, VaultError> {
        let sequence: Option<i64> = self.conn.query_row(
            "SELECT MAX(sequence) FROM document_versions WHERE document_id = ?1",
            rusqlite::params![document_id.as_str()],
            |row| row.get(0),
        )?;
        Ok(sequence.unwrap_or(0) + 1)
    }

    /// Integrity sweep: decrypt every version and recompute its hash.
    /// Returns one entry per failure — hash mismatch or authenticated
    /// decryption refusal — and an empty vec means the vault is intact.
    pub fn verify_all_hashes(
        &self,
        vault_key: &VaultKey,
    ) -> Result<Vec<IntegrityFailure>, VaultError> {
        let mut failures = Vec::new();
        let mut statement = self.conn.prepare(
            "SELECT version_id, document_id, filename_nonce_b64, filename_ciphertext,
                    content_nonce_b64, content_ciphertext, wrapped_dek_nonce_b64,
                    wrapped_dek, content_hash
             FROM document_versions ORDER BY document_id, sequence",
        )?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let version_id = DocumentVersionId::new(row.get::<_, String>(0)?)
                .map_err(|_| VaultError::Internal("stored version_id was empty".to_owned()))?;
            let document_id = DocumentId::new(row.get::<_, String>(1)?)
                .map_err(|_| VaultError::Internal("stored document_id was empty".to_owned()))?;
            let stored_hash: String = row.get(8)?;
            match self.decrypt_content(vault_key, &document_id, &version_id, row) {
                Ok(plaintext) => {
                    let computed = crypto::sha256_hex(&plaintext);
                    if computed.as_str() != stored_hash {
                        failures.push(IntegrityFailure {
                            version_id,
                            reason: format!(
                                "content hash mismatch: stored {stored_hash}, computed {}",
                                computed.as_str()
                            ),
                        });
                    }
                }
                Err(VaultError::Crypto { .. }) => {
                    failures.push(IntegrityFailure {
                        version_id,
                        reason: "authenticated decryption failed — ciphertext or wrapped key was modified"
                            .to_owned(),
                    });
                }
                Err(other) => return Err(other),
            }
        }
        Ok(failures)
    }

    /// Streams every version's decrypted filename + content through the
    /// caller's closure — the search-index rebuild path (Vault doc §50:
    /// "The Vault can rebuild it from the underlying encrypted data").
    pub fn for_each_plaintext(
        &self,
        vault_key: &VaultKey,
        mut visit: impl FnMut(&DocumentId, &DocumentVersionId, &str, &[u8]) -> Result<(), VaultError>,
    ) -> Result<(), VaultError> {
        let mut statement = self.conn.prepare(
            "SELECT version_id, document_id, filename_nonce_b64, filename_ciphertext,
                    content_nonce_b64, content_ciphertext, wrapped_dek_nonce_b64,
                    wrapped_dek
             FROM document_versions ORDER BY document_id, sequence",
        )?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let version_id = DocumentVersionId::new(row.get::<_, String>(0)?)
                .map_err(|_| VaultError::Internal("stored version_id was empty".to_owned()))?;
            let document_id = DocumentId::new(row.get::<_, String>(1)?)
                .map_err(|_| VaultError::Internal("stored document_id was empty".to_owned()))?;
            let filename = self.decrypt_filename(vault_key, &document_id, &version_id, row)?;
            let plaintext = self.decrypt_content(vault_key, &document_id, &version_id, row)?;
            visit(&document_id, &version_id, &filename, &plaintext)?;
        }
        Ok(())
    }

    /// Records a local-analysis run (Vault doc §47 "LocalAnalysis":
    /// analysis_id, inputs, model/tool; §45 "Document analyzed"). The
    /// `used_external_services` flag is schema-constrained to 0 — v1 is
    /// local-only, and the check makes a future networked analysis a
    /// schema change, not a silent default.
    pub fn record_analysis(
        &self,
        analysis_id: &forhemit_contracts::AnalysisId,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        tool: &str,
        summary: &str,
        created_at_unix_nanos: i128,
    ) -> Result<(), VaultError> {
        self.conn.execute(
            "INSERT INTO local_analyses (
                analysis_id, document_id, document_version_id, tool, summary,
                used_external_services, created_at_unix_nanos
            ) VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6)",
            rusqlite::params![
                analysis_id.as_str(),
                document_id.as_str(),
                version_id.as_str(),
                tool,
                summary,
                Self::sqlite_nanos(created_at_unix_nanos)?,
            ],
        )?;
        Ok(())
    }

    fn unwrap_dek(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
        row: &rusqlite::Row<'_>,
    ) -> Result<[u8; 32], VaultError> {
        let dek_nonce = crypto::decode_b64(&row.get::<_, String>("wrapped_dek_nonce_b64")?)?;
        let wrapped_dek: Vec<u8> = row.get("wrapped_dek")?;
        let mut nonce = [0u8; crypto::NONCE_LEN];
        nonce.copy_from_slice(&dek_nonce);
        let dek = crypto::open(
            vault_key.as_bytes(),
            &nonce,
            &dek_aad(&self.vault_id, document_id),
            &wrapped_dek,
            "unwrap document key",
        )?;
        let dek: [u8; 32] = dek.as_slice().try_into().map_err(|_| {
            VaultError::Internal("wrapped document key was not 32 bytes".to_owned())
        })?;
        Ok(dek)
    }

    fn decrypt_content(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        row: &rusqlite::Row<'_>,
    ) -> Result<Vec<u8>, VaultError> {
        let dek = self.unwrap_dek(vault_key, document_id, row)?;
        let content_nonce = crypto::decode_b64(&row.get::<_, String>("content_nonce_b64")?)?;
        let content_ciphertext: Vec<u8> = row.get("content_ciphertext")?;
        let mut nonce = [0u8; crypto::NONCE_LEN];
        nonce.copy_from_slice(&content_nonce);
        let plaintext = crypto::open(
            &dek,
            &nonce,
            &content_aad(version_id),
            &content_ciphertext,
            "open document content",
        )?;
        // Enforce the stored hash after authenticated decryption: a row
        // corrupted outside the crypto layer still fails loudly here.
        let stored_hash: String = row.get("content_hash")?;
        let computed = crypto::sha256_hex(&plaintext);
        if computed.as_str() != stored_hash {
            return Err(VaultError::HashMismatch {
                version_id: version_id.as_str().to_owned(),
            });
        }
        Ok(plaintext.to_vec())
    }

    fn decrypt_filename(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        row: &rusqlite::Row<'_>,
    ) -> Result<String, VaultError> {
        let dek = self.unwrap_dek(vault_key, document_id, row)?;
        let filename_nonce = crypto::decode_b64(&row.get::<_, String>("filename_nonce_b64")?)?;
        let filename_ciphertext: Vec<u8> = row.get("filename_ciphertext")?;
        let mut nonce = [0u8; crypto::NONCE_LEN];
        nonce.copy_from_slice(&filename_nonce);
        let filename = crypto::open(
            &dek,
            &nonce,
            &filename_aad(version_id),
            &filename_ciphertext,
            "open document filename",
        )?;
        String::from_utf8(filename.to_vec())
            .map_err(|_| VaultError::Internal("decrypted filename was not UTF-8".to_owned()))
    }

    /// Decrypts the version's note, if it carries one — sealed exactly
    /// like the filename (same DEK, note-specific AAD). A nonce without
    /// ciphertext or the reverse is an internal inconsistency, surfaced
    /// rather than silently read as "no note".
    fn decrypt_note(
        &self,
        vault_key: &VaultKey,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        row: &rusqlite::Row<'_>,
    ) -> Result<Option<String>, VaultError> {
        let note_nonce_b64: Option<String> = row.get("note_nonce_b64")?;
        let note_ciphertext: Option<Vec<u8>> = row.get("note_ciphertext")?;
        let (nonce_b64, ciphertext) = match (note_nonce_b64, note_ciphertext) {
            (Some(nonce_b64), Some(ciphertext)) => (nonce_b64, ciphertext),
            (None, None) => return Ok(None),
            (None, Some(_)) | (Some(_), None) => {
                return Err(VaultError::Internal(
                    "note nonce and ciphertext must be stored together".to_owned(),
                ))
            }
        };
        let dek = self.unwrap_dek(vault_key, document_id, row)?;
        let mut nonce = [0u8; crypto::NONCE_LEN];
        nonce.copy_from_slice(&crypto::decode_b64(&nonce_b64)?);
        let plaintext = crypto::open(
            &dek,
            &nonce,
            &note_aad(version_id),
            &ciphertext,
            "open document note",
        )?;
        String::from_utf8(plaintext.to_vec())
            .map(Some)
            .map_err(|_| VaultError::Internal("decrypted note was not UTF-8".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    /// A file-backed store — the note-sealing assertions read the raw
    /// database image, which in-memory stores do not have.
    fn file_store(name: &str) -> (VaultStore, std::path::PathBuf) {
        let dir = std::env::temp_dir().join(format!("forhemit-vault-notes-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        let _ = std::fs::remove_file(&path); // fresh database per test
        let store = VaultStore::open(VaultId::new("vault-note-test").unwrap(), &path).unwrap();
        (store, path)
    }

    fn vault_key() -> VaultKey {
        VaultKey::from_bytes(crypto::generate_key())
    }

    #[test]
    fn note_round_trips_and_is_sealed_at_rest() {
        let (store, path) = file_store("note-sealed.sqlite3");
        let key = vault_key();
        let document_id = DocumentId::new("doc-note-1").unwrap();
        let version_id = DocumentVersionId::new("ver-note-1").unwrap();
        let note = "password sent separately";
        store
            .insert_version(
                &key,
                &document_id,
                &version_id,
                1,
                "board-pack.pdf",
                b"document body",
                Some(note),
                0,
                None,
                None,
            )
            .unwrap();

        // The round trip returns the note.
        let stored = store.read_version_with_content(&key, &version_id).unwrap();
        assert_eq!(stored.version.note.as_deref(), Some(note));

        // Ciphertext at rest: the plaintext note appears nowhere in the
        // database image (the same noncontainment check the backup tests
        // use) — including its most sensitive token on its own.
        let on_disk = store.snapshot_bytes().unwrap();
        assert!(!on_disk.windows(note.len()).any(|w| w == note.as_bytes()));
        assert!(!on_disk.windows(10).any(|w| w == b"separately"));
        // And the filename is absent too — the note now matches the
        // filename's sealing pattern exactly.
        assert!(!on_disk
            .windows("board-pack.pdf".len())
            .any(|w| w == b"board-pack.pdf"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn versions_without_a_note_round_trip_as_none() {
        let (store, path) = file_store("note-none.sqlite3");
        let key = vault_key();
        let version_id = DocumentVersionId::new("ver-note-2").unwrap();
        store
            .insert_version(
                &key,
                &DocumentId::new("doc-note-2").unwrap(),
                &version_id,
                1,
                "silent.txt",
                b"body",
                None,
                0,
                None,
                None,
            )
            .unwrap();
        let stored = store.read_version_with_content(&key, &version_id).unwrap();
        assert_eq!(stored.version.note, None);
        let _ = std::fs::remove_file(&path);
    }
}
