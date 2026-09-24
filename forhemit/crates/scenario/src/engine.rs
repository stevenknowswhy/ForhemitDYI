//! The Scenario Engine: audit-first operations over families, versions,
//! and their satellites.
//!
//! Invariants every operation preserves:
//!
//! - **Audit-first** (Implementation Roadmap Phase 1): the event is
//!   emitted **before** the mutation is applied. A refused event refuses
//!   the mutation — a scenario row never exists without its event.
//! - **Immutability** (integrity rule 6): a finalized version's content
//!   is frozen. The only post-finalize transitions are the tracked
//!   readiness/lifecycle status moves (schema doc §36), which append a
//!   status-history row and emit their own event.
//! - **Winner-freedom** (integrity rule 10): no operation scores, ranks,
//!   or recommends.
//!
//! Callers always name the acting [`ActorRecord`] and the activity's
//! [`CorrelationId`] — the engine never fabricates an actor.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use forhemit_contracts::{
    ActorRecord, AssumptionId, AuditEventDraft, AuditEventType, BranchId, BusinessRealityVersionId,
    ComparisonDimensionId, ComparisonId, ComparisonResultId, ConflictId, ConstraintId,
    CorrelationId, EngineId, FactVersionId, NonnegotiableId, ObjectId,
    ProfessionalReviewReferenceId, ScenarioFamilyId, ScenarioImpactId, ScenarioVersionId, UnknownId,
    WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::assumption::{NewAssumption, NewAssumptionRevision, ScenarioAssumption};
use crate::branch::{NewWhatIf, ScenarioBranch};
use crate::comparison::{
    ComparisonDimension, ComparisonResult, NewComparison, ScenarioComparison,
};
use crate::conflict::{
    ConflictResolution, ConflictSeverity, ConflictType, NewConflict, NonnegotiableConflict,
    OwnerDecision, ScenarioConflict,
};
use crate::constraint::{NewConstraint, ScenarioConstraint};
use crate::error::ScenarioError;
use crate::family::{ArchiveFamily, NewFamily, ScenarioFamily};
use crate::nonnegotiable::{NewNonnegotiable, ScenarioNonnegotiable};
use crate::review::{NewReviewReference, ProfessionalReviewReference};
use crate::snapshot::ScenarioSnapshot;
use crate::status::{LifecycleStatus, ReadinessStatus, StatusHistoryEntry};
use crate::unknown::{NewUnknown, ScenarioUnknown, UnknownResolution, UnknownResolutionStatus};
use crate::version::{new_id, new_snapshot_id, NewDraft, NewSuccessorDraft, ScenarioVersion};

/// Derives the Business Reality version address from the pinned
/// fact-version set (SCHEMA.md §4, integrity rule 2): the sorted ids,
/// joined by newlines, SHA-256 digested. Same facts → same address; any
/// revision in the underlying facts → a different address.
pub fn business_reality_version(fact_version_ids: &[FactVersionId]) -> BusinessRealityVersionId {
    let mut ids: Vec<&str> = fact_version_ids.iter().map(|id| id.as_str()).collect();
    ids.sort_unstable();
    let mut hasher = Sha256::new();
    hasher.update(ids.join("\n"));
    let mut hex = String::with_capacity(64);
    for byte in hasher.finalize() {
        use std::fmt::Write as _;
        let _ = write!(hex, "{byte:02x}");
    }
    BusinessRealityVersionId::new(format!("brv_{hex}"))
        .unwrap_or_else(|_| unreachable!("a non-empty prefixed hex digest is always a valid id"))
}

/// What a `ScenarioDraftUpdated` event touches (SCHEMA.md §3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftSection {
    /// Assumption rows.
    Assumptions,
    /// Constraint rows.
    Constraints,
    /// Nonnegotiable-under-test rows.
    Nonnegotiables,
    /// Unknown rows.
    Unknowns,
    /// The draft's name/description.
    Metadata,
    /// The draft's creation — its first event.
    Started,
}

/// What happened to the touched section (SCHEMA.md §3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftChange {
    /// A row was added.
    Added,
    /// A row was removed.
    Removed,
    /// A row was replaced (previous value preserved in the event).
    Revised,
}

/// Payload of a draft-update event (SCHEMA.md §3.2).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftUpdatedPayload {
    /// The draft version.
    pub scenario_version_id: ScenarioVersionId,
    /// Which section the event touches.
    pub section: DraftSection,
    /// What happened.
    pub change: DraftChange,
    /// The touched row's id (the version id for `metadata`/`started`).
    pub object_id: String,
    /// The previous state of the row, where one existed (Audit
    /// integrity rule 4).
    pub previous_value: Option<Value>,
    /// The new state of the row.
    pub new_value: Value,
}

/// A conflict plus its nonnegotiable extension, when the conflict is a
/// nonnegotiable one (SCHEMA.md §2.8–2.9).
#[derive(Clone, Debug, PartialEq)]
pub struct ConflictRecord {
    /// The generic conflict row.
    pub conflict: ScenarioConflict,
    /// The structured nonnegotiable extension, when
    /// `conflict.conflict_type` is `nonnegotiable`.
    pub nonnegotiable: Option<NonnegotiableConflict>,
}

/// A recorded upstream-change impact (SCHEMA.md §2.15): the scenario
/// version is not modified — the impact is surfaced for the owner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioImpact {
    /// The impact row's id.
    pub impact_id: ScenarioImpactId,
    /// The affected scenario version.
    pub scenario_version_id: ScenarioVersionId,
    /// What moved upstream (e.g. "business_reality", "destination").
    pub source_change_type: String,
    /// The moving object's reference.
    pub source_change_id: String,
    /// Which part of the scenario the change touches.
    pub impact_type: ImpactType,
    /// What the change means, in plain language.
    pub impact_description: String,
    /// The condition's weight, in words.
    pub severity: ConflictSeverity,
    /// When the impact was detected.
    pub detected_at: OffsetDateTime,
    /// Open until the owner acknowledges it.
    pub status: ImpactStatus,
}

/// Which part of the scenario an upstream change touches (schema doc
/// §35; v1 detects the reality-facing subset).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactType {
    /// The version tests nonnegotiables against the old reality.
    NonnegotiableAffected,
    /// The version's assumptions draw on the old reality.
    AssumptionAffected,
    /// The version's named unknowns draw on the old reality.
    UnknownAffected,
}

/// Whether the owner has seen the impact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactStatus {
    /// Detected, not yet acknowledged.
    Open,
    /// The owner acknowledged it.
    Acknowledged,
}

/// What `ScenarioEngine::create_what_if` produced: the branch row, the
/// child family it created, and the child family's first version.
#[derive(Clone, Debug)]
pub struct WhatIfCreated {
    /// The branch row — parent always identified (integrity rule 7).
    pub branch: ScenarioBranch,
    /// The child family, mirroring `branched_from_version_id`.
    pub family: ScenarioFamily,
    /// The child family's first version, seeded from the parent.
    pub version: ScenarioVersion,
}

/// The decision that closes a conflict: the owner's nonnegotiable
/// decision, or a generic resolution with a reference.
#[derive(Clone, Debug)]
pub enum ConflictDecision {
    /// A nonnegotiable conflict's owner decision (NONNEGOTIABLE doc §5).
    Owner(OwnerDecision),
    /// A generic conflict's closing resolution.
    Resolved {
        /// Where the resolution lives, if it lives anywhere.
        resolution_reference: Option<String>,
    },
}

/// The engine's in-memory tables (SCHEMA.md §2). The audit stream is the
/// durable record; SQLite mapping is the app shell's composition
/// concern, exactly like the reality and destination crates.
#[derive(Default)]
struct ScenarioTable {
    families: HashMap<ScenarioFamilyId, ScenarioFamily>,
    versions: HashMap<ScenarioVersionId, ScenarioVersion>,
    conflicts: HashMap<ConflictId, ConflictRecord>,
    review_references: Vec<ProfessionalReviewReference>,
    branches: Vec<ScenarioBranch>,
    comparisons: HashMap<ComparisonId, ScenarioComparison>,
    impacts: Vec<ScenarioImpact>,
}

