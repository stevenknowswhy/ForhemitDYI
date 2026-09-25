//! The command layer: thin adapters from the UI to the engine APIs.
//!
//! Every command delegates to an engine — validation, versioning, and
//! audit emission happen inside the engine crates, never here. Commands
//! surface engine refusals verbatim as strings; the UI displays them
//! honestly rather than swallowing them.

use forhemit_audit::new_correlation_id;
use forhemit_contracts::{DestinationId, JourneyInstanceId};
use forhemit_destination::{
    ChangeReason, Completeness, Destination, DestinationContent, DestinationEngine,
};
use forhemit_journey::{
    AnswerValue, JourneyEngine, JourneyError, JourneyInstance, RecordAnswer, ReviseAnswer, SkipNode,
};
use forhemit_reality::fact::FactVersion;
use forhemit_reality::{FactPeriod, FactValue, RecordFact, ReviseFact};
use ulid::Ulid;

use crate::state::AppEngines;
use crate::views::{JourneyView, VerifyView};

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
fn load_instance(engines: &AppEngines) -> Result<Option<JourneyInstance>, String> {
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
