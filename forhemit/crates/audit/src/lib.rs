//! # forhemit-audit
//!
//! The append-only, hash-chained audit event store — Implementation
//! Roadmap Phase 1 ("Build the immutable event model first… Establish the
//! append-only storage pattern and basic integrity verification").
//!
//! Every material change in the platform flows through here: engines emit
//! [`AuditEventDraft`]s, the store derives each event's identity
//! (`event_id` ULID), timestamp (injected clock), payload digest, and
//! chain link, and nothing — no API path, and no trigger bypass that
//! escapes detection — rewrites or removes a stored row. The store
//! implements enginekit's [`AuditSink`] over the draft type, so engine
//! crates written against that boundary plug into this store.
//!
//! ## What verification guarantees
//!
//! [`AuditStore::verify_chain`] recomputes every row's content hash and
//! payload digest and walks the `previous_event_hash` links (Audit doc
//! §16). It detects edited event content, edited stored hashes, altered
//! payload bytes, missing payloads, forged appends, and row deletions
//! that break a link. Deleting the most recent row is the one alteration
//! outside a pure hash chain's reach — external anchoring (signed
//! checkpoints) is the Audit doc's future mechanism and stays out of v1.

mod error;
mod store;

pub use error::{AuditStoreError, TamperEvidence, TamperKind};
pub use store::{AuditStore, ChainStatus, StoredAuditEvent, GENESIS_PREVIOUS_HASH_HEX};

use forhemit_contracts::{CorrelationId, EventId};
use ulid::Ulid;

/// Generates a fresh event identifier — a ULID. Contracts note: "ULID
/// generation for EventId/CorrelationId lands with the audit store task."
pub fn new_event_id() -> EventId {
    EventId::new(Ulid::new().to_string())
        .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
}

/// Generates a fresh correlation identifier — a ULID grouping related
/// activity (Audit doc §18: "Show me everything that happened because of
/// this change").
pub fn new_correlation_id() -> CorrelationId {
    CorrelationId::new(Ulid::new().to_string())
        .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
}
