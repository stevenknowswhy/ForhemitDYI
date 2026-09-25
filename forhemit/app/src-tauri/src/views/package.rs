//! Package display projections and the workspace snapshot mapper.
//!
//! The mapper is the composition root of the Professional Review
//! Package: it reads the destination, journey, reality, and scenario
//! engines through their display projections and fills the package
//! crate's input ports (rendered owner-facing text plus typed
//! readiness). It carries no domain rules of its own — the package
//! engine owns gating, assembly, and the export audit events.

use base64::Engine as _;
use forhemit_destination::content::{
    AllocationChoice, AllocationParticipant, Avoidance, DestinationContent, FinancialObjective,
    FutureIncomeChoice, IncomeBand, IncomeDuration, IncomeInterest, OwnerRole,
    OwnershipParticipant, PreservationGoal, PreservationSelection, ProceedsBand, ProceedsChoice,
    TransitionTiming,
};
use forhemit_destination::{Answer, Objective};
use forhemit_journey::{AnswerValue, InstanceStatus};
use forhemit_package::{
    ExecutiveSummary as Summary, ExportReadiness, ExportedContent, ExportedPackage,
    ProvenanceBlock, Section,
};
use forhemit_reality::fact::FactVersion;
use forhemit_scenario::{ReadinessStatus, ScenarioVersion};
use serde::Serialize;
use time::OffsetDateTime;

use crate::state::AppEngines;

/// The package's provenance block, shown before export (PRP doc §20).
#[derive(Clone, Debug, Serialize)]
pub struct ProvenanceView {
    /// Which journey definition version produced the answers.
    pub journey_version_used: String,
    /// The destination version's number.
    pub destination_version_number: u32,
    /// The Business Reality version this record is pinned to.
    pub business_reality_version_id: String,
    /// The scenario versions included in the package.
    pub included_scenario_version_ids: Vec<String>,
    /// The scenarios left out, each with its readiness in words.
    pub excluded_scenarios: Vec<ExcludedScenarioView>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One scenario the gate left out — named, with its readiness, never hidden.
#[derive(Clone, Debug, Serialize)]
pub struct ExcludedScenarioView {
    /// The scenario version id.
    pub scenario_version_id: String,
    /// The record's name.
    pub name: String,
    /// The version's readiness, in words.
    pub readiness: &'static str,
}

/// Everything the export screen shows before the owner commits: the
/// provenance block, the executive summary, and all 13 sections.
#[derive(Clone, Debug, Serialize)]
pub struct PackagePreviewView {
    /// When the record was created (RFC 3339).
    pub created_at: String,
    /// Whose information this is.
    pub provenance: ProvenanceView,
    /// The one-page executive summary.
    pub summary: Summary,
    /// The 13 canonical sections, in order.
    pub sections: Vec<Section>,
    /// The not-advice disclosure every export must carry.
    pub disclosure: String,
}

/// One exported file — the bytes base64-wrapped so the UI can offer a
/// save, plus the path the engine already wrote it to.
#[derive(Clone, Debug, Serialize)]
pub struct ExportedFileView {
    /// The export format that produced the file.
    pub format: &'static str,
    /// The saved file's name.
    pub file_name: String,
    /// Where the file was saved on this device.
    pub saved_path: String,
    /// The file's bytes, base64-encoded for the frontend.
    pub content_base64: String,
}

/// Formats an engine timestamp as RFC 3339; an unformattable value
/// renders empty rather than fabricating a time.
fn iso(when: OffsetDateTime) -> String {
    when.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}

/// The exported package file row as the UI sees it.
pub fn exported_file_view(
    exported: &ExportedPackage,
    saved_path: &str,
) -> Result<ExportedFileView, String> {
    let (format, content_base64) = match &exported.content {
        ExportedContent::Html(html) => (
            "html",
            base64::engine::general_purpose::STANDARD.encode(html.as_bytes()),
        ),
        ExportedContent::Pdf(bytes) => (
            "pdf",
            base64::engine::general_purpose::STANDARD.encode(bytes),
        ),
    };
    Ok(ExportedFileView {
        format,
        file_name: exported.file_name.clone(),
        saved_path: saved_path.to_owned(),
        content_base64,
    })
}

/// The package port's readiness as words — a separate mapping from the
/// scenario crate's labels, because the port mirrors the enum (inputs.rs).
fn export_readiness_label(readiness: ExportReadiness) -> &'static str {
    match readiness {
        ExportReadiness::Preliminary => "Preliminary",
        ExportReadiness::InformationNeeded => "Information needed",
        ExportReadiness::Modelable => "Modelable",
        ExportReadiness::ReadyForComparison => "Ready for comparison",
        ExportReadiness::ReadyForProfessionalReview => "Ready for professional review",
        ExportReadiness::UnderReview => "Under professional review",
        ExportReadiness::Revised => "Revised after review",
        ExportReadiness::Superseded => "Superseded",
        ExportReadiness::Archived => "Archived",
    }
}

