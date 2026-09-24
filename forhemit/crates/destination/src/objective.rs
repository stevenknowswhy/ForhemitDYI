//! The objective wrapper — a stable reference ID, an answer, and a
//! preference strength.
//!
//! Every meaningful destination objective can become a nonnegotiable
//! (Destination Builder §1), and the NONNEGOTIABLE doc §16 requires the
//! system to default to **Unspecified / No designation** "rather than
//! guessing importance". The stable [`ObjectiveId`] is the machinery the
//! spec points at for downstream engines: nonnegotiable constraints
//! "expose reference IDs so later scenario conflicts can back-reference
//! them".

use crate::answer::Answer;
use forhemit_contracts::{NonnegotiableState, ObjectiveId};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// One owner objective: its (possibly undecided) value, how strongly it is
/// held, and a stable identifier that survives across destination versions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Objective<T> {
    /// Stable cross-version reference for this objective — the ID a later
    /// `NonnegotiableConflict` back-references.
    pub objective_id: ObjectiveId,
    /// The owner's answer for this objective.
    pub value: Answer<T>,
    /// How strongly the objective is held — the 🔴/🟠/🟢 states of the
    /// NONNEGOTIABLE doc §12, defaulting to no designation.
    pub preference: NonnegotiableState,
}

impl<T> Objective<T> {
    /// A fresh objective: new stable ID, unanswered, no preference
    /// designation.
    pub fn empty() -> Self {
        Self {
            objective_id: new_objective_id(),
            value: Answer::Unanswered,
            preference: NonnegotiableState::Unspecified,
        }
    }

    /// Replaces the value, keeping the stable ID and preference state.
    pub fn with_value(self, value: Answer<T>) -> Self {
        Self { value, ..self }
    }

    /// Replaces the preference state, keeping the stable ID and value.
    pub fn with_preference(self, preference: NonnegotiableState) -> Self {
        Self { preference, ..self }
    }

    /// Whether two objectives carry the same owner-visible content — the
    /// answer and the preference strength. The stable ID is deliberately
    /// excluded from the comparison: it exists precisely to survive
    /// across versions, so two versions may describe the same objective
    /// under different `ObjectiveId`-minting construction paths while
    /// meaning identical content.
    pub fn same_content(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.value == other.value && self.preference == other.preference
    }

    /// The objective's reference ID and state, when the owner designated
    /// a strength. Unspecified objectives are omitted — "no designation"
    /// is the doc §16 default, not a constraint.
    pub fn designation(&self) -> Option<(ObjectiveId, NonnegotiableState)> {
        match self.preference {
            NonnegotiableState::Unspecified => None,
            NonnegotiableState::Preference
            | NonnegotiableState::StrongPreference
            | NonnegotiableState::Nonnegotiable => {
                Some((self.objective_id.clone(), self.preference.clone()))
            }
        }
    }
}

/// Mints a fresh stable objective identifier (a ULID).
pub fn new_objective_id() -> ObjectiveId {
    ObjectiveId::new(Ulid::new().to_string())
        .unwrap_or_else(|_| unreachable!("a ULID string is never empty"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    #[test]
    fn empty_objective_defaults_to_unspecified_preference() {
        let objective: Objective<String> = Objective::empty();
        assert_eq!(objective.preference, NonnegotiableState::Unspecified);
        assert_eq!(objective.value, Answer::Unanswered);
    }

    #[test]
    fn builders_keep_the_stable_id() {
        let objective: Objective<String> = Objective::empty()
            .with_value(Answer::NotSure)
            .with_preference(NonnegotiableState::Nonnegotiable);
        assert_eq!(objective.value, Answer::NotSure);
        assert_eq!(objective.preference, NonnegotiableState::Nonnegotiable);
    }

    #[test]
    fn two_objectives_never_share_an_id() {
        let a: Objective<String> = Objective::empty();
        let b: Objective<String> = Objective::empty();
        assert_ne!(a.objective_id, b.objective_id);
    }
}
