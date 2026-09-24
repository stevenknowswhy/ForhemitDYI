//! Engine identity vocabulary.
//!
//! Audit doc §20: every audit record identifies the engine that produced it.
//! The variants cover the v1 engine roster from the implementation spec's
//! scope table; engines deferred with doc receipts (research, marketplace,
//! the transaction layer, …) join through a new contract version, never by
//! reinterpreting stored values.

use serde::{Deserialize, Serialize};

/// The engine that produced an event or owns an object.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineId {
    /// Audit / provenance engine — the append-only event store (crate `audit`).
    Audit,
    /// Local vault — the encrypted document workspace (crate `vault`).
    Vault,
    /// Destination engine — the journey's persistent North Star (crate
    /// `destination`).
    Destination,
    /// Journey runtime — walks the EOJ journey definition (crate `journey`).
    Journey,
    /// Business Reality — owner-reported facts (crate `reality`).
    Reality,
    /// Scenario engine — families, versions, conflicts (crate `scenario`).
    Scenario,
    /// Professional review package assembly and export (crate `package`).
    Package,
    /// The Tauri shell itself (crate `app`).
    App,
}
