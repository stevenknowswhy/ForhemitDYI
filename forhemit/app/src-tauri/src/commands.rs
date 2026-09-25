//! The command layer: thin adapters from the UI to the engine APIs.
//!
//! Every command delegates to an engine — validation, versioning, and
//! audit emission happen inside the engine crates, never here. Commands
//! surface engine refusals verbatim as strings; the UI displays them
//! honestly rather than swallowing them.

use serde::Deserialize;

use forhemit_audit::new_correlation_id;
use forhemit_contracts::{
    AssumptionId, ConflictId, DestinationId, DestinationVersionId, DocumentId, DocumentVersionId,
    FactVersionId, JourneyInstanceId, ObjectiveId, Provenance, ScenarioFamilyId, ScenarioVersionId,
    UnknownId, VaultId, Verification,
};
use forhemit_destination::{
    ChangeReason, Completeness, Destination, DestinationContent, DestinationEngine,
};
use forhemit_journey::{
    AnswerValue, JourneyEngine, JourneyError, JourneyInstance, RecordAnswer, ReviseAnswer, SkipNode,
};
use forhemit_package::{ExportFormat, ExportedContent};
use forhemit_reality::fact::FactVersion;
use forhemit_reality::{FactPeriod, FactValue, RecordFact, ReviseFact};
use forhemit_scenario::{
    AssumptionCategory, BranchType, ComparisonDimensionType, ComparisonOutcome, ConflictSeverity,
    ConflictType, ConstraintType, NewAssumption, NewComparison, NewComparisonDimension,
    NewComparisonResult, NewConflict, NewConstraint, NewDraft, NewFamily, NewNonnegotiable,
    NewUnknown, NewWhatIf, ReadinessStatus, ScenarioType, TypedValue, UnknownImportance,
    UnknownResolution, UnknownResolutionStatus,
};
use forhemit_vault::{validate_recovery_passphrase, VaultEngine, VaultError, VaultStore};
use ulid::Ulid;

use crate::state::{AppEngines, VaultSlot};
use crate::views::{
    comparison_view, conflict_decision_from_wire, document_view, exported_file_view, family_view,
    locked_view, not_set_up_view, package_preview_view, search_hit_view, status_view,
    version_content_view, version_view, version_views, workspace_snapshot, ComparisonView,
    ExportedFileView, FamilyAndVersionView, JourneyView, PackagePreviewView, ScenarioFamilyView,
    ScenarioVersionView, VaultDocumentHistoryView, VaultDocumentView, VaultSearchHitView,
    VaultState, VaultStatusView, VaultVersionContentView, VerifyView,
};

/// A new journey instance id — a ULID-prefixed identifier.
fn new_instance_id() -> JourneyInstanceId {
    match JourneyInstanceId::new(format!("inst_{}", Ulid::new())) {
        Ok(id) => id,
        // A generated ULID id is never empty; this arm is unreachable.
        Err(error) => panic!("generated journey instance id rejected: {error}"),
    }
}

/// Loads the active walk, if one was started and the store still has it.
/// A stale pointer (store without the instance) is a legitimate empty
/// state — the UI offers to start a walk — not an error to surface.
pub(crate) fn load_instance(engines: &AppEngines) -> Result<Option<JourneyInstance>, String> {
    let Some(instance_id) = engines.active_journey()? else {
        return Ok(None);
    };
    let engine = journey_engine(engines);
    match engine.load(&engines.definition, &instance_id) {
        Ok(instance) => Ok(Some(instance)),
        Err(JourneyError::InstanceNotFound { .. }) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

/// Builds a journey engine over the shared stores.
fn journey_engine<'a>(engines: &'a AppEngines) -> JourneyEngine<'a> {
    JourneyEngine::new(
        engines.clock.as_ref(),
        engines.tee.as_ref(),
        engines.journey_store.as_ref(),
        engines.workspace_id.clone(),
    )
}

