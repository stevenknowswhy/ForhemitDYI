// The 13 Destination Builder screens as UI definitions — labels and copy
// from `Destination Builder v1.0.md`. This module is presentation data
// only: it builds the `DestinationContent` wire object the engine
// validates; no domain rules live here.

import type {
  AllocationChoice,
  AllocationShare,
  Answer,
  AnswerValue,
  Avoidance,
  DestinationContent,
  EmployeeOwnershipShape,
  FinancialObjective,
  FutureIncomeChoice,
  IncomeBand,
  IncomeDuration,
  IncomeInterest,
  NonnegotiableState,
  Objective,
  OwnerRole,
  OwnershipParticipant,
  PreservationGoal,
  PreservationSelection,
  ProceedsBand,
  ProceedsChoice,
  TransitionTiming,
} from "./types";

export interface ScreenChoice {
  value: string;
  label: string;
  detail?: string;
}

export interface ScreenDef {
  number: number;
  id: string;
  title: string;
  prompt: string;
  why: string;
  interaction:
    | "decision" | "single" | "proceeds" | "income" | "participants"
    | "shape" | "allocation" | "role" | "timing" | "preservation"
    | "avoidances" | "notes" | "review";
  choices: ScreenChoice[];
  optional: boolean;
}

export const SCREENS: ScreenDef[] = [
  {
    number: 1,
    id: "welcome",
    title: "Your destination",
    prompt:
      "Before exploring any path, this walk captures the outcome you want to create — your Destination. " +
      "It is the first object every journey creates, and it stays your North Star. You can change it " +
      "later — every version is kept.",
    why: "Destination-first: the desired outcome drives everything that follows.",
    interaction: "decision",
    choices: [
      {
        value: "start",
        label: "Start building my destination",
        detail: "Walk the screens — answer, skip, or say “I'm not sure” anywhere.",
      },
      {
        value: "not_sure_yet",
        label: "I'm not sure yet — start with fewer constraints",
        detail: "Creates a destination with no initial constraints. Answering nothing is legitimate.",
      },
    ],
    optional: false,
  },
  {
    number: 2,
    id: "financial_objective",
    title: "The main financial outcome",
    prompt: "Thinking about the money side of a transition — which of these sounds most like you?",
    why: "This anchors the financial area of your Desired Outcome.",
    interaction: "single",
    choices: [
      { value: "cash_now", label: "I want substantial cash at closing" },
      { value: "income_over_time", label: "I'm comfortable receiving some of the value over time" },
      { value: "combination", label: "I'd like a mix of cash now and future income" },
    ],
    optional: false,
  },
  {
    number: 3,
    id: "closing_proceeds",
    title: "Cash at closing",
    prompt: "About how much would you ideally like at closing? You can never be blocked here because you don't know your business's value — “I'm not sure” is a real answer.",
    why: "Sets the desired closing proceeds: a band or your own range — never a valuation.",
    interaction: "proceeds",
    choices: [],
    optional: false,
  },
  {
    number: 4,
    id: "future_income",
    title: "Income after closing",
    prompt: "Would you like the business transition to provide income after closing?",
    why: "Sets the desired future income: interest, amount band, and duration.",
    interaction: "income",
    choices: [],
    optional: false,
  },
  {
    number: 5,
    id: "ownership_participants",
    title: "Who should own the company",
    prompt: "Who would you like to have ownership after the transition? Choose all that apply.",
    why: "Names the participant groups your desired outcome includes.",
    interaction: "participants",
    choices: [
      { value: "all_employees", label: "All employees" },
      { value: "management", label: "Management" },
      { value: "family", label: "Family members" },
      { value: "existing_owners", label: "Existing owners" },
      { value: "outside_investors", label: "Outside investors" },
      { value: "specific_group", label: "A specific employee group" },
      { value: "other", label: "Other" },
    ],
    optional: false,
  },
  {
    number: 6,
    id: "employee_ownership_shape",
    title: "How employee ownership would look",
    prompt: "If employees end up owning part of the company — how would you like that to look? These are descriptions of outcomes, not legal structures.",
    why: "Shapes the employee-ownership outcome conceptually.",
    interaction: "shape",
    choices: [
      { value: "broad", label: "Broadly shared among employees" },
      { value: "employees_plus_management", label: "Employees own part, management has additional ownership" },
      { value: "with_other_owners", label: "Employees own part alongside other owners" },
    ],
    optional: true,
  },
  {
    number: 7,
    id: "ownership_allocation",
    title: "Approximate allocation",
    prompt: "Do you have an approximate ownership split in mind? These percentages describe your desired outcome — they are not a proposed legal ownership structure.",
    why: "Records the approximate desired allocation, if you have one.",
    interaction: "allocation",
    choices: [
      { value: "percentages", label: "Set approximate percentages" },
      { value: "participants_only", label: "I only know who should participate" },
    ],
    optional: true,
  },
  {
    number: 8,
    id: "owner_role",
    title: "Your role after the transition",
    prompt: "On the day the transition is complete, what would you like your role to be?",
    why: "Sets the personal outcome — the role you want afterward.",
    interaction: "role",
    choices: [
      { value: "retired", label: "I'm ready to step away completely" },
      { value: "transition_advisor", label: "I'd like to help for a limited period" },
      { value: "ongoing_advisor", label: "I'd like to remain available occasionally" },
      { value: "continuing_owner_operator", label: "I'd like to remain meaningfully involved" },
    ],
    optional: false,
  },
  {
    number: 9,
    id: "transition_timing",
    title: "Timeframe",
    prompt: "When would you ideally like to reach this destination? The system stores a desired timeframe, not a guaranteed closing date.",
    why: "Sets the timing area of your Desired Outcome.",
    interaction: "timing",
    choices: [
      { value: "within12_months", label: "Within 12 months" },
      { value: "1_to_3_years", label: "1–3 years" },
      { value: "3_to_5_years", label: "3–5 years" },
      { value: "more_than_five_years", label: "More than 5 years" },
      { value: "flexible", label: "I'm flexible" },
    ],
    optional: false,
  },
  {
    number: 10,
    id: "preservation_goals",
    title: "What should remain true",
    prompt: "What would you like to remain true about the company after the transition? Pick up to three, and rank which matters most.",
    why: "Each goal can individually be marked nonnegotiable.",
    interaction: "preservation",
    choices: [
      { value: "employees_remain", label: "Employees remain with the company" },
      { value: "remains_independent", label: "Company remains independent" },
      { value: "same_location", label: "Company stays in the same location" },
      { value: "culture_remains", label: "Company culture remains" },
      { value: "leadership_remains", label: "Current leadership remains" },
      { value: "brand_remains", label: "Brand remains" },
      { value: "family_involved", label: "Family remains involved" },
      { value: "community_presence_remains", label: "Community presence remains" },
      { value: "customers_served", label: "Company continues serving existing customers" },
      { value: "other", label: "Other" },
    ],
    optional: true,
  },
  {
    number: 11,
    id: "avoidances",
    title: "What to avoid",
    prompt: "What would you like to avoid? Select any that apply — each can individually be marked nonnegotiable. Selecting nothing means “Nothing specific.”",
    why: "Avoidances become desired outcomes of their own, with their own strength.",
    interaction: "avoidances",
    choices: [
      { value: "outside_buyer", label: "Selling to an outside buyer" },
      { value: "losing_employee_ownership", label: "Losing employee ownership" },
      { value: "long_term_involvement", label: "Remaining involved long-term" },
      { value: "leaving_employees_behind", label: "Leaving employees behind" },
      { value: "excessive_debt", label: "Excessive debt" },
      { value: "waiting_many_years_for_proceeds", label: "Waiting many years for proceeds" },
      { value: "moving_the_business", label: "Moving the business" },
      { value: "losing_independence", label: "Losing company independence" },
      { value: "major_operational_disruption", label: "Major disruption to operations" },
      { value: "other", label: "Other" },
    ],
    optional: true,
  },
  {
    number: 12,
    id: "additional_context",
    title: "Anything else?",
    prompt: "Additional owner context — anything else you want recorded as part of your desired outcome. Optional.",
    why: "Free text, kept verbatim as you wrote it.",
    interaction: "notes",
    choices: [],
    optional: true,
  },
  {
    number: 13,
    id: "review",
    title: "Review your destination",
    prompt:
      "This is the Desired Outcome built from your answers. It becomes the first object in your journey — " +
      "every version is kept, and you can edit later with a recorded reason.",
    why: "The Destination Builder's checkpoint: confirm it, or keep it as a working draft.",
    interaction: "review",
    choices: [],
    optional: false,
  },
];

