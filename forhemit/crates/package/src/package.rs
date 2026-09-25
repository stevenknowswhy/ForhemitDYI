//! Package assembly and the export action.
//!
//! [`PackageEngine::assemble`] is a pure read of the input snapshot: it
//! applies the readiness gate, builds the provenance block, and returns
//! the assembled package. It emits no audit event — assembling a view is
//! not a material change to any durable object. [`PackageEngine::export`]
//! renders the package through the requested renderers and emits one
//! [`forhemit_contracts::AuditEventType::PackageExported`] event per
//! exported format, recording every source version the package was
//! assembled from (Journey Builder doc §23 "Journey Version Used"; PRP
//! doc §50 event vocabulary). The caller receives bytes only after the
//! events are recorded — audit first (Implementation Roadmap Phase 1).

use std::sync::Arc;

use crate::error::PackageError;
use crate::export::{ExportFormat, ExportedContent, ExportedPackage, HtmlRenderer, PdfRenderer};
use crate::inputs::{ExportReadiness, ScenarioSnapshot, WorkspaceSnapshot};
use crate::sections::{self, ExecutiveSummary, Section, DISCLOSURE};
use forhemit_contracts::{
    ActorRecord, AuditEventDraft, AuditEventType, BusinessRealityVersionId, CausationId,
    CorrelationId, DestinationId, DestinationVersionId, EngineId, JourneyId, ObjectId,
    ScenarioVersionId, WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The provenance block: the exact source versions a package was
/// assembled from (spec Verification row "Review Package export": every
/// export "records Journey Version Used and destination version"). The
/// block is a required field of [`ProfessionalReviewPackage`] — a
/// package without provenance cannot be constructed, and the golden test
/// pins that a serialized package missing it is rejected.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBlock {
    /// The journey definition the walk follows.
    pub journey_id: JourneyId,
    /// The journey definition version the walk was started under —
    /// "Journey Version Used" (Journey Builder doc §23).
    pub journey_version_used: String,
    /// The destination's identity.
    pub destination_id: DestinationId,
    /// The destination version the package is assembled against.
    pub destination_version_id: DestinationVersionId,
    /// That version's 1-based number in the destination's history.
    pub destination_version_number: u32,
    /// The Business Reality version the scenarios were built against
    /// (Scenario Engine Data Model doc §4, integrity rule 2).
    pub business_reality_version_id: BusinessRealityVersionId,
    /// The scenario versions included in the package — the gate's output.
    pub included_scenario_version_ids: Vec<ScenarioVersionId>,
    /// The scenarios the gate kept out, named with the readiness that
    /// excluded them — never silently dropped (PRP doc §52).
    pub excluded_scenarios: Vec<ExcludedScenario>,
    /// When the package was assembled.
    pub created_at: OffsetDateTime,
}

/// One scenario the readiness gate excluded, named in the package and in
/// the export event's payload.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExcludedScenario {
    /// The excluded version's id.
    pub scenario_version_id: ScenarioVersionId,
    /// The excluded version's name.
    pub name: String,
    /// The readiness that kept it out.
    pub readiness: ExportReadiness,
}

/// The assembled Professional Review Package: the one-page executive
/// summary, the 13 canonical sections, and the provenance block — plus
/// the not-advice disclosure carried on every export.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionalReviewPackage {
    /// When the package was assembled.
    pub created_at: OffsetDateTime,
    /// The source versions the package was assembled from.
    pub provenance: ProvenanceBlock,
    /// The one-page executive summary.
    pub summary: ExecutiveSummary,
    /// The 13 canonical sections, in order.
    pub sections: Vec<Section>,
}

impl ProfessionalReviewPackage {
    /// The not-advice disclosure text every export must carry.
    #[must_use]
    pub fn disclosure() -> &'static str {
        DISCLOSURE
    }
}

/// Applies the export readiness gate: only scenario versions at
/// `ready_for_professional_review` may be included (schema doc §40 —
/// readiness answers "can the package include it?"). Excluded versions
/// are returned with the readiness that kept them out, so the package
/// can name them instead of silently dropping them (PRP doc §52).
pub fn gate_scenarios(
    snapshot: &WorkspaceSnapshot,
) -> (
    Vec<&ScenarioSnapshot>,
    Vec<(&ScenarioSnapshot, ExportReadiness)>,
) {
    let mut included = Vec::new();
    let mut excluded = Vec::new();
    for scenario in &snapshot.scenarios {
        if scenario.readiness == ExportReadiness::ReadyForProfessionalReview {
            included.push(scenario);
        } else {
            excluded.push((scenario, scenario.readiness));
        }
    }
    (included, excluded)
}

/// The Professional Review Package engine. Depends only on `contracts`
/// and `enginekit`: sibling-engine state arrives through
/// [`WorkspaceSnapshot`], mapped by the app shell, and material changes
/// leave through the [`AuditSink`].
pub struct PackageEngine<S: AuditSink<AuditEventDraft>> {
    sink: Arc<S>,
    clock: Arc<dyn Clock>,
    workspace_id: WorkspaceId,
}

impl<S: AuditSink<AuditEventDraft>> PackageEngine<S> {
    /// Builds an engine for `workspace_id`, emitting through `sink` and
    /// stamping times from `clock`.
    pub fn new(sink: Arc<S>, clock: Arc<dyn Clock>, workspace_id: WorkspaceId) -> Self {
        Self {
            sink,
            clock,
            workspace_id,
        }
    }

