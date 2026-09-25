//! Scenario display projections — pure functions over the scenario
//! engine's state, no domain rules. Every status arrives pre-rendered
//! into the docs' vocabulary so the UI never derives meaning
//! (THREE DECISION LAYERS: the UI shows which layer said what; the
//! scenario layer proposes, it never recommends).

use forhemit_contracts::ComparisonDimensionId;
use forhemit_scenario::{
    AssumptionCategory, BranchType, ComparisonDimensionType, ComparisonOutcome, ConflictDecision,
    ConflictSeverity, ConflictType, LifecycleStatus, OwnerDecision, ReadinessStatus,
    ScenarioAssumption, ScenarioEngine, ScenarioUnknown, TypedValue, UnknownImportance,
    UnknownResolutionStatus,
};
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Formats an engine timestamp as RFC 3339; an unformattable value
/// renders empty rather than fabricating a time.
fn iso(when: OffsetDateTime) -> String {
    when.format(&Rfc3339).unwrap_or_default()
}

/// The scenario-type vocabulary as the schema doc §41 names the paths.
pub fn scenario_type_label(scenario_type: forhemit_scenario::ScenarioType) -> &'static str {
    use forhemit_scenario::ScenarioType::*;
    match scenario_type {
        Esop => "ESOP",
        DirectEmployeePurchase => "Direct employee purchase",
        ManagementBuyout => "Management buyout",
        EmployeeOwnedAcquisitionEntity => "Employee-owned acquisition entity",
        StagedOwnership => "Staged ownership",
        SellerFinancedEmployeeAcquisition => "Seller-financed employee acquisition",
        Hybrid => "Hybrid",
        RetainAndTransition => "Retain and transition",
        Other => "Other",
    }
}

/// Lifecycle axis — "where is it in its life?" (schema doc §40).
pub fn lifecycle_label(lifecycle: LifecycleStatus) -> &'static str {
    match lifecycle {
        LifecycleStatus::Idea => "Idea",
        LifecycleStatus::Draft => "Draft",
        LifecycleStatus::Preliminary => "Preliminary",
        LifecycleStatus::Modelable => "Modelable",
        LifecycleStatus::Comparable => "Comparable",
        LifecycleStatus::UnderProfessionalReview => "Under professional review",
        LifecycleStatus::Revised => "Revised",
        LifecycleStatus::SelectedForFurtherExploration => "Selected for further exploration",
        LifecycleStatus::HandedOff => "Handed off",
        LifecycleStatus::Historical => "Historical",
        LifecycleStatus::Archived => "Archived",
        LifecycleStatus::Superseded => "Superseded",
    }
}

/// Readiness axis — "can the package include it?" (schema doc §40).
/// A separate axis, never blended with lifecycle.
pub fn readiness_label(readiness: ReadinessStatus) -> &'static str {
    match readiness {
        ReadinessStatus::Preliminary => "Preliminary — early, honest about it",
        ReadinessStatus::InformationNeeded => "Information needed",
        ReadinessStatus::Modelable => "Modelable",
        ReadinessStatus::ReadyForComparison => "Ready for comparison",
        ReadinessStatus::ReadyForProfessionalReview => "Ready for professional review",
        ReadinessStatus::UnderReview => "Under review",
        ReadinessStatus::Revised => "Revised",
        ReadinessStatus::Superseded => "Superseded",
        ReadinessStatus::Archived => "Archived",
    }
}

/// Conflict severity — the condition's weight, in words, never a score
/// (schema doc §17).
pub fn severity_label(severity: ConflictSeverity) -> &'static str {
    match severity {
        ConflictSeverity::Informational => "Informational — worth knowing, nothing more",
        ConflictSeverity::Attention => "Attention — the owner should look at this",
        ConflictSeverity::Material => "Material — this changes the scenario's meaning",
        ConflictSeverity::Blocking => "Blocking — this path cannot proceed until resolved",
    }
}

/// What kind of conflict this is (schema doc §16).
pub fn conflict_type_label(conflict_type: ConflictType) -> &'static str {
    match conflict_type {
        ConflictType::OwnerObjective => "Owner objective conflict",
        ConflictType::Nonnegotiable => "Nonnegotiable conflict",
        ConflictType::Data => "Data conflict",
        ConflictType::Research => "Research conflict",
        ConflictType::Professional => "Professional determination conflict",
        ConflictType::Model => "Model conflict",
        ConflictType::Dependency => "Dependency conflict",
        ConflictType::Other => "Conflict",
    }
}

