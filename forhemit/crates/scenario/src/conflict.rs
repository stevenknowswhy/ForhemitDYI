//! Conflicts (Scenario Engine Data Model doc §16–17): severity describes
//! the condition — it is never a score. Conflicts are rows in their own
//! table (integrity rule 8): a newer scenario version cannot make one
//! disappear, and there is no delete API.

use forhemit_contracts::{
    ActorRecord, ConflictId, DestinationVersionId, NonnegotiableId, ObjectiveId, ScenarioVersionId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// What kind of conflict this is (schema doc §16).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictType {
    /// The scenario contradicts an owner objective.
    OwnerObjective,
    /// The scenario conflicts with a destination nonnegotiable — always
    /// paired with a [`NonnegotiableConflict`] extension.
    Nonnegotiable,
    /// The scenario's data contradicts itself or the record.
    Data,
    /// The scenario contradicts research.
    Research,
    /// The scenario contradicts a professional's determination.
    Professional,
    /// The scenario contradicts a model output.
    Model,
    /// The scenario depends on something unavailable.
    Dependency,
    /// Anything else.
    Other,
}

/// How heavy the condition is, in words — "describes the condition, not
/// produce a scenario score" (schema doc §17).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSeverity {
    /// Worth knowing; nothing more.
    Informational,
    /// The owner should look at this.
    Attention,
    /// This changes the scenario's meaning.
    Material,
    /// The scenario cannot proceed down this path until resolved.
    Blocking,
}

/// The stored conflict row (schema doc §16).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioConflict {
    /// The conflict's id.
    pub conflict_id: ConflictId,
    /// The scenario version the conflict is about.
    pub scenario_version_id: ScenarioVersionId,
    /// What kind of conflict it is.
    pub conflict_type: ConflictType,
    /// The condition's weight, in words.
    pub severity: ConflictSeverity,
    /// What the conflict is, in plain language.
    pub description: String,
    /// Where the conflict came from.
    pub source_reference: Option<String>,
    /// What kind of object the conflict touches, if it touches one.
    pub affected_object_type: Option<String>,
    /// Which object the conflict touches.
    pub affected_object_id: Option<String>,
    /// The recorded resolution, if any; `None` = open.
    pub resolution: Option<ConflictResolution>,
    /// When the conflict was recorded.
    pub created_at: OffsetDateTime,
}

/// The destination-side extension for a nonnegotiable conflict (schema
/// doc §14; NONNEGOTIABLE doc §5): the structured object, not
/// `warning = true`. Shares the conflict's id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NonnegotiableConflict {
    /// The shared conflict id.
    pub conflict_id: ConflictId,
    /// The scenario version.
    pub scenario_version_id: ScenarioVersionId,
    /// Which nonnegotiable row is in conflict.
    pub nonnegotiable_id: NonnegotiableId,
    /// The destination back-reference — the objective whose requirement
    /// the scenario runs against.
    pub destination_objective_id: ObjectiveId,
    /// The destination version whose designation is being tested.
    pub destination_version_id: DestinationVersionId,
    /// The owner's decision; `None` until the **owner** decides — the
    /// engine never sets it, never filters, never auto-relaxes.
    pub owner_decision: Option<OwnerDecision>,
    /// When the owner decided.
    pub decided_at: Option<OffsetDateTime>,
    /// When the conflict was recorded.
    pub created_at: OffsetDateTime,
}

/// The owner's three ways to decide a nonnegotiable conflict
/// (NONNEGOTIABLE doc §5) — decided by the owner, never by the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerDecision {
    /// Keep the must-have; the scenario must change or be abandoned.
    KeepRequirement,
    /// Explore a different scenario.
    ExploreAnotherPath,
    /// Change the requirement — a destination-engine edit that creates a
    /// new destination version (integrity rule 9).
    ChangeRequirement,
}

/// A recorded resolution on any conflict (schema doc §16).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConflictResolution {
    /// The decision that closed the conflict.
    pub decided_by: ActorRecord,
    /// When it was decided.
    pub decided_at: OffsetDateTime,
    /// Where the resolution lives, if it lives anywhere.
    pub resolution_reference: Option<String>,
}

/// The request half of recording a conflict.
#[derive(Clone, Debug)]
pub struct NewConflict {
    /// What kind of conflict it is.
    pub conflict_type: ConflictType,
    /// The condition's weight, in words.
    pub severity: ConflictSeverity,
    /// What the conflict is.
    pub description: String,
    /// Where the conflict came from.
    pub source_reference: Option<String>,
    /// What kind of object the conflict touches, if it touches one.
    pub affected_object_type: Option<String>,
    /// Which object the conflict touches.
    pub affected_object_id: Option<String>,
}