/// Creates the destination from the screen-13 draft — the first object
/// every journey creates.
///
/// # Errors
///
/// Destination engine refusals (invalid content, audit refusal).
pub fn destination_create(
    engines: &AppEngines,
    content: DestinationContent,
) -> Result<Destination, String> {
    let engine = DestinationEngine::new(
        engines.clock.as_ref(),
        engines.tee.as_ref(),
        engines.workspace_id.clone(),
    );
    let destination = engine
        .create(
            DestinationId::new(format!("dst_{}", Ulid::new()))
                .map_err(|error| error.to_string())?,
            content,
            &engines.actor,
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    engines.set_destination(Some(destination.clone()))?;
    Ok(destination)
}

/// Applies a material edit as a new immutable version.
///
/// # Errors
///
/// Destination engine refusals (no destination yet, no material change,
/// invalid allocation, audit refusal).
pub fn destination_edit(
    engines: &AppEngines,
    content: DestinationContent,
    reason: ChangeReason,
    explanation: Option<String>,
) -> Result<Destination, String> {
    let engine = DestinationEngine::new(
        engines.clock.as_ref(),
        engines.tee.as_ref(),
        engines.workspace_id.clone(),
    );
    engines.with_destination_mut(|mut slot| {
        let destination = slot
            .as_mut()
            .ok_or_else(|| "no destination exists yet".to_owned())?;
        engine
            .edit(
                destination,
                content,
                reason,
                explanation,
                &engines.actor,
                new_correlation_id(),
                None,
            )
            .map_err(|error| error.to_string())
    })??;
    destination_get(engines).ok_or_else(|| "destination disappeared".to_owned())
}

/// The owner confirms the destination.
///
/// # Errors
///
/// Destination engine refusals (no destination, illegal transition, audit
/// refusal).
pub fn destination_confirm(engines: &AppEngines) -> Result<Destination, String> {
    transition(engines, |engine, destination, actor, correlation| {
        engine.confirm(destination, actor, correlation, None)
    })
}

/// The owner keeps the destination as a working draft.
///
/// # Errors
///
/// Destination engine refusals.
pub fn destination_mark_working(engines: &AppEngines) -> Result<Destination, String> {
    transition(engines, |engine, destination, actor, correlation| {
        engine.mark_working(destination, actor, correlation, None)
    })
}

/// Archives the destination.
///
/// # Errors
///
/// Destination engine refusals.
pub fn destination_archive(engines: &AppEngines) -> Result<Destination, String> {
    transition(engines, |engine, destination, actor, correlation| {
        engine.archive(destination, actor, correlation, None)
    })
}

/// Runs one destination status transition and persists the aggregate.
fn transition(
    engines: &AppEngines,
    apply: impl FnOnce(
        &DestinationEngine,
        &mut Destination,
        &forhemit_contracts::ActorRecord,
        forhemit_contracts::CorrelationId,
    ) -> Result<(), forhemit_destination::DestinationError>,
) -> Result<Destination, String> {
    let engine = DestinationEngine::new(
        engines.clock.as_ref(),
        engines.tee.as_ref(),
        engines.workspace_id.clone(),
    );
    engines.with_destination_mut(|mut slot| {
        let destination = slot
            .as_mut()
            .ok_or_else(|| "no destination exists yet".to_owned())?;
        apply(&engine, destination, &engines.actor, new_correlation_id())
            .map_err(|error| error.to_string())
    })??;
    destination_get(engines).ok_or_else(|| "destination disappeared".to_owned())
}

/// The destination aggregate with its full history, if one exists.
#[must_use]
pub fn destination_get(engines: &AppEngines) -> Option<Destination> {
    engines
        .with_destination(|slot| slot.cloned())
        .unwrap_or_default()
}

/// The score-free completeness indicator of arbitrary content — the UI's
/// draft on screen 13 computes this before the destination exists.
#[must_use]
pub fn destination_completeness_for(content: &DestinationContent) -> Completeness {
    Completeness::of(content)
}

/// Starts a fresh walk of the pinned EOJ v0.2 journey and points the
/// workspace at it.
///
/// # Errors
///
/// Engine or store failures.
pub fn journey_start(engines: &AppEngines) -> Result<JourneyView, String> {
    let engine = journey_engine(engines);
    let instance = engine
        .start(&engines.definition, new_instance_id(), &engines.actor)
        .map_err(|error| error.to_string())?;
    engines.set_active_journey(&instance.instance_id)?;
    crate::views::journey_view(engines, &instance)
}

/// The active walk's view, if one exists.
///
/// # Errors
///
/// Engine or store failures.
pub fn journey_state(engines: &AppEngines) -> Result<Option<JourneyView>, String> {
    load_instance(engines)?
        .map(|instance| crate::views::journey_view(engines, &instance))
        .transpose()
}

/// Records the answer for the walk's current question.
///
/// # Errors
///
/// Journey engine refusals (not the current position, invalid value,
/// audit refusal).
pub fn journey_record(
    engines: &AppEngines,
    node_id: String,
    value: AnswerValue,
) -> Result<JourneyView, String> {
    let engine = journey_engine(engines);
    let node_id = forhemit_contracts::JourneyNodeId::new(node_id).map_err(|e| e.to_string())?;
    let mut instance =
        load_instance(engines)?.ok_or_else(|| "no journey walk in progress".to_owned())?;
    engine
        .record(
            &mut instance,
            &engines.definition,
            &RecordAnswer { node_id, value },
            &engines.actor,
        )
        .map_err(|error| error.to_string())?;
    crate::views::journey_view(engines, &instance)
}

/// Records a new version of a previously answered question.
///
/// # Errors
///
/// Journey engine refusals (no prior answer, empty reason, invalid value).
pub fn journey_revise(
    engines: &AppEngines,
    node_id: String,
    value: AnswerValue,
    change_reason: String,
) -> Result<JourneyView, String> {
    let engine = journey_engine(engines);
    let node_id = forhemit_contracts::JourneyNodeId::new(node_id).map_err(|e| e.to_string())?;
    let mut instance =
        load_instance(engines)?.ok_or_else(|| "no journey walk in progress".to_owned())?;
    engine
        .revise(
            &mut instance,
            &engines.definition,
            &ReviseAnswer {
                node_id,
                value,
                change_reason,
            },
            &engines.actor,
        )
        .map_err(|error| error.to_string())?;
    crate::views::journey_view(engines, &instance)
}

/// Skips the walk's current optional question.
///
/// # Errors
///
/// Journey engine refusals (a required node, wrong position).
pub fn journey_skip(engines: &AppEngines, node_id: String) -> Result<JourneyView, String> {
    let engine = journey_engine(engines);
    let node_id = forhemit_contracts::JourneyNodeId::new(node_id).map_err(|e| e.to_string())?;
    let mut instance =
        load_instance(engines)?.ok_or_else(|| "no journey walk in progress".to_owned())?;
    engine
        .skip(
            &mut instance,
            &engines.definition,
            &SkipNode { node_id },
            &engines.actor,
        )
        .map_err(|error| error.to_string())?;
    crate::views::journey_view(engines, &instance)
}

/// The current Business Snapshot facts, one per recorded kind, sorted by
/// kind. Each fact serializes its full current version — id, value,
/// period, definition, provenance, verification — so the UI can label
/// "Whose information am I looking at?" without deriving anything.
///
/// # Errors
///
/// The reality engine's fact-table lock is poisoned.
pub fn snapshot_current(engines: &AppEngines) -> Result<Vec<FactVersion>, String> {
    engines.reality.current_facts().map_err(|e| e.to_string())
}

/// Records an owner-reported fact.
///
/// # Errors
///
/// Reality engine refusals (a second fact of one kind, a value that does
/// not fit the kind, audit refusal).
pub fn snapshot_record(
    engines: &AppEngines,
    kind: forhemit_reality::FactKind,
    value: FactValue,
    definition: Option<String>,
) -> Result<Vec<FactVersion>, String> {
    engines
        .reality
        .record_fact(RecordFact {
            kind,
            value,
            period: FactPeriod::Current,
            definition,
            actor: engines.actor.clone(),
            correlation_id: new_correlation_id(),
        })
        .map_err(|error| error.to_string())?;
    snapshot_current(engines)
}

/// Records a new version of an existing fact.
///
/// # Errors
///
/// Reality engine refusals (unknown fact, empty reason, audit refusal).
pub fn snapshot_revise(
    engines: &AppEngines,
    fact_id: String,
    value: FactValue,
    definition: Option<String>,
    change_reason: String,
) -> Result<Vec<FactVersion>, String> {
    let fact_id = forhemit_contracts::FactId::new(fact_id).map_err(|e| e.to_string())?;
    engines
        .reality
        .revise_fact(ReviseFact {
            fact_id,
            value,
            period: FactPeriod::Current,
            definition,
            change_reason,
            actor: engines.actor.clone(),
            correlation_id: new_correlation_id(),
        })
        .map_err(|error| error.to_string())?;
    snapshot_current(engines)
}

/// The session audit log, oldest first.
///
/// # Errors
///
/// The log lock is poisoned.
pub fn audit_recent(engines: &AppEngines) -> Result<Vec<crate::state::AuditLogLine>, String> {
    engines.audit_log()
}

/// Verifies the audit hash chain over the whole store. Tampering is a
/// `VerifyView` with `verified: false` — surfaced loudly, never repaired
/// silently.
///
/// # Errors
///
/// Never for tampering; `Err` is for unexpected internal failures.
pub fn audit_verify(engines: &AppEngines) -> Result<VerifyView, String> {
    match engines.audit.verify_chain() {
        Ok(status) => Ok(VerifyView {
            verified: true,
            event_count: status.event_count,
            detail: "Hash chain verified — every event links to its predecessor.".to_owned(),
        }),
        Err(error) => Ok(VerifyView {
            verified: false,
            event_count: 0,
            detail: error.to_string(),
        }),
    }
}

// ---------------------------------------------------------------------------
// Scenario commands
// ---------------------------------------------------------------------------

/// A typed value as the frontend sends it: the value type's wire name,
/// the value as display text (a decimal string, a month count, "true"),
/// and the currency code for money.
#[derive(Clone, Debug, Deserialize)]
pub struct WireValue {
    /// One of `VALUE_TYPE_OPTIONS` (`number`, `currency`, `percentage`,
    /// `duration_months`, `boolean`, `text`).
    pub value_type: String,
    /// The value as display text.
    pub value: String,
    /// The ISO currency code when `value_type` is `currency`.
    pub currency: Option<String>,
}

/// The create-family request: the engine requires the destination version
/// and the exact Business Reality fact versions the scenario pins
/// (integrity rules 1–2), which the UI reads from the destination state
/// and snapshot views first.
#[derive(Clone, Debug, Deserialize)]
pub struct ScenarioCreateWire {
    /// The family's name.
    pub name: String,
    /// A `ScenarioType` wire name (snake_case).
    pub scenario_type: String,
    /// The draft's description, if any.
    pub description: Option<String>,
    /// The destination version this scenario is built against.
    pub destination_version_id: String,
    /// The Business Reality fact versions pinned at creation.
    pub reality_fact_version_ids: Vec<String>,
}

/// A new assumption as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct AssumptionWire {
    /// An `AssumptionCategory` wire name.
    pub category: String,
    /// The assumption's name.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The typed value.
    pub value: WireValue,
    /// A `Provenance` wire name — how the value entered the system.
    pub provenance: String,
    /// A `Verification` wire name — how well it is verified.
    pub verification: String,
    /// The destination objective this assumption tests, if any.
    pub nonnegotiable_objective_id: Option<String>,
    /// Where the value came from, if anywhere.
    pub source_reference: Option<String>,
}