/// Why a what-if branch exists (schema doc §30).
pub fn branch_type_label(branch_type: BranchType) -> &'static str {
    match branch_type {
        BranchType::WhatIf => "What if…?",
        BranchType::Alternative => "Alternative structure",
        BranchType::StressCase => "Worst-case probe",
        BranchType::ProfessionalRequest => "Requested by a professional",
        BranchType::OwnerRequest => "Requested by the owner",
    }
}

/// How much a named unknown matters (schema doc §15).
pub fn importance_label(importance: UnknownImportance) -> &'static str {
    match importance {
        UnknownImportance::Critical => {
            "Critical — the scenario cannot serve its purpose without it"
        }
        UnknownImportance::Important => "Important — materially changes the picture",
        UnknownImportance::Helpful => "Helpful — nice to know",
    }
}

/// Where an unknown stands (schema doc §15).
pub fn resolution_status_label(status: UnknownResolutionStatus) -> &'static str {
    match status {
        UnknownResolutionStatus::Open => "Open",
        UnknownResolutionStatus::InProgress => "In progress",
        UnknownResolutionStatus::Resolved => "Resolved",
        UnknownResolutionStatus::Waived => "Waived — proceeding without it",
        UnknownResolutionStatus::Superseded => "Superseded",
    }
}

/// What an assumption is about (schema doc §10).
pub fn category_label(category: AssumptionCategory) -> &'static str {
    match category {
        AssumptionCategory::Business => "Business",
        AssumptionCategory::Financial => "Financial",
        AssumptionCategory::Ownership => "Ownership",
        AssumptionCategory::Market => "Market",
        AssumptionCategory::Financing => "Financing",
        AssumptionCategory::Tax => "Tax",
        AssumptionCategory::Legal => "Legal",
        AssumptionCategory::Operational => "Operational",
        AssumptionCategory::Timing => "Timing",
        AssumptionCategory::Employee => "Employees",
        AssumptionCategory::Governance => "Governance",
        AssumptionCategory::Seller => "Seller",
        AssumptionCategory::Buyer => "Buyer",
        AssumptionCategory::ExternalEnvironment => "External environment",
        AssumptionCategory::Other => "Other",
    }
}

/// The owner's decision on a nonnegotiable conflict (NONNEGOTIABLE doc
/// §5) — decided by the owner, never by the engine.
pub fn owner_decision_label(decision: OwnerDecision) -> &'static str {
    match decision {
        OwnerDecision::KeepRequirement => {
            "Keep the nonnegotiable — the scenario must change or be abandoned"
        }
        OwnerDecision::ExploreAnotherPath => "Explore a different scenario",
        OwnerDecision::ChangeRequirement => {
            "Change the requirement — creates a new destination version"
        }
    }
}

/// What a comparison dimension compares (schema doc §33).
pub fn dimension_type_label(dimension: ComparisonDimensionType) -> &'static str {
    match dimension {
        ComparisonDimensionType::OwnerObjective => "Owner objective",
        ComparisonDimensionType::Financial => "Financial",
        ComparisonDimensionType::Ownership => "Ownership",
        ComparisonDimensionType::Personal => "Personal",
        ComparisonDimensionType::Business => "Business continuity",
        ComparisonDimensionType::Timing => "Timing",
        ComparisonDimensionType::Financing => "Financing",
        ComparisonDimensionType::SellerNote => "Seller note",
        ComparisonDimensionType::Uncertainty => "Uncertainty",
        ComparisonDimensionType::InformationCompleteness => "Information completeness",
        ComparisonDimensionType::Evidence => "Evidence quality",
        ComparisonDimensionType::Nonnegotiable => "Nonnegotiable alignment",
        ComparisonDimensionType::ProfessionalReview => "Professional review state",
        ComparisonDimensionType::OutstandingQuestions => "Open questions",
    }
}

/// The factual reading of a comparison cell (schema doc §34) — no rank,
/// no score.
pub fn outcome_label(outcome: ComparisonOutcome) -> &'static str {
    match outcome {
        ComparisonOutcome::Aligns => "Aligns",
        ComparisonOutcome::DoesNotCurrentlyAlign => "Does not currently align",
        ComparisonOutcome::InsufficientInformation => "Not enough information to say",
    }
}

