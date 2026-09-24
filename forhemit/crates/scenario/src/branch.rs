//! What-if branches (Scenario Engine Data Model doc §30–31): explicit
//! lineage whose parent scenario version is always identified
//! (integrity rule 7). Creating a branch never modifies the parent.

use forhemit_contracts::{ActorRecord, BranchId, ScenarioFamilyId, ScenarioVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Why the branch exists (schema doc §30).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchType {
    /// The owner asked "what if …?".
    WhatIf,
    /// A distinct alternative structure.
    Alternative,
    /// A deliberate worst-case probe.
    StressCase,
    /// Requested by a professional.
    ProfessionalRequest,
    /// Requested by the owner outside the what-if flow.
    OwnerRequest,
}

/// The stored branch row (schema doc §30). `changed_assumptions` names
/// the parent version's assumption ids the branch overrides — the
/// parent's values remain readable through the immutable parent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioBranch {
    /// The branch's id.
    pub branch_id: BranchId,
    /// The parent scenario version — never absent (integrity rule 7).
    pub parent_scenario_version_id: ScenarioVersionId,
    /// The child family the branch created.
    pub child_scenario_family_id: ScenarioFamilyId,
    /// Why the branch exists.
    pub branch_type: BranchType,
    /// The branch's reason, in the asker's words.
    pub reason: String,
    /// The parent assumption ids the branch overrides.
    pub changed_assumptions: Vec<forhemit_contracts::AssumptionId>,
    /// When the branch was created.
    pub created_at: OffsetDateTime,
    /// Who created the branch.
    pub created_by: ActorRecord,
}

/// The request half of `ScenarioEngine::create_what_if`.
#[derive(Clone, Debug)]
pub struct NewWhatIf {
    /// The parent scenario version to branch from.
    pub parent_scenario_version_id: ScenarioVersionId,
    /// Why the branch exists.
    pub branch_type: BranchType,
    /// The branch's reason.
    pub reason: String,
    /// The parent assumption ids the branch will override.
    pub changed_assumptions: Vec<forhemit_contracts::AssumptionId>,
    /// The child family's name.
    pub name: String,
}