/// A new unknown as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct UnknownWire {
    /// An optional category label.
    pub category: Option<String>,
    /// What is missing.
    pub description: String,
    /// An `UnknownImportance` wire name.
    pub importance: String,
    /// What to do about it, if known.
    pub required_action: Option<String>,
    /// What it depends on, if anything.
    pub source_dependency: Option<String>,
}

/// A new constraint as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct ConstraintWire {
    /// A `ConstraintType` wire name.
    pub constraint_type: String,
    /// The constraint's name.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The boundary's value, if it has one.
    pub value: Option<WireValue>,
    /// Where the constraint came from, if anywhere.
    pub source_reference: Option<String>,
}

/// A new conflict as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct ConflictWire {
    /// A `ConflictType` wire name.
    pub conflict_type: String,
    /// A `ConflictSeverity` wire name — the condition's weight, in words.
    pub severity: String,
    /// What the conflict is.
    pub description: String,
    /// Where the conflict came from, if anywhere.
    pub source_reference: Option<String>,
    /// What kind of object the conflict touches, if any.
    pub affected_object_type: Option<String>,
    /// Which object the conflict touches.
    pub affected_object_id: Option<String>,
}

/// The what-if request: the parent version, the branch kind, the reason,
/// and the parent assumptions the branch will override.
#[derive(Clone, Debug, Deserialize)]
pub struct WhatIfWire {
    /// The parent scenario version to branch from.
    pub parent_scenario_version_id: String,
    /// A `BranchType` wire name.
    pub branch_type: String,
    /// Why the branch exists.
    pub reason: String,
    /// The parent assumption ids the branch will override.
    pub changed_assumptions: Vec<String>,
    /// The child family's name.
    pub name: String,
}

/// One comparison dimension as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct ComparisonDimensionWire {
    /// A `ComparisonDimensionType` wire name.
    pub dimension_type: String,
    /// The dimension's label.
    pub label: String,
}

/// One comparison result cell as the frontend sends it.
#[derive(Clone, Debug, Deserialize)]
pub struct ComparisonResultWire {
    /// The scenario version the result is about.
    pub scenario_version_id: String,
    /// The dimension's index in the comparison's dimension list.
    pub dimension_index: usize,
    /// The factual reading, if one exists.
    pub value: Option<WireValue>,
    /// A `ComparisonOutcome` wire name — the factual status, never a rank.
    pub outcome: String,
    /// Where the reading came from, if anywhere.
    pub source_reference: Option<String>,
}

/// The record-comparison request.
#[derive(Clone, Debug, Deserialize)]
pub struct ComparisonWire {
    /// What the owner calls the comparison.
    pub name: String,
    /// The scenario versions compared — at least two.
    pub scenario_version_ids: Vec<String>,
    /// The dimensions compared on, in display order.
    pub dimensions: Vec<ComparisonDimensionWire>,
    /// The result cells.
    pub results: Vec<ComparisonResultWire>,
}

/// Decodes a snake_case wire name into a serde-derived engine enum.
fn enum_from_wire<T: serde::de::DeserializeOwned>(wire: &str, what: &str) -> Result<T, String> {
    serde_json::from_value(serde_json::Value::String(wire.to_owned()))
        .map_err(|_| format!("unknown {what}: {wire}"))
}

/// Decodes a wire value into the engine's typed value. `None` passes
/// through — an absent value is a fact about the assumption, not an
/// error.
fn decode_value(wire: &Option<WireValue>) -> Result<Option<TypedValue>, String> {
    let Some(wire) = wire else {
        return Ok(None);
    };
    let amount = |text: &str| {
        forhemit_scenario::FixedDecimal::parse(text).map_err(|error| error.to_string())
    };
    let value = match wire.value_type.as_str() {
        "number" => TypedValue::Number(amount(&wire.value)?),
        "currency" => TypedValue::Currency {
            amount: amount(&wire.value)?,
            currency: forhemit_scenario::CurrencyCode::parse(
                wire.currency.as_deref().unwrap_or("USD"),
            )
            .map_err(|error| error.to_string())?,
        },
        "percentage" => TypedValue::Percentage(amount(&wire.value)?),
        "duration_months" => TypedValue::DurationMonths(
            wire.value
                .trim()
                .parse()
                .map_err(|_| format!("duration must be whole months: {}", wire.value))?,
        ),
        "boolean" => TypedValue::Boolean(match wire.value.trim().to_lowercase().as_str() {
            "true" | "yes" => true,
            "false" | "no" => false,
            other => return Err(format!("boolean must be true or false: {other}")),
        }),
        "text" => TypedValue::Text(wire.value.clone()),
        other => return Err(format!("unknown value type: {other}")),
    };
    Ok(Some(value))
}

