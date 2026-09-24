//! Error surface of the vault engine.
//!
//! Failure states are first-class (implementation spec: "vault-locked
//! state (keychain unavailable) blocks document access without data
//! loss") — every failure mode is a named variant, surfaced loudly, never
//! silently repaired.

use std::fmt;

/// Errors the vault engine reports.
#[derive(Debug)]
pub enum VaultError {
    /// The underlying SQLite layer failed.
    Sqlite(rusqlite::Error),
    /// A vault record or backup envelope could not be serialized.
    Serialization(serde_json::Error),
    /// Authenticated encryption or decryption failed. For decryption this
    /// means the key is wrong, the ciphertext is corrupted, or the
    /// additional-authenticated-data binding does not match (the
    /// signature of a ciphertext moved between rows) — surfaced as
    /// evidence, never repaired silently.
    Crypto {
        /// What the failed operation was doing (e.g. "encrypt document
        /// version", "unwrap document key").
        operation: &'static str,
    },
    /// The OS key store failed (keyring crate error message included).
    KeyStore(String),
    /// No vault key is in the OS key store — the vault-locked state
    /// (Vault doc §50). The vault stays locked without data loss; the
    /// recovery path (`recover_vault_key`) unlocks it.
    VaultKeyUnavailable,
    /// Recovery failed: the passphrase does not derive the key that
    /// opens the escrowed vault-key wrapping (Vault doc §36 — no
    /// backdoor, only the passphrase works).
    RecoveryFailed,
    /// A vault already exists at this database path.
    VaultAlreadyExists,
    /// No vault exists at this database path.
    VaultNotFound,
    /// Caller-supplied input violated a documented invariant (empty
    /// filename, oversized payload, …).
    InvalidInput(String),
    /// A version's stored hash does not match its decrypted content —
    /// surfaced as evidence, never repaired silently (the audit-store
    /// standard applies: loud verification failure).
    HashMismatch {
        /// The version whose content no longer matches its hash.
        version_id: String,
    },
    /// The named document does not exist.
    DocumentNotFound {
        /// The unknown document id.
        document_id: String,
    },
    /// The named version does not exist in the vault.
    VersionNotFound {
        /// The unknown version id.
        version_id: String,
    },
    /// A backup envelope's format version is not understood — the backup
    /// came from a newer app, or the bytes are not a vault backup.
    UnsupportedBackupFormat {
        /// The format version the envelope declared.
        found: u32,
    },
    /// A timestamp fell outside the storable range (nanoseconds since
    /// the Unix epoch must fit a 64-bit integer).
    Time(String),
    /// Reading or writing the backup stream failed.
    Backup(String),
    /// The audit sink refused an event — engine actions fail loudly when
    /// their audit record cannot be written ("audit infrastructure
    /// early", Implementation Roadmap Phase 1).
    Audit(String),
    /// An internal invariant failed — unreachable absent a bug; surfaced
    /// rather than swallowed.
    Internal(String),
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "vault: {error}"),
            Self::Serialization(error) => write!(f, "vault: serialization: {error}"),
            Self::Crypto { operation } => {
                write!(f, "vault: authenticated encryption failed during {operation}")
            }
            Self::KeyStore(details) => write!(f, "vault: OS key store: {details}"),
            Self::VaultKeyUnavailable => write!(
                f,
                "vault: no vault key in the OS key store — vault locked; use the recovery path"
            ),
            Self::RecoveryFailed => {
                write!(f, "vault: recovery passphrase did not open the escrowed vault key")
            }
            Self::VaultAlreadyExists => write!(f, "vault: a vault already exists at this path"),
            Self::VaultNotFound => write!(f, "vault: no vault at this path"),
            Self::InvalidInput(details) => write!(f, "vault: invalid input: {details}"),
            Self::HashMismatch { version_id } => write!(
                f,
                "vault: version {version_id} failed its content-hash check — the record was modified outside the vault"
            ),
            Self::DocumentNotFound { document_id } => {
                write!(f, "vault: document {document_id} not found")
            }
            Self::VersionNotFound { version_id } => {
                write!(f, "vault: version {version_id} not found")
            }
            Self::UnsupportedBackupFormat { found } => {
                write!(f, "vault: backup format version {found} is not understood")
            }
            Self::Time(details) => write!(f, "vault: timestamp: {details}"),
            Self::Backup(details) => write!(f, "vault: backup: {details}"),
            Self::Audit(details) => write!(f, "vault: audit emission failed: {details}"),
            Self::Internal(details) => write!(f, "vault: internal error: {details}"),
        }
    }
}

impl std::error::Error for VaultError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for VaultError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl From<forhemit_contracts::EmptyIdError> for VaultError {
    fn from(_: forhemit_contracts::EmptyIdError) -> Self {
        Self::InvalidInput("identifier must be a non-empty string".to_owned())
    }
}

impl From<serde_json::Error> for VaultError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
