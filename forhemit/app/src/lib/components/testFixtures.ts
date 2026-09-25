// Shared wire-view fixtures for component tests — minimal shapes matching
// ../types. Not shipped: imported only by *.test.ts files.

import type { AnswerValue, ChoiceView, WireTimestamp } from "../types";
import type { AnsweredNode } from "./decisionHistory";

const RECORDING_TIME: WireTimestamp = [2026, 250, 10, 9, 0, 0, 0, 0, 0];

export function makeVersion(
  version: number,
  value: AnswerValue,
): AnsweredNode["versions"][number] {
  return { version, value, change_reason: null, recorded_at: RECORDING_TIME };
}

export function makeAnswered(fields: {
  node_id: string;
  title: string;
  stage: string;
  value?: AnswerValue;
  choices?: ChoiceView[];
  versions?: AnsweredNode["versions"];
}): AnsweredNode {
  const value = fields.value ?? { single: "value_a" };
  return {
    text: `The text of ${fields.title}`,
    interaction: "single_select",
    decision_layer: null,
    choices: fields.choices ?? [],
    value,
    versions: fields.versions ?? [makeVersion(1, value)],
    ...fields,
  };
}

export const CASH_CHOICES: ChoiceView[] = [
  { value: "cash", label: "Get substantial cash at closing" },
  { value: "income", label: "Create income over time" },
];