export const CHANGE_REASONS: { value: string; label: string }[] = [
  { value: "learned_something_new", label: "I learned something new" },
  { value: "priorities_changed", label: "My priorities changed" },
  { value: "professional_suggested_another_approach", label: "My professional suggested another approach" },
  { value: "business_changed", label: "The business changed" },
  { value: "exploring_different_outcome", label: "I want to explore a different outcome" },
  { value: "other", label: "Other" },
];

export const PREFERENCE_OPTIONS: {
  value: NonnegotiableState;
  label: string;
  dot: string;
  detail: string;
}[] = [
  {
    value: "unspecified",
    label: "No designation",
    dot: "·",
    detail: "Default — the system never guesses importance.",
  },
  { value: "preference", label: "I'd like this", dot: "🟢", detail: "Desirable but negotiable." },
  {
    value: "strong_preference",
    label: "Very important to me",
    dot: "🟠",
    detail: "Very important, but potentially changeable.",
  },
  {
    value: "nonnegotiable",
    label: "This is nonnegotiable",
    dot: "🔴",
    detail: "Must-have — never silently relaxed; conflicts surface for your decision.",
  },
];

export const PROCEEDS_BANDS: { value: ProceedsBand; label: string }[] = [
  { value: "under_500k", label: "Under $500K" },
  { value: "500k_to_1m", label: "$500K–$1M" },
  { value: "1m_to_2m", label: "$1M–$2M" },
  { value: "2m_to_3m", label: "$2M–$3M" },
  { value: "3m_to_5m", label: "$3M–$5M" },
  { value: "5m_to_10m", label: "$5M–$10M" },
  { value: "over_10m", label: "$10M+" },
];

