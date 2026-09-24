//! Destination status — process state, not feasibility.
//!
//! Destination Engine doc §6: "These describe process state, not
//! feasibility." The engine's transition rules live with the engine
//! ([`crate::engine`]); this module holds the state vocabulary itself.

use serde::{Deserialize, Serialize};

/// Where the destination stands in its process lifecycle.
///
/// `UnderProfessionalReview` transitions are set by later engines (the
/// professional review flow); this engine moves destinations through
/// Draft, Working, Confirmed, Revised (owner edits after confirmation),
/// and Archived. `Superseded` is never stored as aggregate state: it is
/// the derived status of any version that is no longer the chain head
/// ([`crate::destination::Destination::effective_status`]).
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum DestinationStatus {
    /// The owner is still constructing the destination.
    #[default]
    Draft,
    /// A usable destination the owner expects may change — the Builder
    /// doc §17 checkpoint's "I'm not sure yet" outcome.
    Working,
    /// The owner explicitly confirmed: "This is the outcome I'm trying to
    /// create."
    Confirmed,
    /// A professional is evaluating the destination.
    UnderProfessionalReview,
    /// The owner materially changed the destination after additional
    /// information or professional feedback.
    Revised,
    /// A newer destination version has replaced this version (derived,
    /// never stored as aggregate state).
    Superseded,
    /// Retained historically but no longer active.
    Archived,
}

impl DestinationStatus {
    /// The status as displayed — the doc §6 state names.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Working => "Working",
            Self::Confirmed => "Confirmed",
            Self::UnderProfessionalReview => "Under Professional Review",
            Self::Revised => "Revised",
            Self::Superseded => "Superseded",
            Self::Archived => "Archived",
        }
    }
}

impl std::fmt::Display for DestinationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    #[test]
    fn draft_is_the_default_status() {
        assert_eq!(DestinationStatus::default(), DestinationStatus::Draft);
    }

    #[test]
    fn statuses_round_trip_as_snake_case() {
        for status in [
            DestinationStatus::Draft,
            DestinationStatus::Working,
            DestinationStatus::Confirmed,
            DestinationStatus::UnderProfessionalReview,
            DestinationStatus::Revised,
            DestinationStatus::Superseded,
            DestinationStatus::Archived,
        ] {
            let json = serde_json::to_value(&status).unwrap();
            let back: DestinationStatus = serde_json::from_value(json).unwrap();
            assert_eq!(back, status);
        }
    }
}
