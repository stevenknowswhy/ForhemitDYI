//! Golden-file walkthroughs of the Employee Ownership Journey v0.2 — the
//! spec's "Journey runtime" verification row: three owner paths
//! (ESOP-leaning, management-buyout-leaning, undecided) produce the
//! expected node sequences, conditional questions appear only when their
//! rules fire, and every answer lands in the Storage Scope its question
//! declared. The walkthroughs drive the real engine over the embedded
//! data file — the same file the app ships — never hand-built question
//! lists.

#![allow(clippy::unwrap_used, clippy::expect_used)] // tests: failures must panic the test

use std::sync::Mutex;

use forhemit_contracts::{
    ActionOrigination, ActorClassification, ActorId, ActorKind, ActorRecord, AuditEventDraft,
    AuditEventType, JourneyInstanceId, JourneyNodeId, WorkspaceId,
};
use forhemit_enginekit::{AuditError, AuditSink, Clock};
use forhemit_journey::{
    AnswerValue, InstanceStatus, JourneyEngine, JourneyStore, MemoryJourneyStore, RecordAnswer,
    ReviseAnswer, SkipNode, StorageScope,
};

/// Frozen time — the walkthroughs assert event structure, not timestamps.
struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap()
    }
}

/// Records every draft the engine emits, in order.
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

fn owner_actor() -> ActorRecord {
    ActorRecord {
        actor_id: ActorId::new("owner_1").unwrap(),
        classification: ActorClassification::Human,
        kind: Some(ActorKind::Owner),
        origination: Some(ActionOrigination::HumanInitiated),
    }
}

/// One scripted step: the node the walk must be asking, and the answer
/// (`None` = a question exit / skip).
struct Step {
    node: &'static str,
    answer: Option<AnswerValue>,
}

fn step(node: &'static str, answer: AnswerValue) -> Step {
    Step {
        node,
        answer: Some(answer),
    }
}

fn skipped(node: &'static str) -> Step {
    Step { node, answer: None }
}

/// The outcome of one driven walkthrough.
struct Walk {
    /// The question nodes the walk asked, in order.
    asked: Vec<String>,
    instance: forhemit_journey::JourneyInstance,
    events: Vec<AuditEventDraft>,
    store: MemoryJourneyStore,
    instance_id: JourneyInstanceId,
    correlation_id: forhemit_contracts::CorrelationId,
}

/// Drives `script` through the engine against the embedded EOJ v0.2
/// definition. Each step asserts the walk is actually asking that node —
/// the golden check — then records the answer or the question exit.
fn drive(script: &[Step]) -> Walk {
    let definition = forhemit_journey::load_employee_ownership_v0_2().unwrap();
    let sink = RecordingSink::default();
    let store = MemoryJourneyStore::new();
    let engine = JourneyEngine::new(
        &FixedClock,
        &sink,
        &store,
        WorkspaceId::new("ws_golden").unwrap(),
    );
    let actor = owner_actor();
    let id = JourneyInstanceId::new("inst_golden").unwrap();
    let mut instance = engine.start(&definition, id.clone(), &actor).unwrap();
    let correlation = instance.correlation_id.clone();
    let mut asked = Vec::new();
    for walk_step in script {
        let current = instance
            .current_node
            .clone()
            .expect("the golden script covers the walk's next question");
        assert_eq!(
            current.as_str(),
            walk_step.node,
            "the walk must ask the golden sequence's next question"
        );
        asked.push(current.as_str().to_owned());
        match &walk_step.answer {
            Some(value) => {
                engine
                    .record(
                        &mut instance,
                        &definition,
                        &RecordAnswer {
                            node_id: current.clone(),
                            value: value.clone(),
                        },
                        &actor,
                    )
                    .unwrap();
            }
            None => {
                engine
                    .skip(
                        &mut instance,
                        &definition,
                        &SkipNode {
                            node_id: current.clone(),
                        },
                        &actor,
                    )
                    .unwrap();
            }
        }
    }
    Walk {
        asked,
        instance,
        events: sink.events.into_inner().unwrap(),
        store,
        instance_id: id,
        correlation_id: correlation,
    }
}

/// A complete walkthrough: the script covered every eligible question.
fn walk(script: &[Step]) -> Walk {
    let outcome = drive(script);
    assert!(
        outcome.instance.current_node.is_none(),
        "the scripted walk left questions unanswered"
    );
    assert_eq!(outcome.instance.status, InstanceStatus::Completed);
    outcome
}

