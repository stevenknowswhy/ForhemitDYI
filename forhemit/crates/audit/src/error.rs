//! Error surface of the audit store.
//!
//! Tamper evidence is a named error, never a swallowed failure: chain
//! verification reports exactly which row failed which check (Audit doc
//! §16 — "The system must be able to detect tampering with historical
//! audit records").

use forhemit_contracts::EventId;
use std::fmt;

/// Why chain verification flagged a row (or the database's guards).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TamperKind {
    /// The row's content columns no longer hash to its stored event hash.
    ContentHashMismatch {
        /// The event hash stored in the row.
        stored: String,
        /// The hash recomputed from the row's content.
        recomputed: String,
    },
    /// The row's stored `previous_event_hash` does not match the hash of
    /// the event before it (or the genesis digest for the first row) —
    /// the signature of a deleted row or a forged append.
    ChainLinkMismatch {
        /// The previous-event hash stored in the row.
        stored_previous: String,
        /// The hash the chain expects at this position.
        expected_previous: String,
    },
    /// A payload's bytes no longer hash to the digest its event references.
    PayloadDigestMismatch {
        /// The digest the event references.
        digest: String,
        /// The digest recomputed from the stored payload bytes.
        recomputed: String,
    },
    /// An event references a payload that is no longer present.
    MissingPayload {
        /// The digest that no longer resolves.
        digest: String,
    },
    /// An append-only guard trigger was removed from the database — the
    /// storage-layer enforcement of "nothing ever updates or deletes a
    /// stored row" is gone.
    GuardTriggerMissing {
        /// The name of the missing trigger.
        name: String,
    },
}

impl fmt::Display for TamperKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentHashMismatch { stored, recomputed } => write!(
                f,
                "event content hashes to {recomputed} but the row stores {stored}"
            ),
            Self::ChainLinkMismatch {
                stored_previous,
                expected_previous,
            } => write!(
                f,
                "row's previous_event_hash is {stored_previous} but the chain expects {expected_previous}"
            ),
            Self::PayloadDigestMismatch { digest, recomputed } => write!(
                f,
                "payload referenced by {digest} now hashes to {recomputed}"
            ),
            Self::MissingPayload { digest } => {
                write!(f, "payload {digest} is referenced but missing")
            }
            Self::GuardTriggerMissing { name } => {
                write!(f, "append-only guard trigger {name} was removed")
            }
        }
    }
}

/// Where and what chain verification found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TamperEvidence {
    /// Sequence of the offending row — 0 when the finding is not
    /// row-scoped (a removed guard trigger).
    pub sequence: i64,
    /// Event id of the offending row, when row-scoped and parseable.
    pub event_id: Option<EventId>,
    /// What failed.
    pub kind: TamperKind,
}

impl fmt::Display for TamperEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.event_id {
            Some(event_id) => write!(f, "seq {} ({}): {}", self.sequence, event_id, self.kind),
            None => write!(f, "seq {}: {}", self.sequence, self.kind),
        }
    }
}

/// Errors the audit store reports.
#[derive(Debug)]
pub enum AuditStoreError {
    /// The underlying SQLite layer failed.
    Sqlite(rusqlite::Error),
    /// An event payload or actor record could not be serialized.
    Serialization(serde_json::Error),
    /// A timestamp fell outside the storable range (nanoseconds since the
    /// Unix epoch must fit a 64-bit integer — safe until the year 2262).
    Time(String),
    /// The in-process write lock was poisoned by a panic in another writer.
    LockPoisoned,
    /// A stored value the store itself wrote failed to deserialize — a
    /// database written by a newer app version, or tampering. Surfaced,
    /// never silently ignored.
    UnsupportedStoredEvent(String),
    /// An append-correction named an original event that does not exist.
    CorrectionTargetNotFound {
        /// The event id that was named as the correction target.
        event_id: EventId,
    },
    /// Chain verification found the log's history altered (Audit doc §16).
    TamperDetected(TamperEvidence),
    /// An internal invariant failed — unreachable absent a bug; surfaced
    /// rather than swallowed.
    Internal(String),
}

impl fmt::Display for AuditStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "audit store: {error}"),
            Self::Serialization(error) => write!(f, "audit store: serialization: {error}"),
            Self::Time(details) => write!(f, "audit store: timestamp: {details}"),
            Self::LockPoisoned => write!(f, "audit store: write lock poisoned"),
            Self::UnsupportedStoredEvent(details) => {
                write!(f, "audit store: stored value out of contract: {details}")
            }
            Self::CorrectionTargetNotFound { event_id } => {
                write!(f, "audit store: correction target {event_id} not found")
            }
            Self::TamperDetected(evidence) => write!(f, "audit store: tamper detected: {evidence}"),
            Self::Internal(details) => write!(f, "audit store: internal error: {details}"),
        }
    }
}

impl std::error::Error for AuditStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for AuditStoreError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl From<serde_json::Error> for AuditStoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
