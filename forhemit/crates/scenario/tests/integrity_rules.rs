//! One integration test per data-integrity rule (Scenario Engine Data
//! Model doc §49, all ten), named to match the enforcement table in
//! [`SCHEMA.md`] §5. Backend-only crate: visual evidence is not
//! applicable to this work.
//!
//! [`SCHEMA.md`]: ../scenario/SCHEMA.md
#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test loudly

use std::sync::{Arc, Mutex};

use forhemit_contracts::{
    ActorClassification, ActorId, ActorKind, ActorRecord, AssumptionId, AuditEventDraft,
    CorrelationId, DestinationVersionId, FactVersionId, FinancialModelVersionId, ObjectiveId,
    ProfessionalReviewId, Provenance, ScenarioFamilyId, ScenarioVersionId, Verification,
    WorkspaceId,
};
use forhemit_enginekit::{AuditError, AuditSink, Clock};
use forhemit_scenario::{
    business_reality_version, CurrencyCode, FixedDecimal, NewAssumption, NewAssumptionRevision,
    NewComparison, NewComparisonDimension, NewComparisonResult, NewConflict, NewConstraint,
    NewDraft, NewFamily, NewNonnegotiable, NewReviewReference, NewSuccessorDraft, NewUnknown,
    NewWhatIf, ScenarioEngine, ScenarioError, ScenarioType, TypedValue, UnknownImportance,
};

/// Captures every emitted draft so tests can assert on the events.
#[derive(Default)]
struct RecordingSink {
    events: Mutex<Vec<AuditEventDraft>>,
}

impl AuditSink<AuditEventDraft> for RecordingSink {
    fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
}

/// Advanceable fixed clock so recorded_at ordering is deterministic.
struct TestClock(Mutex<time::OffsetDateTime>);

impl TestClock {
    fn new() -> Arc<Self> {
        Arc::new(Self(Mutex::new(
            time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap(),
        )))
    }

    fn advance(&self, seconds: i64) {
        *self.0.lock().unwrap() += time::Duration::seconds(seconds);
    }
}

