//! The `DesiredOutcome` content tree — the data model of all 13 Destination
//! Builder screens.
//!
//! Field-for-field from `Employee Ownership Journey - Destination Builder
//! v1.0` (screens 1–13 and the §18 object model) and the Destination Engine
//! doc's §4 data model. The content is *owner truth*: what the owner wants,
//! never a feasibility judgment (Destination Engine doc §21 — "Destination
//! = owner truth, rather than Destination = platform judgment").
//!
//! Screen map (screen → fields):
//!
//! | Screen | Fields |
//! | ------ | ------ |
//! | 1 Picture the finish line | (no field — "I'm Not Sure Yet" simply creates the destination) |
//! | 2 Main financial outcome | [`DestinationContent::financial_objective`] |
//! | 3 Cash at closing | [`DestinationContent::closing_proceeds`] |
//! | 4 Future income | [`DestinationContent::future_income`] |
//! | 5 Who owns the company | [`DestinationContent::ownership_participants`] |
//! | 6 Employee ownership shape | [`DestinationContent::employee_ownership_shape`] |
//! | 7 Optional ownership allocation | [`DestinationContent::ownership_allocation`] |
//! | 8 Owner role after transition | [`DestinationContent::owner_role`] |
//! | 9 When should this happen | [`DestinationContent::transition_timing`] |
//! | 10 What should remain true | [`DestinationContent::preservation_goals`] |
//! | 11 What to avoid | [`DestinationContent::avoidances`] |
//! | 12 Anything else | [`DestinationContent::additional_context`] |
//! | 13 Destination review | (reads the fields above; no new data) |

use crate::answer::Answer;
use crate::error::DestinationError;
use crate::objective::Objective;
use forhemit_contracts::{NonnegotiableState, ObjectiveId};
use serde::{Deserialize, Serialize};

/// Screen 2 — the main financial objective.
///
/// "This identifies the **financial objective**, not the transaction
/// mechanism. Do not mention seller notes yet."
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum FinancialObjective {
    /// 💰 "I want substantial cash at closing."
    CashNow,
    /// 💵 "I'm comfortable receiving some of the value over time."
    IncomeOverTime,
    /// ⚖️ "I'd like a mix of cash now and future income."
    Combination,
}

/// Screen 3 — the displayed closing-proceeds bands.
///
/// Serde names pin the doc's band labels ("Under $500K", "$500K–$1M", …)
/// as stable lowercase keys.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ProceedsBand {
    /// Under $500K.
    #[serde(rename = "under_500k")]
    Under500k,
    /// $500K–$1M.
    #[serde(rename = "500k_to_1m")]
    K500kTo1m,
    /// $1M–$2M.
    #[serde(rename = "1m_to_2m")]
    K1mTo2m,
    /// $2M–$3M.
    #[serde(rename = "2m_to_3m")]
    K2mTo3m,
    /// $3M–$5M.
    #[serde(rename = "3m_to_5m")]
    K3mTo5m,
    /// $5M–$10M.
    #[serde(rename = "5m_to_10m")]
    M5To10m,
    /// $10M+.
    #[serde(rename = "over_10m")]
    Over10m,
}

/// Screen 3 — "About how much would you ideally like at closing?"
///
/// "I don't know yet" is [`Answer::NotSure`] on the surrounding objective —
/// the owner "should never be blocked because they don't know the value of
/// their business" (Builder doc §6).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ProceedsChoice {
    /// One of the displayed bands.
    Band(ProceedsBand),
    /// "Enter my own range" — for larger or unusual transactions; either
    /// bound may be left open.
    CustomRange {
        /// The owner's ideal minimum at closing, if stated.
        minimum: Option<u64>,
        /// The owner's ideal maximum at closing, if stated.
        maximum: Option<u64>,
    },
}

/// Screen 4 — "Would you like the business transition to provide income
/// after closing?" (Yes / Maybe / No; "I'm not sure" is the answer-level
/// [`Answer::NotSure`]).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum IncomeInterest {
    /// Yes.
    Yes,
    /// Maybe.
    Maybe,
    /// No.
    No,
}

