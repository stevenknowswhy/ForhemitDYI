//! # forhemit-contracts
//!
//! The versioned cross-engine domain vocabulary — per the implementation
//! spec, `contracts` is "the ONLY cross-engine vocabulary": every engine
//! crate may depend on it (plus `enginekit`), and it has zero workspace
//! dependencies of its own.
//!
//! ## What lives here
//!
//! Doc-grounded vocabulary — enum variants are read verbatim from the design
//! documents, and each type's docs cite its source — typed identifier
//! newtypes, and integrity primitives (validated hash digests, hash-addressed
//! payload references).
//!
//! ## The versioned-contract pattern
//!
//! Structured contract types are versioned through serde's internal tag.
//! Stored data is never reinterpreted in place — a new contract version is a
//! new variant (spec: "stored events are never reinterpreted in place"). The
//! audit event contract, which lands with the audit store task, follows this
//! exact shape:
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
//! #[serde(tag = "event_version", rename_all = "snake_case")]
//! pub enum ExampleEvent {
//!     /// Version 1 — the only version in this example.
//!     V1 {
//!         object_id: String,
//!     },
//! }
//!
//! let event: ExampleEvent =
//!     serde_json::from_str(r#"{"event_version":"v1","object_id":"obj_1"}"#)
//!         .expect("a v1 document");
//! assert_eq!(event, ExampleEvent::V1 { object_id: "obj_1".to_owned() });
//! ```
//!
//! Plain structs reject unknown keys outright (see
//! [`integrity::PayloadRef`]). Tagged enums enforce their version tag
//! instead — serde derive cannot combine an internal tag with
//! `deny_unknown_fields` — so [`audit::AuditEvent`] rejects missing or
//! unknown `event_version` values: older readers refuse newer envelopes
//! instead of reinterpreting them.

pub mod actor;
pub mod audit;
pub mod decision;
pub mod engine;
pub mod ids;
pub mod integrity;
pub mod nonnegotiable;
pub mod provenance;
pub mod schema;

pub use actor::{ActionOrigination, ActorClassification, ActorKind, ActorRecord};
pub use audit::{AuditEvent, AuditEventDraft, AuditEventType};
pub use decision::DecisionLayer;
pub use engine::EngineId;
pub use ids::{
    ActorId,
    AnalysisId,
    AssumptionId,
    BranchId,
    BusinessRealityVersionId,
    CausationId,
    ComparisonDimensionId,
    ComparisonId,
    ComparisonResultId,
    ConflictId,
    ConstraintId,
    CorrelationId,
    DestinationId,
    DestinationVersionId,
    DocumentId,
    DocumentVersionId,
    EmptyIdError,
    EventId,
    FactId,
    FactVersionId,
    FinancialModelVersionId,
    NonnegotiableId,
    ObjectId,
    ObjectiveId,
    ProfessionalReviewId,
    ProfessionalReviewReferenceId,
    ScenarioFamilyId,
    ScenarioImpactId,
    ScenarioVersionId,
    SnapshotId,
    TransactionId,
    UnknownId,
    VaultId,
    WorkspaceId,
};
pub use integrity::{InvalidSha256Hex, PayloadRef, Sha256Hex};
pub use nonnegotiable::NonnegotiableState;
pub use provenance::{Provenance, Verification};
