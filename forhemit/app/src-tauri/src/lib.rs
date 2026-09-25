//! Forhemit desktop shell.
//!
//! The shell owns no domain logic: every command is a thin adapter over an
//! engine crate API (spec art_fL7Z2ate). The composition root — audit
//! store, journey store, reality engine, and the domain state they share —
//! lives in [`state`]; the UI-facing projections live in [`views`].

pub mod commands;
pub mod state;
pub mod views;

#[cfg(test)]
pub(crate) mod test_support;

use state::AppEngines;

// The `tauri::command` wrappers are private by convention: the command
// macro resolves them in this module, and a `pub` command makes the macro
// emit a same-module re-import that collides in the macro namespace
// (E0255). Each wrapper delegates to the engine adapter in `commands`.

#[tauri::command]
fn destination_get(
    state: tauri::State<'_, AppEngines>,
) -> Option<forhemit_destination::Destination> {
    commands::destination_get(&state)
}

#[tauri::command]
fn destination_create(
    state: tauri::State<'_, AppEngines>,
    content: forhemit_destination::DestinationContent,
) -> Result<forhemit_destination::Destination, String> {
    commands::destination_create(&state, content)
}

#[tauri::command]
fn destination_edit(
    state: tauri::State<'_, AppEngines>,
    content: forhemit_destination::DestinationContent,
    reason: forhemit_destination::ChangeReason,
    explanation: Option<String>,
) -> Result<forhemit_destination::Destination, String> {
    commands::destination_edit(&state, content, reason, explanation)
}

#[tauri::command]
fn destination_confirm(
    state: tauri::State<'_, AppEngines>,
) -> Result<forhemit_destination::Destination, String> {
    commands::destination_confirm(&state)
}

#[tauri::command]
fn destination_mark_working(
    state: tauri::State<'_, AppEngines>,
) -> Result<forhemit_destination::Destination, String> {
    commands::destination_mark_working(&state)
}

#[tauri::command]
fn destination_archive(
    state: tauri::State<'_, AppEngines>,
) -> Result<forhemit_destination::Destination, String> {
    commands::destination_archive(&state)
}

#[tauri::command]
fn destination_completeness(
    content: forhemit_destination::DestinationContent,
) -> forhemit_destination::Completeness {
    commands::destination_completeness_for(&content)
}

#[tauri::command]
fn journey_state(
    state: tauri::State<'_, AppEngines>,
) -> Result<Option<views::JourneyView>, String> {
    commands::journey_state(&state)
}

#[tauri::command]
fn journey_start(state: tauri::State<'_, AppEngines>) -> Result<views::JourneyView, String> {
    commands::journey_start(&state)
}

#[tauri::command]
fn journey_record(
    state: tauri::State<'_, AppEngines>,
    node_id: String,
    value: forhemit_journey::AnswerValue,
) -> Result<views::JourneyView, String> {
    commands::journey_record(&state, node_id, value)
}

#[tauri::command]
fn journey_revise(
    state: tauri::State<'_, AppEngines>,
    node_id: String,
    value: forhemit_journey::AnswerValue,
    change_reason: String,
) -> Result<views::JourneyView, String> {
    commands::journey_revise(&state, node_id, value, change_reason)
}

#[tauri::command]
fn journey_skip(
    state: tauri::State<'_, AppEngines>,
    node_id: String,
) -> Result<views::JourneyView, String> {
    commands::journey_skip(&state, node_id)
}

#[tauri::command]
fn snapshot_current(
    state: tauri::State<'_, AppEngines>,
) -> Result<Vec<forhemit_reality::FactVersion>, String> {
    commands::snapshot_current(&state)
}

#[tauri::command]
fn snapshot_record(
    state: tauri::State<'_, AppEngines>,
    kind: forhemit_reality::FactKind,
    value: forhemit_reality::FactValue,
    definition: Option<String>,
) -> Result<Vec<forhemit_reality::FactVersion>, String> {
    commands::snapshot_record(&state, kind, value, definition)
}

#[tauri::command]
fn snapshot_revise(
    state: tauri::State<'_, AppEngines>,
    fact_id: String,
    value: forhemit_reality::FactValue,
    definition: Option<String>,
    change_reason: String,
) -> Result<Vec<forhemit_reality::FactVersion>, String> {
    commands::snapshot_revise(&state, fact_id, value, definition, change_reason)
}

#[tauri::command]
fn audit_recent(state: tauri::State<'_, AppEngines>) -> Result<Vec<state::AuditLogLine>, String> {
    commands::audit_recent(&state)
}

#[tauri::command]
fn audit_verify(state: tauri::State<'_, AppEngines>) -> Result<views::VerifyView, String> {
    commands::audit_verify(&state)
}

/// Runs the desktop app.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::expect_used)] // builder failure at startup is unrecoverable; the template aborts with a diagnostic
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;
            let data_dir = app.path().app_data_dir()?.join("workspace");
            let engines = AppEngines::open(&data_dir)
                .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?;
            app.manage(engines);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            destination_get,
            destination_create,
            destination_edit,
            destination_confirm,
            destination_mark_working,
            destination_archive,
            destination_completeness,
            journey_state,
            journey_start,
            journey_record,
            journey_revise,
            journey_skip,
            snapshot_current,
            snapshot_record,
            snapshot_revise,
            audit_recent,
            audit_verify,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run the Forhemit application");
}