/// Creates a scenario family and its first draft version — the draft is
/// mutable until finalized (schema doc lifecycle axis).
///
/// # Errors
///
/// Scenario engine refusals (clean-text, integrity rules 1–2, audit).
pub fn scenario_create(
    engines: &AppEngines,
    request: ScenarioCreateWire,
) -> Result<FamilyAndVersionView, String> {
    let scenario_type = enum_from_wire::<ScenarioType>(&request.scenario_type, "scenario type")?;
    let family = engines
        .scenario
        .create_family(NewFamily {
            name: request.name.clone(),
            scenario_type,
            actor: engines.actor.clone(),
            correlation_id: new_correlation_id(),
        })
        .map_err(|error| error.to_string())?;
    engines.remember_scenario_family(&family.scenario_family_id)?;
    let version = scenario_start_draft(
        engines,
        family.scenario_family_id.as_str(),
        &request.name,
        request.description.clone(),
        &request.destination_version_id,
        &request.reality_fact_version_ids,
    )?;
    Ok(FamilyAndVersionView {
        family: family_view(&engines.scenario, &family)?,
        version,
    })
}

/// Starts a draft in an existing family.
///
/// # Errors
///
/// Scenario engine refusals.
pub fn scenario_start_draft(
    engines: &AppEngines,
    family_id: &str,
    name: &str,
    description: Option<String>,
    destination_version_id: &str,
    reality_fact_version_ids: &[String],
) -> Result<ScenarioVersionView, String> {
    let family_id = ScenarioFamilyId::new(family_id.to_owned()).map_err(|e| e.to_string())?;
    let destination_version_id =
        DestinationVersionId::new(destination_version_id.to_owned()).map_err(|e| e.to_string())?;
    let mut fact_ids = Vec::new();
    for fact_version_id in reality_fact_version_ids {
        fact_ids.push(FactVersionId::new(fact_version_id.clone()).map_err(|e| e.to_string())?);
    }
    let version = engines
        .scenario
        .start_draft(NewDraft {
            scenario_family_id: family_id,
            name: name.to_owned(),
            description,
            destination_version_id,
            reality_fact_version_ids: fact_ids,
            financial_model_version_id: None,
            actor: engines.actor.clone(),
            correlation_id: new_correlation_id(),
        })
        .map_err(|error| error.to_string())?;
    version_view(&engines.scenario, &version)
}

/// One scenario family with its versions, for the explorer's family pane.
///
/// # Errors
///
/// An unknown family id or an engine read refusal.
pub fn scenario_family_view(
    engines: &AppEngines,
    family_id: String,
) -> Result<ScenarioFamilyView, String> {
    let family_id = ScenarioFamilyId::new(family_id).map_err(|e| e.to_string())?;
    let family = engines
        .scenario
        .family(&family_id)
        .map_err(|error| error.to_string())?;
    family_view(&engines.scenario, &family)
}

/// Every scenario family this session has created or branched — the
/// explorer's browse list. Session-scoped by design: the audit stream is
/// the durable record until engine persistence lands.
///
/// # Errors
///
/// The family id lock is poisoned or a family lookup fails.
pub fn scenario_families(engines: &AppEngines) -> Result<Vec<ScenarioFamilyView>, String> {
    let mut families = Vec::new();
    for family_id in engines.scenario_family_ids()? {
        let family = engines
            .scenario
            .family(&family_id)
            .map_err(|error| error.to_string())?;
        families.push(family_view(&engines.scenario, &family)?);
    }
    Ok(families)
}

/// One scenario version with its full draft — assumptions, unknowns,
/// constraints, nonnegotiables under test, and conflicts.
///
/// # Errors
///
/// An unknown version id or an engine read refusal.
pub fn scenario_version_view(
    engines: &AppEngines,
    version_id: String,
) -> Result<ScenarioVersionView, String> {
    let version_id = ScenarioVersionId::new(version_id).map_err(|e| e.to_string())?;
    let version = engines
        .scenario
        .version(&version_id)
        .map_err(|error| error.to_string())?;
    version_view(&engines.scenario, &version)
}

