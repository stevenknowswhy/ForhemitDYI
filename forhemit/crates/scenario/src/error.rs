//! Error surface of the Scenario Engine.
//!
//! Every variant names the condition it reports; nothing is swallowed. The
//! engine is audit-first: [`ScenarioError::Audit`] means the mutation did
//! **not** happen — no scenario row exists without its audit event.

use forhemit_contracts::{
    AssumptionId, ComparisonId, ConflictId, DestinationVersionId, NonnegotiableId, ScenarioFamilyId,
    ScenarioVersionId, UnknownId,
};
use forhemit_enginekit::AuditError;

use crate::status::{LifecycleStatus, ReadinessStatus};

/// What can go wrong while working with scenarios.
#[derive(Debug)]
pub enum ScenarioError {
    /// The audit store rejected the event. The mutation did not happen —
    /// no scenario row is created without its audit event.
    Audit(AuditError),
    /// A name, description, or reason was left empty; the docs' stance is
    /// that absence is expressed by not recording content, never by a
    /// blank string.
    EmptyText {
        /// Which field was blank.
        field: &'static str,
    },
    /// A typed value is malformed — an out-of-range date, an inverted
    /// range, or a `value` whose shape does not match the declared
    /// `value_type` (schema doc §8: "never a bare string").
    InvalidValue {
        /// What is wrong with the value.
        reason: &'static str,
    },
    /// No scenario family exists under the given id.
    FamilyNotFound {
        /// The unknown family id.
        scenario_family_id: ScenarioFamilyId,
    },
    /// No scenario version exists under the given id.
    VersionNotFound {
        /// The unknown version id.
        scenario_version_id: ScenarioVersionId,
    },
    /// No conflict exists under the given id.
    ConflictNotFound {
        /// The unknown conflict id.
        conflict_id: ConflictId,
    },
    /// No assumption exists under the given id on the given version.
    AssumptionNotFound {
        /// The unknown assumption id.
        assumption_id: AssumptionId,
    },
    /// A draft mutation targeted a version whose lifecycle has left the
    /// mutable draft set (integrity rule 6: historical versions cannot be
    /// mutated). The material change belongs in a successor version
    /// (schema doc §46–47).
    FinalizedVersionImmutable {
        /// The frozen version id.
        scenario_version_id: ScenarioVersionId,
    },
    /// A successor draft was requested from a version that is itself
    /// still a draft. Edit the draft instead of forking it.
    ParentStillMutable {
        /// The draft version id that cannot be superseded.
        scenario_version_id: ScenarioVersionId,
    },
    /// The nonnegotiable under test belongs to a different destination
    /// version than the scenario version pins (integrity rule 1's
    /// companion: a scenario tests the nonnegotiables of the destination
    /// version it references).
    DestinationVersionMismatch {
        /// The destination version the scenario version pins.
        pinned: DestinationVersionId,
        /// The destination version the nonnegotiable names.
        supplied: DestinationVersionId,
    },
    /// A comparison was requested with fewer than two scenario versions
    /// (schema doc §32: a comparison "captures what was compared" — at
    /// least two paths).
    ComparisonNeedsTwoVersions,
    /// A comparison result referenced a scenario version that is not part
    /// of the comparison.
    ComparisonForeignVersion {
        /// The version id outside the comparison.
        scenario_version_id: ScenarioVersionId,
    },
    /// A comparison result referenced a dimension index that does not
    /// exist in the comparison being recorded.
    ComparisonUnknownDimension {
        /// The index that was out of range.
        index: usize,
    },
    /// A comparison recorded two results for the same version and
    /// dimension (`UNIQUE(comparison_id, scenario_version_id,
    /// dimension_id)`).
    ComparisonDuplicateResult,
    /// The same scenario version was listed twice in one comparison —
    /// what was compared must be a set of distinct paths.
    DuplicateComparisonVersion,
    /// The family is already archived; archiving is a terminal family
    /// transition (retention, not deletion, either way).
    FamilyAlreadyArchived {
        /// The archived family id.
        scenario_family_id: ScenarioFamilyId,
    },
    /// A lifecycle transition targeted a version that already left the
    /// mutable draft window — finalizing twice, editing a final.
    NotADraft {
        /// The version that is not a draft.
        scenario_version_id: ScenarioVersionId,
        /// Where its lifecycle actually is.
        lifecycle_status: LifecycleStatus,
    },
    /// A readiness transition targeted a draft. Drafts are always
    /// `preliminary` on the readiness axis; readiness moves only after
    /// finalizing (schema doc §40: the axes are separate).
    DraftNotReadinessGated {
        /// The draft version id.
        scenario_version_id: ScenarioVersionId,
    },
    /// The readiness transition would leave the status unchanged.
    ReadinessUnchanged {
        /// The version id.
        scenario_version_id: ScenarioVersionId,
        /// The readiness that is already in place.
        readiness: ReadinessStatus,
    },
    /// No nonnegotiable-under-test row exists under the given id on the
    /// given version.
    NonnegotiableNotFound {
        /// The unknown nonnegotiable row id.
        nonnegotiable_id: NonnegotiableId,
    },
    /// No unknown exists under the given id on the given version.
    UnknownNotFound {
        /// The unknown unknown id.
        unknown_id: UnknownId,
    },
    /// The conflict is already resolved — a resolution is final; a new
    /// decision is a new event, not a rewrite of this one.
    ConflictAlreadyResolved {
        /// The resolved conflict id.
        conflict_id: ConflictId,
    },
    /// No comparison exists under the given id.
    ComparisonNotFound {
        /// The unknown comparison id.
        comparison_id: ComparisonId,
    },
    /// Serializing an event payload failed.
    Serialization(serde_json::Error),
    /// A condition the type system rules out reached the engine anyway —
    /// a defensive arm for invariant violations. If it fires, it is a bug.
    Internal(String),
}

