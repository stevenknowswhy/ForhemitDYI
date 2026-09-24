//! Scenario versions (Scenario Engine Data Model doc §4, §44, §46): the
//! primary historical object. A version is immutable once finalized
//! (integrity rule 6); a material change creates a successor version
//! whose `supersedes_version_id` chains the history.

use forhemit_contracts::{
    ActorRecord, BusinessRealityVersionId, CorrelationId, DestinationVersionId, FactVersionId,
    ScenarioFamilyId, ScenarioVersionId, SnapshotId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::assumption::ScenarioAssumption;
use crate::constraint::ScenarioConstraint;
use crate::nonnegotiable::ScenarioNonnegotiable;
use crate::snapshot::ScenarioSnapshot;
use crate::status::{LifecycleStatus, ReadinessStatus, ScenarioType, StatusHistoryEntry};
use crate::unknown::ScenarioUnknown;

/// The stored version row (schema doc §4). Every reference is typed and
/// single-valued: there is no link table behind rules 1 and 2, so
/// "exactly one destination version" and "the reality version used" are
/// enforced by the shape itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioVersion {
    /// The version's id.
    pub scenario_version_id: ScenarioVersionId,
    /// The family the version belongs to.
    pub scenario_family_id: ScenarioFamilyId,
    /// The type of path being explored — fixed per family, restated on
    /// the version for single-row reads.
    pub scenario_type: ScenarioType,
    /// The version's number within the family — ≥ 1, unique per family,
    /// gapless.
    pub version_number: u32,
    /// The version this one was created from (lineage); `None` on a
    /// family's first version.
    pub parent_version_id: Option<ScenarioVersionId>,
    /// The version this one officially replaces — equal to
    /// `parent_version_id` for revisions, `None` on branches.
    pub supersedes_version_id: Option<ScenarioVersionId>,
    /// The version's name.
    pub name: String,
    /// The version's description, if any.
    pub description: Option<String>,
    /// Why this version exists — required on successors (schema doc §47).
    pub change_reason: Option<String>,
    /// Where the version is in its life.
    pub lifecycle_status: LifecycleStatus,
    /// Whether the package can include it — the separate axis.
    pub readiness_status: ReadinessStatus,
    /// The 1:1 snapshot pinned at creation.
    pub snapshot: ScenarioSnapshot,
    /// **Integrity rule 1:** exactly one destination version.
    pub destination_version_id: DestinationVersionId,
    /// **Integrity rule 2:** the Business Reality version used to create
    /// it (content address of the pinned fact versions).
    pub business_reality_version_id: BusinessRealityVersionId,
    /// The assumptions the version carries.
    pub assumptions: Vec<ScenarioAssumption>,
    /// The constraints the version carries.
    pub constraints: Vec<ScenarioConstraint>,
    /// The destination nonnegotiables the version is testing.
    pub nonnegotiables: Vec<ScenarioNonnegotiable>,
    /// The version's named unknowns — first-class, never blank zeros.
    pub unknowns: Vec<ScenarioUnknown>,
    /// The version's status history (schema doc §36) — append-only.
    pub status_history: Vec<StatusHistoryEntry>,
    /// Set by `finalize_version`; the transition that froze the row.
    pub finalized_at: Option<OffsetDateTime>,
    /// When the version was created.
    pub created_at: OffsetDateTime,
    /// Who created the version.
    pub created_by: ActorRecord,
}

impl ScenarioVersion {
    /// True while the version may still be edited in place.
    pub fn is_draft(&self) -> bool {
        self.lifecycle_status.is_draft()
    }
}

/// The request half of `ScenarioEngine::start_draft`.
#[derive(Clone, Debug)]
pub struct NewDraft {
    /// The family the draft starts in.
    pub scenario_family_id: ScenarioFamilyId,
    /// The draft's name.
    pub name: String,
    /// The draft's description, if any.
    pub description: Option<String>,
    /// **Integrity rule 1:** the one destination version this draft is
    /// built against.
    pub destination_version_id: DestinationVersionId,
    /// The exact Business Reality fact versions pinned at creation; the
    /// engine derives the rule-2 content address from them.
    pub reality_fact_version_ids: Vec<FactVersionId>,
    /// Reserved: the financial model version the draft builds on, if
    /// any (carried opaquely in v1).
    pub financial_model_version_id: Option<forhemit_contracts::FinancialModelVersionId>,
    /// Who is starting the draft.
    pub actor: ActorRecord,
    /// The activity this belongs to (Audit doc §18).
    pub correlation_id: CorrelationId,
}

/// The request half of `ScenarioEngine::start_successor_draft`
/// (schema doc §47): a material change to a finalized version is
/// recorded as a new version, never as an edit.
#[derive(Clone, Debug)]
pub struct NewSuccessorDraft {
    /// The finalized version being superseded.
    pub parent_version_id: ScenarioVersionId,
    /// **Integrity rule 1, re-pinned:** the destination version the
    /// successor is built against — normally the current destination
    /// version; equal to the parent's pin when only reality moved.
    pub destination_version_id: DestinationVersionId,
    /// The exact Business Reality fact versions pinned now; the engine
    /// derives the successor's rule-2 content address from them.
    pub reality_fact_version_ids: Vec<FactVersionId>,
    /// Why the successor exists — required (schema doc §47).
    pub change_reason: String,
    /// A new name, if the successor should carry one.
    pub name_override: Option<String>,
    /// Who is starting the successor.
    pub actor: ActorRecord,
    /// The activity this belongs to.
    pub correlation_id: CorrelationId,
}

/// The snapshot id accessor lives here so the engine never constructs a
/// version without one.
pub(crate) fn new_snapshot_id() -> SnapshotId {
    new_id("snap", |s| SnapshotId::new(s))
}

/// Mints a prefixed ULID id; the prefix keeps ids readable in exports and
/// audit payloads.
pub(crate) fn new_id<T, E, F>(prefix: &str, make: F) -> T
where
    F: FnOnce(String) -> Result<T, E>,
{
    make(format!("{prefix}_{}", ulid::Ulid::new()))
        .unwrap_or_else(|_| unreachable!("a non-empty prefixed ULID is always a valid id"))
}