impl Clock for TestClock {
    fn now(&self) -> time::OffsetDateTime {
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
    CorrelationId::new("COR-scenario-test").unwrap()
}

fn fact(id: &str) -> FactVersionId {
    FactVersionId::new(id).unwrap()
}

fn engine() -> (Arc<TestClock>, ScenarioEngine<RecordingSink>) {
    let clock = TestClock::new();
    let sink = Arc::new(RecordingSink::default());
    let engine = ScenarioEngine::new(
        sink.clone(),
        clock.clone(),
        WorkspaceId::new("ws_main").unwrap(),
    );
    (clock, engine)
}

fn family(engine: &ScenarioEngine<RecordingSink>) -> ScenarioFamilyId {
    engine
        .create_family(NewFamily {
            name: "Employee ownership paths".to_owned(),
            scenario_type: ScenarioType::Esop,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap()
        .scenario_family_id
}

const DEST_V1: &str = "destver_01JDESTINATION00000000000AA";
const DEST_V2: &str = "destver_01JDESTINATION00000000000AB";

fn dest(id: &str) -> DestinationVersionId {
    DestinationVersionId::new(id).unwrap()
}

fn draft(engine: &ScenarioEngine<RecordingSink>, family_id: ScenarioFamilyId) -> ScenarioVersionId {
    engine
        .start_draft(NewDraft {
            scenario_family_id: family_id,
            name: "ESOP at market rate".to_owned(),
            description: None,
            destination_version_id: dest(DEST_V1),
            reality_fact_version_ids: vec![fact("factver_1"), fact("factver_2")],
            financial_model_version_id: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap()
        .scenario_version_id
}

fn draft_in(
    engine: &ScenarioEngine<RecordingSink>,
    family_id: &ScenarioFamilyId,
) -> ScenarioVersionId {
    draft(engine, family_id.clone())
}

fn assumption_request(name: &str) -> NewAssumption {
    NewAssumption {
        category: forhemit_scenario::AssumptionCategory::Financial,
        name: name.to_owned(),
        description: None,
        value: TypedValue::Number(FixedDecimal::parse("37.5").unwrap()),
        provenance: Provenance::OwnerReported,
        verification: Verification::Unverified,
        nonnegotiable_objective_id: None,
        source_reference: None,
    }
}

// ─────────────────────────── Integrity rule 1 ───────────────────────────

/// Rule 1: every Scenario Version references exactly one Destination
/// version. The pin is a single-valued NOT NULL field of the request (no
/// list, no link table), it survives storage untouched, and each version
/// in the family keeps its own pin.
#[test]
fn rule_1_every_version_references_exactly_one_destination_version() {
    let (clock, engine) = engine();
    let family_id = family(&engine);

    let v1 = draft_in(&engine, &family_id);
    clock.advance(1);
    let v2 = engine
        .start_draft(NewDraft {
            scenario_family_id: family_id.clone(),
            name: "Manager buyout alternative".to_owned(),
            description: None,
            destination_version_id: dest(DEST_V2),
            reality_fact_version_ids: vec![fact("factver_1")],
            financial_model_version_id: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap()
        .scenario_version_id;

    let stored_v1 = engine.version(&v1).unwrap();
    let stored_v2 = engine.version(&v2).unwrap();
    assert_eq!(stored_v1.destination_version_id, dest(DEST_V1));
    assert_eq!(stored_v2.destination_version_id, dest(DEST_V2));
    assert_eq!(stored_v1.scenario_family_id, family_id);
}
// ─────────────────────────── Integrity rule 2 ───────────────────────────

/// Rule 2: every Scenario Version references the Business Reality
/// version used to create it — a content address derived from the pinned
/// fact versions. Same pinned facts (any order) → same address; any
/// change in the fact set → a different address, so a revision can never
/// silently masquerade as the reality the scenario was built on.
#[test]
fn rule_2_every_version_references_the_reality_version_used() {
    let a = fact("factver_1");
    let b = fact("factver_2");
    let c = fact("factver_3");

    // Order-independent: the address is a content address.
    assert_eq!(
        business_reality_version(&[a.clone(), b.clone()]),
        business_reality_version(&[b, a.clone()])
    );
    // Any revision in the pinned set → a different address.
    assert_ne!(
        business_reality_version(std::slice::from_ref(&a)),
        business_reality_version(&[a.clone(), c.clone()])
    );
    assert_ne!(
        business_reality_version(std::slice::from_ref(&a)),
        business_reality_version(&[a.clone(), a.clone()])
    );

    // The engine stores the derived address on the version.
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let version = engine
        .start_draft(NewDraft {
            scenario_family_id: family_id,
            name: "Pinned to reality".to_owned(),
            description: None,
            destination_version_id: dest(DEST_V1),
            reality_fact_version_ids: vec![a, c],
            financial_model_version_id: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();
    assert_eq!(
        version.business_reality_version_id,
        business_reality_version(&[fact("factver_1"), fact("factver_3")])
    );
}

// ─────────────────────────── Integrity rule 3 ───────────────────────────

/// Rule 3: every material financial output references a Financial Model
/// version. The reference is a required field of the model-output
/// reference type (reserved until the financial modeling engine exists)
/// and is carried opaquely by the draft in v1: `None` is honest "no
/// model", and whatever is supplied survives storage unchanged.
#[test]
fn rule_3_model_outputs_reference_a_model_version() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);

    let without_model = engine
        .start_draft(NewDraft {
            financial_model_version_id: None,
            ..same_facts(family_id.clone())
        })
        .unwrap();
    assert_eq!(without_model.snapshot.financial_model_version_id, None);

    let model = FinancialModelVersionId::new("finmodel_01JMODEL0000000000000001").unwrap();
    let with_model = engine
        .start_draft(NewDraft {
            financial_model_version_id: Some(model.clone()),
            ..same_facts(family_id)
        })
        .unwrap();
    assert_eq!(with_model.snapshot.financial_model_version_id, Some(model));
}

/// The shared draft request for rule 3's two drafts.
fn same_facts(family_id: ScenarioFamilyId) -> NewDraft {
    NewDraft {
        scenario_family_id: family_id,
        name: "Model reference carrier".to_owned(),
        description: None,
        destination_version_id: dest(DEST_V1),
        reality_fact_version_ids: vec![fact("factver_1")],
        financial_model_version_id: None,
        actor: owner(),
        correlation_id: correlation(),
    }
}

// ─────────────────────────── Integrity rule 4 ───────────────────────────

/// Rule 4: every material assumption has provenance — and verification
/// as a separate axis. The serde type has no defaults, so a payload
/// without `provenance` cannot be deserialized at all, and the two axes
/// survive storage independently.
#[test]
fn rule_4_every_assumption_has_provenance() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let version_id = draft(&engine, family_id);

    let stored = engine
        .add_assumption(
            version_id.clone(),
            assumption_request("Machinery value"),
            owner(),
            correlation(),
        )
        .unwrap();
    assert_eq!(stored.provenance, Provenance::OwnerReported);
    assert_eq!(stored.verification, Verification::Unverified);

    // Round-trip through serde: both axes present and distinct.
    let json = serde_json::to_string(&stored).unwrap();
    let round: forhemit_scenario::ScenarioAssumption = serde_json::from_str(&json).unwrap();
    assert_eq!(round.provenance, Provenance::OwnerReported);
    assert_eq!(round.verification, Verification::Unverified);

    // The contract has no serde default: stripping the provenance field
    // makes the payload fail deserialization instead of silently
    // inventing a provenance level.
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value.as_object_mut().unwrap().remove("provenance");
    let attempt: Result<forhemit_scenario::ScenarioAssumption, _> = serde_json::from_value(value);
    assert!(
        attempt.is_err(),
        "an assumption without provenance must not deserialize"
    );
}

// ─────────────────────────── Integrity rule 5 ───────────────────────────

/// Rule 5: every professional determination remains attributed — a
/// review reference requires role AND identity, and recording one never
/// flips the scenario itself into an approved or reviewed state (the
/// reference is attribution, not a verdict; schema doc §29).
#[test]
fn rule_5_professional_determinations_remain_attributed() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let version_id = draft(&engine, family_id);

    let before = engine.version(&version_id).unwrap();

    // An anonymous review cannot exist: identity is required.
    let anonymous = NewReviewReference {
        professional_review_id: ProfessionalReviewId::new("profrev_01JPRO000000000000000001")
            .unwrap(),
        review_status: forhemit_scenario::ProfessionalReviewStatus::Reviewed,
        professional_role: "ESOP trustee".to_owned(),
        professional_identity_reference: String::new(),
        feedback_reference: None,
    };
    let attempt = engine.record_professional_review_reference(
        version_id.clone(),
        anonymous,
        owner(),
        correlation(),
    );
    assert!(matches!(attempt, Err(ScenarioError::EmptyText { .. })));

    // An attributed review is recorded — as a reference.
    let attributed = NewReviewReference {
        professional_review_id: ProfessionalReviewId::new("profrev_01JPRO000000000000000002")
            .unwrap(),
        professional_identity_reference: "trustee-license-4471".to_owned(),
        ..anonymous_reference_fields()
    };
    let reference = engine
        .record_professional_review_reference(
            version_id.clone(),
            attributed,
            owner(),
            correlation(),
        )
        .unwrap();
    assert_eq!(reference.professional_role, "ESOP trustee");

    // …and the scenario itself is unchanged: no readiness or lifecycle
    // movement happened behind the owner's back.
    let after = engine.version(&version_id).unwrap();
    assert_eq!(after.lifecycle_status, before.lifecycle_status);
    assert_eq!(after.readiness_status, before.readiness_status);
}

/// The status/role subset reused by the attributed reference.
fn anonymous_reference_fields() -> NewReviewReference {
    NewReviewReference {
        professional_review_id: ProfessionalReviewId::new("profrev_01JPRO000000000000000001")
            .unwrap(),
        review_status: forhemit_scenario::ProfessionalReviewStatus::Reviewed,
        professional_role: "ESOP trustee".to_owned(),
        professional_identity_reference: String::new(),
        feedback_reference: None,
    }
}

// ─────────────────────────── Integrity rule 6 ───────────────────────────

/// Rule 6: historical versions cannot be mutated. Finalizing freezes the
/// version; every draft mutation on it returns
/// `FinalizedVersionImmutable`; even finalizing twice is refused. A
/// material change requires a successor version instead.
#[test]
fn rule_6_historical_versions_cannot_be_mutated() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let version_id = draft(&engine, family_id);

    engine
        .add_assumption(
            version_id.clone(),
            assumption_request("Willing sellers"),
            owner(),
            correlation(),
        )
        .unwrap();
    let finalized = engine
        .finalize_version(version_id.clone(), owner(), correlation())
        .unwrap();
    assert!(!finalized.is_draft());

    // Every mutating API refuses the finalized version.
    let assumption_id = engine
        .version(&version_id)
        .unwrap()
        .assumptions
        .first()
        .map(|a| a.assumption_id.clone())
        .unwrap_or_else(|| AssumptionId::new("asmp_01JPLACEHOLDER00000000001").unwrap());
    let attempts: Vec<Result<(), ScenarioError>> = vec![
        engine
            .add_assumption(
                version_id.clone(),
                assumption_request("Late addition"),
                owner(),
                correlation(),
            )
            .map(|_| ()),
        engine
            .revise_assumption(
                version_id.clone(),
                NewAssumptionRevision {
                    assumption_id,
                    value: TypedValue::Number(FixedDecimal::parse("40").unwrap()),
                    provenance: Provenance::OwnerReported,
                    verification: Verification::Unverified,
                    change_reason: "should be refused".to_owned(),
                },
                owner(),
                correlation(),
            )
            .map(|_| ()),
        engine
            .add_unknown(
                version_id.clone(),
                NewUnknown {
                    category: None,
                    description: "Late unknown".to_owned(),
                    importance: UnknownImportance::Important,
                    required_action: None,
                    source_dependency: None,
                },
                owner(),
                correlation(),
            )
            .map(|_| ()),
        engine
            .add_constraint(
                version_id.clone(),
                NewConstraint {
                    constraint_type: forhemit_scenario::ConstraintType::ExternalConstraint,
                    name: "Late constraint".to_owned(),
                    description: None,
                    value: None,
                    source_reference: None,
                },
                owner(),
                correlation(),
            )
            .map(|_| ()),
        engine
            .update_metadata(
                version_id.clone(),
                "Renamed after finalize".to_owned(),
                None,
                owner(),
                correlation(),
            )
            .map(|_| ()),
    ];
    for (i, attempt) in attempts.iter().enumerate() {
        assert!(
            matches!(
                attempt,
                Err(ScenarioError::FinalizedVersionImmutable { .. })
            ),
            "mutation {i} on a finalized version must be refused"
        );
    }

    // Recording a conflict against a finalized version is allowed — it
    // attaches to the version's conflict table, it does not mutate the
    // version row (rule 8 keeps conflicts outlive-able; schema doc §16).
    let before_conflict = engine.version(&version_id).unwrap();
    engine
        .record_conflict(
            version_id.clone(),
            NewConflict {
                conflict_type: forhemit_scenario::ConflictType::Data,
                severity: forhemit_scenario::ConflictSeverity::Informational,
                description: "Review found a stale input".to_owned(),
                source_reference: None,
                affected_object_type: None,
                affected_object_id: None,
            },
            owner(),
            correlation(),
        )
        .unwrap();
    let after_conflict = engine.version(&version_id).unwrap();
    assert_eq!(
        serde_json::to_string(&before_conflict).unwrap(),
        serde_json::to_string(&after_conflict).unwrap(),
        "recording a conflict must not mutate the finalized version row"
    );

    // Finalizing twice is also a refusal, not a silent second finalize.
    let second = engine.finalize_version(version_id.clone(), owner(), correlation());
    assert!(matches!(second, Err(ScenarioError::NotADraft { .. })));
}

// ─────────────────────────── Integrity rule 7 ───────────────────────────

/// Rule 7: a branch must identify its parent. The what-if request has no
/// optional parent — the field is required — the branch row carries it,
/// the child family mirrors it, and the parent version is untouched by
/// the branch.
#[test]
fn rule_7_a_branch_identifies_its_parent() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let parent_id = draft(&engine, family_id);
    engine
        .finalize_version(parent_id.clone(), owner(), correlation())
        .unwrap();
    let parent_before = engine.version(&parent_id).unwrap();

    let what_if = engine
        .create_what_if(
            NewWhatIf {
                parent_scenario_version_id: parent_id.clone(),
                branch_type: forhemit_scenario::BranchType::WhatIf,
                reason: "What if the sale price rose 10%?".to_owned(),
                changed_assumptions: Vec::new(),
                name: "Higher-price path".to_owned(),
            },
            owner(),
            correlation(),
        )
        .unwrap();

    // The branch row identifies the parent…
    assert_eq!(what_if.branch.parent_scenario_version_id, parent_id.clone());
    // …and the child family mirrors it.
    assert_eq!(
        what_if.family.branched_from_version_id,
        Some(parent_id.clone())
    );
    // The engine's branch index agrees.
    let branches = engine.branches_from(&parent_id).unwrap();
    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].branch_id, what_if.branch.branch_id);
    // Parent untouched: same stored content after the branch.
    let parent_after = engine.version(&parent_id).unwrap();
    assert_eq!(
        serde_json::to_string(&parent_before).unwrap(),
        serde_json::to_string(&parent_after).unwrap()
    );
}

// ─────────────────────────── Integrity rule 8 ───────────────────────────

/// Rule 8: a conflict cannot disappear merely because a new scenario
/// version exists. Conflicts are keyed by the version they were recorded
/// against; creating a successor leaves the parent's conflicts readable.
#[test]
fn rule_8_conflicts_outlive_new_versions() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let v1 = draft(&engine, family_id);
    engine
        .finalize_version(v1.clone(), owner(), correlation())
        .unwrap();

