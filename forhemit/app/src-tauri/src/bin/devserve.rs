//! Dev-only local harness behind the `dev-server` feature.
//!
//! Serves the built frontend from a directory and exposes the SAME command
//! functions the Tauri shell invokes — `commands::*`, never engine internals —
//! over a localhost-only socket. This exists so the walkthrough can be
//! dogfooded in environments without a display server; it is not part of
//! the shipped app and never enabled by default.
//!
//! Usage:
//!   forhemit_devserve --data-dir <dir> --port <port> --asset-dir <dir>

use std::path::{Path, PathBuf};
use std::sync::Arc;

use forhemit_app_lib::state::AppEngines;
use serde_json::Value;
use tiny_http::{Header, Method, Response, Server};

const HELP: &str = "\
forhemit_devserve — local-only command harness for the Forhemit shell

USAGE:
  forhemit_devserve --data-dir <dir> --port <port> --asset-dir <dir>

All three flags are required. The socket binds 127.0.0.1 only.";

struct Config {
    data_dir: PathBuf,
    port: u16,
    asset_dir: PathBuf,
}

/// Exits with a usage diagnostic if flags are missing or malformed.
fn parse_args() -> Option<Config> {
    let mut data_dir = None;
    let mut port = None;
    let mut asset_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next()?;
        match flag.as_str() {
            "--data-dir" => data_dir = Some(PathBuf::from(value)),
            "--asset-dir" => asset_dir = Some(PathBuf::from(value)),
            "--port" => port = Some(value.parse().ok()?),
            _ => return None,
        }
    }
    Some(Config {
        data_dir: data_dir?,
        port: port?,
        asset_dir: asset_dir?,
    })
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

/// Reads a file inside `root`, refusing paths that escape it. Extension-less
/// paths fall back to the shell page so SPA routes survive refresh.
fn asset_under(root: &Path, requested: &str) -> Option<(PathBuf, Vec<u8>)> {
    let relative = requested.trim_start_matches('/');
    let relative = if relative.is_empty() {
        "index.html"
    } else {
        relative
    };
    let mut candidates = vec![root.join(relative)];
    if !relative.contains('.') {
        candidates.push(root.join("index.html"));
    }
    for candidate in candidates {
        if candidate.starts_with(root) {
            if let Ok(bytes) = std::fs::read(&candidate) {
                return Some((candidate, bytes));
            }
        }
    }
    None
}

fn main() {
    let Some(config) = parse_args() else {
        eprintln!("{HELP}");
        std::process::exit(2);
    };
    let engines = match AppEngines::open(&config.data_dir) {
        Ok(engines) => Arc::new(engines),
        Err(error) => {
            eprintln!("failed to open workspace at {:?}: {error}", config.data_dir);
            std::process::exit(1);
        }
    };
    let server = match Server::http(("127.0.0.1", config.port)) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("cannot bind 127.0.0.1:{}: {error}", config.port);
            std::process::exit(1);
        }
    };
    eprintln!(
        "forhemit_devserve: serving {:?} on http://127.0.0.1:{} (data: {:?})",
        config.asset_dir, config.port, config.data_dir
    );

    for mut request in server.incoming_requests() {
        let url = request.url().to_owned();
        match (request.method(), url.as_str()) {
            (Method::Post, path) if path.starts_with("/command/") => {
                let name = path.trim_start_matches("/command/").to_owned();
                let mut body = String::new();
                if std::io::Read::read_to_string(request.as_reader(), &mut body).is_err() {
                    reply(request, 400, "\"bad body\"");
                    continue;
                }
                let (status, payload) = match dispatch(&engines, &name, &body) {
                    Ok(json) => (200, json),
                    Err(error) => (400, format!("\"{error}\"")),
                };
                reply(request, status, &payload);
            }
            (Method::Get, path) => match asset_under(&config.asset_dir, path) {
                Some((asset_path, bytes)) => {
                    let content_type = content_type(&asset_path);
                    // Static header constants — infallible by inspection.
                    #[allow(clippy::expect_used)]
                    let header =
                        Header::from_bytes("Content-Type", content_type).expect("valid header");
                    let _ = request.respond(Response::from_data(bytes).with_header(header));
                }
                None => reply(request, 404, "\"not found\""),
            },
            _ => reply(request, 405, "\"method not allowed\""),
        }
    }
}

/// Sends a JSON string response and logs a short diagnostic on send failure.
#[allow(clippy::expect_used)] // header strings are compile-time constants
fn reply(request: tiny_http::Request, status: u32, body: &str) {
    let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8")
        .expect("valid header");
    if let Err(error) = request.respond(
        Response::from_data(body.as_bytes().to_vec())
            .with_status_code(status)
            .with_header(header),
    ) {
        eprintln!("respond: {error}");
    }
}

