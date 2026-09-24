//! Forhemit desktop shell.
//!
//! The shell owns no domain logic: every Tauri command is a thin adapter over
//! an engine crate API (spec art_fL7Z2ate). Domain state will live behind
//! `tauri::State` as the engine crates land.

/// Placeholder command demonstrating the IPC surface; replaced by engine
/// adapters as engines land.
///
/// Private visibility is deliberate: `generate_handler!` resolves it in this
/// module, and a `pub` command makes the macro emit a same-module macro
/// re-import that collides in the macro namespace (E0255).
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}. Forhemit is running locally — nothing leaves this machine.")
}

/// Runs the desktop app.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::expect_used)] // builder failure at startup is unrecoverable; the template aborts with a diagnostic
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("failed to run the Forhemit application");
}
