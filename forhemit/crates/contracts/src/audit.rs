//! The audit event contract — the versioned shape of one recorded material
//! change.
//!
//! Audit doc §63 defines the field set; the implementation spec pins the
//! Rust shape: a tagged, versioned enum where "stored events are never
//! reinterpreted in place — a new contract version is a new variant".
//! [`AuditEvent::V1`] carries the §63 fields plus the `previous_event_hash`
//! chain link that makes history tamper-evident (Audit doc §16).
//!
//! Engines never assemble a stored event: the audit store derives
//! `event_id` (ULID), `timestamp` (injected clock), and
//! `previous_event_hash` (chain head) at append time. Callers submit an
//! [`AuditEventDraft`] — the caller-supplied half — and receive the stored
//! event back.
//!
//! Strictness note: serde derive cannot combine an internal tag with
//! `deny_unknown_fields`, so unlike plain structs in this crate the tagged
//! envelope does not reject unknown keys at the derive level. The version
//! tag itself is strict — a missing or unknown `event_version` fails
//! deserialization — which is the boundary that matters for stored data:
//! older readers refuse newer envelopes instead of reinterpreting them.

use crate::actor::ActorRecord;
use crate::engine::EngineId;
use crate::ids::{CausationId, CorrelationId, EventId, ObjectId, TransactionId, WorkspaceId};
use crate::integrity::{PayloadRef, Sha256Hex};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The kind of material change an event records.
///
/// Variant names are engine-namespaced by prefix, per the spec sample
/// (`ScenarioConflictDetected` belongs to the scenario engine). New
/// variants join as additive cases as engines land; reorganizing existing
/// variants is a new [`AuditEvent`] contract version, never an edit of
/// stored values (Audit doc integrity rule 12).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// An owner decision was recorded (Audit doc §17 example event).
    OwnerDecisionRecorded,
    /// A scenario changed (Audit doc §17 example event).
    ScenarioChanged,
    /// A correction created a new event referencing the original —
    /// integrity rule 13: "Corrections create additional events rather
    /// than rewriting history".
    CorrectionRecorded,
    /// The Business Reality engine recorded a new fact or the first
    /// version of one (crate `reality`).
    RealityFactRecorded,
    /// A Business Reality fact was revised — a new fact version
    /// superseding the previous one; the previous version is retained,
    /// never edited (Business Reality doc §25).
    RealityFactRevised,
    /// A destination version was created — either the destination's first
    /// version or a new version from a material edit (Destination Engine
    /// doc §26: "Every material change creates a new version").
    DestinationVersionCreated,
    /// The owner explicitly confirmed the destination — "This is the
    /// outcome I'm trying to create" (Destination Engine doc §6).
    DestinationConfirmed,
    /// The owner kept the destination as a working draft at the
    /// destination checkpoint (Destination Builder doc §17).
    DestinationMarkedWorking,
    /// The destination was archived — retained historically but no longer
    /// active (Destination Engine doc §6).
    DestinationArchived,
    /// A document entered the local vault (Vault doc §45: "Document
    /// imported").
    DocumentImported,
    /// A new immutable document version was created (Vault doc §45:
    /// "Version created"; §14: "Versioning is mandatory").
    DocumentVersionCreated,
    /// Local vault analysis ran over a document (Vault doc §45: "Document
    /// analyzed").
    DocumentAnalyzed,
    /// A previous document version was restored — as a new version,
    /// never by rewriting history (Vault doc §45: "Document restored";
    /// §14: "Restore a previous version").
    DocumentRestored,
    /// An encrypted backup of the vault was exported (Vault doc §45:
    /// "Backup completed"; §35: "The Vault should support encrypted
    /// backups").
    BackupCompleted,
    /// A scenario family was created — the birth of one conceptual path,
    /// whether by the owner's explorer or by a what-if branch (Scenario
    /// Engine Data Model doc §3; event contract §3.1).
    ScenarioCreated,
    /// A scenario draft mutation — the draft's creation or one of its
    /// audited edits while mutable (Scenario Engine Data Model doc §46;
    /// event contract §3.2). The payload preserves previous values.
    ScenarioDraftUpdated,
    /// A draft was finalized into history — the version became immutable;
    /// further material change creates a successor version (Scenario
    /// Engine Data Model doc §46, integrity rule 6; event contract §3.3).
    ScenarioVersionFinalized,
    /// A conflict was recorded against a scenario version — including
    /// structured nonnegotiable conflicts, which carry the destination
    /// back-reference instead of a warning flag (Scenario Engine Data
    /// Model doc §16; event contract §3.4).
    ScenarioConflictDetected,
    /// A conflict's resolution was recorded — for nonnegotiable
    /// conflicts, the owner's decision (keep / explore another path /
    /// change the requirement), never an engine decision (NONNEGOTIABLE
    /// doc §5; event contract §3.5).
    ScenarioConflictResolved,
    /// A what-if branch was created from a parent scenario version; the
    /// parent is untouched (Scenario Engine Data Model doc §30–31; event
    /// contract §3.6).
    ScenarioWhatIfCreated,
    /// A scenario version was found pinned to Business Reality or
    /// destination state that has since moved; the version itself is not
    /// modified (Scenario Engine Data Model doc §35; event contract §3.7).
    ScenarioAffectedByRealityChange,
    /// A durable comparison was recorded — what was compared, never which
    /// scenario won (Scenario Engine Data Model doc §32, integrity rule
    /// 10; event contract §3.8).
    ScenarioComparisonRecorded,
    /// A professional review reference was recorded — always attributed
    /// to a role and identity; it never marks the scenario approved
    /// (Scenario Engine Data Model doc §27, §29; event contract §3.9).
    ScenarioProfessionalReviewRecorded,
    /// A finalized version's readiness status changed, with the previous
    /// and new values preserved (Scenario Engine Data Model doc §36, §40;
    /// event contract §3.10).
    ScenarioReadinessChanged,
    /// A scenario family was archived — retention, not deletion; every
    /// version and its history remain readable (Scenario Engine Data
    /// Model doc §3; event contract §3.11).
    ScenarioFamilyArchived,
    /// A Journey Instance was created — an owner started a journey
    /// definition walk (Journey Builder doc §8; event contract §4.1).
    JourneyInstanceStarted,
    /// An answer was recorded — the first version of one (Journey
    /// Builder doc §3 "Answer"; event contract §4.2). The payload carries
    /// the walk's new position and the answer's declared Storage Scope.
    JourneyAnswerRecorded,
    /// An answer was revised — a new answer version superseding the
    /// previous one; the previous version is retained, never edited
    /// (Journey Builder doc §9 "back/edit native"; event contract §4.3).
    JourneyAnswerRevised,
    /// A node was passed without an answer — the owner took a question
    /// exit on an optional node (EOJ v0.2 §30 "Question Exit"; event
    /// contract §4.4).
    JourneyNodeSkipped,
    /// The walk reached its terminal node with every eligible required
    /// question answered — the instance is complete and its outputs may
    /// be assembled (Journey Builder doc §8; event contract §4.5).
    JourneyInstanceCompleted,
}

