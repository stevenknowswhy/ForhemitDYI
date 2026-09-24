//! The three decision layers (THREE DECISION LAYERS doc).
//!
//! "The platform must maintain three distinct layers throughout the entire
//! ownership-transition journey … must never be blended." UI layer-coloring
//! (doc §4: 🟦 owner objective, 🟨 platform scenario, 🟩 professional
//! determination) keys off this type.

use serde::{Deserialize, Serialize};

/// Which of the three decision layers a piece of information belongs to.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionLayer {
    /// What the owner wants — "the source of truth for the owner's wishes,
    /// not a professional conclusion" (doc §1).
    OwnerObjective,
    /// What might accomplish the objective under current assumptions —
    /// "exploratory scenarios, not professional advice" (doc §2).
    PlatformScenario,
    /// What the qualified professional concludes — "outside the platform's
    /// authority" (doc §3); recorded, never manufactured.
    ProfessionalDetermination,
}