fn single(value: &str) -> AnswerValue {
    AnswerValue::Single(value.to_owned())
}

fn multi(values: &[&str]) -> AnswerValue {
    AnswerValue::Multi(values.iter().map(|value| (*value).to_owned()).collect())
}

fn ranking(values: &[&str]) -> AnswerValue {
    AnswerValue::Ranking(values.iter().map(|value| (*value).to_owned()).collect())
}

// ---------------------------------------------------------------------------
// Golden path 1 — ESOP-leaning owner: every conditional fires, five
// nonnegotiables are derived, the walk completes with a full record.
// ---------------------------------------------------------------------------

#[test]
fn esop_leaning_walks_the_full_conditional_chain() {
    let outcome = walk(&[
        step("primary_objective", single("employees_own_the_company")),
        step(
            "secondary_objectives",
            multi(&[
                "preserve_jobs",
                "meaningful_employee_ownership",
                "preserve_company_culture",
            ]),
        ),
        step(
            "secondary_ranking",
            ranking(&[
                "preserve_jobs",
                "meaningful_employee_ownership",
                "preserve_company_culture",
            ]),
        ),
        step(
            "nonnegotiable_sweep",
            multi(&["employees_own_the_company", "preserve_jobs"]),
        ),
        step(
            "avoid_outcomes",
            multi(&["outside_buyer", "losing_employee_ownership"]),
        ),
        step("preferred_timing", single("within_1_year")),
        step("timing_nonnegotiable", single("yes")),
        step("post_transaction_involvement", single("stay_temporarily")),
        step("involvement_duration", single("six_to_eighteen_months")),
        step("involvement_nonnegotiable", single("yes")),
        step("employee_ownership_importance", single("essential")),
        step("revenue_band", single("one_to_five_m")),
        step("cash_flow_band", single("250k_to_1m")),
        step("employee_count_band", single("fifty_to_two_hundred")),
        step("business_debt", single("none")),
        step("ownership_structure_type", single("one_owner")),
        step("owner_dependency", single("team_could_take_over")),
        step("leadership_readiness", single("yes")),
        step("employee_interest", single("very_interested")),
        step(
            "liquidity_preference",
            single("mix_cash_and_future_payments"),
        ),
        step("explore_liquidity_target", single("yes")),
        step("liquidity_target_amount", AnswerValue::Amount(1_000_000)),
        step("seller_financing_interest", single("yes")),
        step("seller_note_intent", single("sell_part_of_it")),
    ]);

    // The golden question sequence, verbatim.
    assert_eq!(
        outcome.asked,
        [
            "primary_objective",
            "secondary_objectives",
            "secondary_ranking",
            "nonnegotiable_sweep",
            "avoid_outcomes",
            "preferred_timing",
            "timing_nonnegotiable",
            "post_transaction_involvement",
            "involvement_duration",
            "involvement_nonnegotiable",
            "employee_ownership_importance",
            "revenue_band",
            "cash_flow_band",
            "employee_count_band",
            "business_debt",
            "ownership_structure_type",
            "owner_dependency",
            "leadership_readiness",
            "employee_interest",
            "liquidity_preference",
            "explore_liquidity_target",
            "liquidity_target_amount",
            "seller_financing_interest",
            "seller_note_intent",
        ]
    );

    // Five derived nonnegotiables, in marking order: the sweep marks the
    // two selected objective values, timing and involvement are marked by
    // their yes answers, and the "Essential" effect marks employee
    // ownership.
    let targets: Vec<&str> = outcome
        .instance
        .nonnegotiables
        .iter()
        .map(|mark| mark.target.as_str())
        .collect();
    assert_eq!(
        targets,
        [
            "employees_own_the_company",
            "preserve_jobs",
            "Transition timing",
            "Post-transaction involvement",
            "Employee ownership",
        ]
    );

    // Every answer is local-private in v1 — nothing leaves the machine.
    let records = outcome
        .instance
        .answers_in_scope(StorageScope::LocalPrivate);
    assert_eq!(records.len(), 24);
    assert_eq!(outcome.instance.answers_by_scope.len(), 1);

    // Audit-first: one started, one event per recorded answer, one
    // completion — all sharing the walk's correlation id.
    assert_eq!(outcome.events.len(), 1 + 24 + 1);
    assert!(matches!(
        outcome.events[0].event_type,
        AuditEventType::JourneyInstanceStarted
    ));
    assert!(matches!(
        outcome.events[outcome.events.len() - 1].event_type,
        AuditEventType::JourneyInstanceCompleted
    ));
    assert!(outcome
        .events
        .iter()
        .all(|event| event.correlation_id == outcome.correlation_id));

    // The completion event records the walk's shape and the
    // nonnegotiables the owner marked.
    let completion = outcome.events.last().unwrap();
    assert_eq!(completion.payload["answered_nodes"], 24);
    assert_eq!(completion.payload["skipped_nodes"], 0);
    assert_eq!(
        completion.payload["marked_nonnegotiables"]
            .as_array()
            .map(Vec::len),
        Some(5)
    );

    // The started event records which journey version produced the walk.
    let started = outcome.events.first().unwrap();
    assert_eq!(started.payload["journey_version"], "0.2");
    assert_eq!(outcome.instance.journey_version, "0.2");

    // The walk persists as a resumable snapshot.
    let loaded = outcome
        .store
        .load(&outcome.instance_id)
        .unwrap()
        .expect("the completed walk was persisted");
    assert_eq!(loaded.state().status, InstanceStatus::Completed);
}