/// Screen 4 — the displayed annual-income bands ("Under $25K", …).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum IncomeBand {
    /// Under $25K.
    #[serde(rename = "under_25k")]
    Under25k,
    /// $25K–$50K.
    #[serde(rename = "25k_to_50k")]
    K25kTo50k,
    /// $50K–$100K.
    #[serde(rename = "50k_to_100k")]
    K50kTo100k,
    /// $100K–$250K.
    #[serde(rename = "100k_to_250k")]
    K100kTo250k,
    /// $250K+.
    #[serde(rename = "over_250k")]
    Over250k,
}

/// Screen 4 — "For approximately how long?" ("1–3 years", …, "I'm not
/// sure" is the answer-level [`Answer::NotSure`]).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum IncomeDuration {
    /// 1–3 years.
    #[serde(rename = "1_to_3_years")]
    Years1To3,
    /// 3–5 years.
    #[serde(rename = "3_to_5_years")]
    Years3To5,
    /// 5–10 years.
    #[serde(rename = "5_to_10_years")]
    Years5To10,
    /// 10+ years.
    #[serde(rename = "over_10_years")]
    Over10Years,
    /// Ongoing.
    Ongoing,
}

/// Screen 4 — the stored **Desired Future Income** (amount range, duration
/// range; per Builder doc §7).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureIncomeChoice {
    /// Whether the owner wants post-closing income at all.
    pub interest: IncomeInterest,
    /// "About how much annual income would you ideally like?" — "I don't
    /// know yet" stays [`Answer::NotSure`] (Builder doc §4).
    pub amount: Answer<IncomeBand>,
    /// "For approximately how long?"
    pub duration: Answer<IncomeDuration>,
}

/// Screen 5 — "Who would you like to have ownership?" (multi-select;
/// "I'm not sure" is the answer-level [`Answer::NotSure`]).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum OwnershipParticipant {
    /// All employees.
    AllEmployees,
    /// Management.
    Management,
    /// A specific employee group — the group, described by the owner.
    SpecificEmployeeGroup(String),
    /// Family members.
    Family,
    /// Existing owners.
    ExistingOwners,
    /// Outside investors.
    OutsideInvestors,
    /// Other — described by the owner.
    Other(String),
}

/// Screen 6 — "How would you like employee ownership to work?" —
/// conceptual choices, "merely descriptions of the owner's desired
/// outcome", never legal structures.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum EmployeeOwnershipShape {
    /// "I want ownership to be broadly shared among employees."
    Broad,
    /// "I want employees to own part of the company and management to have
    /// additional ownership."
    EmployeesPlusManagement,
    /// "I want employees to own part of the company alongside other
    /// owners."
    WithOtherOwners,
}

/// Screen 7 — one share of the approximate desired allocation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AllocationShare {
    /// Which participant group this share describes.
    pub participant: AllocationParticipant,
    /// The owner's approximate percentage (0–100).
    pub percent: u32,
}

/// Screen 7 — which participant group an allocation share describes.
/// "Employees" and "Management" are the doc's named axes (doc §10 example:
/// Employees 60% / Management 30% / Other 10%); anything else is
/// [`AllocationParticipant::Other`] with the owner's label.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AllocationParticipant {
    /// Employees.
    Employees,
    /// Management.
    Management,
    /// Any other group, described by the owner.
    Other(String),
}

/// Screen 7 — "Do you have an approximate ownership split in mind?"
///
/// "Set approximate percentages" totals 100% (validated — the engine never
/// fixes the owner's numbers, Destination Engine doc §40); "I only know who
/// should participate" is [`AllocationChoice::ParticipantsOnly`]; "I'm not
/// sure" is the answer-level [`Answer::NotSure`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AllocationChoice {
    /// Approximate percentages — "These percentages describe your desired
    /// ownership outcome. They are not a proposed legal ownership
    /// structure."
    Percentages(Vec<AllocationShare>),
    /// "I only know who should participate."
    ParticipantsOnly,
}

/// Screen 8 — "On the day the transition is complete, what would you like
/// your role to be?"
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum OwnerRole {
    /// "I'm ready to step away completely."
    Retired,
    /// "I'd like to help for a limited period."
    TransitionAdvisor,
    /// "I'd like to remain available occasionally."
    OngoingAdvisor,
    /// "I'd like to remain meaningfully involved."
    ContinuingOwnerOperator,
}