/// The audit event contract (Audit doc §63 field set; implementation-spec
/// sample for the V1 shape).
///
/// Stored events are never reinterpreted in place: a new contract version
/// is a new variant.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "event_version", rename_all = "snake_case")]
pub enum AuditEvent {
    /// Version 1 of the audit event contract.
    V1 {
        /// Unique event identifier — a ULID, generated by the audit store.
        event_id: EventId,
        /// The kind of material change.
        event_type: AuditEventType,
        /// The engine that produced the event (integrity rule 6: "Every
        /// material event identifies its source engine").
        source_engine: EngineId,
        /// The object the event is about.
        source_object: ObjectId,
        /// Who or what acted (integrity rule 3: "Every material change
        /// identifies who or what caused it").
        actor: ActorRecord,
        /// When the event happened — UTC, from the enginekit clock.
        timestamp: OffsetDateTime,
        /// The workspace the event belongs to.
        workspace_id: WorkspaceId,
        /// The multi-event unit of work this event belongs to, if any.
        transaction_id: Option<TransactionId>,
        /// Shared identifier linking related activity (Audit doc §18).
        correlation_id: CorrelationId,
        /// The immediate event that caused this one, if any (Audit doc §19).
        causation_id: Option<CausationId>,
        /// Hash-addressed reference to the event payload — "hash-addressed,
        /// never inline bulk data".
        payload_reference: PayloadRef,
        /// Hash of the previous event in the log — the chain link behind
        /// tamper detection (Audit doc §16).
        previous_event_hash: Sha256Hex,
    },
}

