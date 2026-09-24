//! Typed identifier newtypes.
//!
//! The audit event contract names every object it touches through
//! identifiers (Audit doc §6: "Audit Event ID … Organization/workspace …
//! Source object … Correlation ID … Causation ID"); typed newtypes keep an
//! [`EventId`] from being passed where a [`WorkspaceId`] is expected. All
//! IDs are serde-transparent newtypes over `String`; ULID generation for
//! [`EventId`]/[`CorrelationId`] lands with the audit store task.

use serde::{Deserialize, Serialize};

/// Returned when an ID is constructed from an empty string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmptyIdError;

impl std::fmt::Display for EmptyIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier must be a non-empty string")
    }
}

impl std::error::Error for EmptyIdError {}

macro_rules! define_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(
            Clone,
            Debug,
            Deserialize,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            schemars::JsonSchema,
            Serialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a new identifier, rejecting empty values.
            pub fn new(value: impl Into<String>) -> Result<Self, EmptyIdError> {
                let value = value.into();
                if value.trim().is_empty() {
                    Err(EmptyIdError)
                } else {
                    Ok(Self(value))
                }
            }

            /// Borrows the underlying string.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(
    /// Identifies one audit event. ULID generation lands with the audit store.
    EventId
);

define_id!(
    /// Groups related activity under one identifier (Audit doc §18: "Show me
    /// everything that happened because of this change").
    CorrelationId
);

define_id!(
    /// Names the immediate event that caused this one (Audit doc §19) — the
    /// direct causal chain, distinct from correlation.
    CausationId
);

define_id!(
    /// Identifies the workspace ("Organization/workspace", Audit doc §6) an
    /// event or object belongs to.
    WorkspaceId
);

define_id!(
    /// Identifies the source object an event is about (Audit doc §6: "Source
    /// object"); version lineage per Audit doc §21.
    ObjectId
);

define_id!(
    /// Identifies a DesiredOutcome — the destination is "the first object
    /// created by every journey" (Destination First - Locked Core
    /// Architecture).
    DestinationId
);

define_id!(
    /// Identifies one immutable version in a destination's history
    /// (Destination Engine doc §26: "Every material change creates a new
    /// version… The previous version remains accessible").
    DestinationVersionId
);

define_id!(
    /// Identifies one objective within a destination. Stable across
    /// destination versions, so downstream engines can back-reference the
    /// same objective in later versions (spec: nonnegotiable constraints
    /// "expose reference IDs so later scenario conflicts can
    /// back-reference them").
    ObjectiveId
);

define_id!(
    /// Identifies the transaction (multi-event unit of work) an event
    /// belongs to — Audit doc §63 field `transaction_id`.
    TransactionId
);

define_id!(
    /// Identifies the actor behind an event — Audit doc §7: the platform
    /// never records a bare "User changed value" without saying who or
    /// what did it.
    ActorId
);

define_id!(
    /// Identifies one Business Reality fact — the stable identity that
    /// survives revisions (Business Reality doc §25: "Historical snapshots
    /// remain available" — versions change, the fact's identity does not).
    /// Scenario assumptions reference facts through this id.
    FactId
);

define_id!(
    /// Identifies one immutable fact version (Business Reality doc §25;
    /// Expanded Reality §11 "Version History"). A revision creates a new
    /// version id and never edits the version it supersedes.
    FactVersionId
);

define_id!(
    /// Identifies a local vault — Vault doc §47: the vault "contains:
    /// vault_id, owner/principal, workspace, devices, encryption state…".
    VaultId
);

define_id!(
    /// Identifies a document in the local vault (Vault doc §47
    /// "Document": "document_id … filename … hash … current_version_id").
    DocumentId
);

define_id!(
    /// Identifies one immutable version of a vault document (Vault doc §47
    /// "DocumentVersion": "version_id … supersedes_version"); Vault doc §14:
    /// "Versioning is mandatory."
    DocumentVersionId
);

define_id!(
    /// Identifies one local-analysis run over a document (Vault doc §47
    /// "LocalAnalysis": "analysis_id, inputs, model/tool…").
    AnalysisId
);

define_id!(
    /// Identifies a ScenarioFamily — one conceptual path through its entire
    /// history (Scenario Engine Data Model doc §3).
    ScenarioFamilyId
);
define_id!(
    /// Identifies one ScenarioVersion — the primary historical object
    /// (Scenario Engine Data Model doc §4). A version is immutable once
    /// finalized; a material change creates a new version id.
    ScenarioVersionId
);
define_id!(
    /// Identifies a ScenarioSnapshot — the pinned reference set ("what did
    /// we actually know when we made this scenario?", Scenario Engine Data
    /// Model doc §6, §51).
    SnapshotId
);
define_id!(
    /// Identifies one ScenarioAssumption — a typed, provenance-stamped
    /// value a scenario is built on (Scenario Engine Data Model doc §7).
    AssumptionId
);
define_id!(
    /// Identifies one ScenarioConstraint (Scenario Engine Data Model doc §12).
    ConstraintId
);
define_id!(
    /// Identifies one ScenarioUnknown — first-class missing information
    /// (Scenario Engine Data Model doc §15), never a blank zero.
    UnknownId
);
define_id!(
    /// Identifies one ScenarioConflict — severity describes the condition,
    /// never a score (Scenario Engine Data Model doc §16–17).
    ConflictId
);
define_id!(
    /// Identifies a ScenarioBranch — explicit what-if lineage whose
    /// parent scenario version is always identified (Scenario Engine Data
    /// Model doc §30, integrity rule 7).
    BranchId
);
define_id!(
    /// Identifies a durable ScenarioComparison — captures what was
    /// compared, never which scenario won (Scenario Engine Data Model doc
    /// §32, integrity rule 10).
    ComparisonId
);
define_id!(
    /// Identifies the Business Reality version a scenario was built
    /// against (Scenario Engine Data Model doc §4, integrity rule 2).
    /// Derived as a content address of the pinned fact-version set by the
    /// scenario engine, not minted: same facts → same id, any revision →
    /// a different id.
    BusinessRealityVersionId
);
define_id!(
    /// Identifies a Financial Model version (Scenario Engine Data Model
    /// doc §24, integrity rule 3: "Every material financial output
    /// references a Financial Model version"). Reserved for the financial
    /// modeling engine; carried opaquely in v1 so the reference cannot
    /// exist without a version.
    FinancialModelVersionId
);
define_id!(
    /// Identifies a professional review (Scenario Engine Data Model doc
    /// §27). Reserved for the professional review engine; carried
    /// opaquely in v1.
    ProfessionalReviewId
);
define_id!(
    /// Identifies a `ScenarioNonnegotiable` — the destination nonnegotiable
    /// a scenario version is testing (Scenario Engine Data Model doc §13).
    NonnegotiableId
);
define_id!(
    /// Identifies a comparison dimension (Scenario Engine Data Model doc
    /// §33). Display order is presentation, never ranking.
    ComparisonDimensionId
);
define_id!(
    /// Identifies one comparison result cell (Scenario Engine Data Model
    /// doc §34) — a factual reading, never a rank.
    ComparisonResultId
);
define_id!(
    /// Identifies a professional review reference row (Scenario Engine
    /// Data Model doc §27; SCHEMA.md §2.17a).
    ProfessionalReviewReferenceId
);
define_id!(
    /// Identifies a recorded upstream-change impact (Scenario Engine Data
    /// Model doc §35; SCHEMA.md §2.15).
    ScenarioImpactId
);