/// Screen 9 — "When would you ideally like to reach this destination?" The
/// system stores a **desired timeframe**, not a guaranteed closing date.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum TransitionTiming {
    /// Within 12 months.
    Within12Months,
    /// 1–3 years.
    #[serde(rename = "1_to_3_years")]
    OneToThreeYears,
    /// 3–5 years.
    #[serde(rename = "3_to_5_years")]
    ThreeToFiveYears,
    /// More than 5 years.
    MoreThanFiveYears,
    /// "I'm flexible."
    Flexible,
}

/// Screen 10 — "What would you like to remain true about the company after
/// the transition?"
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PreservationGoal {
    /// Employees remain with the company.
    EmployeesRemain,
    /// Company remains independent.
    RemainsIndependent,
    /// Company stays in the same location.
    SameLocation,
    /// Company culture remains.
    CultureRemains,
    /// Current leadership remains.
    LeadershipRemains,
    /// Brand remains.
    BrandRemains,
    /// Family remains involved.
    FamilyInvolved,
    /// Community presence remains.
    CommunityPresenceRemains,
    /// Company continues serving existing customers.
    CustomersServed,
    /// Other — described by the owner.
    Other(String),
}

/// Screen 10 — one selected preservation goal and its rank ("Which matters
/// most?" ranks only the selected items).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreservationSelection {
    /// The selected goal.
    pub goal: PreservationGoal,
    /// The owner's rank among the selected goals, if ranked (1 = matters
    /// most).
    pub rank: Option<u32>,
}

/// Screen 11 — "What would you like to avoid?" Negative goals become
/// `DesiredAvoidance`s; "an avoidance can also be marked nonnegotiable"
/// (Destination Engine doc §14).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Avoidance {
    /// Selling to an outside buyer.
    OutsideBuyer,
    /// Losing employee ownership.
    LosingEmployeeOwnership,
    /// Remaining involved long-term.
    LongTermInvolvement,
    /// Leaving employees behind.
    LeavingEmployeesBehind,
    /// Excessive debt.
    ExcessiveDebt,
    /// Waiting many years for proceeds.
    WaitingManyYearsForProceeds,
    /// Moving the business.
    MovingTheBusiness,
    /// Losing company independence.
    LosingIndependence,
    /// Major disruption to operations.
    MajorOperationalDisruption,
    /// Other — described by the owner.
    Other(String),
}

/// The DesiredOutcome content: every field of the 13 Destination Builder
/// screens (see the [module map](self) for the screen → field table).
///
/// Every scalar objective is an [`Objective`] — stable ID, answer, and
/// preference strength. The two per-item lists keep one [`Objective`] per
/// selected item because screens 10 and 11 allow "This is nonnegotiable"
/// on each applicable item.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DestinationContent {
    /// Screen 2 — the main financial outcome.
    pub financial_objective: Objective<FinancialObjective>,
    /// Screen 3 — desired cash at closing (minimum / maximum /
    /// approximate, and whether reaching the minimum is a must-have).
    pub closing_proceeds: Objective<ProceedsChoice>,
    /// Screen 4 — desired future income.
    pub future_income: Objective<FutureIncomeChoice>,
    /// Screen 5 — who should own the company after the transition.
    pub ownership_participants: Objective<Vec<OwnershipParticipant>>,
    /// Screen 6 — how employee ownership should look, when relevant.
    pub employee_ownership_shape: Objective<EmployeeOwnershipShape>,
    /// Screen 7 — approximate desired ownership allocation, when offered.
    pub ownership_allocation: Objective<AllocationChoice>,
    /// Screen 8 — the owner's desired role after the transition.
    pub owner_role: Objective<OwnerRole>,
    /// Screen 9 — the desired transition timeframe.
    pub transition_timing: Objective<TransitionTiming>,
    /// Screen 10 — preservation goals (up to 3 initially, rankable, each
    /// individually markable nonnegotiable).
    pub preservation_goals: Answer<Vec<Objective<PreservationSelection>>>,
    /// Screen 11 — avoidances (each individually markable nonnegotiable;
    /// an empty selection is the doc's "Nothing specific").
    pub avoidances: Answer<Vec<Objective<Avoidance>>>,
    /// Screen 12 — "Additional Owner Context" (optional free text).
    pub additional_context: Answer<String>,
}

