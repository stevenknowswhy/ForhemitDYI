//! Scenario constraints (Scenario Engine Data Model doc §12): a named
//! boundary a scenario must respect — the owner's, a professional's, or
//! the outside world's.

use forhemit_contracts::{ConstraintId, ScenarioVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::value::TypedValue;

/// The constraint's strength (schema doc §12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    /// A hard boundary — the scenario must respect it.
    Hard,
    /// A strong preference — override only with an explicit recorded
    /// decision.
    StrongPreference,
    /// A preference.
    Preference,
    /// The strength itself is unknown.
    Unknown,
    /// Set by a professional.
    ProfessionalRequirement,
    /// Set by an outside party (regulator, lender, …).
    ExternalConstraint,
}

/// The stored constraint row (schema doc §12).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioConstraint {
    /// The constraint's id.
    pub constraint_id: ConstraintId,
    /// The draft version the constraint belongs to.
    pub scenario_version_id: ScenarioVersionId,
    /// How hard the boundary is.
    pub constraint_type: ConstraintType,
    /// The constraint's name.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The boundary's value, if it has one.
    pub value: Option<TypedValue>,
    /// Where the constraint came from.
    pub source_reference: Option<String>,
    /// When the constraint was added.
    pub created_at: OffsetDateTime,
}

/// The request half of adding a constraint to a draft.
#[derive(Clone, Debug)]
pub struct NewConstraint {
    /// How hard the boundary is.
    pub constraint_type: ConstraintType,
    /// The constraint's name.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The boundary's value, if it has one.
    pub value: Option<TypedValue>,
    /// Where the constraint came from.
    pub source_reference: Option<String>,
}
