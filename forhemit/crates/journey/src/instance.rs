//! The Journey Instance — one owner's stateful walk of one definition.
//!
//! "When a user starts a journey, create: Journey Instance … the journey
//! is therefore a stateful process, not simply a form" (Journey Builder
//! doc §8). The instance carries the walk position, the versioned answers
//! bucketed by their declared Storage Scope, recorded skips, and derived
//! nonnegotiables.
//!
//! Every mutation recomputes the position: the first eligible question
//! that is neither answered nor skipped. Eligibility is the question's
//! "Ask Only When Relevant" condition evaluated against the latest
//! answers, so editing an earlier answer recalculates which later
//! questions are relevant (Journey Builder doc §9) — and an answer that
//! made a question relevant can, on revision, stop making it relevant.

use std::collections::BTreeMap;

use forhemit_contracts::{CorrelationId, JourneyId, JourneyInstanceId, JourneyNodeId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::answer::{AnswerRecord, AnswerValue};
use crate::definition::{Effect, JourneyDefinition};
use crate::error::JourneyError;
use crate::scope::StorageScope;

/// Whether the walk has run its course.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceStatus {
    /// Questions remain to answer or exit.
    InProgress,
    /// Every eligible question is answered or skipped — the walk has
    /// reached its end and the outputs may be assembled.
    Completed,
}

/// A nonnegotiable derived from an answer through a definition effect —
/// EOJ v0.2 §11: choosing "Essential" for employee ownership marks it
/// nonnegotiable, and "the system doesn't need to ask this again later".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarkedNonnegotiable {
    /// The requirement marked nonnegotiable (the effect's `target`).
    pub target: String,
    /// The answer node that marked it.
    pub node_id: JourneyNodeId,
    /// When the marking was recorded.
    pub recorded_at: OffsetDateTime,
}

/// The walk's state — the persisted document's payload.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyInstance {
    /// The instance's identity.
    pub instance_id: JourneyInstanceId,
    /// The correlation id every event of this walk shares (Audit doc
    /// §18: "Show me everything that happened because of this change").
    pub correlation_id: CorrelationId,
    /// The journey definition the instance walks.
    pub journey_id: JourneyId,
    /// The definition version the instance was started under — recorded
    /// on every output (Journey Builder doc §23: a package records
    /// "Journey Version Used").
    pub journey_version: String,
    /// Whether the walk has completed.
    pub status: InstanceStatus,
    /// The question the walk is currently asking (None once completed).
    pub current_node: Option<JourneyNodeId>,
    /// Versioned answers, bucketed by the Storage Scope their questions
    /// declared (Journey Builder doc §6).
    pub answers_by_scope: BTreeMap<StorageScope, Vec<AnswerRecord>>,
    /// Questions the owner passed without answering — question exits
    /// (EOJ v0.2 §30), in the order they were taken.
    pub skipped: Vec<JourneyNodeId>,
    /// Nonnegotiables derived from answers through definition effects.
    pub nonnegotiables: Vec<MarkedNonnegotiable>,
    /// The nodes the walk has visited, in visit order — answers and
    /// question exits alike; the audit stream holds the details.
    pub visited: Vec<JourneyNodeId>,
    /// When the instance was created.
    pub started_at: OffsetDateTime,
    /// When the instance last changed.
    pub updated_at: OffsetDateTime,
    /// When the walk completed, if it has.
    pub completed_at: Option<OffsetDateTime>,
}

/// The versioned envelope of a persisted walk (spec: "state persists
/// across app restarts"). Stored data is never reinterpreted in place —
/// a new document version is a new variant.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "instance_version", rename_all = "snake_case")]
pub enum JourneyInstanceDocument {
    /// Version 1 of the journey instance document.
    V1 {
        /// The walk's state.
        state: JourneyInstance,
    },
}

impl JourneyInstanceDocument {
    /// The walk state inside the envelope, whatever its version.
    pub fn state(&self) -> &JourneyInstance {
        match self {
            Self::V1 { state } => state,
        }
    }

    /// Mutable access to the walk state — store-side tests and repair
    /// paths only; the engine drives real walks through its commands.
    pub fn state_mut(&mut self) -> &mut JourneyInstance {
        match self {
            Self::V1 { state } => state,
        }
    }
}