// ---------------------------------------------------------------------------
// Golden path 2 — management-buyout-leaning owner: leaves completely,
// wants mostly cash; three conditionals never fire.
// ---------------------------------------------------------------------------

#[test]
fn management_buyout_leaning_skips_its_irrelevant_conditionals() {
    let outcome = walk(&[
        step("primary_objective", single("substantial_cash_at_closing")),
        step(
            "secondary_objectives",
            multi(&["retire_completely", "more_cash_at_closing", "reduce_debt"]),
        ),
        step(
            "secondary_ranking",
            ranking(&["more_cash_at_closing", "retire_completely", "reduce_debt"]),
        ),
        step(
            "nonnegotiable_sweep",
            multi(&["substantial_cash_at_closing"]),
        ),
        step("avoid_outcomes", multi(&["nothing_specific"])),
        step("preferred_timing", single("one_to_three_years")),
        step("timing_nonnegotiable", single("no")),
        step("post_transaction_involvement", single("leave_completely")),
        // involvement_duration and involvement_nonnegotiable must NOT
        // appear — the owner is leaving completely.
        step(
            "employee_ownership_importance",
            single("open_to_other_options"),
        ),
        step("revenue_band", single("five_to_ten_m")),
        step("cash_flow_band", single("two_point_five_to_five_m")),
        step("employee_count_band", single("under_10")),
        step("business_debt", single("some")),
        step("ownership_structure_type", single("multiple_owners")),
        step("owner_dependency", single("operates_without_me")),
        step("leadership_readiness", single("not_yet")),
        // employee_interest must NOT appear — leadership is not in place.
        step("liquidity_preference", single("mostly_cash_at_closing")),
        step("explore_liquidity_target", single("yes")),
        step("liquidity_target_amount", AnswerValue::Amount(2_500_000)),
        // seller_financing_interest must NOT appear — the owner wants
        // mostly cash at closing.
    ]);

    // The golden sequence: 19 questions, three conditionals absent.
    assert_eq!(
        outcome.asked,
        [
            "primary_objective",
            "secondary_objectives",
            "secondary_ranking",
            "nonnegotiable_sweep",
            "avoid_outcomes",
            "preferred_timing",
            "timing_nonnegotiable",
            "post_transaction_involvement",
            "employee_ownership_importance",
            "revenue_band",
            "cash_flow_band",
            "employee_count_band",
            "business_debt",
            "ownership_structure_type",
            "owner_dependency",
            "leadership_readiness",
            "liquidity_preference",
            "explore_liquidity_target",
            "liquidity_target_amount",
        ]
    );

    // One derived nonnegotiable: the sweep marked the primary objective.
    let targets: Vec<&str> = outcome
        .instance
        .nonnegotiables
        .iter()
        .map(|mark| mark.target.as_str())
        .collect();
    assert_eq!(targets, ["substantial_cash_at_closing"]);

    // 1 started + 19 recorded + 1 completed; nothing was skipped.
    assert_eq!(outcome.events.len(), 21);
    assert!(outcome.instance.skipped.is_empty());
    assert_eq!(
        outcome
            .instance
            .answers_in_scope(StorageScope::LocalPrivate)
            .len(),
        19
    );
}