export const INCOME_BANDS: { value: IncomeBand; label: string }[] = [
  { value: "under_25k", label: "Under $25K" },
  { value: "25k_to_50k", label: "$25K–$50K" },
  { value: "50k_to_100k", label: "$50K–$100K" },
  { value: "100k_to_250k", label: "$100K–$250K" },
  { value: "over_250k", label: "Over $250K" },
];

export const INCOME_DURATIONS: { value: IncomeDuration; label: string }[] = [
  { value: "1_to_3_years", label: "1–3 years" },
  { value: "3_to_5_years", label: "3–5 years" },
  { value: "5_to_10_years", label: "5–10 years" },
  { value: "over_10_years", label: "More than 10 years" },
  { value: "ongoing", label: "Ongoing" },
];

/** A fresh objective — new stable id, unanswered, no designation. */
export function emptyObjective<T>(): Objective<T> {
  return {
    objective_id: `obj_${crypto.randomUUID()}`,
    value: "unanswered",
    preference: "unspecified",
  };
}

export function objectiveWithValue<T>(value: T): Objective<T> {
  // The wire's Answer<T> is "unanswered" | "not_sure" | { answered: T };
  // call sites pass the substantive value, so wrap it as Answered here.
  return { ...emptyObjective<T>(), value: { answered: value } };
}

export function notSureObjective<T>(): Objective<T> {
  return { ...emptyObjective<T>(), value: "not_sure" };
}