/// Manual JSON Schema for the tagged envelope.
///
/// The derive cannot be used here: schemars 1.x has no `time` integration,
/// so the `timestamp` field's `OffsetDateTime` would fail the required
/// trait bound. The schema is pinned by hand instead — one `oneOf` arm per
/// contract version, with `subschema_for` keeping member references in
/// step with the generator's settings.
impl AuditEvent {
    /// The event's identifier, under any contract version.
    pub fn event_id(&self) -> &EventId {
        match self {
            Self::V1 { event_id, .. } => event_id,
        }
    }
}

impl schemars::JsonSchema for AuditEvent {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AuditEvent".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "The audit event contract — Audit doc §63 field set plus the hash-chain link. Stored events are never reinterpreted in place; a new contract version is a new variant.",
            "oneOf": [
                {
                    "type": "object",
                    "description": "Version 1 of the audit event contract.",
                    "properties": {
                        "event_version": { "const": "v1" },
                        "event_id": generator.subschema_for::<EventId>(),
                        "event_type": generator.subschema_for::<AuditEventType>(),
                        "source_engine": generator.subschema_for::<EngineId>(),
                        "source_object": generator.subschema_for::<ObjectId>(),
                        "actor": generator.subschema_for::<ActorRecord>(),
                        "timestamp": {
                            "type": "string",
                            "format": "date-time",
                            "description": "When the event happened — UTC, from the injected clock."
                        },
                        "workspace_id": generator.subschema_for::<WorkspaceId>(),
                        "transaction_id": generator.subschema_for::<Option<TransactionId>>(),
                        "correlation_id": generator.subschema_for::<CorrelationId>(),
                        "causation_id": generator.subschema_for::<Option<CausationId>>(),
                        "payload_reference": generator.subschema_for::<PayloadRef>(),
                        "previous_event_hash": generator.subschema_for::<Sha256Hex>()
                    },
                    "required": [
                        "event_version",
                        "event_id",
                        "event_type",
                        "source_engine",
                        "source_object",
                        "actor",
                        "timestamp",
                        "workspace_id",
                        "correlation_id",
                        "payload_reference",
                        "previous_event_hash"
                    ]
                }
            ]
        })
    }
}

/// The caller-supplied half of an audit event: everything an engine knows
/// before the audit store derives `event_id`, `timestamp`, and
/// `previous_event_hash` at append time.
///
/// The `payload` holds the event's structured detail — for a change event,
/// previous and new value references, per integrity rule 4 ("Material
/// changes preserve previous and new values or references"). The store
/// serializes the payload and records its SHA-256 digest as the stored
/// event's `payload_reference`; the bytes live in the store's
/// content-addressed payload table, keeping events small and history
/// verifiable.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditEventDraft {
    /// The kind of material change.
    pub event_type: AuditEventType,
    /// The engine producing the event (integrity rule 6).
    pub source_engine: EngineId,
    /// The object the event is about.
    pub source_object: ObjectId,
    /// Who or what acted.
    pub actor: ActorRecord,
    /// The workspace the event belongs to.
    pub workspace_id: WorkspaceId,
    /// The multi-event unit of work, if any.
    pub transaction_id: Option<TransactionId>,
    /// Shared identifier linking related activity (Audit doc §18).
    pub correlation_id: CorrelationId,
    /// The immediate event that caused this one, if any (Audit doc §19).
    pub causation_id: Option<CausationId>,
    /// The event's structured payload, addressed by digest once stored.
    pub payload: serde_json::Value,
}