// ---------------------------------------------------------------------------
// Golden path 3 — undecided owner: "I'm not sure" everywhere, optional
// questions exited, no nonnegotiables derived.
// ---------------------------------------------------------------------------

#[test]
fn undecided_owner_walks_with_uncertainty_first_class() {
    let outcome = walk(&[
        step("primary_objective", single("not_sure_help_me_explore")),
        step("secondary_objectives", multi(&["transition_quickly"])),
        // secondary_ranking must NOT appear — one selection, nothing to
        // rank.
        skipped("nonnegotiable_sweep"),
        skipped("avoid_outcomes"),
        step("preferred_timing", single("flexible")),
        step("timing_nonnegotiable", single("no")),
        step("post_transaction_involvement", single("stay_temporarily")),
        step("involvement_duration", single("flexible")),
        step("involvement_nonnegotiable", single("no")),
        step("employee_ownership_importance", single("not_sure")),
        step("revenue_band", single("not_sure")),
        step("cash_flow_band", single("not_sure")),
        step("employee_count_band", single("not_sure")),
        step("business_debt", single("not_sure")),
        step("ownership_structure_type", single("one_owner")),
        step("owner_dependency", single("not_sure")),
        step("leadership_readiness", single("not_sure")),
        // employee_interest must NOT appear.
        step("liquidity_preference", single("not_sure")),
        // explore_liquidity_target must NOT appear — the journey never
        // demands a number from someone who has no idea yet.
        step("seller_financing_interest", single("not_sure")),
        // seller_note_intent must NOT appear.
    ]);

    assert_eq!(
        outcome.asked,
        [
            "primary_objective",
            "secondary_objectives",
            "nonnegotiable_sweep",
            "avoid_outcomes",
            "preferred_timing",
            "timing_nonnegotiable",
            "post_transaction_involvement",
            "involvement_duration",
            "involvement_nonnegotiable",
            "employee_ownership_importance",
            "revenue_band",
            "cash_flow_band",
            "employee_count_band",
            "business_debt",
            "ownership_structure_type",
            "owner_dependency",
            "leadership_readiness",
            "liquidity_preference",
            "seller_financing_interest",
        ]
    );

    // Nothing was marked nonnegotiable — the owner marked nothing.
    assert!(outcome.instance.nonnegotiables.is_empty());

    // The two optional questions were exited, in walk order.
    assert_eq!(
        outcome
            .instance
            .skipped
            .iter()
            .map(|node| node.as_str())
            .collect::<Vec<_>>(),
        ["nonnegotiable_sweep", "avoid_outcomes"]
    );

    // Seventeen answers, two exits: 1 started + 17 recorded + 2 skipped
    // + 1 completed.
    assert_eq!(outcome.events.len(), 21);
    let skips = outcome
        .events
        .iter()
        .filter(|event| matches!(event.event_type, AuditEventType::JourneyNodeSkipped))
        .count();
    assert_eq!(skips, 2);

    // "I'm not sure" is a first-class answer: every answer recorded
    // exactly one version, with no value coerced or defaulted.
    assert!(outcome
        .instance
        .answers_in_scope(StorageScope::LocalPrivate)
        .iter()
        .all(|record| record.versions.len() == 1));
}

// ---------------------------------------------------------------------------
// Revisions: a new version supersedes — the original is retained, never
// erased (Journey Builder §9), and the revision is audited. A revision
// never un-marks a nonnegotiable: a nonnegotiable is never silently
// relaxed (NONNEGOTIABLE - MUST-HAVE CONSTRAINT).
// ---------------------------------------------------------------------------