impl Default for DestinationContent {
    /// A blank destination: every objective unanswered, every preference
    /// unspecified — the state behind screen 1's "I'm Not Sure Yet" path
    /// (which "still creates a destination, but with fewer initial
    /// constraints").
    fn default() -> Self {
        Self {
            financial_objective: Objective::empty(),
            closing_proceeds: Objective::empty(),
            future_income: Objective::empty(),
            ownership_participants: Objective::empty(),
            employee_ownership_shape: Objective::empty(),
            ownership_allocation: Objective::empty(),
            owner_role: Objective::empty(),
            transition_timing: Objective::empty(),
            preservation_goals: Answer::Unanswered,
            avoidances: Answer::Unanswered,
            additional_context: Answer::Unanswered,
        }
    }
}

/// Validates owner input the Destination Engine doc §40 requires the system
/// to surface — and *not* fix: an allocation control "totaling 100%".
pub fn validate_content(content: &DestinationContent) -> Result<(), DestinationError> {
    if let Answer::Answered(AllocationChoice::Percentages(shares)) =
        &content.ownership_allocation.value
    {
        let total: u32 = shares.iter().map(|share| share.percent).sum();
        if total != 100 {
            return Err(DestinationError::AllocationDoesNotTotal100 { total });
        }
    }
    Ok(())
}

impl DestinationContent {
    /// Every objective in this content that carries a preference
    /// designation, as stable reference IDs paired with their state —
    /// the spec's back-reference machinery for later scenario conflicts.
    pub fn designated_objectives(&self) -> Vec<(ObjectiveId, NonnegotiableState)> {
        let mut designated = Vec::new();
        collect_designated(&self.financial_objective, &mut designated);
        collect_designated(&self.closing_proceeds, &mut designated);
        collect_designated(&self.future_income, &mut designated);
        collect_designated(&self.ownership_participants, &mut designated);
        collect_designated(&self.employee_ownership_shape, &mut designated);
        collect_designated(&self.ownership_allocation, &mut designated);
        collect_designated(&self.owner_role, &mut designated);
        collect_designated(&self.transition_timing, &mut designated);
        if let Answer::Answered(goals) = &self.preservation_goals {
            for goal in goals {
                collect_designated(goal, &mut designated);
            }
        }
        if let Answer::Answered(items) = &self.avoidances {
            for item in items {
                collect_designated(item, &mut designated);
            }
        }
        designated
    }
}

/// Collects one objective's designation, if it carries any.
fn collect_designated<T>(
    objective: &Objective<T>,
    designated: &mut Vec<(ObjectiveId, NonnegotiableState)>,
) {
    if let Some(entry) = objective.designation() {
        designated.push(entry);
    }
}

/// Names the content fields that differ between two versions' content —
/// the `changed_fields` of the doc §29 `DestinationChanged` event and the
/// basis of `compareDestinationVersions` (Destination Engine doc §37).
///
/// Objectives compare by owner-visible content (`same_content`), never by
/// their stable ID: the ID exists to survive across versions, so an
/// unchanged objective with a differently minted ID is not a change.
/// List fields compare element-wise by content: a rank change on one
/// preservation goal is a change to `PreservationGoals`, not a new field.
pub fn diff(previous: &DestinationContent, next: &DestinationContent) -> Vec<ChangedField> {
    use ChangedField::*;
    let mut changed = Vec::new();
    if !previous
        .financial_objective
        .same_content(&next.financial_objective)
    {
        changed.push(FinancialObjective);
    }
    if !previous
        .closing_proceeds
        .same_content(&next.closing_proceeds)
    {
        changed.push(ClosingProceeds);
    }
    if !previous.future_income.same_content(&next.future_income) {
        changed.push(FutureIncome);
    }
    if !previous
        .ownership_participants
        .same_content(&next.ownership_participants)
    {
        changed.push(OwnershipParticipants);
    }
    if !previous
        .employee_ownership_shape
        .same_content(&next.employee_ownership_shape)
    {
        changed.push(EmployeeOwnershipShape);
    }
    if !previous
        .ownership_allocation
        .same_content(&next.ownership_allocation)
    {
        changed.push(OwnershipAllocation);
    }
    if !previous.owner_role.same_content(&next.owner_role) {
        changed.push(OwnerRole);
    }
    if !previous
        .transition_timing
        .same_content(&next.transition_timing)
    {
        changed.push(TransitionTiming);
    }
    if answer_of_objectives_changed(&previous.preservation_goals, &next.preservation_goals) {
        changed.push(PreservationGoals);
    }
    if answer_of_objectives_changed(&previous.avoidances, &next.avoidances) {
        changed.push(Avoidances);
    }
    if previous.additional_context != next.additional_context {
        changed.push(AdditionalContext);
    }
    changed
}