/** UI-shape allocation share — converted to the wire shape at build time. */
export interface AllocationDraft {
  group: "employees" | "management" | "other";
  label: string;
  percent: number;
}

export interface DraftState {
  welcome: string;
  financial: { choice: FinancialObjective | null; pref: NonnegotiableState };
  proceeds: {
    mode: "band" | "custom" | null;
    band: ProceedsBand | null;
    min: number | "";
    max: number | "";
    pref: NonnegotiableState;
  };
  income: {
    interest: IncomeInterest | null;
    amount: IncomeBand | null;
    duration: IncomeDuration | null;
    pref: NonnegotiableState;
  };
  participants: {
    selected: string[];
    specificGroup: string;
    other: string;
    pref: NonnegotiableState;
  };
  shape: { choice: EmployeeOwnershipShape | null; pref: NonnegotiableState };
  allocation: {
    mode: "percentages" | "participants_only" | null;
    shares: AllocationDraft[];
    other: string;
    pref: NonnegotiableState;
  };
  role: { choice: OwnerRole | null; pref: NonnegotiableState };
  timing: { choice: TransitionTiming | null; pref: NonnegotiableState };
  preservation: {
    selected: string[];
    other: string;
    ranks: Record<string, number>;
    prefs: Record<string, NonnegotiableState>;
  };
  avoidances: {
    selected: string[];
    other: string;
    prefs: Record<string, NonnegotiableState>;
  };
  notes: string;
}

export function freshDraft(): DraftState {
  return {
    welcome: "",
    financial: { choice: null, pref: "unspecified" },
    proceeds: { mode: null, band: null, min: "" as number | "", max: "" as number | "", pref: "unspecified" },
    income: { interest: null, amount: null, duration: null, pref: "unspecified" },
    participants: { selected: [], specificGroup: "", other: "", pref: "unspecified" },
    shape: { choice: null, pref: "unspecified" },
    allocation: { mode: null, shares: [], other: "", pref: "unspecified" },
    role: { choice: null, pref: "unspecified" },
    timing: { choice: null, pref: "unspecified" },
    preservation: { selected: [], other: "", ranks: {}, prefs: {} },
    avoidances: { selected: [], other: "", prefs: {} },
    notes: "",
  };
}

function participantWire(value: string, draft: DraftState): OwnershipParticipant | null {
  switch (value) {
    case "all_employees": return "all_employees";
    case "management": return "management";
    case "family": return "family";
    case "existing_owners": return "existing_owners";
    case "outside_investors": return "outside_investors";
    case "specific_group":
      return draft.participants.specificGroup.trim()
        ? { specific_employee_group: draft.participants.specificGroup.trim() }
        : null;
    case "other":
      return draft.participants.other.trim() ? { other: draft.participants.other.trim() } : null;
    default: return null;
  }
}

function goalWire(value: string, other: string): PreservationGoal | null {
  switch (value) {
    case "employees_remain":
    case "remains_independent":
    case "same_location":
    case "culture_remains":
    case "leadership_remains":
    case "brand_remains":
    case "family_involved":
    case "community_presence_remains":
    case "customers_served":
      // Unit variants are plain strings on the wire (verified against the
      // engine's serde; the old {key: null} objects were never what Rust sent).
      return value;
    case "other": return other.trim() ? { other: other.trim() } : null;
    default: return null;
  }
}

function avoidanceWire(value: string, other: string): Avoidance | null {
  switch (value) {
    case "outside_buyer":
    case "losing_employee_ownership":
    case "long_term_involvement":
    case "leaving_employees_behind":
    case "excessive_debt":
    case "waiting_many_years_for_proceeds":
    case "moving_the_business":
    case "losing_independence":
    case "major_operational_disruption":
      return value;
    case "other": return other.trim() ? { other: other.trim() } : null;
    default: return null;
  }
}

/** A wire enum value: unit variants arrive as plain strings, payload
 *  variants ("Other — described") as one-key objects. Maps any value to
 *  its variant key; "" for shapes that are neither. */
