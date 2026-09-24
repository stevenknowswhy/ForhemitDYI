//! The Scenario Engine: the representation of a possible path — nothing more.
//!
//! Scenario data can become the backbone of a proposed structure for the
//! company's future, but it never becomes a recommendation (Scenario Engine
//! Data Model doc §52: scenario owns "the representation of a possible
//! path" — never the truth of the inputs, the mathematics, the professional
//! determination, the owner's decision, or the transaction). Comparisons
//! have no winner; conflicts carry a severity that describes the condition,
//! never a score; a nonnegotiable conflict is surfaced for the owner to
//! decide, never filtered or auto-relaxed.
//!
//! This crate implements the relational schema and event contract in
//! [`SCHEMA.md`] — the artifact the Scenario Engine Data Model doc
//! prescribes but never wrote. The engine is audit-first: every mutation
//! emits exactly one audit event **before** the mutation is applied, so a
//! scenario row never exists without its event (Implementation Roadmap
//! Phase 1).
//!
//! [`SCHEMA.md`]: ../scenario/SCHEMA.md

mod assumption;
mod branch;
mod comparison;
mod conflict;
mod constraint;
mod engine;
mod error;
mod family;
mod nonnegotiable;
mod review;
mod snapshot;
mod status;
mod template;
mod unknown;
mod value;
mod version;

pub use assumption::{
    AssumptionCategory, NewAssumption, NewAssumptionRevision, ScenarioAssumption,
};
pub use branch::{BranchType, NewWhatIf, ScenarioBranch};
pub use comparison::{
    ComparisonDimension, ComparisonDimensionType, ComparisonOutcome, ComparisonResult,
    NewComparison, NewComparisonDimension, NewComparisonResult, ScenarioComparison,
};
pub use conflict::{
    ConflictResolution, ConflictSeverity, ConflictType, NewConflict, NonnegotiableConflict,
    ScenarioConflict,
};
pub use constraint::{ConstraintType, NewConstraint, ScenarioConstraint};
pub use engine::{
    business_reality_version, ConflictDecision, ConflictRecord, DraftChange, DraftSection,
    DraftUpdatedPayload, ImpactStatus, ImpactType, ScenarioEngine, ScenarioImpact, WhatIfCreated,
};
pub use error::ScenarioError;
pub use family::{ArchiveFamily, NewFamily, ScenarioFamily};
pub use nonnegotiable::{NewNonnegotiable, ScenarioNonnegotiable};
pub use review::{NewReviewReference, ProfessionalReviewReference, ProfessionalReviewStatus};
pub use snapshot::ScenarioSnapshot;
pub use status::{LifecycleStatus, ReadinessStatus, ScenarioType, StatusHistoryEntry};
pub use template::{
    load_builtin_templates, StructureTemplate, TemplateAssumption, TemplateError, TemplateUnknown,
};
pub use unknown::{
    NewUnknown, ScenarioUnknown, UnknownImportance, UnknownResolution, UnknownResolutionStatus,
};
pub use value::{CurrencyCode, FixedDecimal, TypedValue};
pub use version::{NewDraft, NewSuccessorDraft, ScenarioVersion};

use forhemit_contracts::ObjectiveId;

/// A back-reference from scenario content to the destination objective it
/// exists to test — the destination side of the decision-layers rule
/// (THREE DECISION LAYERS doc §14–15: the scenario layer proposes against
/// the owner-objective layer; it never overwrites it).
pub type NonnegotiableBackref = ObjectiveId;
