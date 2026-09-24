//! The Destination Engine — every material change, versioned and audited.
//!
//! Destination Engine doc §46 rule 8: "Every material change is versioned."
//! The engine owns the mutation rules: it validates, diffs, appends a new
//! immutable version, updates process status, and emits the audit event —
//! emission happens inside the engine, never in the UI command layer. A
//! mutation whose audit event cannot be recorded fails outright: an
//! unrecorded change is a silent history rewrite by another name.

use crate::completeness::Completeness;
use crate::content::{self, DestinationContent};
use crate::destination::Destination;
use crate::error::DestinationError;
use crate::status::DestinationStatus;
use crate::version::{ChangeReason, DestinationVersion};
use forhemit_contracts::{
    ActionOrigination, ActorClassification, ActorId, ActorKind, ActorRecord, AuditEventDraft,
    AuditEventType, CausationId, CorrelationId, DestinationId, DestinationVersionId, EngineId,
    ObjectId, WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};

/// The destination engine: validates and versions destination mutations,
/// emitting one audit event per material change.
pub struct DestinationEngine<'a> {
    clock: &'a dyn Clock,
    audit: &'a dyn AuditSink<AuditEventDraft>,
    workspace_id: WorkspaceId,
}

/// Result of a material edit: the version now at the head of history.
#[derive(Clone, Debug)]
pub struct Edited {
    /// The version that was appended.
    pub new_version_id: DestinationVersionId,
}