function variantKey(value: unknown): string {
  if (typeof value === "string") return value;
  if (typeof value === "object" && value !== null) {
    const keys = Object.keys(value);
    if (keys.length === 1) return keys[0];
  }
  return "";
}

/**
 * Builds the content the engine will validate. Returns an error string
 * for anything the engine would refuse (e.g. shares not totaling 100) —
 * a courtesy pre-check mirroring the engine's rule; the engine remains
 * the validator.
 */
export function buildContent(draft: DraftState): { content: DestinationContent } | { error: string } {
  if (draft.welcome === "not_sure_yet") {
    return {
      content: {
        financial_objective: emptyObjective(),
        closing_proceeds: emptyObjective(),
        future_income: emptyObjective(),
        ownership_participants: emptyObjective(),
        employee_ownership_shape: emptyObjective(),
        ownership_allocation: emptyObjective(),
        owner_role: emptyObjective(),
        transition_timing: emptyObjective(),
        preservation_goals: "unanswered",
        avoidances: "unanswered",
        additional_context: "unanswered",
      },
    };
  }

  if (!draft.financial.choice) {
    return { error: "Screen 2: choose a financial objective (or “I'm not sure”)." };
  }

  let proceeds: Objective<ProceedsChoice>;
  if (draft.proceeds.mode === "band" && draft.proceeds.band) {
    proceeds = objectiveWithValue({ band: draft.proceeds.band });
  } else if (draft.proceeds.mode === "custom") {
    // The bounds bind to number inputs (numbers once filled, "" when empty),
    // so parse tolerantly instead of calling string methods on them.
    const asBound = (value: number | ""): number | null =>
      typeof value === "number" && Number.isFinite(value) ? value : null;
    const min = asBound(draft.proceeds.min);
    const max = asBound(draft.proceeds.max);
    if (min === null && max === null) {
      return { error: "Screen 3: enter at least one bound for your own range, or pick a band." };
    }
    proceeds = objectiveWithValue({ custom_range: { minimum: min, maximum: max } });
  } else {
    proceeds = notSureObjective();
  }
  proceeds = { ...proceeds, preference: draft.proceeds.pref };

  let income: Objective<FutureIncomeChoice>;
  if (draft.income.interest) {
    income = objectiveWithValue({
      interest: draft.income.interest,
      amount: draft.income.amount ? { answered: draft.income.amount } : "not_sure",
      duration: draft.income.duration ? { answered: draft.income.duration } : "not_sure",
    });
  } else {
    income = notSureObjective();
  }
  income = { ...income, preference: draft.income.pref };

  const participants: OwnershipParticipant[] = [];
  for (const value of draft.participants.selected) {
    const wire = participantWire(value, draft);
    if (wire) participants.push(wire);
  }
  const participantsObjective = participants.length > 0
    ? objectiveWithValue<OwnershipParticipant[]>(participants)
    : notSureObjective<OwnershipParticipant[]>();

  const shape = draft.shape.choice
    ? { ...objectiveWithValue(draft.shape.choice), preference: draft.shape.pref }
    : { ...notSureObjective<EmployeeOwnershipShape>(), preference: draft.shape.pref };

  let allocation: Objective<AllocationChoice>;
  if (draft.allocation.mode === "percentages") {
    const shares: AllocationShare[] = draft.allocation.shares
      .filter((s) => s.percent > 0)
      .map((s) => ({
        participant:
          s.group === "employees" ? "employees"
          : s.group === "management" ? "management"
          : { other: s.label.trim() || "other" },
        percent: s.percent,
      }));
    const total = shares.reduce((sum, s) => sum + s.percent, 0);
    if (shares.length === 0) {
      return { error: "Screen 7: add at least one percentage, or pick another option." };
    }
    if (total !== 100) {
      return { error: `Screen 7: percentages total ${total}% — the engine requires exactly 100%.` };
    }
    allocation = objectiveWithValue({ percentages: shares });
  } else if (draft.allocation.mode === "participants_only") {
    allocation = objectiveWithValue("participants_only");
  } else {
    allocation = notSureObjective();
  }
  allocation = { ...allocation, preference: draft.allocation.pref };

  const role = draft.role.choice
    ? { ...objectiveWithValue(draft.role.choice), preference: draft.role.pref }
    : { ...notSureObjective<OwnerRole>(), preference: draft.role.pref };
  const timing = draft.timing.choice
    ? { ...objectiveWithValue(draft.timing.choice), preference: draft.timing.pref }
    : { ...notSureObjective<TransitionTiming>(), preference: draft.timing.pref };

  const goals: Objective<PreservationSelection>[] = [];
  for (const value of draft.preservation.selected) {
    const wire = goalWire(value, draft.preservation.other);
    if (!wire) continue;
    const rank = draft.preservation.ranks[value];
    goals.push({
      ...objectiveWithValue({ goal: wire, rank: rank ?? null }),
      preference: draft.preservation.prefs[value] ?? "unspecified",
    });
  }

  const avoidList: Objective<Avoidance>[] = [];
  for (const value of draft.avoidances.selected) {
    const wire = avoidanceWire(value, draft.avoidances.other);
    if (!wire) continue;
    avoidList.push({
      ...objectiveWithValue(wire),
      preference: draft.avoidances.prefs[value] ?? "unspecified",
    });
  }

  return {
    content: {
      financial_objective: {
        ...objectiveWithValue(draft.financial.choice),
        preference: draft.financial.pref,
      },
      closing_proceeds: proceeds,
      future_income: income,
      ownership_participants: { ...participantsObjective, preference: draft.participants.pref },
      employee_ownership_shape: shape,
      ownership_allocation: allocation,
      owner_role: role,
      transition_timing: timing,
      preservation_goals: goals.length > 0 ? { answered: goals } : "unanswered",
      avoidances: avoidList.length > 0 ? { answered: avoidList } : "unanswered",
      additional_context: draft.notes.trim() ? { answered: draft.notes.trim() } : "unanswered",
    },
  };
}