/// Renders a typed value as the owner sees it — exact decimals, never
/// binary floats (schema doc §8).
pub fn typed_value_label(value: &TypedValue) -> String {
    match value {
        TypedValue::Number(number) => number.as_str().to_owned(),
        TypedValue::Currency { amount, currency } => {
            format!("{} {}", amount.as_str(), currency.as_str())
        }
        TypedValue::Percentage(percentage) => format!("{}%", percentage.as_str()),
        TypedValue::Date(date) => date.to_string(),
        TypedValue::DateRange { from, to } => format!("{from} – {to}"),
        TypedValue::DurationMonths(months) => format!("{months} months"),
        TypedValue::Boolean(flag) => if *flag { "Yes" } else { "No" }.to_owned(),
        TypedValue::Text(text) | TypedValue::Enum(text) | TypedValue::Reference(text) => {
            text.clone()
        }
        TypedValue::Range {
            min,
            max: Some(max),
        } => format!("{} – {}", min.as_str(), max.as_str()),
        TypedValue::Range { min, max: None } => format!("from {}", min.as_str()),
    }
}

/// How a value entered the system (schema doc §9).
pub fn provenance_label(provenance: &forhemit_contracts::Provenance) -> &'static str {
    match provenance {
        forhemit_contracts::Provenance::OwnerReported => "Owner reported",
        forhemit_contracts::Provenance::DocumentSupported => "Supported by a document",
        forhemit_contracts::Provenance::ResearchSupported => "Supported by research",
        forhemit_contracts::Provenance::ProfessionallySupplied => "Supplied by a professional",
        forhemit_contracts::Provenance::ScenarioAssumed => "Assumed for exploration",
        forhemit_contracts::Provenance::SystemDerived => "Derived by the system",
        forhemit_contracts::Provenance::ModelDerived => "Derived by a model",
    }
}

/// How well a value is verified (schema doc §11) — the separate axis.
pub fn verification_label(verification: &forhemit_contracts::Verification) -> &'static str {
    use forhemit_contracts::Verification::*;
    match verification {
        Unknown => "Verification unknown",
        Unverified => "Unverified",
        PartiallyVerified => "Partially verified",
        Verified => "Verified",
        ProfessionallyVerified => "Professionally verified",
        Contested => "Contested",
        Rejected => "Rejected",
    }
}

/// The labels a UI select offers for typed values when the owner adds
/// an assumption.
pub const VALUE_TYPE_OPTIONS: &[(&str, &str)] = &[
    ("number", "Number"),
    ("currency", "Currency"),
    ("percentage", "Percentage"),
    ("duration", "Duration (months)"),
    ("boolean", "Yes / No"),
    ("text", "Text"),
];

