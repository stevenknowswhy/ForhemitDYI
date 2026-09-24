//! Assumption provenance and verification status (Scenario Engine Data Model
//! — Schema Specification).
//!
//! "Separate verification from provenance" (schema doc §11): how a value
//! entered the system and how well it is verified are two independent axes,
//! so they are two independent enums.

use serde::{Deserialize, Serialize};

/// How a value entered the system. Variants are the schema doc's canonical
/// provenance list, verbatim (snake_case of the SCREAMING_CASE names).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Reported by the owner.
    OwnerReported,
    /// Supported by an imported document.
    DocumentSupported,
    /// Supported by research.
    ResearchSupported,
    /// Supplied by a professional.
    ProfessionallySupplied,
    /// Assumed for exploratory modeling.
    ScenarioAssumed,
    /// Derived by the system from other data.
    SystemDerived,
    /// Derived by a model run.
    ModelDerived,
}

/// How well a value is verified — independent of its provenance (schema doc
/// §11 "VerificationStatus").
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verification {
    /// Verification state unknown.
    Unknown,
    /// Known to be unverified.
    Unverified,
    /// Partially verified.
    PartiallyVerified,
    /// Verified.
    Verified,
    /// Verified by a professional.
    ProfessionallyVerified,
    /// Contested.
    Contested,
    /// Rejected.
    Rejected,
}