impl std::fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audit(err) => write!(f, "audit store rejected the event: {err}"),
            Self::EmptyText { field } => write!(f, "{field} must not be empty"),
            Self::InvalidValue { reason } => write!(f, "invalid value: {reason}"),
            Self::FamilyNotFound { scenario_family_id } => {
                write!(f, "scenario family not found: {scenario_family_id}")
            }
            Self::VersionNotFound { scenario_version_id } => {
                write!(f, "scenario version not found: {scenario_version_id}")
            }
            Self::ConflictNotFound { conflict_id } => {
                write!(f, "conflict not found: {conflict_id}")
            }
            Self::AssumptionNotFound { assumption_id } => {
                write!(f, "assumption not found: {assumption_id}")
            }
            Self::FinalizedVersionImmutable { scenario_version_id } => write!(
                f,
                "scenario version {scenario_version_id} is finalized and immutable; \
                 record the change as a successor version"
            ),
            Self::ParentStillMutable { scenario_version_id } => write!(
                f,
                "scenario version {scenario_version_id} is still a draft; \
                 edit the draft instead of forking a successor from it"
            ),
            Self::DestinationVersionMismatch { pinned, supplied } => write!(
                f,
                "nonnegotiable tests destination version {supplied}, but the scenario \
                 version pins {pinned}"
            ),
            Self::ComparisonNeedsTwoVersions => write!(
                f,
                "a comparison captures what was compared: at least two scenario versions"
            ),
            Self::ComparisonForeignVersion { scenario_version_id } => write!(
                f,
                "comparison result references {scenario_version_id}, which is not part \
                 of the comparison"
            ),
            Self::ComparisonUnknownDimension { index } => {
                write!(f, "comparison result references unknown dimension index {index}")
            }
            Self::ComparisonDuplicateResult => {
                write!(f, "a version may appear at most once per comparison dimension")
            }
            Self::DuplicateComparisonVersion => {
                write!(f, "a comparison compares distinct paths; a version was listed twice")
            }
            Self::FamilyAlreadyArchived { scenario_family_id } => {
                write!(f, "scenario family {scenario_family_id} is already archived")
            }
            Self::NotADraft { scenario_version_id, lifecycle_status } => write!(
                f,
                "scenario version {scenario_version_id} is not a draft (lifecycle {}); \
                 a material change creates a successor version",
                lifecycle_status.status_name()
            ),
            Self::DraftNotReadinessGated { scenario_version_id } => write!(
                f,
                "scenario version {scenario_version_id} is still a draft; \
                 readiness moves only after finalizing"
            ),
            Self::ReadinessUnchanged { scenario_version_id, readiness } => write!(
                f,
                "scenario version {scenario_version_id} is already {} on the readiness axis",
                readiness.status_name()
            ),
            Self::NonnegotiableNotFound { nonnegotiable_id } => {
                write!(f, "nonnegotiable under test not found: {nonnegotiable_id}")
            }
            Self::UnknownNotFound { unknown_id } => {
                write!(f, "unknown not found: {unknown_id}")
            }
            Self::ConflictAlreadyResolved { conflict_id } => {
                write!(f, "conflict {conflict_id} is already resolved")
            }
            Self::ComparisonNotFound { comparison_id } => {
                write!(f, "comparison not found: {comparison_id}")
            }
            Self::Serialization(err) => write!(f, "failed to serialize event payload: {err}"),
            Self::Internal(message) => write!(f, "internal engine error: {message}"),
        }
    }
}

impl std::error::Error for ScenarioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Audit(err) => Some(err),
            Self::Serialization(err) => Some(err),
            _ => None,
        }
    }
}