    let conflict = engine
        .record_conflict(
            v1.clone(),
            NewConflict {
                conflict_type: forhemit_scenario::ConflictType::OwnerObjective,
                severity: forhemit_scenario::ConflictSeverity::Material,
                description: "Family tax capacity conflicts with the price goal".to_owned(),
                source_reference: None,
                affected_object_type: None,
                affected_object_id: None,
            },
            owner(),
            correlation(),
        )
        .unwrap();

    // A successor version exists — the conflict stays.
    let successor = engine
        .start_successor_draft(NewSuccessorDraft {
            parent_version_id: v1.clone(),
            destination_version_id: dest(DEST_V1),
            reality_fact_version_ids: vec![fact("factver_1"), fact("factver_2")],
            change_reason: "Reality moved".to_owned(),
            name_override: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();
    assert_ne!(successor.scenario_version_id, v1);

    let still_there = engine.conflicts_of(&v1).unwrap();
    assert_eq!(still_there.len(), 1);
    assert_eq!(still_there[0].conflict.conflict_id, conflict.conflict_id);
    assert_eq!(still_there[0].conflict.scenario_version_id, v1);
}

// ─────────────────────────── Integrity rule 9 ───────────────────────────

/// Rule 9: changing an owner nonnegotiable creates a new Destination
/// version, and scenario versions are pinned, never re-pointed. The
/// successor draft re-pins to the new destination version; the original
/// version still references the destination version it was created
/// against.
#[test]
fn rule_9_nonnegotiable_change_creates_a_new_destination_version() {
    let (_clock, engine) = engine();
    let family_id = family(&engine);
    let v1 = draft(&engine, family_id);
    engine
        .finalize_version(v1.clone(), owner(), correlation())
        .unwrap();

    let successor = engine
        .start_successor_draft(NewSuccessorDraft {
            parent_version_id: v1.clone(),
            destination_version_id: dest(DEST_V2),
            reality_fact_version_ids: vec![fact("factver_1"), fact("factver_2")],
            change_reason: "Owner revised a nonnegotiable; destination re-versioned".to_owned(),
            name_override: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap();

    assert_eq!(successor.destination_version_id, dest(DEST_V2));
    // The original is not re-pointed — the pin is immutable history.
    assert_eq!(
        engine.version(&v1).unwrap().destination_version_id,
        dest(DEST_V1)
    );

    // A nonnegotiable recorded against a destination version other than
    // the scenario's pin is refused, so designations can never silently
    // mix across pins.
    let mismatched = engine.add_nonnegotiable(
        successor.scenario_version_id.clone(),
        NewNonnegotiable {
            destination_objective_id: ObjectiveId::new("obj_01JOBJECTIVE00000000000001").unwrap(),
            destination_version_id: dest(DEST_V1),
            description: "Designated against the old pin".to_owned(),
            value: None,
        },
        owner(),
        correlation(),
    );
    assert!(matches!(
        mismatched,
        Err(ScenarioError::DestinationVersionMismatch { .. })
    ));
}

// ─────────────────────────── Integrity rule 10 ──────────────────────────

/// Rule 10: scenario data cannot become an implicit recommendation.
/// A comparison records what was compared and the factual readings —
/// no winner, no ranking, no score anywhere in the type's serialized
/// form or its outcome vocabulary.
#[test]
fn rule_10_scenario_data_is_never_an_implicit_recommendation() {
    let (clock, engine) = engine();
    let family_id = family(&engine);
    let a = draft_in(&engine, &family_id);
    clock.advance(1);
    let b = engine
        .start_draft(NewDraft {
            scenario_family_id: family_id,
            name: "Second path".to_owned(),
            description: None,
            destination_version_id: dest(DEST_V1),
            reality_fact_version_ids: vec![fact("factver_1"), fact("factver_2")],
            financial_model_version_id: None,
            actor: owner(),
            correlation_id: correlation(),
        })
        .unwrap()
        .scenario_version_id;

    let comparison = engine
        .record_comparison(
            NewComparison {
                name: "Two paths side by side".to_owned(),
                scenario_version_ids: vec![a.clone(), b.clone()],
                dimensions: vec![NewComparisonDimension {
                    dimension_type: forhemit_scenario::ComparisonDimensionType::Financial,
                    label: "After-tax proceeds range".to_owned(),
                    display_order: 1,
                }],
                results: vec![
                    NewComparisonResult {
                        scenario_version_id: a,
                        dimension_index: 0,
                        value: Some(TypedValue::Currency {
                            amount: FixedDecimal::parse("4800000").unwrap(),
                            currency: CurrencyCode::parse("USD").unwrap(),
                        }),
                        outcome: forhemit_scenario::ComparisonOutcome::Aligns,
                        source_reference: None,
                    },
                    NewComparisonResult {
                        scenario_version_id: b,
                        dimension_index: 0,
                        value: None,
                        outcome: forhemit_scenario::ComparisonOutcome::InsufficientInformation,
                        source_reference: None,
                    },
                ],
            },
            owner(),
            correlation(),
        )
        .unwrap();

    // The comparison's serialized form carries no winner, ranking, or
    // score field of any kind.
    let text = serde_json::to_string(&comparison).unwrap();
    for banned in ["winner", "ranking", "score", "recommended"] {
        assert!(
            !text.to_ascii_lowercase().contains(banned),
            "comparison serialization must not contain `{banned}`"
        );
    }

    // The outcome vocabulary is factual statuses, not verdicts — and the
    // type system has no winner variant to smuggle in.
    let outcomes = [
        forhemit_scenario::ComparisonOutcome::Aligns,
        forhemit_scenario::ComparisonOutcome::DoesNotCurrentlyAlign,
        forhemit_scenario::ComparisonOutcome::InsufficientInformation,
    ];
    for outcome in outcomes {
        let as_text = serde_json::to_string(&outcome)
            .unwrap()
            .to_ascii_lowercase();
        assert!(!as_text.contains("win") && !as_text.contains("best"));
    }
}
