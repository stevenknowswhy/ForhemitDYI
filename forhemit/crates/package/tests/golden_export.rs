//! Golden-file export tests (spec Verification row "Review Package
//! export"): a fixture workspace is assembled into the Professional
//! Review Package and exported as self-contained HTML and PDF. The
//! renderers are pure functions of the package, and the fixture pins
//! the clock, so both exports are byte-for-byte reproducible — the
//! committed golden files under `tests/golden/` are the contract.
//!
//! The same fixture pins the honesty rules: executive summary + 13
//! sections present, provenance block present (Journey Version Used,
//! destination version, business-reality version, scenario versions),
//! the not-advice disclosure on every export, excluded scenarios named
//! with the readiness that kept them out, unknowns visible as unknowns,
//! and no "winner" anywhere. A serialized package missing its
//! provenance must be rejected.
//!
//! Regenerate the golden files after an intentional rendering change:
//! `UPDATE_GOLDEN=1 cargo test -p forhemit-package --test golden_export`

#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use forhemit_contracts::{
    ActionOrigination, ActorClassification, ActorId, ActorKind, ActorRecord, AuditEventDraft,
    BusinessRealityVersionId, CorrelationId, DestinationId, DestinationVersionId, JourneyId,
    NonnegotiableState, Provenance, ScenarioVersionId, Verification, WorkspaceId,
};
use forhemit_enginekit::{AuditError, AuditSink, Clock};
use forhemit_package::{
    gate_scenarios, ExportFormat, ExportedContent, PackageEngine, PackageError,
    ProfessionalReviewPackage, WorkspaceSnapshot, SECTION_TITLES,
};

/// The fixture's fixed assembly time — the stamp the golden files pin.
const FIXED_TIME: time::OffsetDateTime = time::macros::datetime!(2026-09-24 12:00:00 UTC);

/// Injection clock so exports are reproducible.
struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> time::OffsetDateTime {
        FIXED_TIME
    }
}

/// Records every event the engine emits, in order.
#[derive(Default)]
struct RecordingSink {
    events: Mutex<Vec<AuditEventDraft>>,
    /// When set, `emit` rejects — the audit-first behavior the export
    /// action must honor (no bytes without a recorded event).
    reject: Mutex<bool>,
}

