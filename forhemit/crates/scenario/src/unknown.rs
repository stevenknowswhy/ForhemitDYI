//! First-class unknowns (Scenario Engine Data Model doc §15): missing
//! information is a named object with an importance — never a blank zero
//! and never a silent omission.

use forhemit_contracts::{ActorRecord, ScenarioVersionId, UnknownId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// How much the unknown matters (schema doc §15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownImportance {
    /// The scenario cannot be used for its purpose without it.
    Critical,
    /// Materially changes the picture.
    Important,
    /// Nice to know.
    Helpful,
}

/// Where the unknown stands (schema doc §15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownResolutionStatus {
    /// Nobody has picked it up.
    Open,
    /// Someone is working on it.
    InProgress,
    /// Answered — see the resolution reference.
    Resolved,
    /// The owner chose to proceed without it.
    Waived,
    /// Overtaken by events.
    Superseded,
}

/// The stored unknown row (schema doc §15). The schema doc's
/// `blocking_level` field has no enumerated vocabulary, so it is omitted
/// from v1 rather than invented as a free string.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioUnknown {
    /// The unknown's id.
    pub unknown_id: UnknownId,
    /// The draft version the unknown belongs to.
    pub scenario_version_id: ScenarioVersionId,
    /// An optional category label.
    pub category: Option<String>,
    /// What is missing, in plain language.
    pub description: String,
    /// How much it matters.
    pub importance: UnknownImportance,
    /// Where the unknown stands.
    pub resolution_status: UnknownResolutionStatus,
    /// What to do about it, if known.
    pub required_action: Option<String>,
    /// What it depends on, if anything.
    pub source_dependency: Option<String>,
    /// When the unknown was added.
    pub created_at: OffsetDateTime,
    /// When the unknown was resolved or waived.
    pub resolved_at: Option<OffsetDateTime>,
    /// Who resolved or waived it.
    pub resolved_by: Option<ActorRecord>,
    /// Where the answer lives.
    pub resolution_reference: Option<String>,
}

/// The request half of adding an unknown to a draft.
#[derive(Clone, Debug)]
pub struct NewUnknown {
    /// An optional category label.
    pub category: Option<String>,
    /// What is missing.
    pub description: String,
    /// How much it matters.
    pub importance: UnknownImportance,
    /// What to do about it, if known.
    pub required_action: Option<String>,
    /// What it depends on, if anything.
    pub source_dependency: Option<String>,
}

/// The request half of resolving an unknown.
#[derive(Clone, Debug)]
pub struct UnknownResolution {
    /// The unknown to resolve.
    pub unknown_id: UnknownId,
    /// The outcome — resolved or waived; the engine refuses `open` /
    /// `in_progress` / `superseded` through [`crate::ScenarioEngine`]'s
    /// typed parameters.
    pub status: UnknownResolutionStatus,
    /// Where the answer lives, if anywhere.
    pub resolution_reference: Option<String>,
}