/** Short human label for a stored answer value, per screen. */
export function describeValue(screenId: string, content: DestinationContent): string {
  const answerText = <T,>(answer: Answer<T>, render: (value: T) => string): string => {
    if (answer === "not_sure") return "I'm not sure";
    if (answer === "unanswered") return "(not answered)";
    return render(answer.answered);
  };
  const bandLabel = (bands: { value: string; label: string }[], value: string): string =>
    bands.find((b) => b.value === value)?.label ?? value;

  switch (screenId) {
    case "financial_objective":
      return answerText(content.financial_objective.value, (v) =>
        v === "cash_now" ? "Substantial cash at closing"
        : v === "income_over_time" ? "Income over time"
        : "A mix of cash now and future income");
    case "closing_proceeds":
      return answerText(content.closing_proceeds.value, (v) =>
        "band" in v ? bandLabel(PROCEEDS_BANDS, v.band)
        : `Custom range: ${v.custom_range.minimum ?? "no minimum"}–${v.custom_range.maximum ?? "no maximum"}`);
    case "future_income":
      return answerText(content.future_income.value, (v) =>
        `Income after closing: ${v.interest}` +
        (v.amount !== "not_sure" && v.amount !== "unanswered"
          ? `, ${bandLabel(INCOME_BANDS, v.amount.answered)}`
          : ", amount not sure") +
        (v.duration !== "not_sure" && v.duration !== "unanswered"
          ? `, for ${bandLabel(INCOME_DURATIONS, v.duration.answered)}`
          : ", duration not sure"));
    case "ownership_participants":
      return answerText(content.ownership_participants.value, (values) =>
        values.map((p) => Object.keys(p)[0].replace(/_/g, " ")).join(", "));
    case "employee_ownership_shape":
      return answerText(content.employee_ownership_shape.value, (v) =>
        v === "broad" ? "Broadly shared among employees"
        : v === "employees_plus_management" ? "Employees plus additional management ownership"
        : "Employees alongside other owners");
    case "ownership_allocation":
      return answerText(content.ownership_allocation.value, (v) =>
        v === "participants_only" ? "Participants only — no percentages"
        : v.percentages.map((s) => `${Object.keys(s.participant)[0].replace(/_/g, " ")} ${s.percent}%`).join(", "));
    case "owner_role":
      return answerText(content.owner_role.value, (v) => v.replace(/_/g, " "));
    case "transition_timing":
      return answerText(content.transition_timing.value, (v) => v.replace(/_/g, " "));
    default:
      return "(see details)";
  }
}

