//! Destination versions — immutable snapshots linked into a chain.
//!
//! Destination Engine doc §26: "Every material change creates a new
//! version… The previous version remains accessible." A stored version is
//! never edited in place; an edit appends a new version whose
//! `previous_version_id` links the chain (§30 "No cascade destruction").

use crate::content::{ChangedField, DestinationContent};
use forhemit_contracts::{ActorRecord, DestinationId, DestinationVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ulid::Ulid;

/// Why the owner changed the destination — Destination Engine doc §27's
/// "Why are you changing this?" list, verbatim.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ChangeReason {
    /// "I learned something new."
    LearnedSomethingNew,
    /// "My priorities changed."
    PrioritiesChanged,
    /// "My professional suggested another approach."
    ProfessionalSuggestedAnotherApproach,
    /// "The business changed."
    BusinessChanged,
    /// "I want to explore a different outcome."
    ExploringDifferentOutcome,
    /// "Other."
    Other,
}

/// What changed and why — the version's provenance. `None` on the initial
/// version (a destination's first version has no prior state to explain).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VersionChange {
    /// The owner's reason for the change (doc §27).
    pub reason: ChangeReason,
    /// The doc §27 optional explanation.
    pub explanation: Option<String>,
    /// Which content fields changed — doc §29's `changed_fields`.
    pub changed_fields: Vec<ChangedField>,
}

/// One immutable destination version: the full content at a point in
/// time, who created it, when, and why (for edits).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DestinationVersion {
    /// This version's identifier — the object ID audit events for the
    /// version reference.
    pub version_id: DestinationVersionId,
    /// The destination this version belongs to.
    pub destination_id: DestinationId,
    /// 1-based position in the destination's history; sequential, no gaps.
    pub version_number: u32,
    /// The version this one was created from — `None` for the initial
    /// version; the chain link that makes history a walkable graph.
    pub previous_version_id: Option<DestinationVersionId>,
    /// When this version was created (from the injected clock).
    pub created_at: OffsetDateTime,
    /// Who created this version.
    pub created_by: ActorRecord,
    /// What changed and why — `None` on the initial version.
    pub change: Option<VersionChange>,
    /// The full DesiredOutcome content at this point in history. Immutable
    /// once stored: an edit creates a new version, never a rewrite.
    pub content: DestinationContent,
}

impl DestinationVersion {
    /// The initial version of a destination.
    pub(crate) fn initial(
        destination_id: DestinationId,
        content: DestinationContent,
        created_by: ActorRecord,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            version_id: new_version_id(),
            destination_id,
            version_number: 1,
            previous_version_id: None,
            created_at,
            created_by,
            change: None,
            content,
        }
    }

    /// A successor version created by a material edit.
    #[allow(clippy::too_many_arguments)] // an immutable snapshot names every one of these
    pub(crate) fn successor(
        previous: &DestinationVersion,
        content: DestinationContent,
        reason: ChangeReason,
        explanation: Option<String>,
        changed_fields: Vec<ChangedField>,
        created_by: ActorRecord,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            version_id: new_version_id(),
            destination_id: previous.destination_id.clone(),
            version_number: previous.version_number + 1,
            previous_version_id: Some(previous.version_id.clone()),
            created_at,
            created_by,
            change: Some(VersionChange {
                reason,
                explanation,
                changed_fields,
            }),
            content,
        }
    }
}

/// Mints a fresh version identifier (a ULID).
pub(crate) fn new_version_id() -> DestinationVersionId {
    DestinationVersionId::new(Ulid::new().to_string())
        .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
}
