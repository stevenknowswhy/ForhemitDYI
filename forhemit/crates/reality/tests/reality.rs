//! Integration tests for the Business Reality engine — the task's
//! verification matrix: provenance round-trips, audit emission per
//! mutation, version history retained, and no shortcut that fakes
//! confidence or verification.
#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test loudly

use std::sync::{Arc, Mutex};

use forhemit_contracts::{
    ActorClassification, ActorId, ActorKind, ActorRecord, AuditEventDraft, AuditEventType,
    CorrelationId, EngineId, FactId, Provenance, Verification, WorkspaceId,
};
use forhemit_enginekit::{AuditError, AuditSink, Clock};
use forhemit_reality::{
    FactKind, FactPeriod, FactRange, FactRecordedPayload, FactRevisedPayload, FactValue,
    FactVersion, RealityEngine, RealityError, RecordFact, ReviseFact,
};
use time::{Duration, OffsetDateTime};

/// Captures every emitted draft so tests can assert on the events.
#[derive(Default)]
struct RecordingSink {
    events: Mutex<Vec<AuditEventDraft>>,
}

impl RecordingSink {
    fn events(&self) -> Vec<AuditEventDraft> {
        self.events.lock().unwrap().clone()
    }
}

impl AuditSink<AuditEventDraft> for RecordingSink {
    fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
}

/// Always rejects — the audit-first ordering test's adversary.
struct FailingSink;

impl AuditSink<AuditEventDraft> for FailingSink {
    fn emit(&self, _event: AuditEventDraft) -> Result<(), AuditError> {
        Err(AuditError("store unavailable".to_owned()))
    }
}

/// Advanceable fixed clock so recorded_at ordering is deterministic.
struct TestClock(Mutex<OffsetDateTime>);

impl TestClock {
    fn new() -> Arc<Self> {
        Arc::new(Self(Mutex::new(
            OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap(),
        )))
    }

    fn advance(&self, seconds: i64) {
        *self.0.lock().unwrap() += Duration::seconds(seconds);
    }
}

impl Clock for TestClock {
    fn now(&self) -> OffsetDateTime {
        *self.0.lock().unwrap()
    }
}

fn owner() -> ActorRecord {
    ActorRecord {
        actor_id: ActorId::new("owner_stefano").unwrap(),
        classification: ActorClassification::Human,
        kind: Some(ActorKind::Owner),
        origination: None,
    }
}

fn correlation() -> CorrelationId {
    CorrelationId::new("COR-journey-step-7").unwrap()
}

fn engine(clock: &Arc<TestClock>) -> (Arc<RecordingSink>, RealityEngine<RecordingSink>) {
    let sink = Arc::new(RecordingSink::default());
    let engine = RealityEngine::new(
        sink.clone(),
        clock.clone(),
        WorkspaceId::new("ws_main").unwrap(),
    );
    (sink, engine)
}

fn revenue_record() -> RecordFact {
    RecordFact {
        kind: FactKind::Revenue,
        value: FactValue::Range(FactRange::new(Some(5_000_000), Some(10_000_000)).unwrap()),
        period: FactPeriod::Current,
        definition: None,
        actor: owner(),
        correlation_id: correlation(),
    }
}

fn revise_revenue(fact_id: FactId, value: FactValue) -> ReviseFact {
    ReviseFact {
        fact_id,
        value,
        period: FactPeriod::Current,
        definition: None,
        change_reason: "2025 books closed".to_owned(),
        actor: owner(),
        correlation_id: correlation(),
    }
}

/// The engine's whole public surface is record/revise; every fact either
/// produces must carry the owner-reported stamps (Business Reality doc
/// §29: never "treat estimates as verified information").
#[test]
fn every_fact_the_api_can_produce_is_owner_reported_and_unverified() {
    let clock = TestClock::new();
    let (_sink, engine) = engine(&clock);

    let kinds = [
        (
            FactKind::Industry,
            FactValue::Text("specialty manufacturing".to_owned()),
        ),
        (FactKind::YearsOperating, FactValue::Number(15)),
        (
            FactKind::Revenue,
            FactValue::Range(FactRange::new(Some(5_000_000), Some(10_000_000)).unwrap()),
        ),
        (
            FactKind::OperatingCashFlow,
            FactValue::Range(FactRange::new(Some(1_000_000), Some(2_000_000)).unwrap()),
        ),
        (
            FactKind::Debt,
            FactValue::Range(FactRange::new(None, Some(3_000_000)).unwrap()),
        ),
        (
            FactKind::EmployeeCount,
            FactValue::Range(FactRange::new(Some(40), Some(75)).unwrap()),
        ),
        (
            FactKind::OwnershipStructure,
            FactValue::Text("founder-owned".to_owned()),
        ),
    ];

    for (kind, value) in kinds {
        let fact = engine
            .record_fact(RecordFact {
                kind,
                value,
                period: FactPeriod::Current,
                definition: None,
                actor: owner(),
                correlation_id: correlation(),
            })
            .unwrap_or_else(|error| panic!("{kind:?}: {error}"));
        assert_eq!(fact.provenance(), Provenance::OwnerReported);
        assert_eq!(fact.verification(), Verification::Unverified);
    }

    // Revisions cannot change the stamps either. The snapshot is sorted
    // by kind; Revenue is the third variant.
    let revenue = engine.current_facts().unwrap().remove(2);
    let revised = engine
        .revise_fact(revise_revenue(
            revenue.fact_id().clone(),
            FactValue::Number(8_200_000),
        ))
        .unwrap();
    assert_eq!(revised.provenance(), Provenance::OwnerReported);
    assert_eq!(revised.verification(), Verification::Unverified);
}