impl AuditSink<AuditEventDraft> for RecordingSink {
    fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
        if *self.reject.lock().unwrap() {
            return Err(AuditError("store closed".to_owned()));
        }
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

/// A fixture workspace: a completed walk against destination version 3,
/// two owner-reported facts, one scenario at professional-review
/// readiness (with an open unknown, an unverified assumption, and a
/// conflict), and one preliminary scenario the gate must exclude.
fn fixture() -> WorkspaceSnapshot {
    WorkspaceSnapshot {
        workspace_id: WorkspaceId::new("ws-fixture").unwrap(),
        journey: forhemit_package::JourneySnapshot {
            journey_id: JourneyId::new("employee_ownership").unwrap(),
            journey_version_used: "0.2.0".to_owned(),
            completed: true,
            answers: vec![
                forhemit_package::AnswerLine {
                    node_id: "primary_objective".to_owned(),
                    title: "What matters most to you about selling the business?".to_owned(),
                    values: vec!["Securing the team's future".to_owned()],
                },
                forhemit_package::AnswerLine {
                    node_id: "preferred_timing".to_owned(),
                    title: "When would you like to step back?".to_owned(),
                    values: vec!["Within two years".to_owned()],
                },
                forhemit_package::AnswerLine {
                    node_id: "avoid_outcomes".to_owned(),
                    title: "Which outcomes do you want to avoid?".to_owned(),
                    values: vec![
                        "A rushed sale at a discount".to_owned(),
                        "Losing key people".to_owned(),
                    ],
                },
                forhemit_package::AnswerLine {
                    node_id: "secondary_ranking".to_owned(),
                    title: "Rank what matters next".to_owned(),
                    values: vec!["Fair price".to_owned(), "Community continuity".to_owned()],
                },
            ],
            skipped_node_ids: vec!["seller_financing_interest".to_owned()],
            marked_nonnegotiables: vec!["Keep all 40 employees for at least two years".to_owned()],
        },
        destination: forhemit_package::DestinationSnapshot {
            destination_id: DestinationId::new("dest-1").unwrap(),
            version_id: DestinationVersionId::new("destver-3").unwrap(),
            version_number: 3,
            objectives: vec![
                forhemit_package::ObjectiveLine {
                    label: "Desired cash at closing".to_owned(),
                    value: "I'm not sure".to_owned(),
                    preference: NonnegotiableState::Unspecified,
                },
                forhemit_package::ObjectiveLine {
                    label: "Team security".to_owned(),
                    value: "All 40 employees kept for two years".to_owned(),
                    preference: NonnegotiableState::Nonnegotiable,
                },
                forhemit_package::ObjectiveLine {
                    label: "Sale price".to_owned(),
                    value: "Above $8M".to_owned(),
                    preference: NonnegotiableState::StrongPreference,
                },
                forhemit_package::ObjectiveLine {
                    label: "Location".to_owned(),
                    value: "Keep the Cleveland plant".to_owned(),
                    preference: NonnegotiableState::Preference,
                },
                forhemit_package::ObjectiveLine {
                    label: "Avoidance: reputation".to_owned(),
                    value: "No public layoffs after closing".to_owned(),
                    preference: NonnegotiableState::Unspecified,
                },
            ],
        },
        reality: forhemit_package::RealitySnapshot {
            business_reality_version_id: BusinessRealityVersionId::new("brver-2").unwrap(),
            facts: vec![
                forhemit_package::FactLine {
                    kind: "Revenue".to_owned(),
                    value: "$6.2M".to_owned(),
                    period: "FY2025".to_owned(),
                    definition: Some("audited figure".to_owned()),
                    provenance: Provenance::OwnerReported,
                    verification: Verification::Verified,
                },
                forhemit_package::FactLine {
                    kind: "Employees".to_owned(),
                    value: "40".to_owned(),
                    period: "current".to_owned(),
                    definition: None,
                    provenance: Provenance::OwnerReported,
                    verification: Verification::Unverified,
                },
            ],
        },
        scenarios: vec![
            forhemit_package::ScenarioSnapshot {
                scenario_version_id: ScenarioVersionId::new("scenver-esop-1").unwrap(),
                family_name: "ESOP path".to_owned(),
                name: "ESOP 100% over 3 years".to_owned(),
                description: Some("Seller-financed ESOP with a staged buyout.".to_owned()),
                scenario_type: "ESOP".to_owned(),
                lifecycle_status: "modelable".to_owned(),
                readiness: forhemit_package::ExportReadiness::ReadyForProfessionalReview,
                assumptions: vec![
                    forhemit_package::AssumptionLine {
                        name: "Sale price".to_owned(),
                        description: None,
                        value: "$8.5M".to_owned(),
                        category: "valuation".to_owned(),
                        provenance: Provenance::OwnerReported,
                        verification: Verification::Unverified,
                    },
                    forhemit_package::AssumptionLine {
                        name: "Bank debt share".to_owned(),
                        description: Some("Owner carries a seller note.".to_owned()),
                        value: "60% of price".to_owned(),
                        category: "financing".to_owned(),
                        provenance: Provenance::ScenarioAssumed,
                        verification: Verification::PartiallyVerified,
                    },
                ],
                unknowns: vec![forhemit_package::UnknownLine {
                    description: "Repurchase liability if profits fall".to_owned(),
                    importance: "critical".to_owned(),
                    resolution_status: "open".to_owned(),
                }],
                conflicts: vec![forhemit_package::ConflictLine {
                    description: "Full liquidity conflicts with staged seller financing".to_owned(),
                    severity: "material".to_owned(),
                }],
            },
            forhemit_package::ScenarioSnapshot {
                scenario_version_id: ScenarioVersionId::new("scenver-mbo-1").unwrap(),
                family_name: "Management buyout".to_owned(),
                name: "MBO with two leaders".to_owned(),
                description: None,
                scenario_type: "Management buyout".to_owned(),
                lifecycle_status: "draft".to_owned(),
                readiness: forhemit_package::ExportReadiness::Preliminary,
                assumptions: Vec::new(),
                unknowns: Vec::new(),
                conflicts: Vec::new(),
            },
        ],
    }
}

/// The engine under test, bound to the fixture workspace and clock.
fn engine(sink: &Arc<RecordingSink>) -> PackageEngine<RecordingSink> {
    PackageEngine::new(
        Arc::clone(sink),
        Arc::new(FixedClock),
        WorkspaceId::new("ws-fixture").unwrap(),
    )
}

/// Returns the golden file path for `name`.
fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

/// Compares `actual` against the golden file, or (re)writes it when
/// `UPDATE_GOLDEN=1` is set for an intentional rendering change.
fn assert_golden(name: &str, actual: &[u8]) {
    let path = golden_path(name);
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, actual).unwrap();
    }
    let expected = std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "golden file {} is missing ({error}) — run with UPDATE_GOLDEN=1 to create it",
            path.display()
        )
    });
    assert!(
        actual == expected.as_slice(),
        "{name} diverged from the golden file — if the rendering change is \
         intentional, re-run with UPDATE_GOLDEN=1 and read the diff"
    );
}

// ---------------------------------------------------------------------------
// Golden files
// ---------------------------------------------------------------------------

#[test]
fn golden_html_export() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    assert_golden("review-package.html", html.as_bytes());
}