/// Whether two objective-list answers differ by content.
fn answer_of_objectives_changed<T: PartialEq>(
    previous: &Answer<Vec<Objective<T>>>,
    next: &Answer<Vec<Objective<T>>>,
) -> bool {
    match (previous, next) {
        (Answer::Answered(previous_list), Answer::Answered(next_list)) => {
            previous_list.len() != next_list.len()
                || !previous_list
                    .iter()
                    .zip(next_list.iter())
                    .all(|(previous_item, next_item)| previous_item.same_content(next_item))
        }
        _ => previous != next,
    }
}

/// A content field that changed in a new destination version.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ChangedField {
    /// The main financial objective (screen 2).
    FinancialObjective,
    /// Cash at closing (screen 3).
    ClosingProceeds,
    /// Future income (screen 4).
    FutureIncome,
    /// Who owns the company (screen 5).
    OwnershipParticipants,
    /// Employee ownership shape (screen 6).
    EmployeeOwnershipShape,
    /// Ownership allocation (screen 7).
    OwnershipAllocation,
    /// Owner role after transition (screen 8).
    OwnerRole,
    /// Transition timing (screen 9).
    TransitionTiming,
    /// Preservation goals (screen 10).
    PreservationGoals,
    /// Avoidances (screen 11).
    Avoidances,
    /// Additional owner context (screen 12).
    AdditionalContext,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    #[test]
    fn blank_content_is_all_unanswered_with_no_designations() {
        let content = DestinationContent::default();
        assert_eq!(content.financial_objective.value, Answer::Unanswered);
        assert_eq!(
            content.owner_role.preference,
            NonnegotiableState::Unspecified
        );
        assert_eq!(content.preservation_goals, Answer::Unanswered);
    }

    #[test]
    fn allocation_totaling_100_is_valid() {
        let mut content = DestinationContent::default();
        content.ownership_allocation.value = Answer::Answered(AllocationChoice::Percentages(vec![
            AllocationShare {
                participant: AllocationParticipant::Employees,
                percent: 60,
            },
            AllocationShare {
                participant: AllocationParticipant::Management,
                percent: 30,
            },
            AllocationShare {
                participant: AllocationParticipant::Other("Family".into()),
                percent: 10,
            },
        ]));
        assert_eq!(validate_content(&content), Ok(()));
    }

    #[test]
    fn allocation_totaling_110_is_surfaced_not_fixed() {
        let mut content = DestinationContent::default();
        content.ownership_allocation.value = Answer::Answered(AllocationChoice::Percentages(vec![
            AllocationShare {
                participant: AllocationParticipant::Employees,
                percent: 60,
            },
            AllocationShare {
                participant: AllocationParticipant::Management,
                percent: 50,
            },
        ]));
        assert_eq!(
            validate_content(&content),
            Err(DestinationError::AllocationDoesNotTotal100 { total: 110 })
        );
    }

    #[test]
    fn diff_names_only_the_changed_fields() {
        let mut next = DestinationContent::default();
        next.owner_role.value = Answer::Answered(OwnerRole::Retired);
        next.transition_timing.value = Answer::NotSure;
        assert_eq!(
            diff(&DestinationContent::default(), &next),
            vec![ChangedField::OwnerRole, ChangedField::TransitionTiming]
        );
    }
}