#[test]
fn every_mutation_emits_exactly_one_audit_event() {
    let clock = TestClock::new();
    let (sink, engine) = engine(&clock);

    let industry = engine
        .record_fact(RecordFact {
            kind: FactKind::Industry,
            value: FactValue::Text("specialty manufacturing".to_owned()),
            period: FactPeriod::Current,
            definition: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();
    engine.record_fact(revenue_record()).unwrap();
    let revenue_id = engine
        .current_facts()
        .unwrap()
        .iter()
        .find(|fact| fact.kind() == FactKind::Revenue)
        .unwrap()
        .fact_id()
        .clone();
    engine
        .revise_fact(revise_revenue(revenue_id, FactValue::Number(8_200_000)))
        .unwrap();

    let events = sink.events();
    assert_eq!(events.len(), 3, "two records + one revision = three events");
    assert_eq!(events[0].event_type, AuditEventType::RealityFactRecorded);
    assert_eq!(events[1].event_type, AuditEventType::RealityFactRecorded);
    assert_eq!(events[2].event_type, AuditEventType::RealityFactRevised);
    for event in &events {
        assert_eq!(event.source_engine, EngineId::Reality);
        assert_eq!(event.workspace_id.as_str(), "ws_main");
        assert_eq!(event.correlation_id.as_str(), "COR-journey-step-7");
        assert_eq!(event.actor.actor_id.as_str(), "owner_stefano");
    }
    // The recorded events name their fact as the source object, so the
    // audit store's history_for_object reconstructs the fact's history.
    assert_eq!(
        events[0].source_object.as_str(),
        industry.fact_id().as_str()
    );
}

#[test]
fn revision_creates_a_new_version_and_retains_history() {
    let clock = TestClock::new();
    let (_sink, engine) = engine(&clock);

    let first = engine.record_fact(revenue_record()).unwrap();
    clock.advance(60);
    let second = engine
        .revise_fact(revise_revenue(
            first.fact_id().clone(),
            FactValue::Number(8_200_000),
        ))
        .unwrap();
    clock.advance(60);
    let third = engine
        .revise_fact(revise_revenue(
            first.fact_id().clone(),
            FactValue::Number(8_700_000),
        ))
        .unwrap();

    // The fact id is stable across versions — the handle scenario
    // assumptions reference.
    assert_eq!(first.fact_id(), second.fact_id());
    assert_eq!(first.fact_id(), third.fact_id());
    assert_ne!(first.fact_version_id(), second.fact_version_id());
    assert_ne!(second.fact_version_id(), third.fact_version_id());

    // The supersedes chain: v2 → v1, v3 → v2.
    assert!(first.supersedes().is_none());
    assert_eq!(second.supersedes(), Some(first.fact_version_id()));
    assert_eq!(third.supersedes(), Some(second.fact_version_id()));

    // History is retained, oldest first, and recorded_at advances with
    // the clock (Business Reality doc §26 — current as of).
    let history = engine.fact_history(first.fact_id()).unwrap();
    assert_eq!(history.len(), 3);
    assert_eq!(history[0], first);
    assert_eq!(history[1], second);
    assert_eq!(history[2], third);
    assert!(history[0].recorded_at() < history[1].recorded_at());
    assert!(history[1].recorded_at() < history[2].recorded_at());
    assert_eq!(engine.current_fact(first.fact_id()).unwrap(), third);
}

#[test]
fn revision_event_preserves_previous_and_new_values() {
    let clock = TestClock::new();
    let (sink, engine) = engine(&clock);

    let fact = engine.record_fact(revenue_record()).unwrap();
    let revised = engine
        .revise_fact(revise_revenue(
            fact.fact_id().clone(),
            FactValue::Number(8_200_000),
        ))
        .unwrap();

    let events = sink.events();
    assert_eq!(events.len(), 2);
    let payload: FactRevisedPayload = serde_json::from_value(events[1].payload.clone()).unwrap();
    assert_eq!(payload.fact_id, *revised.fact_id());
    assert_eq!(payload.fact_version_id, *revised.fact_version_id());
    assert_eq!(payload.kind, FactKind::Revenue);
    assert_eq!(payload.value, FactValue::Number(8_200_000));
    assert_eq!(payload.previous_version_id, *fact.fact_version_id());
    assert_eq!(
        payload.previous_value,
        FactValue::Range(FactRange::new(Some(5_000_000), Some(10_000_000)).unwrap())
    );
    assert_eq!(payload.change_reason, "2025 books closed");
    assert_eq!(payload.provenance, Provenance::OwnerReported);
    assert_eq!(payload.verification, Verification::Unverified);
}

#[test]
fn recorded_event_payload_carries_the_full_provenance() {
    let clock = TestClock::new();
    let (sink, engine) = engine(&clock);

    engine
        .record_fact(RecordFact {
            kind: FactKind::OperatingCashFlow,
            value: FactValue::Range(FactRange::new(Some(1_000_000), Some(2_000_000)).unwrap()),
            period: FactPeriod::FiscalYear(2025),
            definition: Some("owner's adjusted estimate".to_owned()),
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();

    let events = sink.events();
    assert_eq!(events.len(), 1);
    let payload: FactRecordedPayload = serde_json::from_value(events[0].payload.clone()).unwrap();
    assert_eq!(payload.kind, FactKind::OperatingCashFlow);
    assert_eq!(payload.period, FactPeriod::FiscalYear(2025));
    assert_eq!(
        payload.definition.as_deref(),
        Some("owner's adjusted estimate")
    );
    assert_eq!(payload.provenance, Provenance::OwnerReported);
    assert_eq!(payload.verification, Verification::Unverified);
}

#[test]
fn failed_mutations_leave_no_fact_and_no_event() {
    let clock = TestClock::new();
    let (sink, engine) = engine(&clock);

    // A value shape that does not fit the kind (Business Reality doc
    // §31: value and unit belong together).
    let mismatch = engine.record_fact(RecordFact {
        kind: FactKind::Industry,
        value: FactValue::Number(15),
        period: FactPeriod::Current,
        definition: None,
        actor: owner(),
        correlation_id: correlation(),
    });
    assert!(matches!(
        mismatch,
        Err(RealityError::KindValueMismatch { .. })
    ));

    // An inverted range, constructed as a literal (bypassing
    // FactRange::new) — the engine re-validates values built directly as
    // enum variants (fact.rs validate() contract).
    let inverted = engine.record_fact(RecordFact {
        kind: FactKind::Revenue,
        value: FactValue::Range(FactRange {
            lower: Some(10_000_000),
            upper: Some(5_000_000),
        }),
        period: FactPeriod::Current,
        definition: None,
        actor: owner(),
        correlation_id: correlation(),
    });
    assert!(matches!(
        inverted,
        Err(RealityError::InvalidFactRange { .. })
    ));

    let revenue = engine.record_fact(revenue_record()).unwrap();
    assert_eq!(sink.events().len(), 1);

    // A second fact of a kind the snapshot records once.
    let duplicate = engine.record_fact(revenue_record());
    assert!(matches!(
        duplicate,
        Err(RealityError::FactKindAlreadyRecorded { .. })
    ));

    // A revision without a reason.
    let reasonless = ReviseFact {
        change_reason: "   ".to_owned(),
        ..revise_revenue(revenue.fact_id().clone(), FactValue::Number(8_200_000))
    };
    assert!(matches!(
        engine.revise_fact(reasonless),
        Err(RealityError::EmptyChangeReason)
    ));

    // A revision of an unknown fact.
    let unknown = revise_revenue(FactId::new("fact_none").unwrap(), FactValue::Number(1));
    assert!(matches!(
        engine.revise_fact(unknown),
        Err(RealityError::FactNotFound { .. })
    ));

    assert_eq!(
        sink.events().len(),
        1,
        "failed mutations must not emit events"
    );
    assert_eq!(
        engine.fact_history(revenue.fact_id()).unwrap().len(),
        1,
        "failed mutations must not change history"
    );
}

#[test]
fn audit_rejection_leaves_no_fact_behind() {
    let clock = TestClock::new();
    let engine = RealityEngine::new(
        Arc::new(FailingSink),
        clock.clone(),
        WorkspaceId::new("ws_main").unwrap(),
    );

    let rejected = engine.record_fact(revenue_record());
    assert!(matches!(rejected, Err(RealityError::Audit(_))));
    assert!(
        engine.current_facts().unwrap().is_empty(),
        "audit-first: a fact exists only if its event was accepted"
    );
}

#[test]
fn ranges_accept_open_bounds_and_reject_malformed_ones() {
    // "Under $1M" and "$25M+" — the doc §7 range options.
    assert!(FactRange::new(None, Some(1_000_000)).is_ok());
    assert!(FactRange::new(Some(25_000_000), None).is_ok());
    assert!(FactRange::new(Some(5_000_000), Some(10_000_000)).is_ok());
    assert!(FactRange::new(None, None).is_err());
    assert!(FactRange::new(Some(10), Some(5)).is_err());

    let range = FactRange::new(Some(5_000_000), Some(10_000_000)).unwrap();
    let json = serde_json::to_string(&range).unwrap();
    assert_eq!(serde_json::from_str::<FactRange>(&json).unwrap(), range);
}

#[test]
fn fact_versions_roundtrip_through_serde_with_provenance() {
    let clock = TestClock::new();
    let (_sink, engine) = engine(&clock);
    let fact = engine.record_fact(revenue_record()).unwrap();

    let json = serde_json::to_string(&fact).unwrap();
    let roundtripped: FactVersion = serde_json::from_str(&json).unwrap();
    assert_eq!(roundtripped, fact);
    assert_eq!(roundtripped.provenance(), Provenance::OwnerReported);
    assert_eq!(roundtripped.verification(), Verification::Unverified);

    // Unknown fields are rejected on the stored-data contract.
    let mut smuggled = serde_json::to_value(&fact).unwrap();
    smuggled
        .as_object_mut()
        .unwrap()
        .insert("verified".to_owned(), serde_json::json!(true));
    assert!(serde_json::from_value::<FactVersion>(smuggled).is_err());
}

#[test]
fn stored_data_preserves_future_verification_levels_the_api_cannot_create() {
    // The data layer preserves whatever a stored document says (doc §6:
    // preserve the stated/supported/verified difference) so the later
    // activation of the verification levels never reinterprets stored
    // facts. The v1 API cannot produce such a fact — see
    // `every_fact_the_api_can_produce_is_owner_reported_and_unverified`.
    let clock = TestClock::new();
    let (_sink, engine) = engine(&clock);
    let fact = engine.record_fact(revenue_record()).unwrap();

    let mut json = serde_json::to_value(&fact).unwrap();
    json.as_object_mut().unwrap().insert(
        "verification".to_owned(),
        serde_json::json!("professionally_verified"),
    );
    let stored: FactVersion = serde_json::from_value(json).unwrap();
    assert_eq!(stored.verification(), Verification::ProfessionallyVerified);
    // …and even such a stored fact keeps its owner-reported provenance —
    // provenance (how it entered) and verification (how well checked)
    // are independent axes (schema doc §11).
    assert_eq!(stored.provenance(), Provenance::OwnerReported);
}

#[test]
fn current_facts_is_the_snapshot_view() {
    let clock = TestClock::new();
    let (_sink, engine) = engine(&clock);

    assert!(
        engine.current_facts().unwrap().is_empty(),
        "a fresh snapshot has no facts: unknowns are absences, not zeros"
    );

    engine
        .record_fact(RecordFact {
            kind: FactKind::OwnershipStructure,
            value: FactValue::Text("founder-owned".to_owned()),
            period: FactPeriod::Current,
            definition: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();
    engine.record_fact(revenue_record()).unwrap();

    let snapshot = engine.current_facts().unwrap();
    assert_eq!(snapshot.len(), 2);
    // Ordered by kind (Business Reality doc §17 renders the summary).
    let kinds: Vec<FactKind> = snapshot.iter().map(|fact| fact.kind()).collect();
    assert_eq!(kinds, vec![FactKind::Revenue, FactKind::OwnershipStructure]);

    // A revision replaces the current version but not the fact count.
    engine
        .revise_fact(revise_revenue(
            snapshot[0].fact_id().clone(),
            FactValue::Number(8_200_000),
        ))
        .unwrap();
    let snapshot = engine.current_facts().unwrap();
    assert_eq!(snapshot.len(), 2);
    assert_eq!(
        snapshot[0].value(),
        &FactValue::Number(8_200_000),
        "the snapshot shows the current version, not the superseded one"
    );
}
