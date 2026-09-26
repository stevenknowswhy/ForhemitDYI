// Pure presentation logic for the DecisionHistory rail — no API calls, no
// state: shapes in, shapes out. Kept out of the .svelte file so tests can
// exercise grouping directly.

import type { AnsweredView, SkippedNodeView } from "../types";
import { renderAnswerValue } from "../screens";

/** An answered node plus its journey stage — the one additive field the
 * shell's `answered_view` must supply for stage grouping (the Rust view
 * builds it from the definition, exactly as for SkippedNodeView). */
export type AnsweredNode = AnsweredView & { stage: string };

/** One row in the history rail: an answer (clickable — it can be revised)
 * or a skipped step (labeled, never clickable — nothing is recorded to
 * revise). */
export interface HistoryEntry {
  nodeId: string;
  title: string;
  /** Display summary of the recorded answer; empty for skipped steps. */
  summary: string;
  /** Recorded answer versions; the engine appends, never rewrites. */
  versions: number;
  skipped: boolean;
  /** Whether the engine would accept a revision of this answer right
   * now — false renders the entry read-only ("no longer applicable"),
   * never clickable. Skipped steps carry false too: nothing is recorded
   * to revise. */
  revisable: boolean;
}

/** Decisions grouped by journey stage, stages in first-appearance order —
 * the honest shape of a linear-with-conditionals walk. Skipped steps whose
 * stage is unknown (bare node ids from the current wire form) land in a
 * final "Skipped" group. */
export interface StageGroup {
  stage: string;
  label: string;
  entries: HistoryEntry[];
}

/** Human label for a stage key ("team_readiness" → "Team readiness"); the
 * empty stage key is the fallback group for unlabeled skipped steps. */
export function stageLabel(stage: string): string {
  if (!stage) return "Skipped";
  const spaced = stage.replace(/_/g, " ");
  return spaced.charAt(0).toUpperCase() + spaced.slice(1);
}

export function groupDecisions(
  answered: AnsweredNode[],
  skipped: (string | SkippedNodeView)[],
): StageGroup[] {
  const groups: StageGroup[] = [];
  const byStage = new Map<string, StageGroup>();
  const groupFor = (stage: string): StageGroup => {
    let group = byStage.get(stage);
    if (!group) {
      group = { stage, label: stageLabel(stage), entries: [] };
      byStage.set(stage, group);
      groups.push(group);
    }
    return group;
  };

  for (const answer of answered) {
    groupFor(answer.stage).entries.push({
      nodeId: answer.node_id,
      title: answer.title,
      summary: renderAnswerValue(answer.value, answer.choices),
      versions: answer.versions.length,
      skipped: false,
      revisable: answer.revisable,
    });
  }

  for (const node of skipped) {
    if (typeof node === "string") {
      // Bare ids (today's wire form) carry no stage or title — show the id
      // honestly rather than inventing a label.
      groupFor("").entries.push({
        nodeId: node,
        title: node,
        summary: "",
        versions: 0,
        skipped: true,
        revisable: false,
      });
    } else {
      groupFor(node.stage).entries.push({
        nodeId: node.node_id,
        title: node.title,
        summary: "",
        versions: 0,
        skipped: true,
        revisable: false,
      });
    }
  }

  return groups;
}