    /// Assembles the package from the shell-mapped snapshot: gate the
    /// scenarios, build the provenance block, the executive summary, and
    /// the 13 sections. A pure read — no audit event, no mutation.
    ///
    /// # Errors
    /// When the snapshot's workspace id is empty or names a different
    /// workspace than the engine was built for.
    pub fn assemble(
        &self,
        snapshot: &WorkspaceSnapshot,
    ) -> Result<ProfessionalReviewPackage, PackageError> {
        crate::error::validate_workspace(&snapshot.workspace_id)?;
        if snapshot.workspace_id != self.workspace_id {
            return Err(PackageError::WorkspaceMismatch {
                expected: self.workspace_id.as_str().to_owned(),
                got: snapshot.workspace_id.as_str().to_owned(),
            });
        }
        let (included, excluded) = gate_scenarios(snapshot);
        let created_at = self.clock.now();
        let provenance = ProvenanceBlock {
            journey_id: snapshot.journey.journey_id.clone(),
            journey_version_used: snapshot.journey.journey_version_used.clone(),
            destination_id: snapshot.destination.destination_id.clone(),
            destination_version_id: snapshot.destination.version_id.clone(),
            destination_version_number: snapshot.destination.version_number,
            business_reality_version_id: snapshot.reality.business_reality_version_id.clone(),
            included_scenario_version_ids: included
                .iter()
                .map(|scenario| scenario.scenario_version_id.clone())
                .collect(),
            excluded_scenarios: excluded
                .iter()
                .map(|(scenario, readiness)| ExcludedScenario {
                    scenario_version_id: scenario.scenario_version_id.clone(),
                    name: scenario.name.clone(),
                    readiness: *readiness,
                })
                .collect(),
            created_at,
        };
        let summary = sections::build_executive_summary(snapshot, &included);
        let all_sections = sections::build_sections(snapshot, &included, &excluded);
        Ok(ProfessionalReviewPackage {
            created_at,
            provenance,
            summary,
            sections: all_sections,
        })
    }

    /// Exports the package through the requested renderers, emitting one
    /// `PackageExported` audit event per format before the caller sees
    /// any bytes. Each event's payload records the source versions, the
    /// format, the file name, and the disclosure.
    ///
    /// # Errors
    /// When assembly fails, or the audit store rejects an event — in
    /// which case no bytes are returned (audit first).
    pub fn export(
        &self,
        snapshot: &WorkspaceSnapshot,
        formats: &[ExportFormat],
        actor: ActorRecord,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
    ) -> Result<Vec<ExportedPackage>, PackageError> {
        let package = self.assemble(snapshot)?;
        let source_object = package_object_id();
        let mut exported = Vec::new();
        for format in formats {
            let (content, file_name) = match format {
                ExportFormat::Html => {
                    let html = HtmlRenderer.render(&package);
                    (
                        ExportedContent::Html(html),
                        format!("{}{EXT_HTML}", base_file_name(&package)),
                    )
                }
                ExportFormat::Pdf => {
                    let pdf = PdfRenderer.render(&package);
                    (
                        ExportedContent::Pdf(pdf),
                        format!("{}{EXT_PDF}", base_file_name(&package)),
                    )
                }
            };
            let payload = serde_json::json!({
                "file_name": file_name,
                "format": format,
                "journey_id": package.provenance.journey_id,
                "journey_version_used": package.provenance.journey_version_used,
                "destination_id": package.provenance.destination_id,
                "destination_version_id": package.provenance.destination_version_id,
                "destination_version_number": package.provenance.destination_version_number,
                "business_reality_version_id": package.provenance.business_reality_version_id,
                "included_scenario_version_ids": package.provenance.included_scenario_version_ids,
                "excluded_scenario_version_ids": package
                    .provenance
                    .excluded_scenarios
                    .iter()
                    .map(|excluded| excluded.scenario_version_id.clone())
                    .collect::<Vec<_>>(),
                "disclosure": ProfessionalReviewPackage::disclosure(),
                "package_created_at": package.created_at,
            });
            self.sink
                .emit(AuditEventDraft {
                    event_type: AuditEventType::PackageExported,
                    source_engine: EngineId::Package,
                    source_object: source_object.clone(),
                    actor: actor.clone(),
                    workspace_id: self.workspace_id.clone(),
                    transaction_id: None,
                    correlation_id: correlation_id.clone(),
                    causation_id: causation_id.clone(),
                    payload,
                })
                .map_err(PackageError::from)?;
            exported.push(ExportedPackage {
                format: *format,
                file_name,
                content,
            });
        }
        Ok(exported)
    }
}

/// File-name extension for HTML exports.
const EXT_HTML: &str = ".html";
/// File-name extension for PDF exports.
const EXT_PDF: &str = ".pdf";

/// The export's base file name, stamped from the package's assembly time
/// — deterministic for a given package. The shell may rename the file on
/// save; the name here is the engine's suggestion only.
fn base_file_name(package: &ProfessionalReviewPackage) -> String {
    let stamp_format: &[time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[year][month][day]-[hour][minute][second]");
    let stamp = package
        .created_at
        .format(&stamp_format)
        .unwrap_or_else(|_| "package".to_owned());
    format!("forhemit-review-package-{stamp}")
}

/// Mints the export action's audit object id — a fresh ULID per export.
fn package_object_id() -> ObjectId {
    let id = format!("pkg-{}", ulid::Ulid::new());
    ObjectId::new(id).unwrap_or_else(|_| unreachable!("a non-empty id is always a valid ObjectId"))
}
