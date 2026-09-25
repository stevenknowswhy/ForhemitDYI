// Pure presentation logic for the Destination Builder's reworked anatomy —
// no API calls, no state: draft in, view shapes out. Kept out of the
// .svelte file so tests can exercise the mappings directly. Mirrors the
// role decisionHistory.ts plays for the journey walk.

import type { AnsweredView } from "./types";
import type { DraftState, ScreenDef, ScreenChoice } from "../screens";
import { INCOME_BANDS, INCOME_DURATIONS, PROCEEDS_BANDS, SCREENS } from "../screens";
import type { AnsweredNode } from "./decisionHistory";

/** Builder screens sit in one genuinely linear chain — the rail's single
 *  stage group. (The journey walk's stages don't exist in this hardcoded
 *  walk; grouping by screen here would put one entry under 13 headers.) */
const BUILDER_STAGE = "destination_builder";

const ZERO_TIMESTAMP: AnsweredView["versions"][number]["recorded_at"] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

/** Which builder screens carry a recorded answer right now. The review
 *  screen is the checkpoint, not a decision — never in the rail. */
function screenHasAnswer(screen: ScreenDef, draft: DraftState): boolean {
  switch (screen.id) {
    case "welcome": return draft.welcome !== "";
    case "financial_objective": return draft.financial.choice !== null;
    case "closing_proceeds": return draft.proceeds.mode !== null;
    case "future_income": return draft.income.interest !== null;
    case "ownership_participants": return draft.participants.selected.length > 0;
    case "employee_ownership_shape": return draft.shape.choice !== null;
    case "ownership_allocation": return draft.allocation.mode !== null;
    case "owner_role": return draft.role.choice !== null;
    case "transition_timing": return draft.timing.choice !== null;
    case "preservation_goals": return draft.preservation.selected.length > 0;
    case "avoidances": return draft.avoidances.selected.length > 0;
    case "additional_context": return draft.notes.trim() !== "";
    default: return false; // review
  }
}

function bandLabel(bands: { value: string; label: string }[], value: string): string {
  return bands.find((b) => b.value === value)?.label ?? value;
}

/** One-line summary of what the owner answered on a screen — feeds the
 *  rail's node summary. Undefined screens fall through to the title. */
function screenSummary(screen: ScreenDef, draft: DraftState): string {
  const choiceLabel = (choices: ScreenChoice[], value: string | null, fallback: string): string =>
    value === null ? fallback : choices.find((c) => c.value === value)?.label ?? value;

  switch (screen.id) {
    case "welcome":
      return choiceLabel(screen.choices, draft.welcome, "");
    case "financial_objective":
      return choiceLabel(screen.choices, draft.financial.choice, "I'm not sure");
    case "closing_proceeds":
      return draft.proceeds.mode === "band" && draft.proceeds.band
        ? bandLabel(PROCEEDS_BANDS, draft.proceeds.band)
        : draft.proceeds.mode === "custom"
          ? "Custom range"
          : "I don't know yet";
    case "future_income": {
      if (!draft.income.interest) return "I don't know yet";
      const interest = draft.income.interest === "yes" ? "Yes" : draft.income.interest === "maybe" ? "Maybe" : "No";
      const parts = [interest];
      if (draft.income.amount) parts.push(bandLabel(INCOME_BANDS, draft.income.amount));
      if (draft.income.duration) parts.push(`for ${bandLabel(INCOME_DURATIONS, draft.income.duration)}`);
      return `Income after closing: ${parts.join(", ")}`;
    }
    case "ownership_participants":
      return draft.participants.selected.length > 0
        ? draft.participants.selected
            .map((v) => choiceLabel(screen.choices, v, v))
            .join(", ")
        : "I'm not sure";
    case "employee_ownership_shape":
      return choiceLabel(screen.choices, draft.shape.choice, "I'm not sure");
    case "ownership_allocation":
      return choiceLabel(screen.choices, draft.allocation.mode, "I'm not sure");
    case "owner_role":
      return choiceLabel(screen.choices, draft.role.choice, "I'm not sure");
    case "transition_timing":
      return choiceLabel(screen.choices, draft.timing.choice, "I'm not sure");
    case "preservation_goals":
      return draft.preservation.selected.length > 0
        ? draft.preservation.selected
            .map((v) => choiceLabel(screen.choices, v, v))
            .join(", ")
        : "I'm not sure";
    case "avoidances":
      return draft.avoidances.selected.length > 0
        ? draft.avoidances.selected
            .map((v) => choiceLabel(screen.choices, v, v))
            .join(", ")
        : "Nothing specific";
    case "additional_context": {
      const notes = draft.notes.trim();
      return notes.length > 60 ? `${notes.slice(0, 60)}…` : notes;
    }
    default:
      return "";
  }
}

/** The rail's nodes: every screen with a recorded answer (or already
 *  passed), in walk order. Draft-local — nothing here is persisted until
 *  the final wholesale submit. */
export function builderHistory(draft: DraftState, position: number): AnsweredNode[] {
  const nodes: AnsweredNode[] = [];
  SCREENS.forEach((screen, index) => {
    if (screen.interaction === "review") return;
    const passed = index < position;
    if (!passed && !screenHasAnswer(screen, draft)) return;
    const summary = screenSummary(screen, draft);
    nodes.push({
      node_id: screen.id,
      title: screen.title,
      text: screen.prompt,
      interaction: screen.interaction,
      choices: [],
      decision_layer: null,
      value: { single: summary },
      versions: [
        { version: 1, value: { single: summary }, change_reason: null, recorded_at: ZERO_TIMESTAMP },
      ],
      stage: BUILDER_STAGE,
    });
  });
  return nodes;
}

/** The selected option on screens whose choices carry tradeoffs — null on
 *  interactions without per-option tradeoffs (bands, multi-selects, text,
 *  review). The tradeoff column renders from this alone. */
export function selectedScreenChoice(screen: ScreenDef, draft: DraftState): ScreenChoice | null {
  let value: string | null = null;
  switch (screen.id) {
    case "welcome": value = draft.welcome || null; break;
    case "financial_objective": value = draft.financial.choice; break;
    case "employee_ownership_shape": value = draft.shape.choice; break;
    case "owner_role": value = draft.role.choice; break;
    default: return null;
  }
  if (value === null) return null;
  return screen.choices.find((c) => c.value === value) ?? null;
}
