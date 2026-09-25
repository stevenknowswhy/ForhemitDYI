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

// ── Scenario views (views/scenario.rs) ─────────────────────────────────

export interface AssumptionView {
  assumption_id: string;
  category: string;
  name: string;
  description: string | null;
  value_label: string;
  provenance: string;
  verification: string;
  source_reference: string | null;
  created_at: string;
}

export interface UnknownView {
  unknown_id: string;
  category: string | null;
  description: string;
  importance: string;
  resolution_status: string;
  required_action: string | null;
  created_at: string;
}

export interface NonnegotiableUnderTestView {
  nonnegotiable_id: string;
  destination_objective_id: string;
  destination_version_id: string;
  description: string;
}

export interface NonnegotiableConflictView {
  nonnegotiable_id: string;
  destination_objective_id: string;
  destination_version_id: string;
  owner_decision: string | null;
  decided_at: string | null;
}

export interface ConflictView {
  conflict_id: string;
  scenario_version_id: string;
  conflict_type: string;
  severity: string;
  description: string;
  is_nonnegotiable: boolean;
  nonnegotiable: NonnegotiableConflictView | null;
  resolved: boolean;
  resolution_reference: string | null;
  resolved_at: string | null;
  created_at: string;
}

export interface BranchView {
  branch_id: string;
  parent_scenario_version_id: string;
  child_scenario_family_id: string;
  branch_type: string;
  reason: string;
  created_at: string;
}

export interface StatusEntryView {
  previous_lifecycle: string | null;
  new_lifecycle: string;
  previous_readiness: string | null;
  new_readiness: string;
  reason: string | null;
  changed_at: string;
}

export interface ScenarioVersionView {
  scenario_version_id: string;
  family_id: string;
  version_number: number;
  name: string;
  description: string | null;
  change_reason: string | null;
  lifecycle: string;
  readiness: string;
  is_draft: boolean;
  destination_version_id: string;
  business_reality_version_id: string;
  parent_version_id: string | null;
  supersedes_version_id: string | null;
  assumptions: AssumptionView[];
  unknowns: UnknownView[];
  nonnegotiables: NonnegotiableUnderTestView[];
  conflicts: ConflictView[];
  branches: BranchView[];
  status_history: StatusEntryView[];
  finalized_at: string | null;
  created_at: string;
}

export interface ScenarioFamilyView {
  scenario_family_id: string;
  name: string;
  scenario_type: string;
  branched_from_version_id: string | null;
  archived: boolean;
  archive_reason: string | null;
  created_at: string;
  versions: ScenarioVersionView[];
}

export interface FamilyAndVersionView {
  family: ScenarioFamilyView;
  version: ScenarioVersionView;
}

export interface ComparisonDimensionView {
  index: number;
  dimension_type: string;
  label: string;
}

export interface ComparisonCellView {
  scenario_version_id: string;
  dimension_index: number;
  value_label: string | null;
  outcome: string;
}

export interface ComparisonView {
  comparison_id: string;
  name: string;
  scenario_version_ids: string[];
  dimensions: ComparisonDimensionView[];
  cells: ComparisonCellView[];
  created_at: string;
}

// Wire shapes the frontend sends (commands.rs). Values are wire enums —
// snake_case names decoded by the shell into contract enums.

export interface WireValue {
  value_type: "number" | "currency" | "percentage" | "duration_months" | "boolean" | "text";
  value: string;
  currency: string | null;
}

export interface ScenarioCreateWire {
  name: string;
  scenario_type: string;
  description: string | null;
  destination_version_id: string;
  reality_fact_version_ids: string[];
}

export interface AssumptionWire {
  category: string;
  name: string;
  description: string | null;
  value: WireValue;
  provenance: string;
  verification: string;
  nonnegotiable_objective_id: string | null;
  source_reference: string | null;
}

export interface UnknownWire {
  category: string | null;
  description: string;
  importance: string;
  required_action: string | null;
  source_dependency: string | null;
}

export interface ConstraintWire {
  constraint_type: string;
  name: string;
  description: string | null;
  value: WireValue | null;
  source_reference: string | null;
}

export interface NonnegotiableWire {
  destination_objective_id: string;
  destination_version_id: string;
  description: string;
}

export interface ConflictWire {
  conflict_type: string;
  severity: string;
  description: string;
  source_reference: string | null;
  affected_object_type: string | null;
  affected_object_id: string | null;
}

export interface WhatIfWire {
  parent_scenario_version_id: string;
  branch_type: string;
  reason: string;
  changed_assumptions: string[];
  name: string;
}

export interface ComparisonDimensionWire {
  dimension_type: string;
  label: string;
}

export interface ComparisonResultWire {
  scenario_version_id: string;
  dimension_index: number;
  value: WireValue | null;
  outcome: string;
  source_reference: string | null;
}

export interface ComparisonWire {
  name: string;
  scenario_version_ids: string[];
  dimensions: ComparisonDimensionWire[];
  results: ComparisonResultWire[];
}

// ── Vault views (views/vault.rs) ───────────────────────────────────────

export interface VaultStatusView {
  state: "not_set_up" | "locked" | "ready";
  vault_id: string | null;
  document_count: number | null;
  exports_dir: string;
}

export interface VaultDocumentView {
  document_id: string;
  filename: string;
  byte_count: number;
  version_count: number;
  latest_version_id: string;
  latest_created_at: string;
  note: string | null;
}

export interface VaultVersionView {
  version_id: string;
  document_id: string;
  version_number: number;
  filename: string;
  content_hash: string;
  byte_count: number;
  note: string | null;
  restored_from: string | null;
  previous_version_id: string | null;
  created_at: string;
}

export interface VaultSearchHitView {
  document_id: string;
  version_id: string;
  filename: string;
  snippet: string;
}

export interface VaultVersionContentView {
  version_id: string;
  document_id: string;
  filename: string;
  byte_count: number;
  content_base64: string;
  content_text: string | null;
}

// ── Package views (views/package.rs) ───────────────────────────────────

export interface ExcludedScenarioView {
  scenario_version_id: string;
  name: string;
  readiness: string;
}

export interface PackageProvenanceView {
  journey_version_used: string;
  destination_version_number: number;
  business_reality_version_id: string;
  included_scenario_version_ids: string[];
  excluded_scenarios: ExcludedScenarioView[];
  created_at: string;
}

export interface PackageBlock {
  block: "line" | "text" | "entry" | "callout" | "nothing_recorded";
  content:
    | { label: string; value: string }
    | string
    | { title: string; lines: string[] }
    | { label: string; text: string }
    | { context: string };
}

export interface PackageSectionView {
  number: string;
  title: string;
  blocks: PackageBlock[];
}

export interface PackagePreviewView {
  created_at: string;
  provenance: PackageProvenanceView;
  summary: {
    what_i_want: PackageBlock[];
    must_haves: string[];
    strong_preferences: string[];
    wants_to_avoid: string[];
    paths_evaluated: string[];
    main_questions: string[];
  };
  sections: PackageSectionView[];
  disclosure: string;
}

export interface ExportedFileView {
  format: string;
  file_name: string;
  saved_path: string;
  content_base64: string;
}
