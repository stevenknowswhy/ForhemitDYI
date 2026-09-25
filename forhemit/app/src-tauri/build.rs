//! Build script — wires Tauri's codegen (context, assets, platform resources).
//!
//! The command inventory is registered with the ACL manifest so capabilities
//! can gate application commands (see `capabilities/main.json`): without this,
//! Tauri 2 allows every `invoke_handler` command from any local webview
//! content, independent of the capability list.

/// Every command registered in `invoke_handler` (`src/lib.rs`), in the same
/// order. When a command is added there, add it here AND grant
/// `allow-<name>` in `capabilities/main.json` — a command missing from either
/// side is refused at runtime with "Command … not allowed by ACL".
const COMMANDS: &[&str] = &[
    "destination_get",
    "destination_create",
    "destination_edit",
    "destination_confirm",
    "destination_mark_working",
    "destination_archive",
    "destination_completeness",
    "journey_state",
    "journey_start",
    "journey_record",
    "journey_revise",
    "journey_skip",
    "snapshot_current",
    "snapshot_record",
    "snapshot_revise",
    "audit_recent",
    "audit_verify",
    "scenario_create",
    "scenario_families",
    "scenario_family_view",
    "scenario_version_view",
    "scenario_add_assumption",
    "scenario_add_unknown",
    "scenario_resolve_unknown",
    "scenario_add_constraint",
    "scenario_add_nonnegotiable",
    "scenario_record_conflict",
    "scenario_resolve_conflict",
    "scenario_finalize",
    "scenario_set_readiness",
    "scenario_what_if",
    "scenario_comparison",
    "vault_status",
    "vault_setup",
    "vault_recover",
    "vault_import",
    "vault_documents",
    "vault_document_history",
    "vault_document_content",
    "vault_search",
    "vault_backup",
    "package_preview",
    "package_export",
    "update_check",
    "update_install",
];

fn main() {
    let result = tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)),
    );
    #[allow(clippy::expect_used)]
    // build failure is unrecoverable; the diagnostic is the compile error
    result.expect("failed to run tauri-build");
}