/// Adds a typed assumption to a draft version.
///
/// # Errors
///
/// Scenario engine refusals (not a draft, invalid value, audit).
pub fn scenario_add_assumption(
    engines: &AppEngines,
    version_id: String,
    request: AssumptionWire,
) -> Result<ScenarioVersionView, String> {
    let value = decode_value(&Some(request.value))?.ok_or("an assumption needs a value")?;
    let objective_id = match request.nonnegotiable_objective_id {
        Some(wire) => Some(ObjectiveId::new(wire).map_err(|e| e.to_string())?),
        None => None,
    };
    engines
        .scenario
        .add_assumption(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            NewAssumption {
                category: enum_from_wire::<AssumptionCategory>(&request.category, "category")?,
                name: request.name.clone(),
                description: request.description.clone(),
                value,
                provenance: enum_from_wire::<Provenance>(&request.provenance, "provenance")?,
                verification: enum_from_wire::<Verification>(
                    &request.verification,
                    "verification",
                )?,
                nonnegotiable_objective_id: objective_id,
                source_reference: request.source_reference.clone(),
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Adds a first-class unknown to a draft — missing answers stay visible,
/// never blank zeros.
///
/// # Errors
///
/// Scenario engine refusals.
pub fn scenario_add_unknown(
    engines: &AppEngines,
    version_id: String,
    request: UnknownWire,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .add_unknown(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            NewUnknown {
                category: request.category.clone(),
                description: request.description.clone(),
                importance: enum_from_wire::<UnknownImportance>(&request.importance, "importance")?,
                required_action: request.required_action.clone(),
                source_dependency: request.source_dependency.clone(),
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Resolves or waives an unknown — the answer's location is a reference,
/// never inline bulk content.
///
/// # Errors
///
/// Scenario engine refusals (unknown not found, non-terminal status).
pub fn scenario_resolve_unknown(
    engines: &AppEngines,
    version_id: String,
    unknown_id: String,
    status: String,
    resolution_reference: Option<String>,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .resolve_unknown(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            UnknownResolution {
                unknown_id: UnknownId::new(unknown_id).map_err(|e| e.to_string())?,
                status: enum_from_wire::<UnknownResolutionStatus>(&status, "resolution status")?,
                resolution_reference,
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Adds a constraint to a draft.
///
/// # Errors
///
/// Scenario engine refusals.
pub fn scenario_add_constraint(
    engines: &AppEngines,
    version_id: String,
    request: ConstraintWire,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .add_constraint(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            NewConstraint {
                constraint_type: enum_from_wire::<ConstraintType>(
                    &request.constraint_type,
                    "constraint type",
                )?,
                name: request.name.clone(),
                description: request.description.clone(),
                value: decode_value(&request.value)?,
                source_reference: request.source_reference.clone(),
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Puts one of the destination's nonnegotiables under test in this
/// scenario — the structured request, not a boolean flag.
#[derive(Clone, Debug, Deserialize)]
pub struct NonnegotiableWire {
    /// The destination objective being tested.
    pub objective_id: String,
    /// The destination version whose designation is being tested — must
    /// match the scenario version's pinned destination version.
    pub destination_version_id: String,
    /// What the requirement says.
    pub description: String,
    /// The requirement's value, if it has one.
    pub value: Option<WireValue>,
}

/// Puts one of the destination's nonnegotiables under test in this
/// scenario — the structured back-reference, not a boolean flag.
///
/// # Errors
///
/// Scenario engine refusals (objective or version mismatch).
pub fn scenario_add_nonnegotiable(
    engines: &AppEngines,
    version_id: String,
    request: NonnegotiableWire,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .add_nonnegotiable(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            NewNonnegotiable {
                destination_objective_id: ObjectiveId::new(request.objective_id.clone())
                    .map_err(|e| e.to_string())?,
                destination_version_id: DestinationVersionId::new(request.destination_version_id)
                    .map_err(|e| e.to_string())?,
                description: request.description.clone(),
                value: decode_value(&request.value)?,
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Records a conflict on a draft — the system surfaces conditions; it
/// never filters or scores them.
///
/// # Errors
///
/// Scenario engine refusals.
pub fn scenario_record_conflict(
    engines: &AppEngines,
    version_id: String,
    request: ConflictWire,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .record_conflict(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            NewConflict {
                conflict_type: enum_from_wire::<ConflictType>(
                    &request.conflict_type,
                    "conflict type",
                )?,
                severity: enum_from_wire::<ConflictSeverity>(&request.severity, "severity")?,
                description: request.description.clone(),
                source_reference: request.source_reference.clone(),
                affected_object_type: request.affected_object_type.clone(),
                affected_object_id: request.affected_object_id.clone(),
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Closes a conflict with the owner's decision — for a nonnegotiable
/// conflict this is one of the three owner routes (keep the must-have,
/// explore another path, change the requirement); the system never picks.
///
/// # Errors
///
/// Scenario engine refusals (conflict not found, already resolved).
pub fn scenario_resolve_conflict(
    engines: &AppEngines,
    version_id: String,
    conflict_id: String,
    owner_decision: Option<String>,
    resolution_reference: Option<String>,
) -> Result<ScenarioVersionView, String> {
    let decision = conflict_decision_from_wire(owner_decision, resolution_reference)?;
    engines
        .scenario
        .resolve_conflict(
            ConflictId::new(conflict_id).map_err(|e| e.to_string())?,
            decision,
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Finalizes a draft — from here the version is immutable; changes go
/// through successors.
///
/// # Errors
///
/// Scenario engine refusals (not a draft).
pub fn scenario_finalize(
    engines: &AppEngines,
    version_id: String,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .finalize_version(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Moves a finalized version's readiness — the export-gating axis
/// (schema doc §40), distinct from the lifecycle axis.
///
/// # Errors
///
/// Scenario engine refusals (still a draft, unchanged readiness).
pub fn scenario_set_readiness(
    engines: &AppEngines,
    version_id: String,
    readiness: String,
    reason: Option<String>,
) -> Result<ScenarioVersionView, String> {
    engines
        .scenario
        .set_readiness(
            ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?,
            enum_from_wire::<ReadinessStatus>(&readiness, "readiness")?,
            reason,
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    scenario_version_view(engines, version_id)
}

/// Creates a what-if branch: a new family seeded from the parent version,
/// with the named parent assumptions overridden — branches remember where
/// they came from (integrity rule 7).
///
/// # Errors
///
/// Scenario engine refusals (unknown parent, unknown assumptions).
pub fn scenario_what_if(
    engines: &AppEngines,
    request: WhatIfWire,
) -> Result<FamilyAndVersionView, String> {
    let mut changed = Vec::new();
    for assumption_id in &request.changed_assumptions {
        changed.push(AssumptionId::new(assumption_id.clone()).map_err(|e| e.to_string())?);
    }
    let created = engines
        .scenario
        .create_what_if(
            NewWhatIf {
                parent_scenario_version_id: ScenarioVersionId::new(
                    request.parent_scenario_version_id,
                )
                .map_err(|e| e.to_string())?,
                branch_type: enum_from_wire::<BranchType>(&request.branch_type, "branch type")?,
                reason: request.reason.clone(),
                changed_assumptions: changed,
                name: request.name.clone(),
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    engines.remember_scenario_family(&created.family.scenario_family_id)?;
    Ok(FamilyAndVersionView {
        family: family_view(&engines.scenario, &created.family)?,
        version: version_view(&engines.scenario, &created.version)?,
    })
}

/// Records a comparison across finalized versions — factual rows on
/// shared dimensions, explicitly without a winner (schema doc §33).
///
/// # Errors
///
/// Scenario engine refusals (fewer than two versions, duplicates).
pub fn scenario_comparison(
    engines: &AppEngines,
    request: ComparisonWire,
) -> Result<ComparisonView, String> {
    let mut version_ids = Vec::new();
    for version_id in &request.scenario_version_ids {
        version_ids.push(ScenarioVersionId::new(version_id.clone()).map_err(|e| e.to_string())?);
    }
    let mut dimensions = Vec::new();
    for (order, dimension) in request.dimensions.iter().enumerate() {
        dimensions.push(NewComparisonDimension {
            dimension_type: enum_from_wire::<ComparisonDimensionType>(
                &dimension.dimension_type,
                "dimension type",
            )?,
            label: dimension.label.clone(),
            display_order: u32::try_from(order)
                .map_err(|_| "more dimensions than display slots")?,
        });
    }
    let mut results = Vec::new();
    for result in &request.results {
        results.push(NewComparisonResult {
            scenario_version_id: ScenarioVersionId::new(result.scenario_version_id.clone())
                .map_err(|e| e.to_string())?,
            dimension_index: result.dimension_index,
            value: decode_value(&result.value)?,
            outcome: enum_from_wire::<ComparisonOutcome>(&result.outcome, "outcome")?,
            source_reference: result.source_reference.clone(),
        });
    }
    let comparison = engines
        .scenario
        .record_comparison(
            NewComparison {
                name: request.name,
                scenario_version_ids: version_ids,
                dimensions,
                results,
            },
            engines.actor.clone(),
            new_correlation_id(),
        )
        .map_err(|error| error.to_string())?;
    Ok(comparison_view(&comparison))
}

// ---------------------------------------------------------------------------
// Vault commands
// ---------------------------------------------------------------------------

/// The vault's status as the UI shows it: not set up, locked, or ready —
/// with the honest sharing wording either way (Vault doc §47
/// `prepareForSharing` is v1-honest: the owner exports the file).
///
/// # Errors
///
/// An engine open that fails for a reason other than the keychain.
pub fn vault_status(engines: &AppEngines) -> Result<VaultStatusView, String> {
    let exports_label = engines.exports_dir()?.display().to_string();
    engines.with_vault(|slot| match slot {
        VaultSlot::Open { engine } => status_view(engine, VaultState::Ready, &exports_label),
        VaultSlot::NotSetUp => {
            let path = engines.vault_path();
            if !path.exists() {
                return Ok(not_set_up_view(&exports_label));
            }
            // A vault exists on disk but is not open this session: try the
            // keychain before declaring it locked.
            let store = VaultStore::open_discovering(&path).map_err(|error| error.to_string())?;
            match VaultEngine::open(
                store,
                engines.vault_keystore.clone(),
                engines.tee.clone(),
                engines.clock.clone(),
                engines.workspace_id.clone(),
            ) {
                Ok(engine) => {
                    let view = status_view(&engine, VaultState::Ready, &exports_label)?;
                    *slot = VaultSlot::Open {
                        engine: Box::new(engine),
                    };
                    Ok(view)
                }
                Err(VaultError::VaultKeyUnavailable) => Ok(locked_view(&exports_label)),
                Err(error) => Err(error.to_string()),
            }
        }
    })?
}

/// Creates the vault and its recovery escrow — the one-time setup. The
/// passphrase is shown once; it is never stored, only its Argon2id
/// derivation wraps the vault key (Vault doc §9).
///
/// # Errors
///
/// A vault that already exists, or an engine refusal: a recovery
/// passphrase that is blank or under the engine's 12-character policy
/// (enforced in the vault engine, not here), or an audit refusal.
pub fn vault_setup(
    engines: &AppEngines,
    recovery_passphrase: String,
) -> Result<VaultStatusView, String> {
    let exports_label = engines.exports_dir()?.display().to_string();
    // Pre-flight the engine's passphrase policy BEFORE creating anything:
    // a refused setup must leave no vault database behind — an empty
    // vault file would make every later attempt report "a vault already
    // exists". The engine enforces the same policy again inside create.
    validate_recovery_passphrase(&recovery_passphrase).map_err(|error| error.to_string())?;
    engines.with_vault(|slot| {
        if engines.vault_path().exists() {
            return Err(
                "a vault already exists in this workspace — use recovery if you lost the key"
                    .to_owned(),
            );
        }
        let vault_id = VaultId::new(format!("vlt_{}", Ulid::new())).map_err(|e| e.to_string())?;
        let store =
            VaultStore::open(vault_id, &engines.vault_path()).map_err(|error| error.to_string())?;
        let engine = VaultEngine::create(
            store,
            engines.vault_keystore.clone(),
            engines.tee.clone(),
            engines.clock.clone(),
            engines.workspace_id.clone(),
            &recovery_passphrase,
        )
        .map_err(|error| error.to_string())?;
        let view = status_view(&engine, VaultState::Ready, &exports_label)?;
        *slot = VaultSlot::Open {
            engine: Box::new(engine),
        };
        Ok(view)
    })?
}

/// Opens a vault whose keychain entry is gone, by unwrapping the escrowed
/// key with the recovery passphrase (Vault doc §9 recovery arm).
///
/// # Errors
///
/// `RecoveryFailed` for a wrong passphrase — nothing else is exposed; or
/// the vault does not exist.
pub fn vault_recover(
    engines: &AppEngines,
    recovery_passphrase: String,
) -> Result<VaultStatusView, String> {
    let exports_label = engines.exports_dir()?.display().to_string();
    engines.with_vault(|slot| {
        if !engines.vault_path().exists() {
            return Err("no vault exists yet — set up your vault first".to_owned());
        }
        let store = VaultStore::open_discovering(&engines.vault_path())
            .map_err(|error| error.to_string())?;
        let engine = VaultEngine::recover(
            store,
            engines.vault_keystore.clone(),
            engines.tee.clone(),
            engines.clock.clone(),
            engines.workspace_id.clone(),
            &recovery_passphrase,
        )
        .map_err(|error| error.to_string())?;
        let view = status_view(&engine, VaultState::Ready, &exports_label)?;
        *slot = VaultSlot::Open {
            engine: Box::new(engine),
        };
        Ok(view)
    })?
}

/// Maximum decoded payload `vault_import` accepts (security review L2).
/// The engine's decode path is unbounded by design — the shell owns the
/// byte ceiling: 100 MiB of decoded document bytes per import.
const MAX_IMPORT_DECODED_BYTES: usize = 100 * 1024 * 1024;

/// The one clear refusal message for an oversized import.
fn import_payload_too_large() -> String {
    format!(
        "the file is too large: the vault accepts at most {} MiB per import",
        MAX_IMPORT_DECODED_BYTES / (1024 * 1024)
    )
}

/// One payload-size decision: at or under the cap is allowed — exactly
/// 100 MiB is a valid import — anything over is the clear refusal.
fn import_size_decision(decoded_bytes: usize) -> Result<(), String> {
    if decoded_bytes > MAX_IMPORT_DECODED_BYTES {
        Err(import_payload_too_large())
    } else {
        Ok(())
    }
}

/// Decoded size implied by a standard-base64 string's length — exact for
/// well-formed payloads (4 chars per 3 bytes, minus padding). Malformed
/// input fails the decode itself; the post-decode check stays
/// authoritative.
fn base64_decoded_bound(content_base64: &str) -> usize {
    let groups = content_base64.len() / 4;
    let padding = content_base64.len() - content_base64.trim_end_matches('=').len();
    groups.saturating_mul(3).saturating_sub(padding)
}

/// Imports a document from the frontend's file picker: the bytes arrive
/// base64-encoded, the vault encrypts at rest, and importing never
/// uploads — v1 keeps everything on this device.
///
/// # Errors
///
/// The vault is not open; the payload is not valid base64; the payload
/// exceeds the [`MAX_IMPORT_DECODED_BYTES`] ceiling (refused before
/// decoding whenever the encoded size alone proves it); or the vault
/// engine refuses (empty filename, audit refusal).
pub fn vault_import(
    engines: &AppEngines,
    filename: String,
    content_base64: String,
    note: Option<String>,
) -> Result<VaultDocumentView, String> {
    use base64::Engine as _;
    let content_base64 = content_base64.trim();
    // Cheap refusal before decoding whenever the encoded size alone
    // proves the payload oversized.
    import_size_decision(base64_decoded_bound(content_base64))?;
    let content = base64::engine::general_purpose::STANDARD
        .decode(content_base64)
        .map_err(|error| format!("the file content is not valid base64: {error}"))?;
    import_size_decision(content.len())?;
    with_open_vault(engines, |engine| {
        let stored = engine
            .import_document(&engines.actor, &filename, &content, note.as_deref())
            .map_err(|error| error.to_string())?;
        document_view(engine, &stored.version.document_id)
    })
}

/// One document's full version history with this vault's integrity
/// results — a failure surfaces loudly, never repaired silently.
///
/// # Errors
///
/// The vault is not open or the document does not exist.
pub fn vault_document_history(
    engines: &AppEngines,
    document_id: String,
) -> Result<VaultDocumentHistoryView, String> {
    with_open_vault(engines, |engine| {
        let doc_id = DocumentId::new(document_id).map_err(|e| e.to_string())?;
        let document = document_view(engine, &doc_id)?;
        let history = engine
            .version_history(&doc_id)
            .map_err(|error| error.to_string())?;
        Ok(VaultDocumentHistoryView {
            document_id: document.document_id,
            versions: version_views(&history),
        })
    })
}

/// Every imported document — the vault screen's list.
///
/// # Errors
///
/// The vault is not open this session.
pub fn vault_documents(engines: &AppEngines) -> Result<Vec<VaultDocumentView>, String> {
    with_open_vault(engines, |engine| {
        let ids = engine.list_documents().map_err(|error| error.to_string())?;
        let mut documents = Vec::new();
        for id in ids {
            documents.push(document_view(engine, &id)?);
        }
        Ok(documents)
    })
}

/// One version's decrypted content, base64-encoded for display or
/// download.
///
/// # Errors
///
/// The vault is not open or the version does not exist.
pub fn vault_document_content(
    engines: &AppEngines,
    version_id: String,
) -> Result<VaultVersionContentView, String> {
    with_open_vault(engines, |engine| {
        let stored = engine
            .read_version(&DocumentVersionId::new(version_id).map_err(|e| e.to_string())?)
            .map_err(|error| error.to_string())?;
        Ok(version_content_view(&stored))
    })
}

/// Local full-text search over imported content — FTS5 on this device,
/// nothing leaves it.
///
/// # Errors
///
/// The vault is not open or the search index refuses the query.
pub fn vault_search(
    engines: &AppEngines,
    query: String,
) -> Result<Vec<VaultSearchHitView>, String> {
    with_open_vault(engines, |engine| {
        let hits = engine
            .search(&query, 20)
            .map_err(|error| error.to_string())?;
        Ok(hits.iter().map(search_hit_view).collect())
    })
}

/// Exports an `age`-encrypted backup of the whole vault — the owner's
/// offline copy, protected by the recovery passphrase.
///
/// # Errors
///
/// The vault is not open, or the backup write fails.
pub fn vault_backup(
    engines: &AppEngines,
    recovery_passphrase: String,
) -> Result<ExportedFileView, String> {
    use base64::Engine as _;
    with_open_vault(engines, |engine| {
        let exports = engines.exports_dir()?;
        let stamp = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "backup".to_owned());
        let stamp = stamp.replace(':', ""); // a filesystem-safe timestamp
        let path = exports.join(format!("forhemit-vault-backup-{stamp}.age"));
        engine
            .export_backup(&engines.actor, &recovery_passphrase, &path)
            .map_err(|error| error.to_string())?;
        let bytes = std::fs::read(&path).map_err(|error| format!("read backup: {error}"))?;
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "forhemit-vault-backup.age".to_owned());
        Ok(ExportedFileView {
            format: "age-backup",
            file_name,
            saved_path: path.display().to_string(),
            content_base64: base64::engine::general_purpose::STANDARD.encode(bytes),
        })
    })
}

// ---------------------------------------------------------------------------
// Package commands
// ---------------------------------------------------------------------------

/// The package preview: the assembled package's provenance block, the
/// readiness gate's included and excluded scenarios, and the executive
/// summary — everything the owner reviews *before* exporting.
///
/// # Errors
///
/// No destination exists yet, or assembly refuses the snapshot.
pub fn package_preview(engines: &AppEngines) -> Result<PackagePreviewView, String> {
    let snapshot = Box::new(workspace_snapshot(engines)?);
    let engine = forhemit_package::PackageEngine::new(
        engines.tee.clone(),
        engines.clock.clone(),
        engines.workspace_id.clone(),
    );
    let package = engine
        .assemble(&snapshot)
        .map_err(|error| error.to_string())?;
    Ok(package_preview_view(
        &package.provenance,
        &package.summary,
        &package.sections,
    ))
}

/// Exports the package in one format and saves it under the workspace's
/// exports directory — the file itself is the v1 "sharing" surface.
///
/// # Errors
///
/// Assembly or rendering refuses, or the file cannot be written.
pub fn package_export(engines: &AppEngines, format: String) -> Result<ExportedFileView, String> {
    let export_format = match format.as_str() {
        "html" => ExportFormat::Html,
        "pdf" => ExportFormat::Pdf,
        other => return Err(format!("unknown export format: {other}")),
    };
    let snapshot = Box::new(workspace_snapshot(engines)?);
    let engine = forhemit_package::PackageEngine::new(
        engines.tee.clone(),
        engines.clock.clone(),
        engines.workspace_id.clone(),
    );
    let exported = engine
        .export(
            &snapshot,
            &[export_format],
            engines.actor.clone(),
            new_correlation_id(),
            None,
        )
        .map_err(|error| error.to_string())?;
    let exported = exported
        .into_iter()
        .next()
        .ok_or_else(|| "the engine returned no exported content".to_owned())?;
    let exports = engines.exports_dir()?;
    let path = exports.join(&exported.file_name);
    match &exported.content {
        ExportedContent::Html(html) => {
            std::fs::write(&path, html).map_err(|error| format!("write package: {error}"))?;
        }
        ExportedContent::Pdf(pdf) => {
            std::fs::write(&path, pdf).map_err(|error| format!("write package: {error}"))?;
        }
    }
    exported_file_view(&exported, &path.display().to_string())
}

/// Runs `f` with the open vault engine, or refuses with the state the UI
/// must surface (setup prompt / locked notice), never a silent fallback.
fn with_open_vault<R>(
    engines: &AppEngines,
    f: impl FnOnce(&VaultEngine) -> Result<R, String>,
) -> Result<R, String> {
    vault_status(engines)?;
    engines.with_vault(|slot| match slot {
        VaultSlot::Open { engine } => f(engine),
        VaultSlot::NotSetUp => {
            Err("the vault is not available — check its status first".to_owned())
        }
    })?
}

// --- Updater (shell-level release infrastructure) ---
//
// The updater is not a domain engine: it mutates nothing in the owner's
// workspace, so it emits no audit events. Its refusal rules live in
// `updater_manifest` (unit-tested); these adapters wire the plugin's
// fetch / verify / install machinery to the UI. Update checks are explicit —
// the app never phones home on its own (local-first v1).

/// The pending update a pre-flight verdict accepted, held between the
/// owner's "check for updates" and their install decision.
#[derive(Default)]
pub struct PendingUpdate(std::sync::Mutex<Option<tauri_plugin_updater::Update>>);

/// Checks the configured update endpoint and applies the pre-flight verdict
/// before anything is downloaded.
pub(crate) async fn update_check(
    app: &tauri::AppHandle,
    pending: &PendingUpdate,
) -> Result<crate::updater_manifest::UpdateCheckView, String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app
        .updater()
        .map_err(|error| format!("updater unavailable: {error}"))?;
    let current_version = app.package_info().version.to_string();
    match updater.check().await {
        Err(error) => Ok(crate::updater_manifest::UpdateCheckView::Unavailable {
            detail: error.to_string(),
        }),
        Ok(None) => Ok(crate::updater_manifest::UpdateCheckView::UpToDate { current_version }),
        Ok(Some(update)) => {
            // Defense-in-depth: re-derive the refusal decision from the raw
            // manifest the plugin fetched — an unsigned or stale manifest is
            // refused here with a precise reason, before any bytes move.
            let verdict = crate::updater_manifest::verdict(
                &update.raw_json.to_string(),
                &update.current_version,
                &update.target,
                time::OffsetDateTime::now_utc(),
            );
            match verdict {
                Ok(manifest) => {
                    let view = crate::updater_manifest::UpdateCheckView::Available {
                        version: manifest.version,
                        notes: manifest.notes,
                        pub_date: manifest.pub_date,
                        download_url: update.download_url.to_string(),
                    };
                    *pending
                        .0
                        .lock()
                        .map_err(|_| "pending-update state poisoned".to_owned())? = Some(update);
                    Ok(view)
                }
                Err(refusal) => Ok(crate::updater_manifest::UpdateCheckView::Refused { refusal }),
            }
        }
    }
}

/// Downloads, verifies (minisign, against the public key embedded at build
/// time), and installs the pending update. On Windows the installer exits
/// the app during install; on other platforms the owner restarts when
/// convenient.
pub(crate) async fn update_install(pending: &PendingUpdate) -> Result<(), String> {
    let pending_update = pending
        .0
        .lock()
        .map_err(|_| "pending-update state poisoned".to_owned())?
        .take();
    let Some(update) = pending_update else {
        return Err("no pending update — check for updates first".to_owned());
    };
    update
        .download_and_install(
            |_chunk_length, _content_length| {},
            || {}, // download finished
        )
        .await
        .map_err(|error| format!("update install failed: {error}"))
}

// The shell's end-to-end guarantees from the security review: setup
// refusals leave no vault file behind, and the import size cap refuses
// oversized payloads while valid imports keep working.

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    /// A debug-build engine set over a throwaway data dir (the memory
    /// key ring applies); each call gets its own directory.
    fn engines() -> AppEngines {
        let dir = std::env::temp_dir().join(format!(
            "forhemit-commands-test-{}-{}",
            std::process::id(),
            Ulid::new()
        ));
        AppEngines::open(&dir).unwrap()
    }

    fn base64_encode(bytes: &[u8]) -> String {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    #[test]
    fn a_refused_setup_leaves_no_vault_behind() {
        let engines = engines();
        let refused = vault_setup(&engines, "short".to_owned());
        assert!(
            refused.is_err(),
            "the engine policy must refuse a short passphrase"
        );
        // The refusal happened before any vault file existed, so a valid
        // setup still succeeds afterwards.
        assert!(
            vault_setup(&engines, "twelve chars".to_owned()).is_ok(),
            "a refused setup must not block a valid one"
        );
    }

    #[test]
    fn oversized_imports_are_refused_and_valid_imports_still_work() {
        let engines = engines();
        vault_setup(&engines, "correct horse battery staple".to_owned()).unwrap();

        // One byte past the cap, base64-encoded — refused before
        // decoding, with the cap message and nothing else.
        let payload = vec![0u8; MAX_IMPORT_DECODED_BYTES + 16];
        let refused = vault_import(
            &engines,
            "too-big.bin".to_owned(),
            base64_encode(&payload),
            None,
        );
        match refused {
            Ok(_) => panic!("the oversized payload must be refused"),
            Err(message) => assert_eq!(message, import_payload_too_large()),
        }

        // A valid import still works end to end.
        let view = vault_import(
            &engines,
            "2025 P&L.txt".to_owned(),
            base64_encode(b"Q3 EBITDA was 8,240,000."),
            Some("password sent separately".to_owned()),
        );
        assert!(view.is_ok(), "a valid import must still work");
    }

    #[test]
    fn the_cap_boundary_is_inclusive() {
        assert_eq!(import_size_decision(MAX_IMPORT_DECODED_BYTES), Ok(()));
        assert_eq!(
            import_size_decision(MAX_IMPORT_DECODED_BYTES + 1),
            Err(import_payload_too_large())
        );
    }

    #[test]
    fn the_encoded_bound_is_exact_for_well_formed_payloads() {
        assert_eq!(base64_decoded_bound(""), 0);
        assert_eq!(base64_decoded_bound("AQ=="), 1);
        assert_eq!(base64_decoded_bound("AQw="), 2);
        assert_eq!(base64_decoded_bound("AQww"), 3);
        assert_eq!(base64_decoded_bound(&base64_encode(b"Q3 EBITDA")), 9);
    }
}
