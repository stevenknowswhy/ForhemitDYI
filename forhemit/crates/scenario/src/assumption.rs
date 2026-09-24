//! Scenario assumptions (Scenario Engine Data Model doc §7–11): a typed
//! value the scenario is built on, with **provenance** (how it entered
//! the system) and **verification** (how well it is checked) as two
//! separate axes.

use forhemit_contracts::{AssumptionId, ObjectiveId, Provenance, ScenarioVersionId, Verification};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::value::TypedValue;

/// What an assumption is about (schema doc §10). Descriptive, never an
/// authority claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionCategory {
    /// The business itself.
    Business,
    /// Financial figures and models.
    Financial,
    /// Ownership structure and percentages.
    Ownership,
    /// Market conditions.
    Market,
    /// Deal financing.
    Financing,
    /// Tax effects.
    Tax,
    /// Legal and regulatory.
    Legal,
    /// Operating realities.
    Operational,
    /// Timing of the transition.
    Timing,
    /// Employees and culture.
    Employee,
    /// Governance of the company.
    Governance,
    /// The seller's position.
    Seller,
    /// The buyers' position.
    Buyer,
    /// The world outside the deal.
    ExternalEnvironment,
    /// Anything else.
    Other,
}

/// The stored assumption row (schema doc §7). Every field is documented
/// on the row itself; provenance has no serde default (integrity rule 4:
/// a payload without it fails deserialization).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioAssumption {
    /// The assumption's id.
    pub assumption_id: AssumptionId,
    /// The draft version the assumption belongs to.
    pub scenario_version_id: ScenarioVersionId,
    /// What the assumption is about.
    pub category: AssumptionCategory,
    /// The assumption's name — the label the owner sees.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The typed value.
    pub value: TypedValue,
    /// How the value entered the system (schema doc §9: value,
    /// provenance, and verification travel together).
    pub provenance: Provenance,
    /// How well the value is verified — the separate axis (schema doc
    /// §11).
    pub verification: Verification,
    /// The destination objective this assumption exists to test, if it
    /// exists to test one (the implementation spec's
    /// `nonnegotiable_backref`).
    pub nonnegotiable_objective_id: Option<ObjectiveId>,
    /// Where the value came from, as a pointer the owner can follow.
    pub source_reference: Option<String>,
    /// When the assumption was added.
    pub created_at: OffsetDateTime,
}

/// The request half of adding an assumption to a draft.
#[derive(Clone, Debug)]
pub struct NewAssumption {
    /// What the assumption is about.
    pub category: AssumptionCategory,
    /// The assumption's name.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The typed value.
    pub value: TypedValue,
    /// How the value entered the system.
    pub provenance: Provenance,
    /// How well the value is verified.
    pub verification: Verification,
    /// The destination objective this assumption exists to test, if any.
    pub nonnegotiable_objective_id: Option<ObjectiveId>,
    /// Where the value came from, if anywhere.
    pub source_reference: Option<String>,
}

/// The request half of revising an assumption on a draft. The old value
/// is preserved in the audit event (integrity rule 4); the draft row is
/// replaced.
#[derive(Clone, Debug)]
pub struct NewAssumptionRevision {
    /// The assumption to revise — scoped to one draft version.
    pub assumption_id: AssumptionId,
    /// The new typed value.
    pub value: TypedValue,
    /// How the new value entered the system.
    pub provenance: Provenance,
    /// How well the new value is verified.
    pub verification: Verification,
    /// Why the assumption changed.
    pub change_reason: String,
}
