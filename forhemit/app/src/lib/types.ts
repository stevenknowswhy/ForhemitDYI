// Wire types mirroring the Rust contracts' serde shapes exactly.
// These are the ONLY shapes that cross the command boundary; they mirror
// `forhemit-contracts` and the engine types the views serialize.

export type NonnegotiableState =
  | "unspecified"
  | "preference"
  | "strong_preference"
  | "nonnegotiable";

export type Answer<T> = "unanswered" | "not_sure" | { answered: T };

export interface Objective<T> {
  objective_id: string;
  value: Answer<T>;
  preference: NonnegotiableState;
}

export type FinancialObjective = "cash_now" | "income_over_time" | "combination";

export type ProceedsBand =
  | "under_500k"
  | "500k_to_1m"
  | "1m_to_2m"
  | "2m_to_3m"
  | "3m_to_5m"
  | "5m_to_10m"
  | "over_10m";

export type ProceedsChoice =
  | { band: ProceedsBand }
  | { custom_range: { minimum: number | null; maximum: number | null } };

export type IncomeInterest = "yes" | "maybe" | "no";

export type IncomeBand =
  | "under_25k"
  | "25k_to_50k"
  | "50k_to_100k"
  | "100k_to_250k"
  | "over_250k";

export type IncomeDuration =
  | "1_to_3_years"
  | "3_to_5_years"
  | "5_to_10_years"
  | "over_10_years"
  | "ongoing";

export interface FutureIncomeChoice {
  interest: IncomeInterest;
  amount: Answer<IncomeBand>;
  duration: Answer<IncomeDuration>;
}

export type OwnershipParticipant =
  | { all_employees: null }
  | { management: null }
  | { specific_employee_group: string }
  | { family: null }
  | { existing_owners: null }
  | { outside_investors: null }
  | { other: string };

export type EmployeeOwnershipShape =
  | "broad"
  | "employees_plus_management"
  | "with_other_owners";

export type AllocationParticipant =
  | { employees: null }
  | { management: null }
  | { other: string };

export interface AllocationShare {
  participant: AllocationParticipant;
  percent: number;
}

export type AllocationChoice =
  | { percentages: AllocationShare[] }
  | "participants_only";

export type OwnerRole =
  | "retired"
  | "transition_advisor"
  | "ongoing_advisor"
  | "continuing_owner_operator";

export type TransitionTiming =
  | "within12_months"
  | "1_to_3_years"
  | "3_to_5_years"
  | "more_than_five_years"
  | "flexible";

export type PreservationGoal =
  | { employees_remain: null }
  | { remains_independent: null }
  | { same_location: null }
  | { culture_remains: null }
  | { leadership_remains: null }
  | { brand_remains: null }
  | { family_involved: null }
  | { community_presence_remains: null }
  | { customers_served: null }
  | { other: string };

export interface PreservationSelection {
  goal: PreservationGoal;
  rank: number | null;
}

export type Avoidance =
  | { outside_buyer: null }
  | { losing_employee_ownership: null }
  | { long_term_involvement: null }
  | { leaving_employees_behind: null }
  | { excessive_debt: null }
  | { waiting_many_years_for_proceeds: null }
  | { moving_the_business: null }
  | { losing_independence: null }
  | { major_operational_disruption: null }
  | { other: string };

export interface DestinationContent {
  financial_objective: Objective<FinancialObjective>;
  closing_proceeds: Objective<ProceedsChoice>;
  future_income: Objective<FutureIncomeChoice>;
  ownership_participants: Objective<OwnershipParticipant[]>;
  employee_ownership_shape: Objective<EmployeeOwnershipShape>;
  ownership_allocation: Objective<AllocationChoice>;
  owner_role: Objective<OwnerRole>;
  transition_timing: Objective<TransitionTiming>;
  preservation_goals: Answer<Objective<PreservationSelection>[]>;
  avoidances: Answer<Objective<Avoidance>[]>;
  additional_context: Answer<string>;
}

export type AreaState = "established" | "not_established";

export interface Completeness {
  financial: AreaState;
  ownership: AreaState;
  personal: AreaState;
  timing: AreaState;
  preservation: AreaState;
  avoidances: AreaState;
}

export type ChangeReason =
  | "learned_something_new"
  | "priorities_changed"
  | "professional_suggested_another_approach"
  | "business_changed"
  | "exploring_different_outcome"
  | "other";

export type DestinationStatus =
  | "draft"
  | "working"
  | "confirmed"
  | "under_professional_review"
  | "revised"
  | "superseded"
  | "archived";

export interface DestinationVersion {
  version_id: string;
  destination_id: string;
  version_number: number;
  previous_version_id: string | null;
  created_at: string;
  created_by: { actor_id: string; display_name: string };
  change: {
    reason: ChangeReason;
    explanation: string | null;
    changed_fields: string[];
  } | null;
  content: DestinationContent;
}

export interface Destination {
  destination_id: string;
  workspace_id: string;
  status: DestinationStatus;
  created_at: string;
  created_by: { actor_id: string; display_name: string };
  versions: DestinationVersion[];
}

export interface ChoiceView {
  value: string;
  label: string;
}

export interface QuestionView {
  node_id: string;
  title: string;
  text: string;
  why_we_ask: string | null;
  interaction: string;
  choices: ChoiceView[];
  required: boolean;
  decision_layer: string | null;
  stage: string;
  is_current: boolean;
  current_value: AnswerValue | null;
}

export interface ScreenView {
  node_id: string;
  title: string;
  body: string | null;
  stage: string;
}

export interface AnswerVersionView {
  version: number;
  value: AnswerValue;
  change_reason: string | null;
  recorded_at: string;
}

export interface AnsweredView {
  node_id: string;
  title: string;
  text: string;
  interaction: string;
  choices: ChoiceView[];
  decision_layer: string | null;
  value: AnswerValue;
  versions: AnswerVersionView[];
}

export interface MarkedNonnegotiableView {
  target: string;
  node_id: string;
}

export interface ProgressView {
  done: number;
  total: number;
}

export interface JourneyView {
  instance_id: string;
  journey_title: string;
  journey_version: string;
  status: "in_progress" | "completed";
  current: QuestionView | null;
  screens_to_show: ScreenView[];
  outro_screens: ScreenView[];
  answered: AnsweredView[];
  skipped: string[];
  nonnegotiables: MarkedNonnegotiableView[];
  progress: ProgressView;
}

export type AnswerValue =
  | { single: string }
  | { multi: string[] }
  | { ranking: string[] }
  | { amount: number }
  | "confirmed";

export type FactKind =
  | "industry"
  | "years_operating"
  | "revenue"
  | "operating_cash_flow"
  | "debt"
  | "employee_count"
  | "ownership_structure";

export type FactValue =
  | { text: string }
  | { number: number }
  | { range: { lower: number | null; upper: number | null } };

export interface FactVersion {
  fact_id: string;
  fact_version_id: string;
  kind: FactKind;
  value: FactValue;
  period: string;
  definition: string | null;
  provenance: string;
  verification: string;
  supersedes: string | null;
  recorded_at: string;
}

export interface AuditLogLine {
  at: string;
  event_type: string;
  engine: string;
  object: string;
}

export interface VerifyView {
  verified: boolean;
  event_count: number;
  detail: string;
}