impl<'a> DestinationEngine<'a> {
    /// Creates an engine bound to a workspace, a clock, and the audit sink.
    pub fn new(
        clock: &'a dyn Clock,
        audit: &'a dyn AuditSink<AuditEventDraft>,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            clock,
            audit,
            workspace_id,
        }
    }

    /// Creates a destination from its initial content — "the first object
    /// created by every journey" (Destination First - Locked Core
    /// Architecture). Emits `DestinationVersionCreated`.
    pub fn create(
        &self,
        destination_id: DestinationId,
        content: DestinationContent,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
    ) -> Result<Destination, DestinationError> {
        content::validate_content(&content)?;
        let destination = Destination::create(
            destination_id.clone(),
            self.workspace_id.clone(),
            content,
            actor.clone(),
            self.clock.now(),
        );
        let version = destination.head_version();
        self.emit(
            AuditEventType::DestinationVersionCreated,
            destination_id.as_str().to_owned(),
            actor,
            correlation_id,
            None,
            serde_json::json!({
                "destination_id": destination_id.as_str(),
                "version_id": version.version_id.as_str(),
                "version_number": version.version_number,
                "initial": true,
            }),
        )?;
        Ok(destination)
    }

    /// Applies a material edit: validates the new content, diffs it against
    /// the head, appends a new immutable version, and emits
    /// `DestinationVersionCreated`. The previous version is retained
    /// untouched; a confirmed destination becomes Revised.
    #[allow(clippy::too_many_arguments)] // every parameter is doc-named material-change context
    pub fn edit(
        &self,
        destination: &mut Destination,
        new_content: DestinationContent,
        reason: ChangeReason,
        explanation: Option<String>,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<Edited, DestinationError> {
        if destination.status == DestinationStatus::Archived {
            return Err(DestinationError::IllegalTransition {
                from: destination.status.as_str().to_owned(),
                to: "a new version".to_owned(),
            });
        }
        content::validate_content(&new_content)?;

        let changed_fields = content::diff(destination.head_content(), &new_content);
        if changed_fields.is_empty() {
            return Err(DestinationError::NoMaterialChange);
        }

        let previous_version_id = destination.head_version().version_id.clone();
        let new_version = DestinationVersion::successor(
            destination.head_version(),
            new_content,
            reason.clone(),
            explanation,
            changed_fields.clone(),
            actor.clone(),
            self.clock.now(),
        );
        let new_version_id = new_version.version_id.clone();
        let new_version_number = new_version.version_number;

        // A confirmed destination becomes Revised (doc §6: "the owner has
        // materially changed the destination after additional information
        // or professional feedback").
        let becomes_revised = matches!(
            destination.status,
            DestinationStatus::Confirmed | DestinationStatus::UnderProfessionalReview
        );

        // Audit first: the event is emitted before the aggregate is
        // touched. If the sink refuses the event, the mutation is refused
        // with it — no version, no status change, nothing to roll back.
        self.emit(
            AuditEventType::DestinationVersionCreated,
            destination.destination_id.as_str().to_owned(),
            actor,
            correlation_id,
            causation_id,
            serde_json::json!({
                "destination_id": destination.destination_id.as_str(),
                "version_id": new_version_id.as_str(),
                "previous_version_id": previous_version_id.as_str(),
                "version_number": new_version_number,
                "changed_fields": changed_fields,
                "change_reason": reason,
            }),
        )?;

        destination.append_version(new_version);
        if becomes_revised {
            destination.set_status(DestinationStatus::Revised);
        }
        Ok(Edited { new_version_id })
    }

    /// The owner confirms: "This is the outcome I'm trying to create"
    /// (Builder doc §17, screen 13 "Yes, this is it"). Emits
    /// `DestinationConfirmed`.
    pub fn confirm(
        &self,
        destination: &mut Destination,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<(), DestinationError> {
        self.transition(
            destination,
            DestinationStatus::Confirmed,
            AuditEventType::DestinationConfirmed,
            actor,
            correlation_id,
            causation_id,
        )
    }

    /// The owner keeps the destination as a working draft — Builder doc
    /// §17's "I'm not sure yet" outcome. Emits `DestinationMarkedWorking`.
    pub fn mark_working(
        &self,
        destination: &mut Destination,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<(), DestinationError> {
        self.transition(
            destination,
            DestinationStatus::Working,
            AuditEventType::DestinationMarkedWorking,
            actor,
            correlation_id,
            causation_id,
        )
    }

    /// Archives the destination — "retained historically but no longer
    /// active" (doc §6). Emits `DestinationArchived`.
    pub fn archive(
        &self,
        destination: &mut Destination,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<(), DestinationError> {
        self.transition(
            destination,
            DestinationStatus::Archived,
            AuditEventType::DestinationArchived,
            actor,
            correlation_id,
            causation_id,
        )
    }

    /// The score-free completeness indicator of the head content.
    pub fn completeness(&self, destination: &Destination) -> Completeness {
        Completeness::of(destination.head_content())
    }

    /// A legal status transition plus its audit event.
    fn transition(
        &self,
        destination: &mut Destination,
        to: DestinationStatus,
        event_type: AuditEventType,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<(), DestinationError> {
        if destination.status == DestinationStatus::Archived {
            return Err(DestinationError::IllegalTransition {
                from: destination.status.as_str().to_owned(),
                to: to.as_str().to_owned(),
            });
        }
        if destination.status == to {
            return Err(DestinationError::IllegalTransition {
                from: to.as_str().to_owned(),
                to: to.as_str().to_owned(),
            });
        }
        // Audit first: the status change is applied only if the event is
        // accepted, so a refused event leaves the destination untouched.
        self.emit(
            event_type,
            destination.destination_id.as_str().to_owned(),
            actor,
            correlation_id,
            causation_id,
            serde_json::json!({
                "destination_id": destination.destination_id.as_str(),
                "version_id": destination.head_version().version_id.as_str(),
                "status": to,
            }),
        )?;
        destination.set_status(to);
        Ok(())
    }

    /// Emits one audit event; failure is the caller's failure.
    fn emit(
        &self,
        event_type: AuditEventType,
        source_object: String,
        actor: &ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
        payload: serde_json::Value,
    ) -> Result<(), DestinationError> {
        self.audit
            .emit(AuditEventDraft {
                event_type,
                source_engine: EngineId::Destination,
                source_object: ObjectId::new(source_object)
                    .unwrap_or_else(|_| unreachable!("a destination id is never empty")),
                actor: actor.clone(),
                workspace_id: self.workspace_id.clone(),
                transaction_id: None,
                correlation_id,
                causation_id,
                payload,
            })
            .map_err(|error| DestinationError::AuditRefused(error.to_string()))
    }
}

/// Convenience constructor for an owner actor record.
pub fn owner_actor(actor_id: &str) -> ActorRecord {
    ActorRecord {
        actor_id: ActorId::new(actor_id)
            .unwrap_or_else(|_| unreachable!("callers pass non-empty actor ids")),
        classification: ActorClassification::Human,
        kind: Some(ActorKind::Owner),
        origination: Some(ActionOrigination::HumanInitiated),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use crate::answer::Answer;
    use forhemit_enginekit::SystemClock;
    use std::sync::Mutex;

    /// In-memory audit sink that records event types for assertions.
    #[derive(Default)]
    struct MemoryAudit {
        events: Mutex<Vec<AuditEventDraft>>,
    }

    impl MemoryAudit {
        fn event_types(&self) -> Vec<AuditEventType> {
            self.events
                .lock()
                .unwrap()
                .iter()
                .map(|event| event.event_type.clone())
                .collect()
        }
    }

    impl AuditSink<AuditEventDraft> for MemoryAudit {
        fn emit(&self, event: AuditEventDraft) -> Result<(), forhemit_enginekit::AuditError> {
            self.events.lock().unwrap().push(event);
            Ok(())
        }
    }

    fn engine<'a>(audit: &'a MemoryAudit) -> DestinationEngine<'a> {
        DestinationEngine::new(&SystemClock, audit, WorkspaceId::new("ws-1").unwrap())
    }

    fn actor() -> ActorRecord {
        owner_actor("owner-1")
    }

    fn correlation() -> CorrelationId {
        CorrelationId::new("cor-1").unwrap()
    }

    #[test]
    fn create_emits_a_version_created_event() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();
        assert_eq!(destination.history().len(), 1);
        assert_eq!(
            audit.event_types(),
            vec![AuditEventType::DestinationVersionCreated]
        );
    }

    #[test]
    fn an_edit_appends_and_retains() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();
        let v1_id = destination.head_version().version_id.clone();
        let v1_content = destination.head_content().clone();

        let mut new_content = DestinationContent::default();
        new_content.transition_timing.value =
            Answer::Answered(crate::content::TransitionTiming::Within12Months);
        engine
            .edit(
                &mut destination,
                new_content,
                ChangeReason::PrioritiesChanged,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap();

        // v1 retained exactly as it was; v2 is the head and links back.
        assert_eq!(destination.history().len(), 2);
        assert_eq!(destination.version(&v1_id).unwrap().content, v1_content);
        assert_eq!(destination.head_version().previous_version_id, Some(v1_id));
        assert_eq!(destination.head_version().version_number, 2);
    }

    #[test]
    fn editing_a_confirmed_destination_marks_it_revised() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();
        engine
            .confirm(&mut destination, &actor(), correlation(), None)
            .unwrap();

        let mut new_content = DestinationContent::default();
        new_content.owner_role.value =
            Answer::Answered(crate::content::OwnerRole::TransitionAdvisor);
        engine
            .edit(
                &mut destination,
                new_content,
                ChangeReason::LearnedSomethingNew,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap();

        assert_eq!(destination.status, DestinationStatus::Revised);
        assert_eq!(
            audit.event_types(),
            vec![
                AuditEventType::DestinationVersionCreated,
                AuditEventType::DestinationConfirmed,
                AuditEventType::DestinationVersionCreated,
            ]
        );
    }

    #[test]
    fn a_non_material_edit_creates_no_version_and_no_event() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();

        let error = engine
            .edit(
                &mut destination,
                DestinationContent::default(),
                ChangeReason::Other,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap_err();

        assert_eq!(error, DestinationError::NoMaterialChange);
        assert_eq!(destination.history().len(), 1);
        assert_eq!(
            audit.event_types(),
            vec![AuditEventType::DestinationVersionCreated]
        );
    }

    #[test]
    fn an_invalid_allocation_is_refused_before_any_version_is_created() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();

        let mut bad = DestinationContent::default();
        bad.ownership_allocation.value =
            Answer::Answered(crate::content::AllocationChoice::Percentages(vec![
                crate::content::AllocationShare {
                    participant: crate::content::AllocationParticipant::Employees,
                    percent: 60,
                },
                crate::content::AllocationShare {
                    participant: crate::content::AllocationParticipant::Management,
                    percent: 50,
                },
            ]));
        let error = engine
            .edit(
                &mut destination,
                bad,
                ChangeReason::Other,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap_err();

        assert_eq!(
            error,
            DestinationError::AllocationDoesNotTotal100 { total: 110 }
        );
        assert_eq!(destination.history().len(), 1);
    }

    #[test]
    fn nonnegotiable_states_persist_across_versions() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut initial = DestinationContent::default();
        initial.financial_objective.value =
            Answer::Answered(crate::content::FinancialObjective::CashNow);
        initial.financial_objective.preference =
            forhemit_contracts::NonnegotiableState::Nonnegotiable;

        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                initial,
                &actor(),
                correlation(),
            )
            .unwrap();
        let v1_objective_id = destination
            .head_content()
            .financial_objective
            .objective_id
            .clone();

        // The UI's edit pattern: clone the current content, change one
        // answer, submit. The nonnegotiable designation and the stable
        // objective id must survive into the new version.
        let mut edited = destination.head_content().clone();
        edited.transition_timing.value =
            Answer::Answered(crate::content::TransitionTiming::Within12Months);
        engine
            .edit(
                &mut destination,
                edited,
                ChangeReason::PrioritiesChanged,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap();

        let v2 = destination.head_content();
        assert_eq!(v2.financial_objective.objective_id, v1_objective_id);
        assert_eq!(
            v2.financial_objective.preference,
            forhemit_contracts::NonnegotiableState::Nonnegotiable
        );
        // And the back-reference machinery still sees it.
        let designated = destination.designated_objectives();
        assert_eq!(designated.len(), 1);
        assert_eq!(designated[0].0, v1_objective_id);
    }

    #[test]
    fn archived_destinations_refuse_edits_and_transitions() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();
        engine
            .archive(&mut destination, &actor(), correlation(), None)
            .unwrap();

        let edit_error = engine
            .edit(
                &mut destination,
                DestinationContent::default(),
                ChangeReason::Other,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap_err();
        assert!(matches!(
            edit_error,
            DestinationError::IllegalTransition { .. }
        ));

        let confirm_error = engine
            .confirm(&mut destination, &actor(), correlation(), None)
            .unwrap_err();
        assert!(matches!(
            confirm_error,
            DestinationError::IllegalTransition { .. }
        ));
    }

    #[test]
    fn audit_failure_refuses_the_mutation() {
        struct BrokenAudit;
        impl AuditSink<AuditEventDraft> for BrokenAudit {
            fn emit(&self, _event: AuditEventDraft) -> Result<(), forhemit_enginekit::AuditError> {
                Err(forhemit_enginekit::AuditError(
                    "audit store unavailable".to_owned(),
                ))
            }
        }

        let engine = DestinationEngine::new(
            &SystemClock,
            &BrokenAudit,
            WorkspaceId::new("ws-1").unwrap(),
        );
        let error = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap_err();
        assert!(matches!(error, DestinationError::AuditRefused(_)));
    }

    /// Audit-first consistency: a refused event must leave the aggregate
    /// exactly as it was — no appended version, no status change.
    #[test]
    fn an_audit_failure_leaves_the_destination_untouched() {
        struct FlakyAudit(Mutex<bool>); // true = refuse the next event
        impl AuditSink<AuditEventDraft> for FlakyAudit {
            fn emit(&self, _event: AuditEventDraft) -> Result<(), forhemit_enginekit::AuditError> {
                if *self.0.lock().unwrap() {
                    Err(forhemit_enginekit::AuditError(
                        "audit store unavailable".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }

        let audit = FlakyAudit(Mutex::new(false));
        let engine =
            DestinationEngine::new(&SystemClock, &audit, WorkspaceId::new("ws-1").unwrap());
        let mut destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                DestinationContent::default(),
                &actor(),
                correlation(),
            )
            .unwrap();
        let head_before = destination.head_version().version_id.clone();
        let status_before = destination.status.clone();

        // A material edit with the sink refusing: refused outright...
        *audit.0.lock().unwrap() = true;
        let mut new_content = DestinationContent::default();
        new_content.transition_timing.value =
            Answer::Answered(crate::content::TransitionTiming::Within12Months);
        let edit_error = engine
            .edit(
                &mut destination,
                new_content,
                ChangeReason::PrioritiesChanged,
                None,
                &actor(),
                correlation(),
                None,
            )
            .unwrap_err();
        assert!(matches!(edit_error, DestinationError::AuditRefused(_)));

        // ...and the aggregate is untouched: same head, same status,
        // same history length.
        assert_eq!(destination.history().len(), 1);
        assert_eq!(destination.head_version().version_id, head_before);
        assert_eq!(destination.status, status_before);

        // A status transition under a refusing sink behaves the same way.
        let confirm_error = engine
            .confirm(&mut destination, &actor(), correlation(), None)
            .unwrap_err();
        assert!(matches!(confirm_error, DestinationError::AuditRefused(_)));
        assert_eq!(destination.status, status_before);
    }

    #[test]
    fn completeness_is_derived_not_stored() {
        let audit = MemoryAudit::default();
        let engine = engine(&audit);
        let mut initial = DestinationContent::default();
        initial.financial_objective.value =
            Answer::Answered(crate::content::FinancialObjective::CashNow);
        let destination = engine
            .create(
                DestinationId::new("dst-1").unwrap(),
                initial,
                &actor(),
                correlation(),
            )
            .unwrap();

        let completeness = engine.completeness(&destination);
        assert_eq!(
            completeness.financial,
            crate::completeness::AreaState::Established
        );
        assert_eq!(
            completeness.ownership,
            crate::completeness::AreaState::NotEstablished
        );
        assert!(!completeness.ready_for_exploration());
    }
}