/// The Scenario Engine for one workspace.
///
/// Cheap to clone: all handles share one table and one sink. The engine
/// owns no persistence — durable storage is the app shell's composition
/// concern; in v1 the engine is the domain model and the audit stream is
/// the durable record.
pub struct ScenarioEngine<S: AuditSink<AuditEventDraft>> {
    sink: Arc<S>,
    clock: Arc<dyn Clock>,
    workspace_id: WorkspaceId,
    table: Arc<Mutex<ScenarioTable>>,
}

impl<S: AuditSink<AuditEventDraft>> Clone for ScenarioEngine<S> {
    fn clone(&self) -> Self {
        Self {
            sink: self.sink.clone(),
            clock: self.clock.clone(),
            workspace_id: self.workspace_id.clone(),
            table: self.table.clone(),
        }
    }
}

impl<S: AuditSink<AuditEventDraft>> ScenarioEngine<S> {
    /// Builds an engine for `workspace_id`, emitting through `sink` and
    /// stamping times from `clock`.
    pub fn new(sink: Arc<S>, clock: Arc<dyn Clock>, workspace_id: WorkspaceId) -> Self {
        Self {
            sink,
            clock,
            workspace_id,
            table: Arc::new(Mutex::new(ScenarioTable::default())),
        }
    }

    // ------------------------------------------------------------------
    // Families
    // ------------------------------------------------------------------

    /// Creates a scenario family — the birth of one conceptual path.
    ///
    /// # Errors
    /// An empty name, or an audit-store rejection — in every failure
    /// case nothing is created and no event is emitted.
    pub fn create_family(&self, request: NewFamily) -> Result<ScenarioFamily, ScenarioError> {
        let name = clean_text(&request.name, "name")?;
        let family = ScenarioFamily {
            scenario_family_id: new_id("fam", ScenarioFamilyId::new),
            workspace_id: self.workspace_id.clone(),
            name,
            scenario_type: request.scenario_type,
            branched_from_version_id: None,
            created_at: self.clock.now(),
            created_by: request.actor.clone(),
            archived_at: None,
            archive_reason: None,
        };

        self.emit(
            AuditEventType::ScenarioCreated,
            object_id(family.scenario_family_id.as_str()),
            json!({
                "scenario_family_id": family.scenario_family_id,
                "name": family.name,
                "scenario_type": family.scenario_type,
                "branched_from_version_id": family.branched_from_version_id,
                "branch_id": Option::<BranchId>::None,
            }),
            request.actor,
            request.correlation_id,
        )?;

        self.lock_table()?
            .families
            .insert(family.scenario_family_id.clone(), family.clone());
        Ok(family)
    }

    /// Archives a family — retention, not deletion: every version and
    /// its history remain readable.
    ///
    /// # Errors
    /// Unknown family, an already-archived family, an empty reason, or
    /// an audit-store rejection.
    pub fn archive_family(&self, request: ArchiveFamily) -> Result<ScenarioFamily, ScenarioError> {
        let reason = clean_text(&request.archive_reason, "archive_reason")?;
        let mut table = self.lock_table()?;
        let family = table
            .families
            .get_mut(&request.scenario_family_id)
            .ok_or_else(|| ScenarioError::FamilyNotFound {
                scenario_family_id: request.scenario_family_id.clone(),
            })?;
        if family.archived_at.is_some() {
            return Err(ScenarioError::FamilyAlreadyArchived {
                scenario_family_id: family.scenario_family_id.clone(),
            });
        }
        let archived_at = self.clock.now();
        self.emit(
            AuditEventType::ScenarioFamilyArchived,
            object_id(family.scenario_family_id.as_str()),
            json!({
                "scenario_family_id": family.scenario_family_id,
                "archive_reason": reason,
            }),
            request.actor.clone(),
            request.correlation_id,
        )?;
        family.archived_at = Some(archived_at);
        family.archive_reason = Some(reason);
        Ok(family.clone())
    }

    // ------------------------------------------------------------------
    // Drafts
    // ------------------------------------------------------------------

    /// Starts a draft — the version row exists from this moment, pinned
    /// to one destination version (integrity rule 1) and the Business
    /// Reality version address of the pinned fact set (integrity rule 2).
    ///
    /// # Errors
    /// Unknown family, an empty name, or an audit-store rejection.
    pub fn start_draft(&self, request: NewDraft) -> Result<ScenarioVersion, ScenarioError> {
        let name = clean_text(&request.name, "name")?;
        if let Some(description) = request.description.as_deref() {
            if description.trim().is_empty() {
                return Err(ScenarioError::EmptyText {
                    field: "description",
                });
            }
        }
        let table = self.lock_table()?;
        let family = table
            .families
            .get(&request.scenario_family_id)
            .ok_or_else(|| ScenarioError::FamilyNotFound {
                scenario_family_id: request.scenario_family_id.clone(),
            })?
            .clone();
        let version_count = table
            .versions
            .values()
            .filter(|v| v.scenario_family_id == family.scenario_family_id)
            .count();
        drop(table);

        let scenario_version_id = new_id("sv", ScenarioVersionId::new);
        let created_at = self.clock.now();
        let reality_version = business_reality_version(&request.reality_fact_version_ids);
        let snapshot = ScenarioSnapshot {
            snapshot_id: new_snapshot_id(),
            scenario_version_id: scenario_version_id.clone(),
            destination_version_id: request.destination_version_id.clone(),
            business_reality_version_id: reality_version.clone(),
            reality_fact_version_ids: request.reality_fact_version_ids.clone(),
            financial_model_version_id: request.financial_model_version_id.clone(),
            research_snapshot_id: None,
            evidence_snapshot_id: None,
            financing_reference_set: None,
            seller_note_reference_set: None,
            professional_review_reference_set: None,
            created_at,
        };
        let version = ScenarioVersion {
            scenario_version_id: scenario_version_id.clone(),
            scenario_family_id: family.scenario_family_id.clone(),
            scenario_type: family.scenario_type,
            version_number: u32::try_from(version_count + 1)
                .map_err(|_| internal("version count overflows u32"))?,
            parent_version_id: None,
            supersedes_version_id: None,
            name,
            description: request.description.clone(),
            change_reason: None,
            lifecycle_status: LifecycleStatus::Draft,
            readiness_status: ReadinessStatus::Preliminary,
            snapshot,
            destination_version_id: request.destination_version_id.clone(),
            business_reality_version_id: reality_version.clone(),
            assumptions: Vec::new(),
            constraints: Vec::new(),
            nonnegotiables: Vec::new(),
            unknowns: Vec::new(),
            status_history: Vec::new(),
            finalized_at: None,
            created_at,
            created_by: request.actor.clone(),
        };

        self.emit(
            AuditEventType::ScenarioDraftUpdated,
            object_id(version.scenario_version_id.as_str()),
            draft_started_payload(&version),
            request.actor,
            request.correlation_id,
        )?;

        self.lock_table()?
            .versions
            .insert(scenario_version_id.clone(), version.clone());
        Ok(version)
    }