#[test]
fn golden_pdf_export() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let pdf = forhemit_package::PdfRenderer.render(&package);
    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(pdf.ends_with(b"%%EOF\n"));
    assert_golden("review-package.pdf", &pdf);
}

// ---------------------------------------------------------------------------
// Content invariants, asserted on the same fixture
// ---------------------------------------------------------------------------

#[test]
fn html_contains_summary_and_all_thirteen_sections() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    assert!(html.contains("<h2>Executive Summary</h2>"));
    for (position, title) in SECTION_TITLES.iter().enumerate() {
        let heading = format!("<h2>{}. {}</h2>", position + 1, title);
        assert!(
            html.contains(&heading),
            "section {} ({title}) is missing from the HTML export",
            position + 1
        );
    }
    assert_eq!(package.sections.len(), 13);
}

#[test]
fn html_records_full_provenance() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    for required in [
        "Journey version used:</dt><dd>0.2.0</dd>",
        "Destination version:</dt><dd>destver-3</dd>",
        "Destination version number:</dt><dd>3</dd>",
        "Business reality version:</dt><dd>brver-2</dd>",
        "Scenario versions included:</dt><dd>scenver-esop-1</dd>",
        "Scenarios excluded:</dt><dd>MBO with two leaders (readiness: Preliminary)</dd>",
    ] {
        assert!(
            html.contains(required),
            "provenance line missing: {required}"
        );
    }
}

#[test]
fn html_carries_the_disclosure_top_and_bottom() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    assert_eq!(
        html.matches(ProfessionalReviewPackage::disclosure())
            .count(),
        2
    );
}

#[test]
fn excluded_scenario_is_named_not_dropped() {
    let snapshot = fixture();
    let (included, excluded) = gate_scenarios(&snapshot);
    assert_eq!(included.len(), 1);
    assert_eq!(included[0].scenario_version_id.as_str(), "scenver-esop-1");
    assert_eq!(excluded.len(), 1);
    assert_eq!(excluded[0].0.name, "MBO with two leaders");
    assert_eq!(
        excluded[0].1,
        forhemit_package::ExportReadiness::Preliminary
    );
}

#[test]
fn unknowns_stay_visible_never_zeros() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    // The unknown appears with its importance and status...
    assert!(html.contains("Repurchase liability if profits fall"));
    assert!(html.contains("importance: critical"));
    // ...the summary names the included path without ranking it...
    assert!(html.contains("ESOP 100% over 3 years"));
    // ...and the questions section derives from the open unknown.
    assert!(html.contains("Please help resolve: Repurchase liability if profits fall"));
    // No fabricated zeros standing in for unknowns.
    assert!(!html.contains("Repurchase liability: 0"));
}

#[test]
fn comparisons_never_declare_a_winner() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let html = forhemit_package::HtmlRenderer.render(&package);
    for banned in ["winner", "recommended path", "best scenario", "optimal"] {
        assert!(
            !html.to_ascii_lowercase().contains(banned),
            "the export reads as a recommendation: found {banned:?}"
        );
    }
    // Conflicts stay conditions with severity in words.
    assert!(html.contains("severity: material"));
}

#[test]
fn pdf_carries_disclosure_and_provenance() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let pdf = forhemit_package::PdfRenderer.render(&package);
    let pdf = String::from_utf8_lossy(&pdf);
    // The header stamp, the full disclosure, and the required
    // provenance labels are literal text in the content streams.
    for required in [
        "Professional Review Package",
        ProfessionalReviewPackage::disclosure(),
        "Journey version used: 0.2.0",
        "Business reality version: brver-2",
        // The excluded-scenarios line's parens are PDF-escaped, so the
        // readiness phrase is asserted without them.
        "Scenarios excluded: MBO with two leaders",
        "readiness: Preliminary",
        "Not a recommendation or valuation.",
    ] {
        assert!(pdf.contains(required), "PDF export is missing: {required}");
    }
    for (position, title) in SECTION_TITLES.iter().enumerate() {
        let heading = format!("{}. {}", position + 1, title);
        assert!(
            pdf.contains(&heading),
            "section {} ({title}) is missing from the PDF export",
            position + 1
        );
    }
}

// ---------------------------------------------------------------------------
// Provenance is structurally required
// ---------------------------------------------------------------------------

#[test]
fn a_package_without_provenance_is_rejected() {
    // `{}` lacks every required field — in particular it has no
    // provenance, and the point of the test is that serde rejects the
    // shape rather than defaulting one into existence.
    let error = serde_json::from_str::<ProfessionalReviewPackage>("{}")
        .expect_err("a package without provenance must not deserialize");
    assert!(
        error.to_string().contains("missing field"),
        "the rejection must be a missing-field error, got: {error}"
    );
}