/// One assumption, rendered with its two provenance axes kept separate.
#[derive(Clone, Debug, Serialize)]
pub struct AssumptionView {
    /// The assumption's id.
    pub assumption_id: String,
    /// The category.
    pub category: &'static str,
    /// The record's name.
    pub name: String,
    /// What the record says, in plain language.
    pub description: Option<String>,
    /// The value label.
    pub value_label: String,
    /// Whose information this is.
    pub provenance: &'static str,
    /// How the value has been checked.
    pub verification: &'static str,
    /// The source reference.
    pub source_reference: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One first-class unknown — missing information, named.
#[derive(Clone, Debug, Serialize)]
pub struct UnknownView {
    /// The unknown's id.
    pub unknown_id: String,
    /// The category.
    pub category: Option<String>,
    /// What the record says, in plain language.
    pub description: String,
    /// How much the gap matters, in words.
    pub importance: &'static str,
    /// Where the unknown stands.
    pub resolution_status: &'static str,
    /// The required action.
    pub required_action: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One destination nonnegotiable the version tests.
#[derive(Clone, Debug, Serialize)]
pub struct NonnegotiableUnderTestView {
    /// The nonnegotiable id.
    pub nonnegotiable_id: String,
    /// The destination objective id.
    pub destination_objective_id: String,
    /// The destination version this record is pinned to.
    pub destination_version_id: String,
    /// What the record says, in plain language.
    pub description: String,
}

/// One conflict, with the nonnegotiable extension when it is one — the
/// structured object surfaced for the owner's decision, never a warning
/// flag.
#[derive(Clone, Debug, Serialize)]
pub struct ConflictView {
    /// The conflict's id.
    pub conflict_id: String,
    /// The scenario version id.
    pub scenario_version_id: String,
    /// What kind of conflict it is.
    pub conflict_type: &'static str,
    /// How the condition weighs on the decision, in words — never a score.
    pub severity: &'static str,
    /// What the record says, in plain language.
    pub description: String,
    /// The is nonnegotiable.
    pub is_nonnegotiable: bool,
    /// Present only when `is_nonnegotiable` — the destination
    /// back-reference and the owner's decision, if made.
    pub nonnegotiable: Option<NonnegotiableConflictView>,
    /// The resolved.
    pub resolved: bool,
    /// The owner's decision, when the owner decided (display label).
    pub owner_decision: Option<&'static str>,
    /// Where the resolution came from, when recorded.
    pub resolution_reference: Option<String>,
    /// When the conflict was resolved (RFC 3339).
    pub resolved_at: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// The structured nonnegotiable extension (schema doc §14).
#[derive(Clone, Debug, Serialize)]
pub struct NonnegotiableConflictView {
    /// The nonnegotiable id.
    pub nonnegotiable_id: String,
    /// The destination objective id.
    pub destination_objective_id: String,
    /// The destination version this record is pinned to.
    pub destination_version_id: String,
    /// The owner decision.
    pub owner_decision: Option<&'static str>,
    /// The decided at.
    pub decided_at: Option<String>,
}

/// One what-if branch row — the parent is always identified (integrity
/// rule 7).
#[derive(Clone, Debug, Serialize)]
pub struct BranchView {
    /// The branch id.
    pub branch_id: String,
    /// The parent scenario version id.
    pub parent_scenario_version_id: String,
    /// The child scenario family id.
    pub child_scenario_family_id: String,
    /// Whether the branch is a what-if or a revision.
    pub branch_type: &'static str,
    /// The reason.
    pub reason: String,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One cell of a comparison — a factual outcome per (version, dimension).
/// No winner column exists anywhere.
#[derive(Clone, Debug, Serialize)]
pub struct ComparisonCellView {
    /// The scenario version id.
    pub scenario_version_id: String,
    /// The dimension index.
    pub dimension_index: usize,
    /// The value label.
    pub value_label: Option<String>,
    /// The comparison outcome in words — never a winner or score.
    pub outcome: &'static str,
}

/// One durable comparison (schema doc §32): what was compared, never
/// which scenario won.
#[derive(Clone, Debug, Serialize)]
pub struct ComparisonView {
    /// The comparison's id.
    pub comparison_id: String,
    /// The record's name.
    pub name: String,
    /// The versions being compared.
    pub scenario_version_ids: Vec<String>,
    /// The dimensions the comparison is organized by.
    pub dimensions: Vec<ComparisonDimensionView>,
    /// One row per scenario version, one cell per dimension.
    pub cells: Vec<ComparisonCellView>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One comparison dimension, with its display order — presentation,
/// never ranking.
#[derive(Clone, Debug, Serialize)]
pub struct ComparisonDimensionView {
    /// The index.
    pub index: usize,
    /// The dimension type.
    pub dimension_type: &'static str,
    /// The choice's label as shown in the journey.
    pub label: String,
}

/// One row of a version's status history (schema doc §36).
#[derive(Clone, Debug, Serialize)]
pub struct StatusEntryView {
    /// The previous lifecycle.
    pub previous_lifecycle: Option<&'static str>,
    /// The new lifecycle.
    pub new_lifecycle: &'static str,
    /// The previous readiness.
    pub previous_readiness: Option<&'static str>,
    /// The new readiness.
    pub new_readiness: &'static str,
    /// The reason.
    pub reason: Option<String>,
    /// The changed at.
    pub changed_at: String,
}

/// A family — one conceptual path. Holds no scenario content.
#[derive(Clone, Debug, Serialize)]
pub struct ScenarioFamilyView {
    /// The scenario family id.
    pub scenario_family_id: String,
    /// The record's name.
    pub name: String,
    /// The scenario type.
    pub scenario_type: &'static str,
    /// The branched from version id.
    pub branched_from_version_id: Option<String>,
    /// The archived.
    pub archived: bool,
    /// The archive reason.
    pub archive_reason: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
    /// The family's versions, ordered by version number.
    pub versions: Vec<ScenarioVersionView>,
}

/// A version summary inside its family listing.
#[derive(Clone, Debug, Serialize)]
pub struct ScenarioVersionView {
    /// The scenario version id.
    pub scenario_version_id: String,
    /// The family id.
    pub family_id: String,
    /// The version's number.
    pub version_number: u32,
    /// The record's name.
    pub name: String,
    /// What the record says, in plain language.
    pub description: Option<String>,
    /// The change reason.
    pub change_reason: Option<String>,
    /// The lifecycle.
    pub lifecycle: &'static str,
    /// The version's readiness, in words.
    pub readiness: &'static str,
    /// The is draft.
    pub is_draft: bool,
    /// The destination version this record is pinned to.
    pub destination_version_id: String,
    /// The Business Reality version this record is pinned to.
    pub business_reality_version_id: String,
    /// The version this one was branched or revised from.
    pub parent_version_id: Option<String>,
    /// The supersedes version id.
    pub supersedes_version_id: Option<String>,
    /// The version's typed assumptions.
    pub assumptions: Vec<AssumptionView>,
    /// The version's first-class unknowns.
    pub unknowns: Vec<UnknownView>,
    /// The destination nonnegotiables under test in this version.
    pub nonnegotiables: Vec<NonnegotiableUnderTestView>,
    /// The conflicts attached to this version, if any.
    pub conflicts: Vec<ConflictView>,
    /// The branches.
    pub branches: Vec<BranchView>,
    /// The status history.
    pub status_history: Vec<StatusEntryView>,
    /// The finalized at.
    pub finalized_at: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// A scenario family with its versions and satellites — everything the
/// explorer screen renders for one family.
///
/// # Errors
///
/// The engine's table lock is poisoned.
pub fn family_view(
    engine: &ScenarioEngine<crate::state::TeeSink>,
    family: &forhemit_scenario::ScenarioFamily,
) -> Result<ScenarioFamilyView, String> {
    let versions = engine
        .versions_of_family(&family.scenario_family_id)
        .map_err(|error| error.to_string())?;
    let mut version_views = Vec::new();
    for version in &versions {
        version_views.push(version_view(engine, version)?);
    }
    Ok(ScenarioFamilyView {
        scenario_family_id: family.scenario_family_id.as_str().to_owned(),
        name: family.name.clone(),
        scenario_type: scenario_type_label(family.scenario_type),
        branched_from_version_id: family
            .branched_from_version_id
            .as_ref()
            .map(|id| id.as_str().to_owned()),
        archived: family.archived_at.is_some(),
        archive_reason: family.archive_reason.clone(),
        created_at: iso(family.created_at),
        versions: version_views,
    })
}

/// Everything the explorer renders for one version, including its
/// conflicts and branches (which are rows in their own tables —
/// integrity rule 8).
///
/// # Errors
///
/// The engine refuses a read (unknown version or poisoned lock).
pub fn version_view(
    engine: &ScenarioEngine<crate::state::TeeSink>,
    version: &forhemit_scenario::ScenarioVersion,
) -> Result<ScenarioVersionView, String> {
    let conflicts = engine
        .conflicts_of(&version.scenario_version_id)
        .map_err(|error| error.to_string())?;
    let branches = engine
        .branches_from(&version.scenario_version_id)
        .map_err(|error| error.to_string())?;
    Ok(ScenarioVersionView {
        scenario_version_id: version.scenario_version_id.as_str().to_owned(),
        family_id: version.scenario_family_id.as_str().to_owned(),
        version_number: version.version_number,
        name: version.name.clone(),
        description: version.description.clone(),
        change_reason: version.change_reason.clone(),
        lifecycle: lifecycle_label(version.lifecycle_status),
        readiness: readiness_label(version.readiness_status),
        is_draft: version.is_draft(),
        destination_version_id: version.destination_version_id.as_str().to_owned(),
        business_reality_version_id: version.business_reality_version_id.as_str().to_owned(),
        parent_version_id: version
            .parent_version_id
            .as_ref()
            .map(|id| id.as_str().to_owned()),
        supersedes_version_id: version
            .supersedes_version_id
            .as_ref()
            .map(|id| id.as_str().to_owned()),
        assumptions: version.assumptions.iter().map(assumption_view).collect(),
        unknowns: version.unknowns.iter().map(unknown_view).collect(),
        nonnegotiables: version
            .nonnegotiables
            .iter()
            .map(|row| NonnegotiableUnderTestView {
                nonnegotiable_id: row.nonnegotiable_id.as_str().to_owned(),
                destination_objective_id: row.destination_objective_id.as_str().to_owned(),
                destination_version_id: row.destination_version_id.as_str().to_owned(),
                description: row.description.clone(),
            })
            .collect(),
        conflicts: conflicts.iter().map(conflict_view).collect(),
        branches: branches.iter().map(branch_view).collect(),
        status_history: version
            .status_history
            .iter()
            .map(|entry| StatusEntryView {
                previous_lifecycle: entry.previous_lifecycle.map(lifecycle_label),
                new_lifecycle: lifecycle_label(entry.new_lifecycle),
                previous_readiness: entry.previous_readiness.map(readiness_label),
                new_readiness: readiness_label(entry.new_readiness),
                reason: entry.reason.clone(),
                changed_at: iso(entry.changed_at),
            })
            .collect(),
        finalized_at: version.finalized_at.map(iso),
        created_at: iso(version.created_at),
    })
}

fn assumption_view(assumption: &ScenarioAssumption) -> AssumptionView {
    AssumptionView {
        assumption_id: assumption.assumption_id.as_str().to_owned(),
        category: category_label(assumption.category),
        name: assumption.name.clone(),
        description: assumption.description.clone(),
        value_label: typed_value_label(&assumption.value),
        provenance: provenance_label(&assumption.provenance),
        verification: verification_label(&assumption.verification),
        source_reference: assumption.source_reference.clone(),
        created_at: iso(assumption.created_at),
    }
}

fn unknown_view(unknown: &ScenarioUnknown) -> UnknownView {
    UnknownView {
        unknown_id: unknown.unknown_id.as_str().to_owned(),
        category: unknown.category.clone(),
        description: unknown.description.clone(),
        importance: importance_label(unknown.importance),
        resolution_status: resolution_status_label(unknown.resolution_status),
        required_action: unknown.required_action.clone(),
        created_at: iso(unknown.created_at),
    }
}

fn conflict_view(record: &forhemit_scenario::ConflictRecord) -> ConflictView {
    let conflict = &record.conflict;
    ConflictView {
        conflict_id: conflict.conflict_id.as_str().to_owned(),
        scenario_version_id: conflict.scenario_version_id.as_str().to_owned(),
        conflict_type: conflict_type_label(conflict.conflict_type),
        severity: severity_label(conflict.severity),
        description: conflict.description.clone(),
        is_nonnegotiable: record.nonnegotiable.is_some(),
        nonnegotiable: record
            .nonnegotiable
            .as_ref()
            .map(|extension| NonnegotiableConflictView {
                nonnegotiable_id: extension.nonnegotiable_id.as_str().to_owned(),
                destination_objective_id: extension.destination_objective_id.as_str().to_owned(),
                destination_version_id: extension.destination_version_id.as_str().to_owned(),
                owner_decision: extension.owner_decision.map(owner_decision_label),
                decided_at: extension.decided_at.map(iso),
            }),
        resolved: conflict.resolution.is_some(),
        owner_decision: conflict
            .resolution
            .as_ref()
            .and_then(|resolution| resolution.owner_decision)
            .or_else(|| {
                record
                    .nonnegotiable
                    .as_ref()
                    .and_then(|extension| extension.owner_decision)
            })
            .map(owner_decision_label),
        resolution_reference: conflict
            .resolution
            .as_ref()
            .and_then(|resolution| resolution.resolution_reference.clone()),
        resolved_at: conflict
            .resolution
            .as_ref()
            .map(|resolution| iso(resolution.decided_at)),
        created_at: iso(conflict.created_at),
    }
}

fn branch_view(branch: &forhemit_scenario::ScenarioBranch) -> BranchView {
    BranchView {
        branch_id: branch.branch_id.as_str().to_owned(),
        parent_scenario_version_id: branch.parent_scenario_version_id.as_str().to_owned(),
        child_scenario_family_id: branch.child_scenario_family_id.as_str().to_owned(),
        branch_type: branch_type_label(branch.branch_type),
        reason: branch.reason.clone(),
        created_at: iso(branch.created_at),
    }
}

/// A created family with its first version — the create-family and
/// what-if commands' return shape, so the UI can jump straight to the
/// new draft.
#[derive(Clone, Debug, Serialize)]
pub struct FamilyAndVersionView {
    /// The family (with all its versions).
    pub family: ScenarioFamilyView,
    /// The family's new draft version.
    pub version: ScenarioVersionView,
}

/// A recorded comparison as the UI shows it — factual rows on shared
/// dimensions, never a winner (schema doc §33).
pub fn comparison_view(comparison: &forhemit_scenario::ScenarioComparison) -> ComparisonView {
    // Results point at dimensions by id; the view indexes them so the
    // frontend can build a grid without knowing the ids.
    let dimension_index = |dimension_id: &ComparisonDimensionId| {
        comparison
            .dimensions
            .iter()
            .position(|dimension| &dimension.dimension_id == dimension_id)
            .unwrap_or(usize::MAX) // tests: a result always references a dimension of its own comparison
    };
    let mut cells = Vec::new();
    for result in &comparison.results {
        cells.push(ComparisonCellView {
            scenario_version_id: result.scenario_version_id.as_str().to_owned(),
            dimension_index: dimension_index(&result.dimension_id),
            value_label: result.value.as_ref().map(typed_value_label),
            outcome: outcome_label(result.outcome),
        });
    }
    ComparisonView {
        comparison_id: comparison.comparison_id.as_str().to_owned(),
        name: comparison.name.clone(),
        scenario_version_ids: comparison
            .scenario_version_ids
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
        dimensions: comparison
            .dimensions
            .iter()
            .enumerate()
            .map(|(index, dimension)| ComparisonDimensionView {
                index,
                dimension_type: dimension_type_label(dimension.dimension_type),
                label: dimension.label.clone(),
            })
            .collect(),
        cells,
        created_at: iso(comparison.created_at),
    }
}

/// The owner's decision on a nonnegotiable conflict, as the wire spells
/// it — decoded back to the engine vocabulary.
pub fn owner_decision_from_wire(decision: &str) -> Result<OwnerDecision, String> {
    match decision {
        "keep_requirement" => Ok(OwnerDecision::KeepRequirement),
        "explore_another_path" => Ok(OwnerDecision::ExploreAnotherPath),
        "change_requirement" => Ok(OwnerDecision::ChangeRequirement),
        other => Err(format!("unknown owner decision: {other}")),
    }
}

/// The conflict-closing decision: the owner's call on a nonnegotiable,
/// or a generic resolution with a reference.
pub fn conflict_decision_from_wire(
    owner_decision: Option<String>,
    resolution_reference: Option<String>,
) -> Result<ConflictDecision, String> {
    match owner_decision {
        Some(decision) => Ok(ConflictDecision::Owner(owner_decision_from_wire(
            &decision,
        )?)),
        None => Ok(ConflictDecision::Resolved {
            resolution_reference,
        }),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // tests: failures must panic the test

    use super::*;

    #[test]
    fn typed_values_render_exactly() {
        let currency = TypedValue::Currency {
            amount: forhemit_scenario::FixedDecimal::parse("2400000").unwrap(),
            currency: forhemit_scenario::CurrencyCode::parse("USD").unwrap(),
        };
        assert_eq!(typed_value_label(&currency), "2400000 USD");
        let percentage =
            TypedValue::Percentage(forhemit_scenario::FixedDecimal::parse("37.5").unwrap());
        assert_eq!(typed_value_label(&percentage), "37.5%");
        let open_range = TypedValue::Range {
            min: forhemit_scenario::FixedDecimal::parse("10").unwrap(),
            max: None,
        };
        assert_eq!(typed_value_label(&open_range), "from 10");
    }

    #[test]
    fn wire_decisions_decode_to_engine_vocabulary() {
        assert!(matches!(
            owner_decision_from_wire("keep_requirement").unwrap(),
            OwnerDecision::KeepRequirement
        ));
        assert!(owner_decision_from_wire("winner").is_err());
        // A decision without an owner call is the generic resolution.
        match conflict_decision_from_wire(None, Some("see notes".to_owned())).unwrap() {
            ConflictDecision::Resolved {
                resolution_reference,
            } => {
                assert_eq!(resolution_reference.as_deref(), Some("see notes"));
            }
            ConflictDecision::Owner(_) => panic!("generic decision decoded as an owner decision"),
        }
    }
}
