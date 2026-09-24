//! The Journey Engine — audit-first transitions of the walk.
//!
//! The engine holds the clock, the audit sink, the instance store, and
//! the workspace; it owns every rule a mutation must pass (position,
//! eligibility, answer shape, skip restrictions, revision reasons) and
//! emits exactly one audit event per state transition **before** the
//! mutation lands — a walk state never exists without its event. The
//! audit stream carries the full walk history; the instance document in
//! the store is the resumable snapshot.

use forhemit_contracts::{
    AnswerId, AnswerVersionId, AuditEventDraft, AuditEventType, EngineId, JourneyNodeId, ObjectId,
    WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use serde::{Deserialize, Serialize};

use crate::answer::{AnswerRecord, AnswerValue, AnswerVersion};
use crate::definition::JourneyDefinition;
use crate::error::JourneyError;
use crate::instance::{InstanceStatus, JourneyInstance};
use crate::scope::StorageScope;
use crate::store::JourneyStore;

/// Audit payload of [`AuditEventType::JourneyInstanceStarted`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceStartedPayload {
    /// The journey the walk follows.
    pub journey_id: String,
    /// The definition version the walk started under.
    pub journey_version: String,
    /// The instance's identity.
    pub instance_id: String,
}

/// Audit payload of [`AuditEventType::JourneyAnswerRecorded`] — the
/// first version of an answer.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerRecordedPayload {
    /// The question answered.
    pub node_id: String,
    /// The answer's stable identity across versions.
    pub answer_id: String,
    /// The version's identity.
    pub version_id: String,
    /// One-based version number — always 1 here.
    pub version: u32,
    /// The Storage Scope the question declared.
    pub storage_scope: StorageScope,
    /// The recorded value.
    pub value: AnswerValue,
}

/// Audit payload of [`AuditEventType::JourneyAnswerRevised`] — a new
/// version superseding the previous one.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerRevisedPayload {
    /// The question revised.
    pub node_id: String,
    /// The answer's stable identity across versions.
    pub answer_id: String,
    /// The new version's identity.
    pub version_id: String,
    /// The new version's one-based number.
    pub version: u32,
    /// The version this one supersedes.
    pub supersedes: String,
    /// The Storage Scope the question declared.
    pub storage_scope: StorageScope,
    /// The revised value.
    pub value: AnswerValue,
    /// Why the answer changed.
    pub change_reason: String,
}

/// Audit payload of [`AuditEventType::JourneyNodeSkipped`] — a question
/// exit on an optional node (EOJ v0.2 §30).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeSkippedPayload {
    /// The question passed without an answer.
    pub node_id: String,
}

/// Audit payload of [`AuditEventType::JourneyInstanceCompleted`] — every
/// eligible question is answered or skipped.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceCompletedPayload {
    /// How many questions carry answers.
    pub answered_nodes: usize,
    /// How many questions were exited.
    pub skipped_nodes: usize,
    /// Nonnegotiables marked along the way.
    pub marked_nonnegotiables: Vec<String>,
}

/// Record an answer on the walk's current question.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordAnswer {
    /// The question being answered.
    pub node_id: JourneyNodeId,
    /// The owner's response.
    pub value: AnswerValue,
}

/// Revise an existing answer — a new version that supersedes the
/// previous one; the original is retained (Journey Builder doc §9).
#[derive(Clone, Debug, PartialEq)]
pub struct ReviseAnswer {
    /// The question being revised.
    pub node_id: JourneyNodeId,
    /// The revised response.
    pub value: AnswerValue,
    /// Why the answer changed — never empty.
    pub change_reason: String,
}

/// Skip the walk's current optional question (EOJ v0.2 §30).
#[derive(Clone, Debug, PartialEq)]
pub struct SkipNode {
    /// The question being passed.
    pub node_id: JourneyNodeId,
}

/// The journey engine. Cheap to construct; all state lives in the
/// caller-held instance and the store.
pub struct JourneyEngine<'a> {
    clock: &'a dyn Clock,
    audit: &'a dyn AuditSink<AuditEventDraft>,
    store: &'a dyn JourneyStore,
    workspace_id: WorkspaceId,
}