#[test]
fn a_package_missing_only_provenance_is_rejected() {
    // A complete package, serialized — then the provenance field removed.
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let mut value = serde_json::to_value(&package).unwrap();
    let removed = value.as_object_mut().unwrap().remove("provenance");
    assert!(removed.is_some());
    let error = serde_json::from_value::<ProfessionalReviewPackage>(value)
        .expect_err("a package missing its provenance must not deserialize");
    assert!(error.to_string().contains("provenance"));
}

#[test]
fn the_fixture_package_round_trips() {
    let sink = Arc::new(RecordingSink::default());
    let package = engine(&sink).assemble(&fixture()).unwrap();
    let json = serde_json::to_string(&package).unwrap();
    let reparsed: ProfessionalReviewPackage = serde_json::from_str(&json).unwrap();
    assert_eq!(reparsed, package);
}

// ---------------------------------------------------------------------------
// The export action and its audit events
// ---------------------------------------------------------------------------

#[test]
fn export_emits_one_event_per_format_before_returning_bytes() {
    let sink = Arc::new(RecordingSink::default());
    let exported = engine(&sink)
        .export(
            &fixture(),
            &[ExportFormat::Html, ExportFormat::Pdf],
            owner_actor(),
            CorrelationId::new("cor-pkg-1").unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(exported.len(), 2);
    assert_eq!(exported[0].format, ExportFormat::Html);
    assert_eq!(
        exported[0].file_name,
        "forhemit-review-package-20260924-120000.html"
    );
    assert_eq!(exported[1].format, ExportFormat::Pdf);
    assert_eq!(
        exported[1].file_name,
        "forhemit-review-package-20260924-120000.pdf"
    );

    let events = sink.events.lock().unwrap();
    assert_eq!(events.len(), 2, "one PackageExported event per format");
    for (position, event) in events.iter().enumerate() {
        assert_eq!(
            event.event_type,
            forhemit_contracts::AuditEventType::PackageExported
        );
        assert_eq!(event.source_engine, forhemit_contracts::EngineId::Package);
        let payload = &event.payload;
        assert_eq!(payload["journey_version_used"], "0.2.0");
        assert_eq!(payload["destination_version_id"], "destver-3");
        assert_eq!(payload["destination_version_number"], 3);
        assert_eq!(payload["business_reality_version_id"], "brver-2");
        assert_eq!(
            payload["included_scenario_version_ids"][0],
            "scenver-esop-1"
        );
        assert_eq!(payload["excluded_scenario_version_ids"][0], "scenver-mbo-1");
        assert_eq!(
            payload["disclosure"],
            ProfessionalReviewPackage::disclosure()
        );
        let expected_format = if position == 0 { "html" } else { "pdf" };
        assert_eq!(payload["format"], expected_format);
    }
    // HTML content matches the golden render.
    match &exported[0].content {
        ExportedContent::Html(html) => assert_golden("review-package.html", html.as_bytes()),
        other => panic!("expected HTML content, got {other:?}"),
    }
}

#[test]
fn a_rejected_audit_event_means_no_bytes() {
    let sink = Arc::new(RecordingSink::default());
    *sink.reject.lock().unwrap() = true;
    let error = engine(&sink)
        .export(
            &fixture(),
            &[ExportFormat::Html],
            owner_actor(),
            CorrelationId::new("cor-pkg-2").unwrap(),
            None,
        )
        .expect_err("an unrecorded export must fail");
    assert_eq!(
        error,
        PackageError::AuditRejected("store closed".to_owned())
    );
    assert!(sink.events.lock().unwrap().is_empty());
}

#[test]
fn a_cross_workspace_snapshot_is_rejected() {
    let sink = Arc::new(RecordingSink::default());
    let mut snapshot = fixture();
    snapshot.workspace_id = WorkspaceId::new("ws-other").unwrap();
    let error = engine(&sink)
        .assemble(&snapshot)
        .expect_err("a snapshot from another workspace must not assemble");
    assert_eq!(
        error,
        PackageError::WorkspaceMismatch {
            expected: "ws-fixture".to_owned(),
            got: "ws-other".to_owned(),
        }
    );
}

#[test]
fn an_empty_workspace_id_is_rejected() {
    // The typed constructor rejects empty ids, so the empty value can
    // only reach the engine through deserialization (ids are serde
    // transparent) — the exact path a corrupted snapshot would take.
    let sink = Arc::new(RecordingSink::default());
    let mut value = serde_json::to_value(fixture()).unwrap();
    value["workspace_id"] = serde_json::Value::String(String::new());
    let snapshot: WorkspaceSnapshot = serde_json::from_value(value).unwrap();
    let error = engine(&sink)
        .assemble(&snapshot)
        .expect_err("an empty workspace id must not assemble");
    assert_eq!(error, PackageError::EmptyWorkspace);
}
