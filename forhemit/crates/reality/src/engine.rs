//! The Business Reality engine: records owner-reported Business Snapshot
//! facts behind the audit-first discipline.
//!
//! Every mutation emits exactly one audit event and the mutation is
//! applied only after its event was accepted by the sink — a fact version
//! exists only if its event exists (Implementation Roadmap Phase 1:
//! "audit infrastructure early"). The engine holds the sink through
//! [`AuditSink`], so the app shell wires the concrete audit store in at
//! the composition root; the engine itself can never read, edit, or
//! delete audit history.
//!
//! Lineage note: the sink boundary returns no stored-event id, so a
//! revision links to its predecessor through the event payload
//! (`previous_version_id` / `previous_value`, Audit integrity rule 4)
//! and the shared [`CorrelationId`] rather than through `causation_id`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use forhemit_contracts::{
    ActorRecord, AuditEventDraft, AuditEventType, CorrelationId, EngineId, FactId, FactVersionId,
    ObjectId, Provenance, Verification, WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ulid::Ulid;

use crate::error::RealityError;
use crate::fact::{FactKind, FactPeriod, FactValue, FactVersion, FactVersionInput};

/// A fact to record — the request half of
/// [`RealityEngine::record_fact`].
#[derive(Clone, Debug)]
pub struct RecordFact {
    /// Which Business Snapshot field the fact states.
    pub kind: FactKind,
    /// The owner-stated value.
    pub value: FactValue,
    /// The period the figure describes.
    pub period: FactPeriod,
    /// The metric label or qualifier as the owner stated it, if any
    /// (Expanded Reality §12).
    pub definition: Option<String>,
    /// Who stated the fact — the owner (Audit doc §7).
    pub actor: ActorRecord,
    /// The activity this recording belongs to (Audit doc §18) — the
    /// journey step that collected the answer.
    pub correlation_id: CorrelationId,
}

/// A revision of an existing fact — the request half of
/// [`RealityEngine::revise_fact`]. The fact's kind is fixed by its
/// identity; a revision restates the value.
#[derive(Clone, Debug)]
pub struct ReviseFact {
    /// The fact to revise — its stable id, not a version id.
    pub fact_id: FactId,
    /// The new owner-stated value.
    pub value: FactValue,
    /// The period the new figure describes.
    pub period: FactPeriod,
    /// The metric label or qualifier as the owner stated it now, if any.
    pub definition: Option<String>,
    /// Why the fact changed — "Why did it change?" is the first question
    /// the owner-facing history answers (Business Reality doc §25).
    pub change_reason: String,
    /// Who stated the revision.
    pub actor: ActorRecord,
    /// The activity this revision belongs to (Audit doc §18).
    pub correlation_id: CorrelationId,
}

/// The audit payload of a `RealityFactRecorded` event: the fact's first
/// version, with its provenance stamps.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactRecordedPayload {
    /// The fact's stable id.
    pub fact_id: FactId,
    /// The recorded version's id.
    pub fact_version_id: FactVersionId,
    /// Which Business Snapshot field the fact states.
    pub kind: FactKind,
    /// The owner-stated value.
    pub value: FactValue,
    /// The period the figure describes.
    pub period: FactPeriod,
    /// The metric label as the owner stated it, if any.
    pub definition: Option<String>,
    /// How the fact entered the system — `owner_reported` in v1.
    pub provenance: Provenance,
    /// How well the fact is verified — `unverified` in v1.
    pub verification: Verification,
    /// When the owner stated the fact.
    pub recorded_at: OffsetDateTime,
}

/// The audit payload of a `RealityFactRevised` event: the new version
/// plus the previous version's value — integrity rule 4, "Material
/// changes preserve previous and new values or references".
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactRevisedPayload {
    /// The fact's stable id.
    pub fact_id: FactId,
    /// The new version's id.
    pub fact_version_id: FactVersionId,
    /// Which Business Snapshot field the fact states.
    pub kind: FactKind,
    /// The new owner-stated value.
    pub value: FactValue,
    /// The superseded version's id — never edited, still readable.
    pub previous_version_id: FactVersionId,
    /// The superseded version's value, preserved in the event itself.
    pub previous_value: FactValue,
    /// The period the new figure describes.
    pub period: FactPeriod,
    /// The metric label as the owner stated it now, if any.
    pub definition: Option<String>,
    /// How the fact entered the system — `owner_reported` in v1.
    pub provenance: Provenance,
    /// How well the fact is verified — `unverified` in v1.
    pub verification: Verification,
    /// Why the fact changed.
    pub change_reason: String,
    /// When the owner stated the revision.
    pub recorded_at: OffsetDateTime,
}