/** Renders one stored answer value for the journey answer list. */
export function renderAnswerValue(
  value: AnswerValue,
  choices: { value: string; label: string }[],
): string {
  if (value === "confirmed") return "Confirmed";
  if ("single" in value) {
    return choices.find((c) => c.value === value.single)?.label ?? value.single;
  }
  if ("multi" in value) {
    return value.multi
      .map((v) => choices.find((c) => c.value === v)?.label ?? v)
      .join(", ");
  }
  if ("ranking" in value) {
    return value.ranking
      .map((v, index) => `${index + 1}. ${choices.find((c) => c.value === v)?.label ?? v}`)
      .join("; ");
  }
  return `$${value.amount.toLocaleString("en-US")}`;
}

/** Human label for a decision layer value from the journey data file. */
export function layerLabel(
  layer: string | null,
): { label: string; kind: "owner" | "platform" | "professional" } | null {
  if (!layer) return null;
  if (layer === "owner_objective") return { label: "Your objective — you said this", kind: "owner" };
  if (layer === "platform_scenario") {
    return { label: "Platform scenario layer — recorded by Forhemit", kind: "platform" };
  }
  if (layer === "professional_determination") {
    return { label: "Professional determination — for your advisor", kind: "professional" };
  }
  return { label: layer.replace(/_/g, " "), kind: "platform" };
}

/**
 * Rebuilds a UI draft from stored content so "edit" opens the builder
 * pre-filled. Best-effort reverse of buildContent: the engine is the
 * source of truth; this only restores what the screens can show.
 */
