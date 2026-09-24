//! Error surface of the Business Reality engine.
//!
//! Every variant names the condition it reports; nothing is swallowed.
//! The engine is audit-first: [`RealityError::Audit`] means the fact was
//! **not** recorded — a fact version never exists without its audit event.

use forhemit_contracts::FactId;
use forhemit_enginekit::AuditError;

use crate::fact::FactKind;

/// What can go wrong while recording or revising a Business Snapshot fact.
#[derive(Debug)]
pub enum RealityError {
    /// The audit store rejected the event. The mutation did not happen —
    /// no fact version is created without its audit event.
    Audit(AuditError),
    /// The fact's value shape does not fit its kind (Business Reality
    /// doc §31: a field carries a value *and* a unit — a revenue fact is
    /// not an employee count).
    KindValueMismatch {
        /// The kind the fact was filed under.
        kind: FactKind,
        /// The shape of the supplied value ("text", "number", "range").
        actual: &'static str,
        /// The shapes the kind accepts.
        accepted: &'static str,
    },
    /// A range value is malformed — missing both bounds, or a lower bound
    /// above the upper bound.
    InvalidFactRange {
        /// What is wrong with the range.
        reason: &'static str,
    },
    /// A text fact was left empty or blank; absence is expressed by not
    /// recording a fact, never by a blank string.
    EmptyTextValue,
    /// A definition label was supplied empty (Business Reality doc §12 —
    /// preserve the label used by the source, or none at all).
    EmptyDefinition,
    /// A revision was filed without a reason; "Why did it change?" is the
    /// first question the owner-facing history must answer.
    EmptyChangeReason,
    /// No fact exists under the given stable id.
    FactNotFound {
        /// The unknown fact id.
        fact_id: FactId,
    },
    /// A second fact of a kind the Business Snapshot records once (Business
    /// Reality doc §8 — the snapshot has one revenue, one industry, …).
    /// Revise the existing fact instead of recording a duplicate.
    FactKindAlreadyRecorded {
        /// The kind that already has a current fact.
        kind: FactKind,
        /// The fact id of the existing current fact.
        fact_id: FactId,
    },
    /// Serializing an event payload failed.
    Serialization(serde_json::Error),
    /// A condition the type system rules out reached the engine anyway —
    /// a bug, reported rather than papered over.
    Internal(String),
}

impl std::fmt::Display for RealityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audit(error) => f.write_str(&format!("audit store rejected the event: {error}")),
            Self::KindValueMismatch {
                kind,
                actual,
                accepted,
            } => f.write_str(&format!(
                "fact kind {kind:?} accepts {accepted} values, got {actual}"
            )),
            Self::InvalidFactRange { reason } => {
                f.write_str(&format!("invalid fact range: {reason}"))
            }
            Self::EmptyTextValue => f.write_str("text fact value must not be empty"),
            Self::EmptyDefinition => f.write_str("fact definition must not be empty when given"),
            Self::EmptyChangeReason => f.write_str("a fact revision requires a change reason"),
            Self::FactNotFound { fact_id } => f.write_str(&format!("no fact with id {fact_id}")),
            Self::FactKindAlreadyRecorded { kind, fact_id } => f.write_str(&format!(
                "a current fact of kind {kind:?} already exists (fact {fact_id}); revise it instead"
            )),
            Self::Serialization(error) => {
                f.write_str(&format!("event payload serialization failed: {error}"))
            }
            Self::Internal(message) => f.write_str(&format!("internal error: {message}")),
        }
    }
}

impl std::error::Error for RealityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Audit(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}