/// The facts recorded so far, keyed by stable fact id and by kind.
#[derive(Default)]
struct FactTable {
    /// Each fact's full version history, oldest first.
    by_id: HashMap<FactId, Vec<FactVersion>>,
    /// The current fact per snapshot kind — the snapshot records one
    /// fact per kind.
    current_by_kind: HashMap<FactKind, FactId>,
}

/// The Business Reality engine for one workspace.
///
/// Cheap to clone: all handles share one fact table and one sink. The
/// engine owns no persistence — durable fact storage is the app shell's
/// composition concern; in v1 the engine is the domain model and the
/// audit stream is the durable record.
pub struct RealityEngine<S: AuditSink<AuditEventDraft>> {
    sink: Arc<S>,
    clock: Arc<dyn Clock>,
    workspace_id: WorkspaceId,
    facts: Arc<Mutex<FactTable>>,
}

impl<S: AuditSink<AuditEventDraft>> Clone for RealityEngine<S> {
    fn clone(&self) -> Self {
        Self {
            sink: self.sink.clone(),
            clock: self.clock.clone(),
            workspace_id: self.workspace_id.clone(),
            facts: self.facts.clone(),
        }
    }
}

impl<S: AuditSink<AuditEventDraft>> RealityEngine<S> {
    /// Builds an engine for `workspace_id`, emitting through `sink` and
    /// stamping fact times from `clock`.
    pub fn new(sink: Arc<S>, clock: Arc<dyn Clock>, workspace_id: WorkspaceId) -> Self {
        Self {
            sink,
            clock,
            workspace_id,
            facts: Arc::new(Mutex::new(FactTable::default())),
        }
    }

    /// Records a fact the owner stated — the fact's first version.
    ///
    /// # Errors
    ///
    /// A value that does not fit its kind, a second fact of a kind the
    /// snapshot records once, or an audit-store rejection — in every
    /// failure case nothing is recorded and no event is emitted.
    pub fn record_fact(&self, request: RecordFact) -> Result<FactVersion, RealityError> {
        let version = FactVersion::owner_reported(
            FactVersionInput {
                fact_id: Self::new_fact_id(),
                kind: request.kind,
                value: request.value,
                period: request.period,
                definition: request.definition,
                supersedes: None,
            },
            Self::new_fact_version_id(),
            self.clock.now(),
        )?;

        let mut table = self.lock_table()?;
        if let Some(existing) = table.current_by_kind.get(&request.kind) {
            return Err(RealityError::FactKindAlreadyRecorded {
                kind: request.kind,
                fact_id: existing.clone(),
            });
        }

        self.emit(
            AuditEventType::RealityFactRecorded,
            &version,
            None,
            None,
            request.actor,
            request.correlation_id,
        )?;

        table
            .current_by_kind
            .insert(request.kind, version.fact_id().clone());
        table
            .by_id
            .insert(version.fact_id().clone(), vec![version.clone()]);
        Ok(version)
    }

    /// Records a new version of an existing fact; the previous version
    /// is retained untouched and the revision's event preserves both
    /// values (Audit integrity rule 4).
    ///
    /// # Errors
    ///
    /// An unknown fact id, an empty change reason, a value that does not
    /// fit the fact's kind, or an audit-store rejection — in every
    /// failure case the fact's history is unchanged and no event is
    /// emitted.
    pub fn revise_fact(&self, request: ReviseFact) -> Result<FactVersion, RealityError> {
        if request.change_reason.trim().is_empty() {
            return Err(RealityError::EmptyChangeReason);
        }

        let mut table = self.lock_table()?;
        let (previous_kind, previous_version_id, previous_value) =
            {
                let history = table.by_id.get(&request.fact_id).ok_or_else(|| {
                    RealityError::FactNotFound {
                        fact_id: request.fact_id.clone(),
                    }
                })?;
                let previous = history.last().ok_or_else(|| RealityError::FactNotFound {
                    fact_id: request.fact_id.clone(),
                })?;
                (
                    previous.kind(),
                    previous.fact_version_id().clone(),
                    previous.value().clone(),
                )
            };

        let version = FactVersion::owner_reported(
            FactVersionInput {
                fact_id: request.fact_id.clone(),
                kind: previous_kind,
                value: request.value,
                period: request.period,
                definition: request.definition,
                supersedes: Some(previous_version_id.clone()),
            },
            Self::new_fact_version_id(),
            self.clock.now(),
        )?;

        self.emit(
            AuditEventType::RealityFactRevised,
            &version,
            Some((&previous_version_id, &previous_value)),
            Some(&request.change_reason),
            request.actor,
            request.correlation_id,
        )?;

        if let Some(history) = table.by_id.get_mut(&request.fact_id) {
            history.push(version.clone());
        }
        Ok(version)
    }

