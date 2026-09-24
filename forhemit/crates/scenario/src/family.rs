//! Scenario families (Scenario Engine Data Model doc §3): one conceptual
//! path through its entire history. A family holds **no** mutable
//! scenario content — assumptions, unknowns, and constraints belong to
//! versions.

use forhemit_contracts::{ActorRecord, ScenarioFamilyId, ScenarioVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The stored family row (schema doc §3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioFamily {
    /// The family's id.
    pub scenario_family_id: ScenarioFamilyId,
    /// The workspace the family belongs to.
    pub workspace_id: forhemit_contracts::WorkspaceId,
    /// The family's name.
    pub name: String,
    /// The type of path being explored.
    pub scenario_type: crate::status::ScenarioType,
    /// Set iff this family was created by a what-if branch — the mirror
    /// link to the branch row (schema doc §30).
    pub branched_from_version_id: Option<ScenarioVersionId>,
    /// When the family was created.
    pub created_at: OffsetDateTime,
    /// Who created the family.
    pub created_by: ActorRecord,
    /// When the family was archived, if it was.
    pub archived_at: Option<OffsetDateTime>,
    /// Why the family was archived — required iff `archived_at` is set.
    pub archive_reason: Option<String>,
}

/// The request half of `ScenarioEngine::create_family`.
#[derive(Clone, Debug)]
pub struct NewFamily {
    /// The family's name.
    pub name: String,
    /// The type of path being explored.
    pub scenario_type: crate::status::ScenarioType,
    /// Who is creating it.
    pub actor: ActorRecord,
    /// The activity this belongs to (Audit doc §18).
    pub correlation_id: forhemit_contracts::CorrelationId,
}

/// The request half of `ScenarioEngine::archive_family` — retention, not
/// deletion.
#[derive(Clone, Debug)]
pub struct ArchiveFamily {
    /// The family to archive.
    pub scenario_family_id: ScenarioFamilyId,
    /// Why the family is being retired.
    pub archive_reason: String,
    /// Who is archiving it.
    pub actor: ActorRecord,
    /// The activity this belongs to.
    pub correlation_id: forhemit_contracts::CorrelationId,
}
