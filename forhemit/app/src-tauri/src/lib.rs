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

#[tauri::command]
fn scenario_create(
    state: tauri::State<'_, AppEngines>,
    request: commands::ScenarioCreateWire,
) -> Result<views::FamilyAndVersionView, String> {
    commands::scenario_create(&state, request)
}

#[tauri::command]
fn scenario_families(
    state: tauri::State<'_, AppEngines>,
) -> Result<Vec<views::ScenarioFamilyView>, String> {
    commands::scenario_families(&state)
}

#[tauri::command]
fn scenario_family_view(
    state: tauri::State<'_, AppEngines>,
    family_id: String,
) -> Result<views::ScenarioFamilyView, String> {
    commands::scenario_family_view(&state, family_id)
}

#[tauri::command]
fn scenario_version_view(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_version_view(&state, version_id)
}

#[tauri::command]
fn scenario_add_assumption(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    request: commands::AssumptionWire,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_add_assumption(&state, version_id, request)
}

#[tauri::command]
fn scenario_add_unknown(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    request: commands::UnknownWire,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_add_unknown(&state, version_id, request)
}

#[tauri::command]
fn scenario_resolve_unknown(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    unknown_id: String,
    status: String,
    resolution_reference: Option<String>,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_resolve_unknown(&state, version_id, unknown_id, status, resolution_reference)
}

#[tauri::command]
fn scenario_add_constraint(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    request: commands::ConstraintWire,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_add_constraint(&state, version_id, request)
}

#[tauri::command]
fn scenario_add_nonnegotiable(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    request: commands::NonnegotiableWire,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_add_nonnegotiable(&state, version_id, request)
}

#[tauri::command]
fn scenario_record_conflict(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    request: commands::ConflictWire,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_record_conflict(&state, version_id, request)
}

#[tauri::command]
fn scenario_resolve_conflict(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    conflict_id: String,
    owner_decision: Option<String>,
    resolution_reference: Option<String>,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_resolve_conflict(
        &state,
        version_id,
        conflict_id,
        owner_decision,
        resolution_reference,
    )
}

#[tauri::command]
fn scenario_finalize(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_finalize(&state, version_id)
}

#[tauri::command]
fn scenario_set_readiness(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
    readiness: String,
    reason: Option<String>,
) -> Result<views::ScenarioVersionView, String> {
    commands::scenario_set_readiness(&state, version_id, readiness, reason)
}

#[tauri::command]
fn scenario_what_if(
    state: tauri::State<'_, AppEngines>,
    request: commands::WhatIfWire,
) -> Result<views::FamilyAndVersionView, String> {
    commands::scenario_what_if(&state, request)
}

#[tauri::command]
fn scenario_comparison(
    state: tauri::State<'_, AppEngines>,
    request: commands::ComparisonWire,
) -> Result<views::ComparisonView, String> {
    commands::scenario_comparison(&state, request)
}

#[tauri::command]
fn vault_status(state: tauri::State<'_, AppEngines>) -> Result<views::VaultStatusView, String> {
    commands::vault_status(&state)
}

#[tauri::command]
fn vault_setup(
    state: tauri::State<'_, AppEngines>,
    recovery_passphrase: String,
) -> Result<views::VaultStatusView, String> {
    commands::vault_setup(&state, recovery_passphrase)
}

#[tauri::command]
fn vault_recover(
    state: tauri::State<'_, AppEngines>,
    recovery_passphrase: String,
) -> Result<views::VaultStatusView, String> {
    commands::vault_recover(&state, recovery_passphrase)
}

#[tauri::command]
fn vault_import(
    state: tauri::State<'_, AppEngines>,
    filename: String,
    content_base64: String,
    note: Option<String>,
) -> Result<views::VaultDocumentView, String> {
    commands::vault_import(&state, filename, content_base64, note)
}

#[tauri::command]
fn vault_document_history(
    state: tauri::State<'_, AppEngines>,
    document_id: String,
) -> Result<views::VaultDocumentView, String> {
    commands::vault_document_history(&state, document_id)
}

#[tauri::command]
fn vault_document_content(
    state: tauri::State<'_, AppEngines>,
    version_id: String,
) -> Result<views::VaultVersionContentView, String> {
    commands::vault_document_content(&state, version_id)
}

#[tauri::command]
fn vault_documents(
    state: tauri::State<'_, AppEngines>,
) -> Result<Vec<views::VaultDocumentView>, String> {
    commands::vault_documents(&state)
}

#[tauri::command]
fn vault_search(
    state: tauri::State<'_, AppEngines>,
    query: String,
) -> Result<Vec<views::VaultSearchHitView>, String> {
    commands::vault_search(&state, query)
}

#[tauri::command]
fn vault_backup(
    state: tauri::State<'_, AppEngines>,
    recovery_passphrase: String,
) -> Result<views::ExportedFileView, String> {
    commands::vault_backup(&state, recovery_passphrase)
}

#[tauri::command]
fn package_preview(
    state: tauri::State<'_, AppEngines>,
) -> Result<views::PackagePreviewView, String> {
    commands::package_preview(&state)
}

#[tauri::command]
fn package_export(
    state: tauri::State<'_, AppEngines>,
    format: String,
) -> Result<views::ExportedFileView, String> {
    commands::package_export(&state, format)
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
            scenario_create,
            scenario_families,
            scenario_family_view,
            scenario_version_view,
            scenario_add_assumption,
            scenario_add_unknown,
            scenario_resolve_unknown,
            scenario_add_constraint,
            scenario_add_nonnegotiable,
            scenario_record_conflict,
            scenario_resolve_conflict,
            scenario_finalize,
            scenario_set_readiness,
            scenario_what_if,
            scenario_comparison,
            vault_status,
            vault_setup,
            vault_recover,
            vault_import,
            vault_documents,
            vault_document_history,
            vault_document_content,
            vault_search,
            vault_backup,
            package_preview,
            package_export,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run the Forhemit application");
}