/// The package preview the UI shows before export.
pub fn package_preview_view(
    provenance: &ProvenanceBlock,
    summary: &Summary,
    sections: &[Section],
) -> PackagePreviewView {
    PackagePreviewView {
        created_at: iso(provenance.created_at),
        provenance: ProvenanceView {
            created_at: iso(provenance.created_at),
            journey_version_used: provenance.journey_version_used.clone(),
            destination_version_number: provenance.destination_version_number,
            business_reality_version_id: provenance.business_reality_version_id.as_str().to_owned(),
            included_scenario_version_ids: provenance
                .included_scenario_version_ids
                .iter()
                .map(|id| id.as_str().to_owned())
                .collect(),
            excluded_scenarios: provenance
                .excluded_scenarios
                .iter()
                .map(|excluded| ExcludedScenarioView {
                    scenario_version_id: excluded.scenario_version_id.as_str().to_owned(),
                    name: excluded.name.clone(),
                    readiness: export_readiness_label(excluded.readiness),
                })
                .collect(),
        },
        summary: summary.clone(),
        sections: sections.to_vec(),
        disclosure: forhemit_package::ProfessionalReviewPackage::disclosure().to_owned(),
    }
}

/// The scenario crate's readiness mapped to the package port — an
/// exhaustive match, so a new readiness state forces this adapter to
/// decide.
fn export_readiness(readiness: ReadinessStatus) -> ExportReadiness {
    match readiness {
        ReadinessStatus::Preliminary => ExportReadiness::Preliminary,
        ReadinessStatus::InformationNeeded => ExportReadiness::InformationNeeded,
        ReadinessStatus::Modelable => ExportReadiness::Modelable,
        ReadinessStatus::ReadyForComparison => ExportReadiness::ReadyForComparison,
        ReadinessStatus::ReadyForProfessionalReview => ExportReadiness::ReadyForProfessionalReview,
        ReadinessStatus::UnderReview => ExportReadiness::UnderReview,
        ReadinessStatus::Revised => ExportReadiness::Revised,
        ReadinessStatus::Superseded => ExportReadiness::Superseded,
        ReadinessStatus::Archived => ExportReadiness::Archived,
    }
}

/// The answer wrapper as words; "I'm not sure" stays first-class.
fn answer_text<T>(answer: &Answer<T>, render: impl Fn(&T) -> String) -> Option<String> {
    match answer {
        Answer::Unanswered => None,
        Answer::NotSure => Some("I'm not sure".to_owned()),
        Answer::Answered(value) => Some(render(value)),
    }
}

/// Renders one objective line, or drops it when never addressed.
fn objective_line<T>(
    label: &str,
    objective: &Objective<T>,
    render: impl Fn(&T) -> String,
) -> Option<forhemit_package::ObjectiveLine> {
    let value = answer_text(&objective.value, render)?;
    Some(forhemit_package::ObjectiveLine {
        label: label.to_owned(),
        value,
        preference: objective.preference.clone(),
    })
}