impl<'a> JourneyEngine<'a> {
    /// Creates an engine bound to a workspace, a clock, the audit sink,
    /// and the instance store.
    pub fn new(
        clock: &'a dyn Clock,
        audit: &'a dyn AuditSink<AuditEventDraft>,
        store: &'a dyn JourneyStore,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            clock,
            audit,
            store,
            workspace_id,
        }
    }

    /// Starts a walk of `definition` — a fresh instance positioned on
    /// the first eligible question. The instance's correlation id is
    /// derived from its id, so every event of the walk shares it
    /// (Audit doc §18). Emits `JourneyInstanceStarted`.
    ///
    /// # Errors
    ///
    /// An invalid id, an audit refusal (nothing is created), or a store
    /// failure after the event was accepted.
    pub fn start(
        &self,
        definition: &JourneyDefinition,
        instance_id: forhemit_contracts::JourneyInstanceId,
        actor: &forhemit_contracts::ActorRecord,
    ) -> Result<JourneyInstance, JourneyError> {
        let correlation_id =
            forhemit_contracts::CorrelationId::new(format!("journey-{}", instance_id.as_str()))
                .unwrap_or_else(|_| unreachable!("the derived correlation id is never empty"));
        let now = self.clock.now();
        let instance =
            JourneyInstance::start(instance_id.clone(), correlation_id.clone(), definition, now);
        self.emit(
            AuditEventType::JourneyInstanceStarted,
            instance_id.as_str().to_owned(),
            actor,
            correlation_id,
            None,
            &InstanceStartedPayload {
                journey_id: definition.journey_id.as_str().to_owned(),
                journey_version: definition.version.as_str().to_owned(),
                instance_id: instance_id.as_str().to_owned(),
            },
        )?;
        self.store.save(&instance.to_document())?;
        Ok(instance)
    }

    /// Loads and restores a walk — the app-restart resume path. The
    /// restored instance is checked against `definition`: an instance
    /// only ever walks the journey (and version) it started under.
    ///
    /// # Errors
    ///
    /// [`JourneyError::InstanceNotFound`] when the store has no such
    /// instance; [`JourneyError::StoreState`] on an unparsable
    /// document; [`JourneyError::DefinitionMismatch`] on a journey or
    /// version mismatch.
    pub fn load(
        &self,
        definition: &JourneyDefinition,
        instance_id: &forhemit_contracts::JourneyInstanceId,
    ) -> Result<JourneyInstance, JourneyError> {
        let document =
            self.store
                .load(instance_id)?
                .ok_or_else(|| JourneyError::InstanceNotFound {
                    instance_id: instance_id.as_str().to_owned(),
                })?;
        let instance = JourneyInstance::from_document(document)?;
        if instance.journey_id != definition.journey_id
            || instance.journey_version != definition.version.as_str()
        {
            return Err(JourneyError::DefinitionMismatch {
                instance_journey: instance.journey_id.clone(),
                definition_journey: definition.journey_id.clone(),
            });
        }
        Ok(instance)
    }

    /// Records the first version of an answer on the walk's current
    /// question. Emits `JourneyAnswerRecorded`, and
    /// `JourneyInstanceCompleted` when this answer finishes the walk.
    ///
    /// # Errors
    ///
    /// A node that does not exist or is a screen, a node other than the
    /// current position, an ineligible node, a value that does not fit
    /// the question, an audit refusal, or a store failure.
    pub fn record(
        &self,
        instance: &mut JourneyInstance,
        definition: &JourneyDefinition,
        command: &RecordAnswer,
        actor: &forhemit_contracts::ActorRecord,
    ) -> Result<AnswerRecordedPayload, JourneyError> {
        self.check_walk(instance, definition, &command.node_id)?;
        let node = definition
            .node(&command.node_id)
            .unwrap_or_else(|| unreachable!("check_walk verified the node exists"));
        let question = node
            .question()
            .unwrap_or_else(|| unreachable!("check_walk verified the node is a question"));

        let resolved = self.resolved_choices(instance, definition, question);
        question.validate_answer(&command.node_id, &command.value, &resolved)?;

        let now = self.clock.now();
        let answer_id = AnswerId::new(format!("ans_{}", command.node_id.as_str()))
            .unwrap_or_else(|_| unreachable!("the node id is non-empty"));
        let version_id = AnswerVersionId::new(format!("{}_v1", answer_id.as_str()))
            .unwrap_or_else(|_| unreachable!("the derived version id is never empty"));
        let record = AnswerRecord {
            answer_id: answer_id.clone(),
            node_id: command.node_id.clone(),
            versions: vec![AnswerVersion {
                version_id: version_id.clone(),
                version: 1,
                value: command.value.clone(),
                storage_scope: question.storage_scope,
                recorded_at: now,
                supersedes: None,
                change_reason: None,
            }],
        };

        self.emit(
            AuditEventType::JourneyAnswerRecorded,
            command.node_id.as_str().to_owned(),
            actor,
            instance.correlation_id.clone(),
            None,
            &AnswerRecordedPayload {
                node_id: command.node_id.as_str().to_owned(),
                answer_id: answer_id.as_str().to_owned(),
                version_id: version_id.as_str().to_owned(),
                version: 1,
                storage_scope: question.storage_scope,
                value: command.value.clone(),
            },
        )?;
        // Effects read the answer being recorded — derived nonnegotiables
        // (EOJ v0.2 §11) are evaluated before the walk recomputes.
        instance.apply_effects(definition, &command.node_id, &command.value, now);
        self.apply_and_persist(
            instance,
            definition,
            now,
            actor,
            |instance, definition, now| {
                instance.apply_answer(definition, record, now);
            },
        )?;
        Ok(AnswerRecordedPayload {
            node_id: command.node_id.as_str().to_owned(),
            answer_id: answer_id.as_str().to_owned(),
            version_id: version_id.as_str().to_owned(),
            version: 1,
            storage_scope: question.storage_scope,
            value: command.value.clone(),
        })
    }

    /// Revises an existing answer: validates the new value, then appends
    /// a version that supersedes the previous one — the original is
    /// retained verbatim (Journey Builder doc §9). Emits
    /// `JourneyAnswerRevised`. Unlike record and skip, a revision
    /// targets a previously answered node and needs no walk position;
    /// the walk then recomputes — a revision can unlock questions that
    /// only became relevant.
    ///
    /// # Errors
    ///
    /// No prior answer under the node, an empty change reason, an
    /// ineligible node, a value that does not fit the question, an
    /// audit refusal, or a store failure.
    pub fn revise(
        &self,
        instance: &mut JourneyInstance,
        definition: &JourneyDefinition,
        command: &ReviseAnswer,
        actor: &forhemit_contracts::ActorRecord,
    ) -> Result<AnswerRevisedPayload, JourneyError> {
        self.check_mismatch(instance, definition)?;
        if command.change_reason.trim().is_empty() {
            return Err(JourneyError::EmptyChangeReason);
        }
        if !instance.is_eligible(definition, &command.node_id) {
            return Err(JourneyError::NodeNotEligible {
                node_id: command.node_id.clone(),
            });
        }
        let node = definition
            .node(&command.node_id)
            .ok_or_else(|| JourneyError::UnknownNode {
                node_id: command.node_id.clone(),
            })?;
        let question = node
            .question()
            .ok_or_else(|| JourneyError::NodeIsNotAQuestion {
                node_id: command.node_id.clone(),
            })?;
        let existing = instance
            .answer(&command.node_id)
            .ok_or_else(|| JourneyError::AnswerNotFound {
                node_id: command.node_id.clone(),
            })?
            .clone();
        let resolved = self.resolved_choices(instance, definition, question);
        question.validate_answer(&command.node_id, &command.value, &resolved)?;

        let now = self.clock.now();
        let previous = existing.latest();
        let previous_version_id = previous.version_id.clone();
        let previous_version_number = previous.version;
        let answer_id = existing.answer_id.clone();
        let version_id = AnswerVersionId::new(format!(
            "{}_v{}",
            answer_id.as_str(),
            previous_version_number + 1
        ))
        .unwrap_or_else(|_| unreachable!("the derived version id is never empty"));
        let scope = question.storage_scope;
        let new_version = AnswerVersion {
            version_id: version_id.clone(),
            version: previous_version_number + 1,
            value: command.value.clone(),
            storage_scope: scope,
            recorded_at: now,
            supersedes: Some(previous_version_id.clone()),
            change_reason: Some(command.change_reason.clone()),
        };

        self.emit(
            AuditEventType::JourneyAnswerRevised,
            command.node_id.as_str().to_owned(),
            actor,
            instance.correlation_id.clone(),
            None,
            &AnswerRevisedPayload {
                node_id: command.node_id.as_str().to_owned(),
                answer_id: answer_id.as_str().to_owned(),
                version_id: version_id.as_str().to_owned(),
                version: new_version.version,
                supersedes: previous_version_id.as_str().to_owned(),
                storage_scope: scope,
                value: command.value.clone(),
                change_reason: command.change_reason.clone(),
            },
        )?;
        self.apply_and_persist(
            instance,
            definition,
            now,
            actor,
            |instance, definition, now| {
                instance.apply_answer(
                    definition,
                    AnswerRecord {
                        answer_id: answer_id.clone(),
                        node_id: command.node_id.clone(),
                        versions: vec![new_version.clone()],
                    },
                    now,
                );
            },
        )?;
        Ok(AnswerRevisedPayload {
            node_id: command.node_id.as_str().to_owned(),
            answer_id: answer_id.as_str().to_owned(),
            version_id: version_id.as_str().to_owned(),
            version: new_version.version,
            supersedes: previous_version_id.as_str().to_owned(),
            storage_scope: scope,
            value: command.value.clone(),
            change_reason: command.change_reason.clone(),
        })
    }

    /// Skips the walk's current optional question (EOJ v0.2 §30: only
    /// optional nodes offer a question exit). Emits
    /// `JourneyNodeSkipped`, and `JourneyInstanceCompleted` when the
    /// exit finishes the walk.
    ///
    /// # Errors
    ///
    /// A node that does not exist or is a screen, a node other than the
    /// current position, a required node, an audit refusal, or a store
    /// failure.
    pub fn skip(
        &self,
        instance: &mut JourneyInstance,
        definition: &JourneyDefinition,
        command: &SkipNode,
        actor: &forhemit_contracts::ActorRecord,
    ) -> Result<NodeSkippedPayload, JourneyError> {
        self.check_walk(instance, definition, &command.node_id)?;
        let node = definition
            .node(&command.node_id)
            .unwrap_or_else(|| unreachable!("check_walk verified the node exists"));
        let question = node
            .question()
            .unwrap_or_else(|| unreachable!("check_walk verified the node is a question"));
        if question.required {
            return Err(JourneyError::SkipOfRequiredNode {
                node_id: command.node_id.clone(),
            });
        }
        let now = self.clock.now();
        self.emit(
            AuditEventType::JourneyNodeSkipped,
            command.node_id.as_str().to_owned(),
            actor,
            instance.correlation_id.clone(),
            None,
            &NodeSkippedPayload {
                node_id: command.node_id.as_str().to_owned(),
            },
        )?;
        self.apply_and_persist(
            instance,
            definition,
            now,
            actor,
            |instance, definition, now| {
                instance.apply_skip(definition, command.node_id.clone(), now);
            },
        )?;
        Ok(NodeSkippedPayload {
            node_id: command.node_id.as_str().to_owned(),
        })
    }

    /// Shared preconditions of record and skip: the definition matches
    /// the instance, the node exists, is a question, and is the walk's
    /// current position (which also implies eligibility — the position
    /// only ever names eligible questions).
    fn check_walk(
        &self,
        instance: &JourneyInstance,
        definition: &JourneyDefinition,
        node_id: &JourneyNodeId,
    ) -> Result<(), JourneyError> {
        self.check_mismatch(instance, definition)?;
        let node = definition
            .node(node_id)
            .ok_or_else(|| JourneyError::UnknownNode {
                node_id: node_id.clone(),
            })?;
        if node.kind != crate::definition::NodeKind::Question {
            return Err(JourneyError::NodeIsNotAQuestion {
                node_id: node_id.clone(),
            });
        }
        if instance.current_node.as_ref() != Some(node_id) {
            return Err(JourneyError::NotTheCurrentPosition {
                current: instance.current_node.clone(),
                requested: node_id.clone(),
            });
        }
        Ok(())
    }

    fn check_mismatch(
        &self,
        instance: &JourneyInstance,
        definition: &JourneyDefinition,
    ) -> Result<(), JourneyError> {
        if instance.journey_id != definition.journey_id
            || instance.journey_version != definition.version.as_str()
        {
            return Err(JourneyError::DefinitionMismatch {
                instance_journey: instance.journey_id.clone(),
                definition_journey: definition.journey_id.clone(),
            });
        }
        Ok(())
    }

    /// The choice set a `choices_from` question resolves from the
    /// answers so far, in source-question order — values the owner
    /// actually selected. Fixed-choice questions resolve to an empty
    /// slice; their allowed values come from the definition.
    fn resolved_choices(
        &self,
        instance: &JourneyInstance,
        definition: &JourneyDefinition,
        question: &crate::definition::QuestionDef,
    ) -> Vec<String> {
        let mut resolved = Vec::new();
        for source in question.choices_from.iter().flatten() {
            let Some(node) = definition.node(source) else {
                continue;
            };
            let Some(source_question) = node.question() else {
                continue;
            };
            // A source's own choices constrain which values can appear.
            for allowed in &source_question.choices {
                if instance
                    .latest_value(source)
                    .is_some_and(|value| value.contains(&allowed.value))
                {
                    resolved.push(allowed.value.clone());
                }
            }
        }
        resolved
    }

    /// Applies the mutation closure, emits `JourneyInstanceCompleted`
    /// when the walk just finished, and persists the new snapshot.
    /// Audit-first ordering: the closure runs after the caller's event
    /// was already accepted, and the completion event (if any) before
    /// the save.
    fn apply_and_persist(
        &self,
        instance: &mut JourneyInstance,
        definition: &JourneyDefinition,
        now: time::OffsetDateTime,
        actor: &forhemit_contracts::ActorRecord,
        mutate: impl FnOnce(&mut JourneyInstance, &JourneyDefinition, time::OffsetDateTime),
    ) -> Result<(), JourneyError> {
        let was_in_progress = instance.status == InstanceStatus::InProgress;
        mutate(instance, definition, now);

        if was_in_progress && instance.status == InstanceStatus::Completed {
            let answered = instance
                .answers_by_scope
                .values()
                .map(|records| records.len())
                .sum::<usize>();
            let marked = instance
                .nonnegotiables
                .iter()
                .map(|mark| mark.target.clone())
                .collect::<Vec<_>>();
            self.emit(
                AuditEventType::JourneyInstanceCompleted,
                instance.instance_id.as_str().to_owned(),
                actor,
                instance.correlation_id.clone(),
                None,
                &InstanceCompletedPayload {
                    answered_nodes: answered,
                    skipped_nodes: instance.skipped.len(),
                    marked_nonnegotiables: marked,
                },
            )?;
        }

        self.store.save(&instance.to_document())
    }

    /// Builds and emits one audit event for the journey engine.
    fn emit<P: Serialize>(
        &self,
        event_type: AuditEventType,
        source_object: String,
        actor: &forhemit_contracts::ActorRecord,
        correlation_id: forhemit_contracts::CorrelationId,
        causation_id: Option<forhemit_contracts::CausationId>,
        payload: &P,
    ) -> Result<(), JourneyError> {
        self.audit
            .emit(AuditEventDraft {
                event_type,
                source_engine: EngineId::Journey,
                source_object: ObjectId::new(source_object)
                    .unwrap_or_else(|_| unreachable!("engine callers pass non-empty object ids")),
                actor: actor.clone(),
                workspace_id: self.workspace_id.clone(),
                transaction_id: None,
                correlation_id,
                causation_id,
                payload: serde_json::to_value(payload).map_err(JourneyError::Serialization)?,
            })
            .map_err(JourneyError::Audit)
    }
}