/// Dispatches one named command with its JSON arguments to the same
/// adapter functions the Tauri shell calls. Unknown names, bad arguments,
/// and engine refusals all return an error string the harness surfaces
/// with HTTP 400 — engine refusals are displayed, never swallowed.
fn dispatch(engines: &AppEngines, name: &str, body: &str) -> Result<String, String> {
    let args: Value = if body.trim().is_empty() {
        Value::Null
    } else {
        serde_json::from_str(body).map_err(|error| format!("bad json: {error}"))?
    };
    let arg = |key: &str| -> Value { args.get(key).cloned().unwrap_or(Value::Null) };
    use forhemit_app_lib::commands;
    match name {
        "destination_get" => to_json(Ok(commands::destination_get(engines))),
        "destination_create" => to_json(commands::destination_create(
            engines,
            serde_json::from_value(arg("content")).map_err(|e| e.to_string())?,
        )),
        "destination_edit" => to_json(commands::destination_edit(
            engines,
            serde_json::from_value(arg("content")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("reason")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("explanation")).map_err(|e| e.to_string())?,
        )),
        "destination_confirm" => to_json(commands::destination_confirm(engines)),
        "destination_mark_working" => to_json(commands::destination_mark_working(engines)),
        "destination_archive" => to_json(commands::destination_archive(engines)),
        "destination_completeness" => to_json(Ok(commands::destination_completeness_for(
            &serde_json::from_value(arg("content")).map_err(|e| e.to_string())?,
        ))),
        "journey_state" => to_json(commands::journey_state(engines)),
        "journey_start" => to_json(commands::journey_start(engines)),
        "journey_record" => to_json(commands::journey_record(
            engines,
            serde_json::from_value(arg("node_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("value")).map_err(|e| e.to_string())?,
        )),
        "journey_revise" => to_json(commands::journey_revise(
            engines,
            serde_json::from_value(arg("node_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("value")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("change_reason")).map_err(|e| e.to_string())?,
        )),
        "journey_skip" => to_json(commands::journey_skip(
            engines,
            serde_json::from_value(arg("node_id")).map_err(|e| e.to_string())?,
        )),
        "snapshot_current" => to_json(commands::snapshot_current(engines)),
        "snapshot_record" => to_json(commands::snapshot_record(
            engines,
            serde_json::from_value(arg("kind")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("value")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("definition")).map_err(|e| e.to_string())?,
        )),
        "snapshot_revise" => to_json(commands::snapshot_revise(
            engines,
            serde_json::from_value(arg("fact_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("value")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("definition")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("change_reason")).map_err(|e| e.to_string())?,
        )),
        "audit_recent" => to_json(commands::audit_recent(engines)),
        "audit_verify" => to_json(commands::audit_verify(engines)),
        "scenario_create" => to_json(commands::scenario_create(
            engines,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_families" => to_json(commands::scenario_families(engines)),
        "scenario_family_view" => to_json(commands::scenario_family_view(
            engines,
            serde_json::from_value(arg("family_id")).map_err(|e| e.to_string())?,
        )),
        "scenario_version_view" => to_json(commands::scenario_version_view(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
        )),
        "scenario_add_assumption" => to_json(commands::scenario_add_assumption(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_add_unknown" => to_json(commands::scenario_add_unknown(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_resolve_unknown" => to_json(commands::scenario_resolve_unknown(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("unknown_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("status")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("resolution_reference")).map_err(|e| e.to_string())?,
        )),
        "scenario_add_constraint" => to_json(commands::scenario_add_constraint(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_add_nonnegotiable" => to_json(commands::scenario_add_nonnegotiable(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_record_conflict" => to_json(commands::scenario_record_conflict(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_resolve_conflict" => to_json(commands::scenario_resolve_conflict(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("conflict_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("owner_decision")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("resolution_reference")).map_err(|e| e.to_string())?,
        )),
        "scenario_finalize" => to_json(commands::scenario_finalize(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
        )),
        "scenario_set_readiness" => to_json(commands::scenario_set_readiness(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("readiness")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("reason")).map_err(|e| e.to_string())?,
        )),
        "scenario_what_if" => to_json(commands::scenario_what_if(
            engines,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "scenario_comparison" => to_json(commands::scenario_comparison(
            engines,
            serde_json::from_value(arg("request")).map_err(|e| e.to_string())?,
        )),
        "vault_status" => to_json(commands::vault_status(engines)),
        "vault_setup" => to_json(commands::vault_setup(
            engines,
            serde_json::from_value(arg("recovery_passphrase")).map_err(|e| e.to_string())?,
        )),
        "vault_recover" => to_json(commands::vault_recover(
            engines,
            serde_json::from_value(arg("recovery_passphrase")).map_err(|e| e.to_string())?,
        )),
        "vault_import" => to_json(commands::vault_import(
            engines,
            serde_json::from_value(arg("filename")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("content_base64")).map_err(|e| e.to_string())?,
            serde_json::from_value(arg("note")).map_err(|e| e.to_string())?,
        )),
        "vault_documents" => to_json(commands::vault_documents(engines)),
        "vault_document_history" => to_json(commands::vault_document_history(
            engines,
            serde_json::from_value(arg("document_id")).map_err(|e| e.to_string())?,
        )),
        "vault_document_content" => to_json(commands::vault_document_content(
            engines,
            serde_json::from_value(arg("version_id")).map_err(|e| e.to_string())?,
        )),
        "vault_search" => to_json(commands::vault_search(
            engines,
            serde_json::from_value(arg("query")).map_err(|e| e.to_string())?,
        )),
        "vault_backup" => to_json(commands::vault_backup(
            engines,
            serde_json::from_value(arg("recovery_passphrase")).map_err(|e| e.to_string())?,
        )),
        "package_preview" => to_json(commands::package_preview(engines)),
        "package_export" => to_json(commands::package_export(
            engines,
            serde_json::from_value(arg("format")).map_err(|e| e.to_string())?,
        )),
        other => Err(format!("unknown command: {other}")),
    }
}

/// Serializes a command result for the HTTP surface: `Ok` unwraps to the
/// bare JSON value the frontend expects; `Err` returns the engine's
/// refusal message, which the dispatcher maps to HTTP 400.
fn to_json<T: serde::Serialize>(result: Result<T, String>) -> Result<String, String> {
    match result {
        Ok(value) => serde_json::to_string(&value).map_err(|error| format!("serialize: {error}")),
        Err(message) => Err(message),
    }
}