/// Formats a u64 amount in whole currency units.
fn money(amount: u64) -> String {
    format!("${amount}")
}

/// Renders a Business Snapshot fact's value as the owner stated it.
fn fact_value_label(value: &forhemit_reality::FactValue) -> String {
    match value {
        forhemit_reality::FactValue::Text(text) => text.clone(),
        forhemit_reality::FactValue::Number(number) => number.to_string(),
        forhemit_reality::FactValue::Range(range) => match (range.lower, range.upper) {
            (Some(lower), Some(upper)) => format!("${lower}–${upper}"),
            (Some(lower), None) => format!("${lower} and up"),
            (None, Some(upper)) => format!("under ${upper}"),
            (None, None) => "range (no bounds stated)".to_owned(),
        },
    }
}

/// Formats an optional u64 amount.
fn maybe_money(amount: Option<u64>) -> String {
    amount
        .map(money)
        .unwrap_or_else(|| "an open amount".to_owned())
}

/// Renders a proceeds band exactly as the Destination Builder displayed it.
fn proceeds_band_label(band: &ProceedsBand) -> &'static str {
    match band {
        ProceedsBand::Under500k => "Under $500K",
        ProceedsBand::K500kTo1m => "$500K–$1M",
        ProceedsBand::K1mTo2m => "$1M–$2M",
        ProceedsBand::K2mTo3m => "$2M–$3M",
        ProceedsBand::K3mTo5m => "$3M–$5M",
        ProceedsBand::M5To10m => "$5M–$10M",
        ProceedsBand::Over10m => "$10M+",
    }
}

fn proceeds_choice_label(choice: &ProceedsChoice) -> String {
    match choice {
        ProceedsChoice::Band(band) => proceeds_band_label(band).to_owned(),
        ProceedsChoice::CustomRange { minimum, maximum } => {
            format!(
                "Between {} and {}",
                maybe_money(*minimum),
                maybe_money(*maximum)
            )
        }
    }
}

fn income_band_label(band: &IncomeBand) -> &'static str {
    match band {
        IncomeBand::Under25k => "Under $25K",
        IncomeBand::K25kTo50k => "$25K–$50K",
        IncomeBand::K50kTo100k => "$50K–$100K",
        IncomeBand::K100kTo250k => "$100K–$250K",
        IncomeBand::Over250k => "$250K+",
    }
}

fn income_duration_label(duration: &IncomeDuration) -> &'static str {
    match duration {
        IncomeDuration::Years1To3 => "1–3 years",
        IncomeDuration::Years3To5 => "3–5 years",
        IncomeDuration::Years5To10 => "5–10 years",
        IncomeDuration::Over10Years => "10+ years",
        IncomeDuration::Ongoing => "ongoing",
    }
}

fn future_income_label(choice: &FutureIncomeChoice) -> String {
    let interest = match choice.interest {
        IncomeInterest::Yes => "Yes",
        IncomeInterest::Maybe => "Maybe",
        IncomeInterest::No => "No",
    };
    let amount = answer_text(&choice.amount, |band| income_band_label(band).to_owned())
        .unwrap_or_else(|| "amount not sure yet".to_owned());
    let duration = answer_text(&choice.duration, |span| {
        income_duration_label(span).to_owned()
    })
    .unwrap_or_else(|| "duration not sure yet".to_owned());
    format!("{interest} — {amount} a year for {duration}")
}

fn participant_label(participant: &OwnershipParticipant) -> String {
    match participant {
        OwnershipParticipant::AllEmployees => "All employees".to_owned(),
        OwnershipParticipant::Management => "Management".to_owned(),
        OwnershipParticipant::SpecificEmployeeGroup(group) => {
            format!("Employee group: {group}")
        }
        OwnershipParticipant::Family => "Family members".to_owned(),
        OwnershipParticipant::ExistingOwners => "Existing owners".to_owned(),
        OwnershipParticipant::OutsideInvestors => "Outside investors".to_owned(),
        OwnershipParticipant::Other(text) => format!("Other: {text}"),
    }
}