    /// The fact's current (latest) version.
    ///
    /// # Errors
    ///
    /// [`RealityError::FactNotFound`] when no fact has the given id.
    pub fn current_fact(&self, fact_id: &FactId) -> Result<FactVersion, RealityError> {
        let table = self.lock_table()?;
        table
            .by_id
            .get(fact_id)
            .and_then(|history| history.last())
            .cloned()
            .ok_or_else(|| RealityError::FactNotFound {
                fact_id: fact_id.clone(),
            })
    }

    /// The fact's full version history, oldest first (Business Reality
    /// doc §25 — historical snapshots remain available).
    ///
    /// # Errors
    ///
    /// [`RealityError::FactNotFound`] when no fact has the given id.
    pub fn fact_history(&self, fact_id: &FactId) -> Result<Vec<FactVersion>, RealityError> {
        let table = self.lock_table()?;
        table
            .by_id
            .get(fact_id)
            .cloned()
            .ok_or_else(|| RealityError::FactNotFound {
                fact_id: fact_id.clone(),
            })
    }

    /// Every current fact, one per recorded kind, ordered by kind — the
    /// Business Snapshot view (Business Reality doc §17). Kinds never
    /// recorded are simply absent: an unknown fact is an absence, never
    /// a zero.
    ///
    /// # Errors
    ///
    /// [`RealityError::Internal`] when the fact table's lock is poisoned
    /// — a panic while the table was held; a snapshot cannot be served
    /// honestly from a table in an unknown state.
    pub fn current_facts(&self) -> Result<Vec<FactVersion>, RealityError> {
        let table = self.lock_table()?;
        let mut current = Vec::with_capacity(table.current_by_kind.len());
        for fact_id in table.current_by_kind.values() {
            if let Some(history) = table.by_id.get(fact_id) {
                if let Some(version) = history.last() {
                    current.push(version.clone());
                }
            }
        }
        current.sort_by_key(|version| version.kind());
        Ok(current)
    }

    fn lock_table(&self) -> Result<std::sync::MutexGuard<'_, FactTable>, RealityError> {
        self.facts
            .lock()
            .map_err(|_| RealityError::Internal("fact table lock poisoned".to_owned()))
    }

    fn new_fact_id() -> FactId {
        FactId::new(Ulid::new().to_string())
            .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
    }

    fn new_fact_version_id() -> FactVersionId {
        FactVersionId::new(Ulid::new().to_string())
            .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
    }

    /// Emits one audit event about a fact version. The payload carries
    /// the version's full provenance and, for revisions, the superseded
    /// version's value (Audit integrity rule 4).
    fn emit(
        &self,
        event_type: AuditEventType,
        version: &FactVersion,
        previous: Option<(&FactVersionId, &FactValue)>,
        change_reason: Option<&str>,
        actor: ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<(), RealityError> {
        let payload = match previous {
            None => serde_json::to_value(FactRecordedPayload {
                fact_id: version.fact_id().clone(),
                fact_version_id: version.fact_version_id().clone(),
                kind: version.kind(),
                value: version.value().clone(),
                period: version.period(),
                definition: version.definition().map(str::to_owned),
                provenance: version.provenance(),
                verification: version.verification(),
                recorded_at: version.recorded_at(),
            })
            .map_err(RealityError::Serialization)?,
            Some((previous_version_id, previous_value)) => {
                serde_json::to_value(FactRevisedPayload {
                    fact_id: version.fact_id().clone(),
                    fact_version_id: version.fact_version_id().clone(),
                    kind: version.kind(),
                    value: version.value().clone(),
                    previous_version_id: previous_version_id.clone(),
                    previous_value: previous_value.clone(),
                    period: version.period(),
                    definition: version.definition().map(str::to_owned),
                    provenance: version.provenance(),
                    verification: version.verification(),
                    change_reason: change_reason
                        .ok_or_else(|| {
                            RealityError::Internal(
                                "a revision event always carries a change reason".to_owned(),
                            )
                        })?
                        .to_owned(),
                    recorded_at: version.recorded_at(),
                })
                .map_err(RealityError::Serialization)?
            }
        };

        self.sink
            .emit(AuditEventDraft {
                event_type,
                source_engine: EngineId::Reality,
                source_object: ObjectId::new(version.fact_id().as_str()).map_err(|_| {
                    RealityError::Internal(
                        "a non-empty FactId is always a valid ObjectId".to_owned(),
                    )
                })?,
                actor,
                workspace_id: self.workspace_id.clone(),
                transaction_id: None,
                correlation_id,
                // The sink boundary returns no stored-event id, so the
                // direct-causation link lands when the composition root
                // wires the concrete store; version lineage rides in the
                // payload.
                causation_id: None,
                payload,
            })
            .map_err(RealityError::Audit)
    }
}
