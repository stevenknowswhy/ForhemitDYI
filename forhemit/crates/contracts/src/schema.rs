//! JSON Schema export for the contract vocabulary.
//!
//! Spec: the contracts crate exports JSON Schema "so a future professional
//! portal can consume the same vocabulary". `cargo run -p forhemit-contracts
//! --bin export_schemas -- <dir>` writes one self-contained draft-07 document
//! per type plus `index.json`; CI regenerates into the committed
//! `forhemit/schema/` directory and fails on drift, and
//! `tests/schema_export.rs` keeps the type list honest.

use schemars::generate::SchemaSettings;
use schemars::SchemaGenerator;

/// Every public contract type, sorted. Kept in lockstep with
/// [`export_per_type`] by `tests/schema_export.rs`.
pub const CONTRACT_TYPES: &[&str] = &[
    "ActionOrigination",
    "ActorClassification",
    "ActorId",
    "ActorKind",
    "ActorRecord",
    "AnalysisId",
    "AuditEvent",
    "AuditEventType",
    "CausationId",
    "CorrelationId",
    "DecisionLayer",
    "DestinationId",
    "DestinationVersionId",
    "DocumentId",
    "DocumentVersionId",
    "EngineId",
    "EventId",
    "FactId",
    "FactVersionId",
    "NonnegotiableState",
    "ObjectId",
    "ObjectiveId",
    "PayloadRef",
    "Provenance",
    "Sha256Hex",
    "TransactionId",
    "VaultId",
    "Verification",
    "WorkspaceId",
];

/// Exports one self-contained JSON Schema document per contract type, sorted
/// by type name.
pub fn export_per_type() -> Vec<(&'static str, String)> {
    vec![
        type_schema::<crate::ActionOrigination>("ActionOrigination"),
        type_schema::<crate::ActorClassification>("ActorClassification"),
        type_schema::<crate::ActorId>("ActorId"),
        type_schema::<crate::ActorKind>("ActorKind"),
        type_schema::<crate::ActorRecord>("ActorRecord"),
        type_schema::<crate::AnalysisId>("AnalysisId"),
        type_schema::<crate::AuditEvent>("AuditEvent"),
        type_schema::<crate::AuditEventType>("AuditEventType"),
        type_schema::<crate::CausationId>("CausationId"),
        type_schema::<crate::CorrelationId>("CorrelationId"),
        type_schema::<crate::DecisionLayer>("DecisionLayer"),
        type_schema::<crate::DestinationId>("DestinationId"),
        type_schema::<crate::DestinationVersionId>("DestinationVersionId"),
        type_schema::<crate::DocumentId>("DocumentId"),
        type_schema::<crate::DocumentVersionId>("DocumentVersionId"),
        type_schema::<crate::EngineId>("EngineId"),
        type_schema::<crate::EventId>("EventId"),
        type_schema::<crate::FactId>("FactId"),
        type_schema::<crate::FactVersionId>("FactVersionId"),
        type_schema::<crate::NonnegotiableState>("NonnegotiableState"),
        type_schema::<crate::ObjectId>("ObjectId"),
        type_schema::<crate::ObjectiveId>("ObjectiveId"),
        type_schema::<crate::PayloadRef>("PayloadRef"),
        type_schema::<crate::Provenance>("Provenance"),
        type_schema::<crate::Sha256Hex>("Sha256Hex"),
        type_schema::<crate::TransactionId>("TransactionId"),
        type_schema::<crate::VaultId>("VaultId"),
        type_schema::<crate::Verification>("Verification"),
        type_schema::<crate::WorkspaceId>("WorkspaceId"),
    ]
}

/// Exports `index.json` — the type roster with its schema dialect.
pub fn export_index() -> String {
    let index = serde_json::json!({
        "$schema": "https://json-schema.org/draft-07/schema#",
        "title": "forhemit-contracts",
        "types": CONTRACT_TYPES,
    });
    serialize(&index)
}

fn type_schema<T: schemars::JsonSchema>(name: &'static str) -> (&'static str, String) {
    let mut generator = SchemaGenerator::new(SchemaSettings::draft07());
    let schema = generator.root_schema_for::<T>();
    (name, serialize(&schema))
}

#[allow(clippy::expect_used)] // serializing in-memory schema values cannot fail; a panic is the honest failure mode
fn serialize(value: &impl serde::Serialize) -> String {
    serde_json::to_string_pretty(value).expect("schema serialization is infallible")
}
