//! Error surface of the destination engine.
//!
//! The engine never silently repairs owner input or quietly reshapes
//! history (Destination Engine doc §40 and §46 rule 2): every refusal is a
//! named, structured error the caller can surface verbatim.

use forhemit_contracts::{DestinationId, DestinationVersionId};
use std::fmt;

/// Why a destination operation was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DestinationError {
    /// No destination with this identifier is known to the engine.
    UnknownDestination(DestinationId),
    /// No version with this identifier exists in the destination's history.
    UnknownVersion {
        /// The destination that was asked about.
        destination: DestinationId,
        /// The version that could not be found.
        version: DestinationVersionId,
    },
    /// The requested status transition is not a legal process step
    /// (Destination Engine doc §6: statuses "describe process state, not
    /// feasibility" — and archived destinations are "no longer active").
    IllegalTransition {
        /// The destination's current status.
        from: String,
        /// The requested status.
        to: String,
    },
    /// An ownership allocation does not total 100% (Destination Engine doc
    /// §40: the system says "Your allocation currently totals 110%" — it
    /// does not fix the percentages automatically).
    AllocationDoesNotTotal100 {
        /// The total the owner's shares currently sum to.
        total: u32,
    },
    /// An update changed nothing — "every material change creates a new
    /// version" (Destination Engine doc §26), so a non-material change
    /// creates no version and no event.
    NoMaterialChange,
    /// The audit store refused the event for a material change. The
    /// mutation is refused with it: nothing changes without its audit
    /// record (Implementation Roadmap Phase 1 — audit first).
    AuditRefused(String),
}

impl fmt::Display for DestinationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownDestination(id) => {
                write!(f, "unknown destination {id}")
            }
            Self::UnknownVersion {
                destination,
                version,
            } => write!(f, "destination {destination} has no version {version}"),
            Self::IllegalTransition { from, to } => {
                write!(f, "illegal destination status transition {from} → {to}")
            }
            Self::AllocationDoesNotTotal100 { total } => {
                write!(
                    f,
                    "your allocation currently totals {total}% — it must total 100%"
                )
            }
            Self::NoMaterialChange => {
                write!(f, "the edit changed nothing; no new version was created")
            }
            Self::AuditRefused(reason) => {
                write!(
                    f,
                    "the audit store refused the event; the change was not applied: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for DestinationError {}