fn ownership_shape_label(
    shape: &forhemit_destination::content::EmployeeOwnershipShape,
) -> &'static str {
    use forhemit_destination::content::EmployeeOwnershipShape::*;
    match shape {
        Broad => "Broadly shared among employees",
        EmployeesPlusManagement => "Employees own part, management additional ownership",
        WithOtherOwners => "Employees own part alongside other owners",
    }
}

fn allocation_choice_label(choice: &AllocationChoice) -> String {
    match choice {
        AllocationChoice::Percentages(shares) => shares
            .iter()
            .map(|share| {
                let participant = match &share.participant {
                    AllocationParticipant::Employees => "Employees".to_owned(),
                    AllocationParticipant::Management => "Management".to_owned(),
                    AllocationParticipant::Other(label) => format!("Other ({label})"),
                };
                format!("{participant} {}%", share.percent)
            })
            .collect::<Vec<_>>()
            .join(", "),
        AllocationChoice::ParticipantsOnly => {
            "Who should participate is known; approximate percentages are not".to_owned()
        }
    }
}

fn owner_role_label(role: OwnerRole) -> &'static str {
    match role {
        OwnerRole::Retired => "Step away completely",
        OwnerRole::TransitionAdvisor => "Help for a limited period",
        OwnerRole::OngoingAdvisor => "Remain available occasionally",
        OwnerRole::ContinuingOwnerOperator => "Remain meaningfully involved",
    }
}

fn timing_label(timing: TransitionTiming) -> &'static str {
    match timing {
        TransitionTiming::Within12Months => "Within 12 months",
        TransitionTiming::OneToThreeYears => "1–3 years",
        TransitionTiming::ThreeToFiveYears => "3–5 years",
        TransitionTiming::MoreThanFiveYears => "More than 5 years",
        TransitionTiming::Flexible => "I'm flexible",
    }
}

fn preservation_goal_label(goal: &PreservationGoal) -> String {
    match goal {
        PreservationGoal::EmployeesRemain => "Employees remain with the company".to_owned(),
        PreservationGoal::RemainsIndependent => "Company remains independent".to_owned(),
        PreservationGoal::SameLocation => "Company stays in the same location".to_owned(),
        PreservationGoal::CultureRemains => "Company culture remains".to_owned(),
        PreservationGoal::LeadershipRemains => "Current leadership remains".to_owned(),
        PreservationGoal::BrandRemains => "Brand remains".to_owned(),
        PreservationGoal::FamilyInvolved => "Family remains involved".to_owned(),
        PreservationGoal::CommunityPresenceRemains => "Community presence remains".to_owned(),
        PreservationGoal::CustomersServed => {
            "Company continues serving existing customers".to_owned()
        }
        PreservationGoal::Other(text) => format!("Other: {text}"),
    }
}

fn preservation_selection_label(selection: &PreservationSelection) -> String {
    let rank = selection
        .rank
        .map(|rank| format!(" (rank {rank})"))
        .unwrap_or_default();
    format!("{}{rank}", preservation_goal_label(&selection.goal))
}

fn avoidance_label(avoidance: &Avoidance) -> String {
    match avoidance {
        Avoidance::OutsideBuyer => "Selling to an outside buyer".to_owned(),
        Avoidance::LosingEmployeeOwnership => "Losing employee ownership".to_owned(),
        Avoidance::LongTermInvolvement => "Remaining involved long-term".to_owned(),
        Avoidance::LeavingEmployeesBehind => "Leaving employees behind".to_owned(),
        Avoidance::ExcessiveDebt => "Excessive debt".to_owned(),
        Avoidance::WaitingManyYearsForProceeds => "Waiting many years for proceeds".to_owned(),
        Avoidance::MovingTheBusiness => "Moving the business".to_owned(),
        Avoidance::LosingIndependence => "Losing company independence".to_owned(),
        Avoidance::MajorOperationalDisruption => "Major disruption to operations".to_owned(),
        Avoidance::Other(text) => format!("Other: {text}"),
    }
}