impl JourneyInstance {
    /// Starts a walk of `definition` — the fresh instance positioned on
    /// the definition's first eligible question.
    pub(crate) fn start(
        instance_id: JourneyInstanceId,
        correlation_id: CorrelationId,
        definition: &JourneyDefinition,
        now: OffsetDateTime,
    ) -> Self {
        let mut instance = Self {
            instance_id,
            correlation_id,
            journey_id: definition.journey_id.clone(),
            journey_version: definition.version.as_str().to_owned(),
            status: InstanceStatus::InProgress,
            current_node: None,
            answers_by_scope: BTreeMap::new(),
            skipped: Vec::new(),
            nonnegotiables: Vec::new(),
            visited: Vec::new(),
            started_at: now,
            updated_at: now,
            completed_at: None,
        };
        instance.current_node = instance.next_position(definition);
        instance
    }

    /// The answer record for a node, if the walk has one — scanned
    /// across scope buckets.
    pub fn answer(&self, node_id: &JourneyNodeId) -> Option<&AnswerRecord> {
        self.answers_by_scope
            .values()
            .flat_map(|records| records.iter())
            .find(|record| &record.node_id == node_id)
    }

    /// The latest answer value for a node, if any — how conditions read
    /// the answers so far.
    pub fn latest_value(&self, node_id: &JourneyNodeId) -> Option<AnswerValue> {
        self.answer(node_id)
            .map(|record| record.latest().value.clone())
    }

    /// Whether the node was passed without an answer.
    pub fn is_skipped(&self, node_id: &JourneyNodeId) -> bool {
        self.skipped.contains(node_id)
    }

    /// Answers bucketed under one storage scope.
    pub fn answers_in_scope(&self, scope: StorageScope) -> Vec<&AnswerRecord> {
        self.answers_by_scope
            .get(&scope)
            .map(|records| records.iter().collect())
            .unwrap_or_default()
    }

    /// The eligibility check ("Ask Only When Relevant", EOJ v0.2 §28):
    /// a question with no condition is always eligible; a conditional
    /// question fires only when its condition holds on the answers so
    /// far. Public read-only: the shell's view layer reports this same
    /// predicate per answered node (the `revisable` view flag) so the
    /// history rail never offers a revision the engine would refuse.
    pub fn is_eligible(&self, definition: &JourneyDefinition, node_id: &JourneyNodeId) -> bool {
        let Some(node) = definition.node(node_id) else {
            return false;
        };
        let Some(question) = node.question() else {
            return false;
        };
        match &question.show_when {
            None => true,
            Some(condition) => {
                let latest = |question_node: &JourneyNodeId| self.latest_value(question_node);
                condition.evaluate(&latest)
            }
        }
    }

    /// The walk's next position: the first question, in definition
    /// order, that is eligible and neither answered nor skipped.
    pub(crate) fn next_position(&self, definition: &JourneyDefinition) -> Option<JourneyNodeId> {
        definition
            .question_nodes()
            .map(|node| &node.node_id)
            .find(|node_id| {
                self.is_eligible(definition, node_id)
                    && self.answer(node_id).is_none()
                    && !self.is_skipped(node_id)
            })
            .cloned()
    }

    /// Files a new answer version under its node's declared scope,
    /// recomputes the walk, and reports whether the walk just completed.
    /// Called by the engine only after its audit event was accepted — a
    /// walk state never exists without its event.
    pub(crate) fn apply_answer(
        &mut self,
        definition: &JourneyDefinition,
        record: AnswerRecord,
        now: OffsetDateTime,
    ) {
        let scope = record.latest().storage_scope;
        let node_id = record.node_id.clone();
        let mut new_version = record.versions;
        let version = new_version.pop().unwrap_or_else(|| {
            unreachable!("the record being filed always carries its new version")
        });
        let bucket = self.answers_by_scope.entry(scope).or_default();
        match bucket
            .iter_mut()
            .find(|existing| existing.node_id == node_id)
        {
            Some(existing) => existing.versions.push(version),
            None => bucket.push(AnswerRecord {
                answer_id: record.answer_id,
                node_id: record.node_id,
                versions: vec![version],
            }),
        }
        if !self.visited.contains(&node_id) {
            self.visited.push(node_id);
        }
        self.updated_at = now;
        self.recompute(definition);
    }

    /// Records a question exit and recomputes the walk.
    pub(crate) fn apply_skip(
        &mut self,
        definition: &JourneyDefinition,
        node_id: JourneyNodeId,
        now: OffsetDateTime,
    ) {
        if !self.skipped.contains(&node_id) {
            self.skipped.push(node_id.clone());
        }
        if !self.visited.contains(&node_id) {
            self.visited.push(node_id);
        }
        self.updated_at = now;
        self.recompute(definition);
    }