export function draftFromContent(content: DestinationContent): DraftState {
  const draft = freshDraft();
  const unwrap = <T,>(objective: Objective<T>): T | null => {
    const value = objective.value;
    return typeof value === "object" && value !== null && "answered" in value
      ? value.answered
      : null;
  };

  draft.welcome = "start";

  const financial = unwrap(content.financial_objective);
  if (financial) {
    draft.financial.choice = financial;
    draft.financial.pref = content.financial_objective.preference;
  }

  const proceeds = unwrap(content.closing_proceeds);
  if (proceeds) {
    draft.proceeds.pref = content.closing_proceeds.preference;
    if ("band" in proceeds) {
      draft.proceeds.mode = "band";
      draft.proceeds.band = proceeds.band;
    } else {
      draft.proceeds.mode = "custom";
      draft.proceeds.min = proceeds.custom_range.minimum ?? "";
      draft.proceeds.max = proceeds.custom_range.maximum ?? "";
    }
  }

  const income = unwrap(content.future_income);
  if (income) {
    draft.income.pref = content.future_income.preference;
    draft.income.interest = income.interest;
    if (typeof income.amount === "object" && income.amount !== null && "answered" in income.amount) {
      draft.income.amount = income.amount.answered;
    }
    if (typeof income.duration === "object" && income.duration !== null && "answered" in income.duration) {
      draft.income.duration = income.duration.answered;
    }
  }

  const participants = unwrap(content.ownership_participants);
  if (participants) {
    draft.participants.pref = content.ownership_participants.preference;
    draft.participants.selected = participants
      .map(variantKey)
      .filter((key) =>
        ["all_employees", "management", "family", "existing_owners", "outside_investors"].includes(key),
    );
    const specific = participants.find((p) => variantKey(p) === "specific_employee_group");
    if (specific && typeof specific === "object" && "specific_employee_group" in specific) {
      draft.participants.selected.push("specific_group");
      draft.participants.specificGroup = specific.specific_employee_group;
    }
    const other = participants.find((p) => variantKey(p) === "other");
    if (other && typeof other === "object" && "other" in other) {
      draft.participants.selected.push("other");
      draft.participants.other = other.other;
    }
  }

  const shape = unwrap(content.employee_ownership_shape);
  if (shape) {
    draft.shape.choice = shape;
    draft.shape.pref = content.employee_ownership_shape.preference;
  }

  const allocation = unwrap(content.ownership_allocation);
  if (allocation) {
    draft.allocation.pref = content.ownership_allocation.preference;
    if (allocation === "participants_only") {
      draft.allocation.mode = "participants_only";
    } else {
      draft.allocation.mode = "percentages";
      draft.allocation.shares = allocation.percentages.map((share) => {
        const participant = share.participant;
        if (participant === "employees") return { group: "employees" as const, label: "", percent: share.percent };
        if (participant === "management") return { group: "management" as const, label: "", percent: share.percent };
        return { group: "other" as const, label: participant.other, percent: share.percent };
      });
    }
  }

  const role = unwrap(content.owner_role);
  if (role) {
    draft.role.choice = role;
    draft.role.pref = content.owner_role.preference;
  }

  const timing = unwrap(content.transition_timing);
  if (timing) {
    draft.timing.choice = timing;
    draft.timing.pref = content.transition_timing.preference;
  }

  const goals =
    typeof content.preservation_goals === "object" &&
    content.preservation_goals !== null &&
    "answered" in content.preservation_goals
      ? content.preservation_goals.answered
      : [];
  // The wire Objective carries the selection inside its Answer value;
  // unit-variant goals are strings, the "other" goal a payload object.
  const selections = goals.flatMap((goal) => {
    const value = goal.value;
    return typeof value === "object" && value !== null && "answered" in value
      ? [value.answered]
      : [];
  });
  draft.preservation.selected = selections
    .map((sel) => variantKey(sel.goal))
    .filter((key) => key !== "other");
  for (const goal of goals) {
    const value = goal.value;
    if (typeof value !== "object" || value === null || !("answered" in value)) continue;
    const sel = value.answered;
    const key = variantKey(sel.goal);
    if (sel.rank !== null) draft.preservation.ranks[key] = sel.rank;
    draft.preservation.prefs[key] = goal.preference;
  }
  const goalOther = selections.find((sel) => variantKey(sel.goal) === "other");
  if (goalOther && typeof goalOther.goal === "object" && "other" in goalOther.goal) {
    draft.preservation.selected.push("other");
    draft.preservation.other = goalOther.goal.other;
  }

  const avoidances =
    typeof content.avoidances === "object" && content.avoidances !== null && "answered" in content.avoidances
      ? content.avoidances.answered
      : [];
  const avoidanceList = avoidances.flatMap((a) => {
    const value = a.value;
    return typeof value === "object" && value !== null && "answered" in value
      ? [value.answered]
      : [];
  });
  draft.avoidances.selected = avoidanceList
    .map(variantKey)
    .filter((key) => key !== "" && key !== "other");
  const avoidOther = avoidanceList.find((a) => variantKey(a) === "other");
  if (avoidOther && typeof avoidOther === "object" && "other" in avoidOther) {
    draft.avoidances.selected.push("other");
    draft.avoidances.other = avoidOther.other;
  }
  for (const a of avoidances) {
    const value = a.value;
    if (typeof value !== "object" || value === null || !("answered" in value)) continue;
    const key = variantKey(value.answered);
    if (key === "") continue;
    draft.avoidances.prefs[key] = a.preference;
  }

  const notes = content.additional_context;
  if (typeof notes === "object" && notes !== null && "answered" in notes) {
    draft.notes = notes.answered;
  }

  return draft;
}
