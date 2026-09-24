//! Preference-strength vocabulary (NONNEGOTIABLE - MUST-HAVE CONSTRAINT doc).
//!
//! "A nonnegotiable is never silently relaxed, ignored, or overridden" (spec).
//! The doc's §12 defines three explicit states and one default: "The owner
//! should not be required to classify everything. The default should be:
//! No designation."

use serde::{Deserialize, Serialize};

/// How strongly an owner's answer is held. Defaults to
/// [`NonnegotiableState::Unspecified`].
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum NonnegotiableState {
    /// No designation — the doc's default unless the owner explicitly
    /// indicates a level of importance.
    #[default]
    Unspecified,
    /// "I would like this" — 🟢 desirable but negotiable.
    Preference,
    /// "This is very important to me" — 🟠 very important, but potentially
    /// changeable.
    StrongPreference,
    /// "I do not want to proceed with a path that does not satisfy this" —
    /// 🔴 must-have; a hard constraint for the scenario engine (doc §3).
    Nonnegotiable,
}