    /// Starts a successor draft from a finalized version (schema doc
    /// §47): the material-change path that keeps history immutable.
    /// Content is seeded from the parent with fresh row ids; the
    /// successor is pinned to the destination version the caller hands
    /// in — after a nonnegotiable change, that is the destination
    /// engine's new version (integrity rule 9's flow).
    ///
    /// # Errors
    /// Unknown parent, a parent still in the draft window, an empty
    /// change reason, or an audit-store rejection.
    pub fn start_successor_draft(
        &self,
        request: NewSuccessorDraft,
    ) -> Result<ScenarioVersion, ScenarioError> {
        let change_reason = clean_text(&request.change_reason, "change_reason")?;
        let table = self.lock_table()?;
        let parent = table
            .versions
            .get(&request.parent_version_id)
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: request.parent_version_id.clone(),
            })?
            .clone();
        drop(table);
        if parent.is_draft() {
            return Err(ScenarioError::ParentStillMutable {
                scenario_version_id: parent.scenario_version_id.clone(),
            });
        }
        let name = match request.name_override.as_deref() {
            Some(name) => clean_text(name, "name")?,
            None => parent.name.clone(),
        };
        let description = if request.name_override.is_none() {
            parent.description.clone()
        } else {
            None
        };
        let successor_id = new_id("sv", ScenarioVersionId::new);
        let created_at = self.clock.now();
        let reality_version = business_reality_version(&request.reality_fact_version_ids);
        let snapshot = ScenarioSnapshot {
            snapshot_id: new_snapshot_id(),
            scenario_version_id: successor_id.clone(),
            destination_version_id: request.destination_version_id.clone(),
            business_reality_version_id: reality_version.clone(),
            reality_fact_version_ids: request.reality_fact_version_ids.clone(),
            financial_model_version_id: parent.snapshot.financial_model_version_id.clone(),
            research_snapshot_id: None,
            evidence_snapshot_id: None,
            financing_reference_set: None,
            seller_note_reference_set: None,
            professional_review_reference_set: None,
            created_at,
        };
        let version = ScenarioVersion {
            scenario_version_id: successor_id.clone(),
            scenario_family_id: parent.scenario_family_id.clone(),
            scenario_type: parent.scenario_type,
            version_number: parent.version_number + 1,
            parent_version_id: Some(parent.scenario_version_id.clone()),
            supersedes_version_id: Some(parent.scenario_version_id.clone()),
            name,
            description,
            change_reason: Some(change_reason.clone()),
            lifecycle_status: LifecycleStatus::Draft,
            readiness_status: ReadinessStatus::Preliminary,
            snapshot,
            destination_version_id: request.destination_version_id.clone(),
            business_reality_version_id: reality_version.clone(),
            assumptions: seed_assumptions(&parent.assumptions, &successor_id, created_at),
            constraints: seed_constraints(&parent.constraints, &successor_id, created_at),
            nonnegotiables: seed_nonnegotiables(&parent.nonnegotiables, &successor_id, created_at),
            unknowns: seed_unknowns(&parent.unknowns, &successor_id, created_at),
            status_history: Vec::new(),
            finalized_at: None,
            created_at,
            created_by: request.actor.clone(),
        };

        self.emit(
            AuditEventType::ScenarioDraftUpdated,
            object_id(version.scenario_version_id.as_str()),
            json!({
                "scenario_version_id": version.scenario_version_id,
                "section": DraftSection::Started,
                "change": DraftChange::Added,
                "object_id": version.scenario_version_id,
                "previous_value": Value::Null,
                "new_value": {
                    "name": version.name,
                    "scenario_type": version.scenario_type,
                    "destination_version_id": version.destination_version_id,
                    "business_reality_version_id": version.business_reality_version_id,
                    "supersedes_version_id": version.supersedes_version_id,
                    "change_reason": version.change_reason,
                },
            }),
            request.actor,
            request.correlation_id,
        )?;

        self.lock_table()?
            .versions
            .insert(successor_id.clone(), version.clone());
        Ok(version)
    }

    /// Finalizes a draft: the transition that makes the version
    /// historical (integrity rule 6). From here the version's content is
    /// frozen; further material change creates a successor.
    ///
    /// # Errors
    /// Unknown version, a version not in the draft window, or an
    /// audit-store rejection.
    pub fn finalize_version(
        &self,
        scenario_version_id: ScenarioVersionId,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioVersion, ScenarioError> {
        let mut table = self.lock_table()?;
        let version = table
            .versions
            .get_mut(&scenario_version_id)
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: scenario_version_id.clone(),
            })?;
        if !version.is_draft() {
            return Err(ScenarioError::NotADraft {
                scenario_version_id: version.scenario_version_id.clone(),
                lifecycle_status: version.lifecycle_status,
            });
        }
        let finalized_at = self.clock.now();
        self.emit(
            AuditEventType::ScenarioVersionFinalized,
            object_id(version.scenario_version_id.as_str()),
            json!({
                "scenario_version_id": version.scenario_version_id,
                "scenario_family_id": version.scenario_family_id,
                "version_number": version.version_number,
                "lifecycle_status": LifecycleStatus::Preliminary,
                "readiness_status": ReadinessStatus::Preliminary,
                "snapshot_id": version.snapshot.snapshot_id,
                "destination_version_id": version.destination_version_id,
                "business_reality_version_id": version.business_reality_version_id,
                "assumption_count": version.assumptions.len(),
                "unknown_count": version.unknowns.len(),
            }),
            actor.clone(),
            correlation_id,
        )?;
        version.status_history.push(StatusHistoryEntry {
            previous_lifecycle: Some(version.lifecycle_status),
            new_lifecycle: LifecycleStatus::Preliminary,
            previous_readiness: Some(version.readiness_status),
            new_readiness: ReadinessStatus::Preliminary,
            reason: None,
            changed_by: actor,
            changed_at: finalized_at,
        });
        version.lifecycle_status = LifecycleStatus::Preliminary;
        version.readiness_status = ReadinessStatus::Preliminary;
        version.finalized_at = Some(finalized_at);
        Ok(version.clone())
    }

    /// Moves a finalized version along the readiness axis (schema doc
    /// §40) — the package-export gate. Never touches the lifecycle axis
    /// or any content; the transition is recorded in status history
    /// (schema doc §36) and audited.
    ///
    /// # Errors
    /// Unknown version, a version still in the draft window, an
    /// unchanged readiness, or an audit-store rejection.
    pub fn set_readiness(
        &self,
        scenario_version_id: ScenarioVersionId,
        new_readiness: ReadinessStatus,
        reason: Option<String>,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioVersion, ScenarioError> {
        let mut table = self.lock_table()?;
        let version = table
            .versions
            .get_mut(&scenario_version_id)
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: scenario_version_id.clone(),
            })?;
        if version.is_draft() {
            return Err(ScenarioError::DraftNotReadinessGated {
                scenario_version_id: version.scenario_version_id.clone(),
            });
        }
        if version.readiness_status == new_readiness {
            return Err(ScenarioError::ReadinessUnchanged {
                scenario_version_id: version.scenario_version_id.clone(),
                readiness: new_readiness,
            });
        }
        self.emit(
            AuditEventType::ScenarioReadinessChanged,
            object_id(version.scenario_version_id.as_str()),
            json!({
                "scenario_version_id": version.scenario_version_id,
                "previous_readiness": version.readiness_status,
                "new_readiness": new_readiness,
                "reason": reason,
            }),
            actor.clone(),
            correlation_id,
        )?;
        version.status_history.push(StatusHistoryEntry {
            previous_lifecycle: None,
            new_lifecycle: version.lifecycle_status,
            previous_readiness: Some(version.readiness_status),
            new_readiness,
            reason,
            changed_by: actor,
            changed_at: self.clock.now(),
        });
        version.readiness_status = new_readiness;
        Ok(version.clone())
    }

    // ------------------------------------------------------------------
    // Draft-scope content edits
    // ------------------------------------------------------------------

    /// Adds an assumption to a draft.
    ///
    /// # Errors
    /// Unknown version, a finalized (immutable) version, an invalid
    /// value or empty name, or an audit-store rejection.
    pub fn add_assumption(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewAssumption,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioAssumption, ScenarioError> {
        let name = clean_text(&request.name, "assumption name")?;
        request.value.validate()?;
        let assumption = ScenarioAssumption {
            assumption_id: new_id("asm", AssumptionId::new),
            scenario_version_id: scenario_version_id.clone(),
            category: request.category,
            name,
            description: request.description,
            value: request.value,
            provenance: request.provenance,
            verification: request.verification,
            nonnegotiable_objective_id: request.nonnegotiable_objective_id,
            source_reference: request.source_reference,
            created_at: self.clock.now(),
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Assumptions,
            DraftChange::Added,
            assumption.assumption_id.as_str(),
            None,
            &assumption,
            actor,
            correlation_id,
            |version| version.assumptions.push(assumption.clone()),
        )?;
        Ok(assumption)
    }

    /// Revises an assumption on a draft — the previous value is
    /// preserved in the audit event (Audit integrity rule 4).
    ///
    /// # Errors
    /// Unknown version or assumption, a finalized version, an invalid
    /// value, or an audit-store rejection.
    pub fn revise_assumption(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewAssumptionRevision,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioAssumption, ScenarioError> {
        clean_text(&request.change_reason, "change_reason")?;
        request.value.validate()?;
        let previous = {
            let table = self.lock_table()?;
            let version = self.version_in(&table, &scenario_version_id)?;
            version
                .assumptions
                .iter()
                .find(|a| a.assumption_id == request.assumption_id)
                .cloned()
                .ok_or_else(|| ScenarioError::AssumptionNotFound {
                    assumption_id: request.assumption_id.clone(),
                })?
        };
        let revised = ScenarioAssumption {
            value: request.value,
            provenance: request.provenance,
            verification: request.verification,
            ..previous.clone()
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Assumptions,
            DraftChange::Revised,
            revised.assumption_id.as_str(),
            Some(&previous),
            &revised,
            actor,
            correlation_id,
            |version| {
                if let Some(slot) = version
                    .assumptions
                    .iter_mut()
                    .find(|a| a.assumption_id == request.assumption_id)
                {
                    *slot = revised.clone();
                }
            },
        )?;
        Ok(revised)
    }

    /// Removes an assumption from a draft — recorded, not erased: the
    /// event preserves the removed row (Audit integrity rule 4).
    ///
    /// # Errors
    /// Unknown version or assumption, a finalized version, or an
    /// audit-store rejection.
    pub fn remove_assumption(
        &self,
        scenario_version_id: ScenarioVersionId,
        assumption_id: AssumptionId,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioAssumption, ScenarioError> {
        let previous = {
            let table = self.lock_table()?;
            let version = self.version_in(&table, &scenario_version_id)?;
            version
                .assumptions
                .iter()
                .find(|a| a.assumption_id == assumption_id)
                .cloned()
                .ok_or_else(|| ScenarioError::AssumptionNotFound {
                    assumption_id: assumption_id.clone(),
                })?
        };
        let previous_json =
            serde_json::to_value(&previous).map_err(ScenarioError::Serialization)?;
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Assumptions,
            DraftChange::Removed,
            assumption_id.as_str(),
            Some(&previous_json),
            &Value::Null,
            actor,
            correlation_id,
            |version| {
                version
                    .assumptions
                    .retain(|a| a.assumption_id != assumption_id)
            },
        )?;
        Ok(previous)
    }

    /// Adds a constraint to a draft.
    ///
    /// # Errors
    /// Unknown version, a finalized version, an empty name, or an
    /// audit-store rejection.
    pub fn add_constraint(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewConstraint,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioConstraint, ScenarioError> {
        let name = clean_text(&request.name, "constraint name")?;
        if let Some(value) = &request.value {
            value.validate()?;
        }
        let constraint = ScenarioConstraint {
            constraint_id: new_id("con", ConstraintId::new),
            scenario_version_id: scenario_version_id.clone(),
            constraint_type: request.constraint_type,
            name,
            description: request.description,
            value: request.value,
            source_reference: request.source_reference,
            created_at: self.clock.now(),
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Constraints,
            DraftChange::Added,
            constraint.constraint_id.as_str(),
            None,
            &constraint,
            actor,
            correlation_id,
            |version| version.constraints.push(constraint.clone()),
        )?;
        Ok(constraint)
    }

    /// Adds a destination nonnegotiable under test to a draft. The row
    /// must test the nonnegotiables of the destination version the
    /// scenario version pins — a scenario never silently mixes
    /// destination generations.
    ///
    /// # Errors
    /// Unknown version, a finalized version, a destination-version
    /// mismatch, an empty description, or an audit-store rejection.
    pub fn add_nonnegotiable(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewNonnegotiable,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioNonnegotiable, ScenarioError> {
        let description = clean_text(&request.description, "nonnegotiable description")?;
        if let Some(value) = &request.value {
            value.validate()?;
        }
        let pinned = self
            .version(&scenario_version_id)?
            .destination_version_id
            .clone();
        if pinned != request.destination_version_id {
            return Err(ScenarioError::DestinationVersionMismatch {
                pinned,
                supplied: request.destination_version_id,
            });
        }
        let row = ScenarioNonnegotiable {
            nonnegotiable_id: new_id("nn", NonnegotiableId::new),
            scenario_version_id: scenario_version_id.clone(),
            destination_objective_id: request.destination_objective_id,
            destination_version_id: request.destination_version_id,
            description,
            value: request.value,
            created_at: self.clock.now(),
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Nonnegotiables,
            DraftChange::Added,
            row.nonnegotiable_id.as_str(),
            None,
            &row,
            actor,
            correlation_id,
            |version| version.nonnegotiables.push(row.clone()),
        )?;
        Ok(row)
    }

    /// Adds a first-class unknown to a draft — missing information is
    /// named, never a blank zero.
    ///
    /// # Errors
    /// Unknown version, a finalized version, an empty description, or an
    /// audit-store rejection.
    pub fn add_unknown(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewUnknown,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioUnknown, ScenarioError> {
        let description = clean_text(&request.description, "unknown description")?;
        if let Some(category) = request.category.as_deref() {
            if category.trim().is_empty() {
                return Err(ScenarioError::EmptyText { field: "category" });
            }
        }
        let unknown = ScenarioUnknown {
            unknown_id: new_id("unk", UnknownId::new),
            scenario_version_id: scenario_version_id.clone(),
            category: request.category,
            description,
            importance: request.importance,
            resolution_status: UnknownResolutionStatus::Open,
            required_action: request.required_action,
            source_dependency: request.source_dependency,
            resolved_at: None,
            resolved_by: None,
            resolution_reference: None,
            created_at: self.clock.now(),
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Unknowns,
            DraftChange::Added,
            unknown.unknown_id.as_str(),
            None,
            &unknown,
            actor,
            correlation_id,
            |version| version.unknowns.push(unknown.clone()),
        )?;
        Ok(unknown)
    }

    /// Resolves or waives an unknown on a draft.
    ///
    /// # Errors
    /// Unknown version or unknown, a finalized version, a resolution
    /// status that is not terminal (`resolved`/`waived`), or an
    /// audit-store rejection.
    pub fn resolve_unknown(
        &self,
        scenario_version_id: ScenarioVersionId,
        resolution: UnknownResolution,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioUnknown, ScenarioError> {
        if !matches!(
            resolution.status,
            UnknownResolutionStatus::Resolved | UnknownResolutionStatus::Waived
        ) {
            return Err(ScenarioError::Internal(
                "resolve_unknown only records terminal statuses (resolved/waived)".to_owned(),
            ));
        }
        let previous = {
            let table = self.lock_table()?;
            let version = self.version_in(&table, &scenario_version_id)?;
            version
                .unknowns
                .iter()
                .find(|u| u.unknown_id == resolution.unknown_id)
                .cloned()
                .ok_or_else(|| ScenarioError::UnknownNotFound {
                    unknown_id: resolution.unknown_id.clone(),
                })?
        };
        let resolved = ScenarioUnknown {
            resolution_status: resolution.status,
            resolved_at: Some(self.clock.now()),
            resolved_by: Some(actor.clone()),
            resolution_reference: resolution.resolution_reference.clone(),
            ..previous.clone()
        };
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Unknowns,
            DraftChange::Revised,
            resolved.unknown_id.as_str(),
            Some(&previous),
            &resolved,
            actor,
            correlation_id,
            |version| {
                if let Some(slot) = version
                    .unknowns
                    .iter_mut()
                    .find(|u| u.unknown_id == resolution.unknown_id)
                {
                    *slot = resolved.clone();
                }
            },
        )?;
        Ok(resolved)
    }

    /// Renames a draft (metadata edit).
    ///
    /// # Errors
    /// Unknown version, a finalized version, an empty name, or an
    /// audit-store rejection.
    pub fn update_metadata(
        &self,
        scenario_version_id: ScenarioVersionId,
        name: String,
        description: Option<String>,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioVersion, ScenarioError> {
        let name = clean_text(&name, "name")?;
        let previous = self.version(&scenario_version_id)?;
        self.apply_draft_edit(
            &scenario_version_id,
            DraftSection::Metadata,
            DraftChange::Revised,
            scenario_version_id.as_str(),
            Some(&json!({
                "name": previous.name,
                "description": previous.description,
            })),
            &json!({ "name": name, "description": description }),
            actor,
            correlation_id,
            |version| {
                version.name = name.clone();
                version.description = description.clone();
            },
        )?;
        self.version(&scenario_version_id)
    }

    // ------------------------------------------------------------------
    // Conflicts
    // ------------------------------------------------------------------

    /// Records a conflict against a version — drafts and finalized
    /// versions alike (integrity rule 8: conflicts are their own rows,
    /// not version content).
    ///
    /// # Errors
    /// Unknown version, an empty description, or an audit-store
    /// rejection.
    pub fn record_conflict(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewConflict,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioConflict, ScenarioError> {
        let description = clean_text(&request.description, "conflict description")?;
        self.version(&scenario_version_id)?;
        let conflict = ScenarioConflict {
            conflict_id: new_id("cnf", ConflictId::new),
            scenario_version_id: scenario_version_id.clone(),
            conflict_type: request.conflict_type,
            severity: request.severity,
            description,
            source_reference: request.source_reference,
            affected_object_type: request.affected_object_type,
            affected_object_id: request.affected_object_id,
            resolution: None,
            created_at: self.clock.now(),
        };
        self.emit(
            AuditEventType::ScenarioConflictDetected,
            object_id(conflict.conflict_id.as_str()),
            json!({
                "conflict_id": conflict.conflict_id,
                "scenario_version_id": conflict.scenario_version_id,
                "conflict_type": conflict.conflict_type,
                "severity": conflict.severity,
                "description": conflict.description,
                "nonnegotiable": Value::Null,
            }),
            actor,
            correlation_id,
        )?;
        self.lock_table()?.conflicts.insert(
            conflict.conflict_id.clone(),
            ConflictRecord {
                conflict: conflict.clone(),
                nonnegotiable: None,
            },
        );
        Ok(conflict)
    }

    /// Records a **nonnegotiable** conflict: the structured object with
    /// the destination back-reference (schema doc §14), surfaced for the
    /// owner to decide — never filtered, never scored, never
    /// auto-relaxed (NONNEGOTIABLE doc §5).
    ///
    /// # Errors
    /// Unknown version or nonnegotiable row, an empty description, or an
    /// audit-store rejection.
    pub fn record_nonnegotiable_conflict(
        &self,
        scenario_version_id: ScenarioVersionId,
        nonnegotiable_id: NonnegotiableId,
        severity: ConflictSeverity,
        description: String,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ConflictRecord, ScenarioError> {
        let description = clean_text(&description, "conflict description")?;
        let pinned = self.version(&scenario_version_id)?;
        let row = pinned
            .nonnegotiables
            .iter()
            .find(|nn| nn.nonnegotiable_id == nonnegotiable_id)
            .ok_or_else(|| ScenarioError::NonnegotiableNotFound {
                nonnegotiable_id: nonnegotiable_id.clone(),
            })?
            .clone();
        let conflict_id = new_id("cnf", ConflictId::new);
        let conflict = ScenarioConflict {
            conflict_id: conflict_id.clone(),
            scenario_version_id: scenario_version_id.clone(),
            conflict_type: ConflictType::Nonnegotiable,
            severity,
            description,
            source_reference: None,
            affected_object_type: Some("nonnegotiable".to_owned()),
            affected_object_id: Some(row.nonnegotiable_id.as_str().to_owned()),
            resolution: None,
            created_at: self.clock.now(),
        };
        let extension = NonnegotiableConflict {
            conflict_id: conflict_id.clone(),
            scenario_version_id: scenario_version_id.clone(),
            nonnegotiable_id: row.nonnegotiable_id.clone(),
            destination_objective_id: row.destination_objective_id.clone(),
            destination_version_id: row.destination_version_id.clone(),
            owner_decision: None,
            decided_at: None,
            created_at: conflict.created_at,
        };
        self.emit(
            AuditEventType::ScenarioConflictDetected,
            object_id(conflict.conflict_id.as_str()),
            json!({
                "conflict_id": conflict.conflict_id,
                "scenario_version_id": conflict.scenario_version_id,
                "conflict_type": conflict.conflict_type,
                "severity": conflict.severity,
                "description": conflict.description,
                "nonnegotiable": {
                    "nonnegotiable_id": extension.nonnegotiable_id,
                    "destination_objective_id": extension.destination_objective_id,
                    "destination_version_id": extension.destination_version_id,
                },
            }),
            actor,
            correlation_id,
        )?;
        let record = ConflictRecord {
            conflict: conflict.clone(),
            nonnegotiable: Some(extension),
        };
        self.lock_table()?
            .conflicts
            .insert(conflict_id.clone(), record.clone());
        Ok(record)
    }

    /// Records a resolution on a conflict — for nonnegotiable conflicts
    /// this is the **owner's** decision (NONNEGOTIABLE doc §5); the
    /// engine supplies the timestamp and the event, never the decision.
    ///
    /// # Errors
    /// Unknown conflict, a conflict already resolved, or an audit-store
    /// rejection.
    pub fn resolve_conflict(
        &self,
        conflict_id: ConflictId,
        decision: ConflictDecision,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ConflictRecord, ScenarioError> {
        let mut table = self.lock_table()?;
        let record = table
            .conflicts
            .get_mut(&conflict_id)
            .ok_or_else(|| ScenarioError::ConflictNotFound {
                conflict_id: conflict_id.clone(),
            })?;
        if record.conflict.resolution.is_some() {
            return Err(ScenarioError::ConflictAlreadyResolved {
                conflict_id: record.conflict.conflict_id.clone(),
            });
        }
        let decided_at = self.clock.now();
        let (owner_decision, resolution_reference) = match decision {
            ConflictDecision::Owner(decision) => (Some(decision), None),
            ConflictDecision::Resolved {
                resolution_reference,
            } => (None, resolution_reference),
        };
        self.emit(
            AuditEventType::ScenarioConflictResolved,
            object_id(record.conflict.conflict_id.as_str()),
            json!({
                "conflict_id": record.conflict.conflict_id,
                "scenario_version_id": record.conflict.scenario_version_id,
                "owner_decision": owner_decision,
                "decided_by": actor,
            }),
            actor.clone(),
            correlation_id,
        )?;
        record.conflict.resolution = Some(ConflictResolution {
            decided_by: actor,
            decided_at,
            resolution_reference: resolution_reference.clone(),
        });
        if let Some(extension) = &mut record.nonnegotiable {
            extension.owner_decision = owner_decision;
            extension.decided_at = Some(decided_at);
        }
        Ok(record.clone())
    }

    // ------------------------------------------------------------------
    // Professional review references
    // ------------------------------------------------------------------

    /// Records an attributed professional review reference (integrity
    /// rule 5): role and identity are required — the type makes an
    /// unattributed reference unconstructible. Never marks the scenario
    /// approved (schema doc §29).
    ///
    /// # Errors
    /// Unknown version, an empty role/identity, or an audit-store
    /// rejection.
    pub fn record_professional_review_reference(
        &self,
        scenario_version_id: ScenarioVersionId,
        request: NewReviewReference,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ProfessionalReviewReference, ScenarioError> {
        let professional_role = clean_text(&request.professional_role, "professional role")?;
        let identity = clean_text(
            &request.professional_identity_reference,
            "professional identity reference",
        )?;
        self.version(&scenario_version_id)?;
        let reference = ProfessionalReviewReference {
            reference_id: new_id("ref", ProfessionalReviewReferenceId::new),
            scenario_version_id: scenario_version_id.clone(),
            professional_review_id: request.professional_review_id,
            review_status: request.review_status,
            professional_role,
            professional_identity_reference: identity,
            feedback_reference: request.feedback_reference,
            created_at: self.clock.now(),
        };
        self.emit(
            AuditEventType::ScenarioProfessionalReviewRecorded,
            object_id(reference.scenario_version_id.as_str()),
            json!({
                "reference_id": reference.reference_id,
                "scenario_version_id": reference.scenario_version_id,
                "professional_review_id": reference.professional_review_id,
                "professional_role": reference.professional_role,
                "professional_identity_reference": reference.professional_identity_reference,
                "review_status": reference.review_status,
            }),
            actor,
            correlation_id,
        )?;
        self.lock_table()?
            .review_references
            .push(reference.clone());
        Ok(reference)
    }

    // ------------------------------------------------------------------
    // Branches
    // ------------------------------------------------------------------

    /// Creates a what-if branch (schema doc §30–31): a new family whose
    /// first version is seeded from the parent. The parent version is
    /// never modified, and the branch row always identifies the parent
    /// (integrity rule 7).
    ///
    /// # Errors
    /// Unknown parent, an empty name/reason, or an audit-store
    /// rejection.
    pub fn create_what_if(
        &self,
        request: NewWhatIf,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<WhatIfCreated, ScenarioError> {
        let name = clean_text(&request.name, "family name")?;
        let reason = clean_text(&request.reason, "branch reason")?;
        let parent = self.version(&request.parent_scenario_version_id)?.clone();

        let branch_id = new_id("brn", BranchId::new);
        let family_id = new_id("fam", ScenarioFamilyId::new);
        let child_version_id = new_id("sv", ScenarioVersionId::new);
        let now = self.clock.now();

        let family = ScenarioFamily {
            scenario_family_id: family_id.clone(),
            workspace_id: self.workspace_id.clone(),
            name,
            scenario_type: parent.scenario_type,
            branched_from_version_id: Some(parent.scenario_version_id.clone()),
            created_at: now,
            created_by: actor.clone(),
            archived_at: None,
            archive_reason: None,
        };
        let branch = ScenarioBranch {
            branch_id: branch_id.clone(),
            parent_scenario_version_id: parent.scenario_version_id.clone(),
            child_scenario_family_id: family_id.clone(),
            branch_type: request.branch_type,
            reason,
            changed_assumptions: request.changed_assumptions.clone(),
            created_at: now,
            created_by: actor.clone(),
        };
        let reality_version = parent.business_reality_version_id.clone();
        let snapshot = ScenarioSnapshot {
            snapshot_id: new_snapshot_id(),
            scenario_version_id: child_version_id.clone(),
            destination_version_id: parent.destination_version_id.clone(),
            business_reality_version_id: reality_version.clone(),
            reality_fact_version_ids: parent.snapshot.reality_fact_version_ids.clone(),
            financial_model_version_id: parent.snapshot.financial_model_version_id.clone(),
            research_snapshot_id: None,
            evidence_snapshot_id: None,
            financing_reference_set: None,
            seller_note_reference_set: None,
            professional_review_reference_set: None,
            created_at: now,
        };
        let version = ScenarioVersion {
            scenario_version_id: child_version_id.clone(),
            scenario_family_id: family_id.clone(),
            scenario_type: parent.scenario_type,
            version_number: 1,
            parent_version_id: Some(parent.scenario_version_id.clone()),
            supersedes_version_id: None,
            name: family.name.clone(),
            description: None,
            change_reason: Some(branch.reason.clone()),
            lifecycle_status: LifecycleStatus::Draft,
            readiness_status: ReadinessStatus::Preliminary,
            snapshot,
            destination_version_id: parent.destination_version_id.clone(),
            business_reality_version_id: reality_version,
            assumptions: seed_assumptions(&parent.assumptions, &child_version_id, now),
            constraints: seed_constraints(&parent.constraints, &child_version_id, now),
            nonnegotiables: seed_nonnegotiables(&parent.nonnegotiables, &child_version_id, now),
            unknowns: seed_unknowns(&parent.unknowns, &child_version_id, now),
            status_history: Vec::new(),
            finalized_at: None,
            created_at: now,
            created_by: actor.clone(),
        };

        // Two material facts, two events, both before any row exists.
        self.emit(
            AuditEventType::ScenarioCreated,
            object_id(family.scenario_family_id.as_str()),
            json!({
                "scenario_family_id": family.scenario_family_id,
                "name": family.name,
                "scenario_type": family.scenario_type,
                "branched_from_version_id": family.branched_from_version_id,
                "branch_id": branch.branch_id,
            }),
            actor.clone(),
            correlation_id.clone(),
        )?;
        self.emit(
            AuditEventType::ScenarioWhatIfCreated,
            object_id(branch.branch_id.as_str()),
            json!({
                "branch_id": branch.branch_id,
                "branch_type": branch.branch_type,
                "parent_scenario_version_id": branch.parent_scenario_version_id,
                "child_scenario_family_id": branch.child_scenario_family_id,
                "reason": branch.reason,
                "changed_assumptions": branch.changed_assumptions,
            }),
            actor,
            correlation_id,
        )?;

        let mut table = self.lock_table()?;
        table.families.insert(family_id, family.clone());
        table
            .versions
            .insert(child_version_id, version.clone());
        table.branches.push(branch.clone());
        Ok(WhatIfCreated {
            branch,
            family,
            version,
        })
    }

    // ------------------------------------------------------------------
    // Comparisons
    // ------------------------------------------------------------------

    /// Records a durable comparison (schema doc §32) — what was
    /// compared, never which scenario won.
    ///
    /// # Errors
    /// Fewer than two versions, a foreign version, an unknown dimension
    /// index, a duplicate result cell, an empty name, or an audit-store
    /// rejection.
    pub fn record_comparison(
        &self,
        request: NewComparison,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<ScenarioComparison, ScenarioError> {
        let name = clean_text(&request.name, "comparison name")?;
        if request.scenario_version_ids.len() < 2 {
            return Err(ScenarioError::ComparisonNeedsTwoVersions);
        }
        let mut unique_versions = request.scenario_version_ids.clone();
        unique_versions.sort();
        unique_versions.dedup();
        if unique_versions.len() != request.scenario_version_ids.len() {
            return Err(ScenarioError::DuplicateComparisonVersion);
        }
        for version_id in &request.scenario_version_ids {
            self.version(version_id)?;
        }
        if request.dimensions.is_empty() {
            return Err(ScenarioError::InvalidValue {
                reason: "a comparison needs at least one dimension",
            });
        }
        let comparison_id = new_id("cmp", ComparisonId::new);
        let created_at = self.clock.now();
        let dimensions: Vec<ComparisonDimension> = request
            .dimensions
            .iter()
            .map(|dimension| ComparisonDimension {
                dimension_id: new_id("dim", ComparisonDimensionId::new),
                comparison_id: comparison_id.clone(),
                dimension_type: dimension.dimension_type,
                label: dimension.label.clone(),
                display_order: dimension.display_order,
            })
            .collect();
        let mut results = Vec::new();
        for result in &request.results {
            let dimension = dimensions
                .get(result.dimension_index)
                .ok_or(ScenarioError::ComparisonUnknownDimension {
                    index: result.dimension_index,
                })?;
            if !request
                .scenario_version_ids
                .contains(&result.scenario_version_id)
            {
                return Err(ScenarioError::ComparisonForeignVersion {
                    scenario_version_id: result.scenario_version_id.clone(),
                });
            }
            if results.iter().any(|existing: &ComparisonResult| {
                existing.scenario_version_id == result.scenario_version_id
                    && existing.dimension_id == dimension.dimension_id
            }) {
                return Err(ScenarioError::ComparisonDuplicateResult);
            }
            if let Some(value) = &result.value {
                value.validate()?;
            }
            results.push(ComparisonResult {
                result_id: new_id("res", ComparisonResultId::new),
                comparison_id: comparison_id.clone(),
                scenario_version_id: result.scenario_version_id.clone(),
                dimension_id: dimension.dimension_id.clone(),
                value: result.value.clone(),
                outcome: result.outcome,
                source_reference: result.source_reference.clone(),
                created_at,
            });
        }
        let comparison = ScenarioComparison {
            comparison_id: comparison_id.clone(),
            name,
            scenario_version_ids: request.scenario_version_ids.clone(),
            dimensions,
            results,
            created_at,
            created_by: actor.clone(),
        };
        self.emit(
            AuditEventType::ScenarioComparisonRecorded,
            object_id(comparison.comparison_id.as_str()),
            json!({
                "comparison_id": comparison.comparison_id,
                "name": comparison.name,
                "scenario_version_ids": comparison.scenario_version_ids,
                "dimension_count": comparison.dimensions.len(),
                "result_count": comparison.results.len(),
            }),
            actor,
            correlation_id,
        )?;
        self.lock_table()?
            .comparisons
            .insert(comparison_id.clone(), comparison.clone());
        Ok(comparison)
    }

    // ------------------------------------------------------------------
    // Upstream change detection
    // ------------------------------------------------------------------

    /// Detects whether the current Business Reality fact-version set
    /// digests to a different reality version than the one a version
    /// pinned (integrity rule 2's flip side). The version is never
    /// re-pointed — the impact is recorded and surfaced for the owner,
    /// and the successor flow (§47) is how a scenario picks the change
    /// up.
    ///
    /// # Errors
    /// Unknown version, an empty source reference, or an audit-store
    /// rejection.
    pub fn detect_reality_change(
        &self,
        scenario_version_id: ScenarioVersionId,
        current_fact_version_ids: &[FactVersionId],
        source_change_type: &str,
        source_change_id: &str,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<Option<ScenarioImpact>, ScenarioError> {
        let source_change_type = clean_text(source_change_type, "source_change_type")?;
        let source_change_id = clean_text(source_change_id, "source_change_id")?;
        let version = self.version(&scenario_version_id)?.clone();
        let current_version = business_reality_version(current_fact_version_ids);
        if current_version == version.business_reality_version_id {
            return Ok(None);
        }
        // Conservative priority: nonnegotiables first (the must-haves
        // are what the owner needs to re-examine), then assumptions,
        // then unknowns. A version with none of the three still gets an
        // honest `unknown_affected` — new facts mean the version's open
        // questions are what to revisit.
        let impact_type = if !version.nonnegotiables.is_empty() {
            ImpactType::NonnegotiableAffected
        } else if !version.assumptions.is_empty() {
            ImpactType::AssumptionAffected
        } else {
            ImpactType::UnknownAffected
        };
        let impact = ScenarioImpact {
            impact_id: new_id("imp", ScenarioImpactId::new),
            scenario_version_id: scenario_version_id.clone(),
            source_change_type,
            source_change_id,
            impact_type,
            impact_description: format!(
                "The Business Reality this scenario was built against ({}) no longer \
                 matches the current facts ({}). The scenario version is unchanged; \
                 start a successor version to pick the change up.",
                version.business_reality_version_id, current_version
            ),
            severity: ConflictSeverity::Attention,
            detected_at: self.clock.now(),
            status: ImpactStatus::Open,
        };
        self.emit(
            AuditEventType::ScenarioAffectedByRealityChange,
            object_id(version.scenario_version_id.as_str()),
            json!({
                "scenario_version_id": impact.scenario_version_id,
                "impact_type": impact.impact_type,
                "pinned_business_reality_version_id": version.business_reality_version_id,
                "current_business_reality_version_id": current_version,
                "severity": impact.severity,
            }),
            actor,
            correlation_id,
        )?;
        self.lock_table()?.impacts.push(impact.clone());
        Ok(Some(impact))
    }

    // ------------------------------------------------------------------
    // Read accessors
    // ------------------------------------------------------------------

    /// Returns one family.
    ///
    /// # Errors
    /// Unknown family.
    pub fn family(
        &self,
        scenario_family_id: &ScenarioFamilyId,
    ) -> Result<ScenarioFamily, ScenarioError> {
        self.lock_table()?
            .families
            .get(scenario_family_id)
            .cloned()
            .ok_or_else(|| ScenarioError::FamilyNotFound {
                scenario_family_id: scenario_family_id.clone(),
            })
    }

    /// Returns one version.
    ///
    /// # Errors
    /// Unknown version.
    pub fn version(
        &self,
        scenario_version_id: &ScenarioVersionId,
    ) -> Result<ScenarioVersion, ScenarioError> {
        self.lock_table()?
            .versions
            .get(scenario_version_id)
            .cloned()
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: scenario_version_id.clone(),
            })
    }

    /// Returns every version of a family, ordered by version number.
    ///
    /// # Errors
    /// Unknown family.
    pub fn versions_of_family(
        &self,
        scenario_family_id: &ScenarioFamilyId,
    ) -> Result<Vec<ScenarioVersion>, ScenarioError> {
        self.family(scenario_family_id)?;
        let mut versions: Vec<ScenarioVersion> = self
            .lock_table()?
            .versions
            .values()
            .filter(|v| &v.scenario_family_id == scenario_family_id)
            .cloned()
            .collect();
        versions.sort_by_key(|v| v.version_number);
        Ok(versions)
    }

    /// Returns every conflict recorded against a version, with their
    /// nonnegotiable extensions (integrity rule 8: these outlive newer
    /// versions).
    ///
    /// # Errors
    /// Unknown version.
    pub fn conflicts_of(
        &self,
        scenario_version_id: &ScenarioVersionId,
    ) -> Result<Vec<ConflictRecord>, ScenarioError> {
        self.version(scenario_version_id)?;
        let mut records: Vec<ConflictRecord> = self
            .lock_table()?
            .conflicts
            .values()
            .filter(|record| &record.conflict.scenario_version_id == scenario_version_id)
            .cloned()
            .collect();
        records.sort_by(|a, b| a.conflict.created_at.cmp(&b.conflict.created_at));
        Ok(records)
    }

    /// Returns every branch created from a version.
    ///
    /// # Errors
    /// Unknown version.
    pub fn branches_from(
        &self,
        parent_scenario_version_id: &ScenarioVersionId,
    ) -> Result<Vec<ScenarioBranch>, ScenarioError> {
        self.version(parent_scenario_version_id)?;
        Ok(self
            .lock_table()?
            .branches
            .iter()
            .filter(|b| &b.parent_scenario_version_id == parent_scenario_version_id)
            .cloned()
            .collect())
    }

    /// Returns one comparison.
    ///
    /// # Errors
    /// Unknown comparison.
    pub fn comparison(
        &self,
        comparison_id: &ComparisonId,
    ) -> Result<ScenarioComparison, ScenarioError> {
        self.lock_table()?
            .comparisons
            .get(comparison_id)
            .cloned()
            .ok_or_else(|| ScenarioError::ComparisonNotFound {
                comparison_id: comparison_id.clone(),
            })
    }

    /// Returns every professional review reference recorded for a
    /// version (integrity rule 5's read side).
    ///
    /// # Errors
    /// Unknown version.
    pub fn review_references_of(
        &self,
        scenario_version_id: &ScenarioVersionId,
    ) -> Result<Vec<ProfessionalReviewReference>, ScenarioError> {
        self.version(scenario_version_id)?;
        Ok(self
            .lock_table()?
            .review_references
            .iter()
            .filter(|r| &r.scenario_version_id == scenario_version_id)
            .cloned()
            .collect())
    }

    /// Returns every impact recorded against a version.
    ///
    /// # Errors
    /// Unknown version.
    pub fn impacts_of(
        &self,
        scenario_version_id: &ScenarioVersionId,
    ) -> Result<Vec<ScenarioImpact>, ScenarioError> {
        self.version(scenario_version_id)?;
        Ok(self
            .lock_table()?
            .impacts
            .iter()
            .filter(|i| &i.scenario_version_id == scenario_version_id)
            .cloned()
            .collect())
    }

    // ------------------------------------------------------------------
    // Internals
    // ------------------------------------------------------------------

    fn lock_table(&self) -> Result<std::sync::MutexGuard<'_, ScenarioTable>, ScenarioError> {
        self.table
            .lock()
            .map_err(|_| internal("scenario table lock poisoned"))
    }

    fn version_in(
        &self,
        table: &ScenarioTable,
        scenario_version_id: &ScenarioVersionId,
    ) -> Result<ScenarioVersion, ScenarioError> {
        table
            .versions
            .get(scenario_version_id)
            .cloned()
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: scenario_version_id.clone(),
            })
    }

    /// The shared draft-edit path: validate mutability, emit the audit
    /// event, then apply. `apply` mutates the stored draft only after
    /// the event was accepted — audit-first ordering.
    fn apply_draft_edit<T: Serialize>(
        &self,
        scenario_version_id: &ScenarioVersionId,
        section: DraftSection,
        change: DraftChange,
        touched_id: &str,
        previous_value: Option<&T>,
        new_value: &T,
        actor: ActorRecord,
        correlation_id: CorrelationId,
        apply: impl FnOnce(&mut ScenarioVersion),
    ) -> Result<(), ScenarioError> {
        let mut table = self.lock_table()?;
        let version = table
            .versions
            .get_mut(scenario_version_id)
            .ok_or_else(|| ScenarioError::VersionNotFound {
                scenario_version_id: scenario_version_id.clone(),
            })?;
        if !version.is_draft() {
            return Err(ScenarioError::FinalizedVersionImmutable {
                scenario_version_id: version.scenario_version_id.clone(),
            });
        }
        let previous_json = previous_value
            .map(serde_json::to_value)
            .transpose()
            .map_err(ScenarioError::Serialization)?;
        let new_json = serde_json::to_value(new_value).map_err(ScenarioError::Serialization)?;
        self.emit(
            AuditEventType::ScenarioDraftUpdated,
            object_id(version.scenario_version_id.as_str()),
            json!({
                "scenario_version_id": version.scenario_version_id,
                "section": section,
                "change": change,
                "object_id": touched_id,
                "previous_value": previous_json,
                "new_value": new_json,
            }),
            actor,
            correlation_id,
        )?;
        apply(version);
        Ok(())
    }

    /// Emits one audit event about the scenario engine's objects.
    fn emit(
        &self,
        event_type: AuditEventType,
        source_object: ObjectId,
        payload: Value,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<(), ScenarioError> {
        self.sink
            .emit(AuditEventDraft {
                event_type,
                source_engine: EngineId::Scenario,
                source_object,
                actor,
                workspace_id: self.workspace_id.clone(),
                transaction_id: None,
                correlation_id,
                causation_id: None,
                payload,
            })
            .map_err(ScenarioError::Audit)
    }
}

/// Wraps a non-empty id string as an `ObjectId` — every engine id is
/// non-empty by construction.
fn object_id(id: &str) -> ObjectId {
    ObjectId::new(id).unwrap_or_else(|_| unreachable!("a non-empty id is always a valid ObjectId"))
}

/// Trims and validates required text; blank strings are refused —
/// absence is expressed by not recording content.
fn clean_text(text: &str, field: &'static str) -> Result<String, ScenarioError> {
    if text.trim().is_empty() {
        return Err(ScenarioError::EmptyText { field });
    }
    Ok(text.trim().to_owned())
}

/// A defensive internal-error constructor.
fn internal(message: &str) -> ScenarioError {
    ScenarioError::Internal(message.to_owned())
}

/// The `started` draft event's `new_value` (SCHEMA.md §3.2).
fn draft_started_payload(version: &ScenarioVersion) -> Value {
    json!({
        "scenario_version_id": version.scenario_version_id,
        "section": DraftSection::Started,
        "change": DraftChange::Added,
        "object_id": version.scenario_version_id,
        "previous_value": Value::Null,
        "new_value": {
            "name": version.name,
            "scenario_type": version.scenario_type,
            "destination_version_id": version.destination_version_id,
            "business_reality_version_id": version.business_reality_version_id,
        },
    })
}

/// Seeds a successor/branch version's assumptions from a parent — fresh
/// row ids, values as stated.
fn seed_assumptions(
    assumptions: &[ScenarioAssumption],
    scenario_version_id: &ScenarioVersionId,
    created_at: OffsetDateTime,
) -> Vec<ScenarioAssumption> {
    assumptions
        .iter()
        .map(|a| ScenarioAssumption {
            assumption_id: new_id("asm", AssumptionId::new),
            scenario_version_id: scenario_version_id.clone(),
            category: a.category,
            name: a.name.clone(),
            description: a.description.clone(),
            value: a.value.clone(),
            provenance: a.provenance.clone(),
            verification: a.verification.clone(),
            nonnegotiable_objective_id: a.nonnegotiable_objective_id.clone(),
            source_reference: a.source_reference.clone(),
            created_at,
        })
        .collect()
}

/// Seeds constraints the same way as [`seed_assumptions`].
fn seed_constraints(
    constraints: &[ScenarioConstraint],
    scenario_version_id: &ScenarioVersionId,
    created_at: OffsetDateTime,
) -> Vec<ScenarioConstraint> {
    constraints
        .iter()
        .map(|c| ScenarioConstraint {
            constraint_id: new_id("con", ConstraintId::new),
            scenario_version_id: scenario_version_id.clone(),
            constraint_type: c.constraint_type,
            name: c.name.clone(),
            description: c.description.clone(),
            value: c.value.clone(),
            source_reference: c.source_reference.clone(),
            created_at,
        })
        .collect()
}

/// Seeds nonnegotiable rows the same way as [`seed_assumptions`].
fn seed_nonnegotiables(
    nonnegotiables: &[ScenarioNonnegotiable],
    scenario_version_id: &ScenarioVersionId,
    created_at: OffsetDateTime,
) -> Vec<ScenarioNonnegotiable> {
    nonnegotiables
        .iter()
        .map(|nn| ScenarioNonnegotiable {
            nonnegotiable_id: new_id("nn", NonnegotiableId::new),
            scenario_version_id: scenario_version_id.clone(),
            destination_objective_id: nn.destination_objective_id.clone(),
            destination_version_id: nn.destination_version_id.clone(),
            description: nn.description.clone(),
            value: nn.value.clone(),
            created_at,
        })
        .collect()
}

/// Seeds unknowns the same way as [`seed_assumptions`]; seeded unknowns
/// come back open — the successor re-asks them.
fn seed_unknowns(
    unknowns: &[ScenarioUnknown],
    scenario_version_id: &ScenarioVersionId,
    created_at: OffsetDateTime,
) -> Vec<ScenarioUnknown> {
    unknowns
        .iter()
        .map(|u| ScenarioUnknown {
            unknown_id: new_id("unk", UnknownId::new),
            scenario_version_id: scenario_version_id.clone(),
            category: u.category.clone(),
            description: u.description.clone(),
            importance: u.importance,
            resolution_status: UnknownResolutionStatus::Open,
            required_action: u.required_action.clone(),
            source_dependency: u.source_dependency.clone(),
            resolved_at: None,
            resolved_by: None,
            resolution_reference: None,
            created_at,
        })
        .collect()
}
