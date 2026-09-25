//! Test-only helper: an [`AppEngines`] over a scratch workspace directory.
#![cfg(test)]

use crate::state::AppEngines;

/// Opens engines over a fresh temporary workspace. The temp directory is
/// intentionally leaked (test process lifetime) so the engines' open
/// SQLite handles and persisted files stay valid for the whole test.
pub(crate) fn opened_engines() -> AppEngines {
    let dir = tempfile::TempDir::new().expect("scratch workspace dir");
    let path = dir.path().to_path_buf();
    std::mem::forget(dir);
    AppEngines::open(&path).expect("engines open over scratch workspace")
}
