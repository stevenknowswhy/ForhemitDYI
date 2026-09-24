//! The five Employee Ownership Journey v0.2 scenario structure
//! templates — **journey data, not code** (the task's fourth
//! deliverable).
//!
//! The templates ship as JSON files under `templates/`, embedded at
//! compile time, and are the starting point the journey offers when an
//! owner opens one of the five structures. Seeded assumptions carry
//! `scenario_assumed` or `owner_reported` provenance and unverified
//! verification; seeded unknowns are first-class names for what the
//! owner does not know yet. A template is a starting point to confirm
//! or replace — never a recommendation.

use serde::Deserialize;

use crate::assumption::{AssumptionCategory, NewAssumption};
use crate::status::ScenarioType;
use crate::unknown::{NewUnknown, UnknownImportance};
use crate::value::TypedValue;

/// One v0.2 structure template.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructureTemplate {
    /// Stable id, equal to the file stem (`"esop"`, …).
    pub template_id: String,
    /// The name the journey shows.
    pub name: String,
    /// The scenario type the template's family is created with.
    pub scenario_type: ScenarioType,
    /// What the path is, in plain language.
    pub description: String,
    /// Assumptions the draft starts with.
    #[serde(default)]
    pub seed_assumptions: Vec<TemplateAssumption>,
    /// Unknowns the draft starts with.
    #[serde(default)]
    pub seed_unknowns: Vec<TemplateUnknown>,
}

/// One seeded assumption row.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateAssumption {
    /// What the assumption is about.
    pub category: AssumptionCategory,
    /// The assumption's name.
    pub name: String,
    /// The longer description, if any.
    #[serde(default)]
    pub description: Option<String>,
    /// The typed starting value.
    pub value: TypedValue,
    /// How the value entered the system — `scenario_assumed` or
    /// `owner_reported` on shipped templates.
    pub provenance: forhemit_contracts::Provenance,
    /// How well the value is checked — `unknown` or `unverified` on
    /// shipped templates.
    pub verification: forhemit_contracts::Verification,
    /// Where the value came from, if anywhere.
    #[serde(default)]
    pub source_reference: Option<String>,
}

/// One seeded unknown row.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateUnknown {
    /// What area the unknown is about.
    #[serde(default)]
    pub category: Option<String>,
    /// What is not known.
    pub description: String,
    /// How much it matters.
    pub importance: UnknownImportance,
    /// What the owner would do to resolve it, if anything.
    #[serde(default)]
    pub required_action: Option<String>,
    /// Which engine or journey surface can resolve it.
    #[serde(default)]
    pub source_dependency: Option<String>,
}

/// Why a template file could not be loaded.
#[derive(Debug)]
pub enum TemplateError {
    /// The embedded JSON did not parse or match the schema above.
    Malformed {
        /// Which embedded template failed.
        template_id: String,
        /// The parse/schema error.
        source: serde_json::Error,
    },
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed {
                template_id,
                source,
            } => write!(f, "template {template_id} is malformed: {source}"),
        }
    }
}

impl std::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Malformed { source, .. } => Some(source),
        }
    }
}

/// The embedded template files, in the v0.2 order.
const TEMPLATE_SOURCES: [(&str, &str); 5] = [
    ("esop", include_str!("../templates/esop.json")),
    (
        "direct_employee_purchase",
        include_str!("../templates/direct_employee_purchase.json"),
    ),
    (
        "management_buyout",
        include_str!("../templates/management_buyout.json"),
    ),
    (
        "employee_owned_acquisition_entity",
        include_str!("../templates/employee_owned_acquisition_entity.json"),
    ),
    (
        "staged_ownership",
        include_str!("../templates/staged_ownership.json"),
    ),
];

/// Loads the five v0.2 structure templates.
///
/// # Errors
/// [`TemplateError::Malformed`] if an embedded template does not parse
/// against the template schema — a data bug that must fail loudly, not
/// silently skip a structure.
pub fn load_builtin_templates() -> Result<Vec<StructureTemplate>, TemplateError> {
    TEMPLATE_SOURCES
        .iter()
        .map(|(id, source)| {
            serde_json::from_str(source).map_err(|source| TemplateError::Malformed {
                template_id: (*id).to_owned(),
                source,
            })
        })
        .collect()
}

impl StructureTemplate {
    /// The template's seeded assumptions as engine requests.
    pub fn assumption_requests(&self) -> Vec<NewAssumption> {
        self.seed_assumptions
            .iter()
            .map(|seed| NewAssumption {
                category: seed.category,
                name: seed.name.clone(),
                description: seed.description.clone(),
                value: seed.value.clone(),
                provenance: seed.provenance.clone(),
                verification: seed.verification.clone(),
                nonnegotiable_objective_id: None,
                source_reference: seed.source_reference.clone(),
            })
            .collect()
    }

    /// The template's seeded unknowns as engine requests.
    pub fn unknown_requests(&self) -> Vec<NewUnknown> {
        self.seed_unknowns
            .iter()
            .map(|seed| NewUnknown {
                category: seed.category.clone(),
                description: seed.description.clone(),
                importance: seed.importance,
                required_action: seed.required_action.clone(),
                source_dependency: seed.source_dependency.clone(),
            })
            .collect()
    }
}
