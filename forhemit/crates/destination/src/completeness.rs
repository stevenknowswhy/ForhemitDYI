//! The completeness indicator — structural, never a score.
//!
//! Destination Builder doc §25: *"I would **not** create a numerical
//! 'quality score' for the owner's destination. Instead, use a simple
//! completeness indicator"* listing which areas "have been established"
//! (Destination Engine doc §20 shows the ✓/? presentation and the two
//! derived labels: "Ready for exploration" vs "Working draft").

use crate::answer::Answer;
use crate::content::DestinationContent;
use serde::{Deserialize, Serialize};

/// Whether one destination area has been established.
///
/// There are exactly two states — no degrees, no quality levels, no
/// numbers (the docs forbid scoring the owner, and Destination Engine doc
/// §21 keeps destination confidence out of this engine entirely).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AreaState {
    /// The owner gave at least one substantive answer in the area.
    Established,
    /// The area has not been established yet — unanswered, or answered
    /// only with an explicit "I'm not sure" (honest unknowns are not
    /// established areas).
    NotEstablished,
}

/// Which of the six destination areas the Destination Engine doc §20
/// tracks are established, plus the derived overall label.
///
/// This type is deliberately score-free: six binary area states and a
/// two-way label, nothing else. Serialization carries no numeric fields —
/// asserted by test.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Completeness {
    /// Screens 2–4 — the financial outcome.
    pub financial: AreaState,
    /// Screens 5–7 — the ownership outcome.
    pub ownership: AreaState,
    /// Screen 8 — the personal outcome (owner role).
    pub personal: AreaState,
    /// Screen 9 — timing.
    pub timing: AreaState,
    /// Screen 10 — preservation goals.
    pub preservation: AreaState,
    /// Screen 11 — avoidances.
    pub avoidances: AreaState,
}

/// The doc §20/§25 overall label derived from the six areas.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum CompletenessLabel {
    /// All six areas established — "Destination ready for exploration".
    ReadyForExploration,
    /// At least one area still open — "Working draft".
    WorkingDraft,
}

impl Completeness {
    /// Derives the indicator from a destination's content.
    ///
    /// An area is established when at least one of its fields holds a
    /// substantive answer; an explicit "I'm not sure" keeps the area open
    /// rather than manufacturing establishment (the doc's `?` state).
    pub fn of(content: &DestinationContent) -> Self {
        Self {
            financial: established(&[
                content.financial_objective.value.answered().is_some(),
                content.closing_proceeds.value.answered().is_some(),
                content.future_income.value.answered().is_some(),
            ]),
            ownership: established(&[
                content.ownership_participants.value.answered().is_some(),
                content.employee_ownership_shape.value.answered().is_some(),
                content.ownership_allocation.value.answered().is_some(),
            ]),
            personal: established(&[content.owner_role.value.answered().is_some()]),
            timing: established(&[content.transition_timing.value.answered().is_some()]),
            preservation: established(&[matches!(
                content.preservation_goals,
                Answer::Answered(ref goals) if !goals.is_empty()
            )]),
            avoidances: established(&[matches!(
                content.avoidances,
                Answer::Answered(ref items) if !items.is_empty()
            )]),
        }
    }

    /// Whether every area is established — the doc §20 "ready for
    /// exploration" condition.
    pub fn ready_for_exploration(&self) -> bool {
        self.label() == CompletenessLabel::ReadyForExploration
    }

    /// The doc §20/§25 overall label: "Destination ready for exploration"
    /// or "Working draft".
    pub fn label(&self) -> CompletenessLabel {
        let areas = [
            self.financial,
            self.ownership,
            self.personal,
            self.timing,
            self.preservation,
            self.avoidances,
        ];
        if areas.iter().all(|state| *state == AreaState::Established) {
            CompletenessLabel::ReadyForExploration
        } else {
            CompletenessLabel::WorkingDraft
        }
    }
}

/// One established (any true) or not-established (all false) area.
fn established(answered: &[bool]) -> AreaState {
    if answered.iter().any(|was_answered| *was_answered) {
        AreaState::Established
    } else {
        AreaState::NotEstablished
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use crate::content::{
        Avoidance, FinancialObjective, FutureIncomeChoice, IncomeBand, IncomeDuration,
        IncomeInterest, OwnerRole, OwnershipParticipant, PreservationGoal, PreservationSelection,
        ProceedsBand, ProceedsChoice, TransitionTiming,
    };

    #[test]
    fn blank_content_is_a_working_draft_with_nothing_established() {
        let completeness = Completeness::of(&DestinationContent::default());
        assert_eq!(completeness.label(), CompletenessLabel::WorkingDraft);
        assert!(!completeness.ready_for_exploration());
        for area in [
            completeness.financial,
            completeness.ownership,
            completeness.personal,
            completeness.timing,
            completeness.preservation,
            completeness.avoidances,
        ] {
            assert_eq!(area, AreaState::NotEstablished);
        }
    }

    #[test]
    fn not_sure_keeps_the_area_open() {
        let mut content = DestinationContent::default();
        content.owner_role.value = Answer::NotSure;
        let completeness = Completeness::of(&content);
        assert_eq!(completeness.personal, AreaState::NotEstablished);
    }

    #[test]
    fn a_substantive_answer_establishes_its_area() {
        let mut content = DestinationContent::default();
        content.transition_timing.value = Answer::Answered(TransitionTiming::Flexible);
        let completeness = Completeness::of(&content);
        assert_eq!(completeness.timing, AreaState::Established);
        assert_eq!(completeness.financial, AreaState::NotEstablished);
    }

    #[test]
    fn a_full_destination_reads_ready_for_exploration() {
        let mut content = DestinationContent::default();
        content.financial_objective.value = Answer::Answered(FinancialObjective::CashNow);
        content.closing_proceeds.value =
            Answer::Answered(ProceedsChoice::Band(ProceedsBand::K3mTo5m));
        content.future_income.value = Answer::Answered(FutureIncomeChoice {
            interest: IncomeInterest::Yes,
            amount: Answer::Answered(IncomeBand::K50kTo100k),
            duration: Answer::Answered(IncomeDuration::Years5To10),
        });
        content.ownership_participants.value =
            Answer::Answered(vec![OwnershipParticipant::AllEmployees]);
        content.owner_role.value = Answer::Answered(OwnerRole::Retired);
        content.transition_timing.value = Answer::Answered(TransitionTiming::Within12Months);
        content.preservation_goals = Answer::Answered(vec![crate::objective::Objective::empty()
            .with_value(Answer::Answered(PreservationSelection {
                goal: PreservationGoal::EmployeesRemain,
                rank: Some(1),
            }))]);
        content.avoidances = Answer::Answered(vec![crate::objective::Objective::empty()
            .with_value(Answer::Answered(Avoidance::OutsideBuyer))]);

        let completeness = Completeness::of(&content);
        assert!(completeness.ready_for_exploration());
        assert_eq!(completeness.label(), CompletenessLabel::ReadyForExploration);
    }
}