/// Every answered objective of the content, rendered — the package's
/// destination snapshot lines.
pub fn objective_lines(content: &DestinationContent) -> Vec<forhemit_package::ObjectiveLine> {
    let mut lines = Vec::new();
    macro_rules! line {
        ($label:literal, $objective:expr, $render:expr) => {
            lines.extend(objective_line($label, &$objective, $render));
        };
    }
    line!(
        "Main financial objective",
        content.financial_objective,
        |value: &FinancialObjective| match value {
            FinancialObjective::CashNow => "Substantial cash at closing".to_owned(),
            FinancialObjective::IncomeOverTime => "Some of the value over time".to_owned(),
            FinancialObjective::Combination => "A mix of cash now and future income".to_owned(),
        }
    );
    line!(
        "Desired cash at closing",
        content.closing_proceeds,
        |value: &ProceedsChoice| { proceeds_choice_label(value) }
    );
    line!(
        "Desired future income",
        content.future_income,
        |value: &FutureIncomeChoice| { future_income_label(value) }
    );
    line!(
        "Who should have ownership",
        content.ownership_participants,
        |values: &Vec<OwnershipParticipant>| values
            .iter()
            .map(participant_label)
            .collect::<Vec<_>>()
            .join(", ")
    );
    line!(
        "How employee ownership should work",
        content.employee_ownership_shape,
        |shape: &forhemit_destination::content::EmployeeOwnershipShape| ownership_shape_label(
            shape
        )
        .to_owned()
    );
    line!(
        "Ownership allocation",
        content.ownership_allocation,
        |choice: &AllocationChoice| { allocation_choice_label(choice) }
    );
    line!(
        "Role after the transition",
        content.owner_role,
        |role: &OwnerRole| { owner_role_label(role.clone()).to_owned() }
    );
    line!(
        "Transition timeframe",
        content.transition_timing,
        |timing: &TransitionTiming| { timing_label(timing.clone()).to_owned() }
    );
    lines.extend(
        answer_text(
            &content.preservation_goals,
            |goals: &Vec<Objective<PreservationSelection>>| {
                goals
                    .iter()
                    .filter_map(|objective| {
                        answer_text(&objective.value, |selection| {
                            preservation_selection_label(selection)
                        })
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        )
        .map(|value| forhemit_package::ObjectiveLine {
            label: "What should remain true".to_owned(),
            value,
            preference: forhemit_contracts::NonnegotiableState::Unspecified,
        }),
    );
    lines.extend(
        answer_text(&content.avoidances, |items: &Vec<Objective<Avoidance>>| {
            items
                .iter()
                .filter_map(|objective| answer_text(&objective.value, avoidance_label))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .map(|value| forhemit_package::ObjectiveLine {
            label: "What to avoid".to_owned(),
            value,
            preference: forhemit_contracts::NonnegotiableState::Unspecified,
        }),
    );
    lines.extend(
        answer_text(&content.additional_context, |text: &String| text.clone()).map(|value| {
            forhemit_package::ObjectiveLine {
                label: "Additional owner context".to_owned(),
                value,
                preference: forhemit_contracts::NonnegotiableState::Unspecified,
            }
        }),
    );
    lines
}

/// One journey answer line — values rendered with their choice labels.
fn answer_line(
    engines: &AppEngines,
    node_id: &forhemit_contracts::JourneyNodeId,
    title: &str,
    value: &AnswerValue,
) -> forhemit_package::AnswerLine {
    let scalar = value.scalars().to_vec();
    let values: Vec<String> = match value {
        AnswerValue::Confirmed => Vec::new(),
        AnswerValue::Amount(amount) => vec![money(*amount)],
        _ => scalar
            .iter()
            .map(|chosen| {
                engines
                    .definition
                    .node(node_id)
                    .and_then(|node| node.question())
                    .and_then(|question| {
                        question
                            .choices
                            .iter()
                            .find(|choice| &choice.value == chosen)
                    })
                    .map(|choice| choice.label.clone())
                    .unwrap_or_else(|| chosen.to_string())
            })
            .collect(),
    };
    forhemit_package::AnswerLine {
        node_id: node_id.as_str().to_owned(),
        title: title.to_owned(),
        values,
    }
}

/// Maps the scenario crate's readiness so the gate can run, then builds
/// the package-facing scenario snapshot.
fn scenario_snapshot(
    engines: &AppEngines,
    family_name: &str,
    version: &ScenarioVersion,
) -> Result<forhemit_package::ScenarioSnapshot, String> {
    Ok(forhemit_package::ScenarioSnapshot {
        scenario_version_id: version.scenario_version_id.clone(),
        family_name: family_name.to_owned(),
        name: version.name.clone(),
        description: version.description.clone(),
        scenario_type: crate::views::scenario::scenario_type_label(version.scenario_type)
            .to_owned(),
        lifecycle_status: crate::views::scenario::lifecycle_label(version.lifecycle_status)
            .to_owned(),
        readiness: export_readiness(version.readiness_status),
        assumptions: version
            .assumptions
            .iter()
            .map(|assumption| forhemit_package::AssumptionLine {
                name: assumption.name.clone(),
                description: assumption.description.clone(),
                value: crate::views::scenario::typed_value_label(&assumption.value),
                category: crate::views::scenario::category_label(assumption.category).to_owned(),
                provenance: assumption.provenance.clone(),
                verification: assumption.verification.clone(),
            })
            .collect(),
        unknowns: version
            .unknowns
            .iter()
            .map(|unknown| forhemit_package::UnknownLine {
                description: unknown.description.clone(),
                importance: crate::views::scenario::importance_label(unknown.importance).to_owned(),
                resolution_status: crate::views::scenario::resolution_status_label(
                    unknown.resolution_status,
                )
                .to_owned(),
            })
            .collect(),
        conflicts: engines
            .scenario
            .conflicts_of(&version.scenario_version_id)
            .map_err(|error| error.to_string())?
            .iter()
            .map(|record| forhemit_package::ConflictLine {
                description: record.conflict.description.clone(),
                severity: crate::views::scenario::severity_label(record.conflict.severity)
                    .to_owned(),
            })
            .collect(),
    })
}

/// Fills the package engine's workspace snapshot from the engines'
/// current state — the shell's one mapping duty (inputs.rs module doc).
/// In-memory scenario families live in the session's id list; journey
/// answers render with their choice labels; Business Reality facts carry
/// their provenance stamps.
///
/// # Errors
///
/// An engine read refuses (poisoned lock), or no destination exists yet.
pub fn workspace_snapshot(
    engines: &AppEngines,
) -> Result<forhemit_package::WorkspaceSnapshot, String> {
    let workspace_id = engines.workspace_id.clone();
    let destination_snapshot = engines
        .with_destination(|slot| {
            let destination = slot.ok_or_else(|| {
                "no destination exists yet — build your destination before exporting a package"
                    .to_owned()
            })?;
            let head = destination.head_version();
            Ok(forhemit_package::DestinationSnapshot {
                destination_id: destination.destination_id.clone(),
                version_id: head.version_id.clone(),
                version_number: head.version_number,
                objectives: objective_lines(&head.content),
            })
        })
        .map_err(|error| error.to_string())
        .and_then(|inner| inner)?;

    let mut answers = Vec::new();
    let mut skipped_node_ids = Vec::new();
    let mut marked_nonnegotiables = Vec::new();
    let mut completed = false;
    if let Some(instance) = crate::commands::load_instance(engines)? {
        completed = instance.status == InstanceStatus::Completed;
        for node_id in &instance.visited {
            if let Some(record) = instance.answer(node_id) {
                let title = engines
                    .definition
                    .node(node_id)
                    .map_or_else(String::new, |node| node.title.clone());
                answers.push(answer_line(
                    engines,
                    node_id,
                    &title,
                    &record.latest().value,
                ));
            }
        }
        skipped_node_ids = instance
            .skipped
            .iter()
            .map(|node_id| node_id.as_str().to_owned())
            .collect();
        marked_nonnegotiables = instance
            .nonnegotiables
            .iter()
            .map(|mark| mark.target.clone())
            .collect();
    }
    let journey_snapshot = forhemit_package::JourneySnapshot {
        journey_id: engines.definition.journey_id.clone(),
        journey_version_used: engines.definition.version.as_str().to_owned(),
        completed,
        answers,
        skipped_node_ids,
        marked_nonnegotiables,
    };

    let facts = engines
        .reality
        .current_facts()
        .map_err(|error| error.to_string())?;
    // The same content-addressed derivation the scenario engine uses
    // (integrity rule 2) — same facts → same address, never a minted id.
    let fact_version_ids: Vec<forhemit_contracts::FactVersionId> = facts
        .iter()
        .map(|fact| fact.fact_version_id().clone())
        .collect();
    let reality_version = forhemit_scenario::business_reality_version(&fact_version_ids);
    let reality_snapshot = forhemit_package::RealitySnapshot {
        business_reality_version_id: reality_version,
        facts: facts
            .iter()
            .map(|fact: &FactVersion| forhemit_package::FactLine {
                kind: format!("{:?}", fact.kind()),
                value: fact_value_label(fact.value()),
                period: format!("{:?}", fact.period()),
                definition: fact.definition().map(str::to_owned),
                provenance: fact.provenance(),
                verification: fact.verification(),
            })
            .collect(),
    };

    let mut scenarios = Vec::new();
    for family_id in engines.scenario_family_ids()? {
        let family = engines
            .scenario
            .family(&family_id)
            .map_err(|error| error.to_string())?;
        for version in engines
            .scenario
            .versions_of_family(&family_id)
            .map_err(|error| error.to_string())?
        {
            scenarios.push(scenario_snapshot(engines, &family.name, &version)?);
        }
    }

    Ok(forhemit_package::WorkspaceSnapshot {
        workspace_id,
        journey: journey_snapshot,
        destination: destination_snapshot,
        reality: reality_snapshot,
        scenarios,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // tests: failures must panic the test

    use super::*;

    #[test]
    fn objective_lines_render_answered_values_and_drop_blank_ones() {
        let mut content = DestinationContent::default();
        content.financial_objective.value = Answer::Answered(FinancialObjective::Combination);
        content.closing_proceeds.value =
            Answer::Answered(ProceedsChoice::Band(ProceedsBand::K1mTo2m));
        content.closing_proceeds.preference = forhemit_contracts::NonnegotiableState::Nonnegotiable;
        content.ownership_participants.value = Answer::NotSure;

        let lines = objective_lines(&content);
        let financial = lines
            .iter()
            .find(|line| line.label == "Main financial objective")
            .expect("answered objective renders");
        assert_eq!(financial.value, "A mix of cash now and future income");
        let proceeds = lines
            .iter()
            .find(|line| line.label == "Desired cash at closing")
            .expect("answered objective renders");
        assert_eq!(proceeds.value, "$1M–$2M");
        assert_eq!(
            proceeds.preference,
            forhemit_contracts::NonnegotiableState::Nonnegotiable
        );
        let participants = lines
            .iter()
            .find(|line| line.label == "Who should have ownership")
            .expect("not-sure renders as its own line");
        assert_eq!(participants.value, "I'm not sure");
        assert!(!lines.iter().any(|line| line.label == "What to avoid"));
    }

    #[test]
    fn every_readiness_maps_onto_the_package_port() {
        assert!(matches!(
            export_readiness(ReadinessStatus::ReadyForProfessionalReview),
            ExportReadiness::ReadyForProfessionalReview
        ));
        assert!(matches!(
            export_readiness(ReadinessStatus::Preliminary),
            ExportReadiness::Preliminary
        ));
    }
}