#[test]
fn revising_an_answer_supersedes_without_erasing_history() {
    // Drive through the question sequence the walk actually follows: one
    // secondary selection means the ranking never fires, and the sweep,
    // avoid, and timing nodes sit between it and the employee-ownership
    // question. The walk is still in progress when the revision lands.
    let mut outcome = drive(&[
        step("primary_objective", single("employees_own_the_company")),
        step("secondary_objectives", multi(&["preserve_jobs"])),
        step("nonnegotiable_sweep", multi(&["employees_own_the_company"])),
        step("avoid_outcomes", multi(&["losing_employee_ownership"])),
        step("preferred_timing", single("within_1_year")),
        step("timing_nonnegotiable", single("no")),
        step("post_transaction_involvement", single("leave_completely")),
        step("employee_ownership_importance", single("essential")),
    ]);

    // The Essential effect marked employee ownership.
    assert!(outcome
        .instance
        .nonnegotiables
        .iter()
        .any(|mark| mark.target == "Employee ownership"));

    let definition = forhemit_journey::load_employee_ownership_v0_2().unwrap();
    let sink = RecordingSink::default();
    let engine = JourneyEngine::new(
        &FixedClock,
        &sink,
        &outcome.store,
        WorkspaceId::new("ws_golden").unwrap(),
    );
    let actor = owner_actor();
    let node = JourneyNodeId::new("employee_ownership_importance").unwrap();

    engine
        .revise(
            &mut outcome.instance,
            &definition,
            &ReviseAnswer {
                node_id: node.clone(),
                value: single("not_sure"),
                change_reason: "Reconsidered after seeing the priorities".to_owned(),
            },
            &actor,
        )
        .unwrap();

    // The record now carries two versions: the original retained, the
    // revision on top.
    let record = outcome
        .instance
        .answer(&node)
        .expect("the revised answer is still recorded");
    assert_eq!(record.versions.len(), 2);
    assert_eq!(
        record.versions[0].value,
        AnswerValue::Single("essential".to_owned())
    );
    assert_eq!(
        record.versions[1].value,
        AnswerValue::Single("not_sure".to_owned())
    );
    assert_eq!(
        record.versions[1].supersedes,
        Some(record.versions[0].version_id.clone())
    );

    // The derived nonnegotiable survives the revision — un-marking is
    // never silent.
    assert!(outcome
        .instance
        .nonnegotiables
        .iter()
        .any(|mark| mark.target == "Employee ownership"));

    // The revision was audited with its change reason.
    let revised = sink
        .events
        .lock()
        .unwrap()
        .last()
        .expect("the revision was audited")
        .clone();
    assert!(matches!(
        revised.event_type,
        AuditEventType::JourneyAnswerRevised
    ));
    assert_eq!(
        revised.payload["change_reason"],
        "Reconsidered after seeing the priorities"
    );
    assert_eq!(revised.payload["version"], 2);
}

// ---------------------------------------------------------------------------
// Guardrails: required questions cannot be exited; answers are accepted
// only at the walk's current position; refused transitions leave no
// event and no persisted state.
// ---------------------------------------------------------------------------

#[test]
fn required_questions_cannot_be_skipped_and_future_answers_are_rejected() {
    let definition = forhemit_journey::load_employee_ownership_v0_2().unwrap();
    let sink = RecordingSink::default();
    let store = MemoryJourneyStore::new();
    let engine = JourneyEngine::new(
        &FixedClock,
        &sink,
        &store,
        WorkspaceId::new("ws_golden").unwrap(),
    );
    let actor = owner_actor();
    let mut instance = engine
        .start(
            &definition,
            JourneyInstanceId::new("inst_guard").unwrap(),
            &actor,
        )
        .unwrap();

    // primary_objective is required — a question exit is refused.
    let skip_required = engine.skip(
        &mut instance,
        &definition,
        &SkipNode {
            node_id: JourneyNodeId::new("primary_objective").unwrap(),
        },
        &actor,
    );
    assert!(matches!(
        skip_required,
        Err(forhemit_journey::JourneyError::SkipOfRequiredNode { .. })
    ));

    // The walk is still on the opening question: answering a later node
    // directly is refused — the guided conversation asks one question.
    let future = engine.record(
        &mut instance,
        &definition,
        &RecordAnswer {
            node_id: JourneyNodeId::new("revenue_band").unwrap(),
            value: single("one_to_five_m"),
        },
        &actor,
    );
    assert!(matches!(
        future,
        Err(forhemit_journey::JourneyError::NotTheCurrentPosition { .. })
    ));

    // Nothing was persisted or emitted for the refused transitions: only
    // the start event exists, and the store holds only the fresh walk.
    assert_eq!(sink.events.lock().unwrap().len(), 1);
    assert_eq!(store.list().unwrap().len(), 1);
}