    /// Re-evaluates derived nonnegotiables for an answer just recorded —
    /// an effect that fires adds its marking; one that stops firing
    /// never un-marks it (a nonnegotiable is never silently relaxed).
    /// Returns the targets this answer newly marked.
    pub(crate) fn apply_effects(
        &mut self,
        definition: &JourneyDefinition,
        node_id: &JourneyNodeId,
        value: &AnswerValue,
        now: OffsetDateTime,
    ) -> Vec<String> {
        let Some(question) = definition.node(node_id).and_then(|node| node.question()) else {
            return Vec::new();
        };
        let mut marked = Vec::new();
        for effect in &question.effects {
            let when = match effect {
                Effect::MarkNonnegotiable { when, .. }
                | Effect::MarkAnswerValuesNonnegotiable { when } => when,
            };
            // Effects read the answer just recorded, even though the
            // record itself is applied afterwards by `apply_answer`.
            let fired = {
                let latest_for_condition = |question_node: &JourneyNodeId| {
                    if question_node == node_id {
                        Some(value.clone())
                    } else {
                        self.latest_value(question_node)
                    }
                };
                when.evaluate(&latest_for_condition)
            };
            if !fired {
                continue;
            }
            let mut targets: Vec<String> = match effect {
                Effect::MarkNonnegotiable { target, .. } => vec![target.clone()],
                Effect::MarkAnswerValuesNonnegotiable { .. } => value
                    .scalars()
                    .into_iter()
                    .map(|scalar| {
                        // Prefer the choice's human label; a value drawn
                        // from another question's answers falls back to
                        // its raw value.
                        question
                            .choices
                            .iter()
                            .find(|choice| choice.value == scalar)
                            .map(|choice| choice.label.clone())
                            .unwrap_or_else(|| scalar.to_owned())
                    })
                    .collect(),
            };
            for target in targets.drain(..) {
                if !self
                    .nonnegotiables
                    .iter()
                    .any(|existing| existing.target == target)
                {
                    self.nonnegotiables.push(MarkedNonnegotiable {
                        target: target.clone(),
                        node_id: node_id.clone(),
                        recorded_at: now,
                    });
                    marked.push(target);
                }
            }
        }
        marked
    }

    /// Recomputes position and status after any mutation.
    fn recompute(&mut self, definition: &JourneyDefinition) {
        self.current_node = self.next_position(definition);
        if self.current_node.is_none() {
            if self.status != InstanceStatus::Completed {
                self.status = InstanceStatus::Completed;
                self.completed_at = Some(self.updated_at);
            }
        } else {
            // A revision that made new questions relevant reopens a
            // completed walk — never leaves it stranded at Completed.
            self.status = InstanceStatus::InProgress;
            self.completed_at = None;
        }
    }

    /// The versioned persisted form.
    pub fn to_document(&self) -> JourneyInstanceDocument {
        JourneyInstanceDocument::V1 {
            state: self.clone(),
        }
    }

    /// Restores a walk from its persisted form.
    ///
    /// # Errors
    ///
    /// [`JourneyError::StoreState`] when the document is not the
    /// current instance version — older readers refuse newer documents
    /// instead of reinterpreting them.
    pub fn from_document(document: JourneyInstanceDocument) -> Result<Self, JourneyError> {
        match document {
            JourneyInstanceDocument::V1 { state } => Ok(state),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use forhemit_contracts::CorrelationId;

    use super::*;

    fn fresh_instance() -> JourneyInstance {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let definition = crate::load_employee_ownership_v0_2().unwrap();
        JourneyInstance::start(
            JourneyInstanceId::new("inst_1").unwrap(),
            CorrelationId::new("cor_1").unwrap(),
            &definition,
            now,
        )
    }

    #[test]
    fn document_round_trip_preserves_state() {
        let instance = fresh_instance();
        let json = serde_json::to_string(&instance.to_document()).unwrap();
        let restored: JourneyInstanceDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(instance, JourneyInstance::from_document(restored).unwrap());
    }

    #[test]
    fn document_carries_the_version_tag() {
        let instance = fresh_instance();
        let json = serde_json::to_value(instance.to_document()).unwrap();
        assert_eq!(json.get("instance_version"), Some(&serde_json::json!("v1")));
    }

    #[test]
    fn a_fresh_walk_positions_on_the_first_question() {
        let instance = fresh_instance();
        assert_eq!(
            instance.current_node.as_ref().map(JourneyNodeId::as_str),
            Some("primary_objective")
        );
        assert_eq!(instance.status, InstanceStatus::InProgress);
        assert!(instance.answers_by_scope.is_empty());
    }
}
