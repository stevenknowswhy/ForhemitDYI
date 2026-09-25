// Unit tests for the Destination Builder's pure view logic: the history-rail
// mapping (draft → AnsweredNode[]) and the selected-choice lookup the tradeoff
// column renders from. No mounting, no API.

import { describe, it, expect } from "vitest";
import { builderHistory, selectedScreenChoice } from "./builderHistory";
import { SCREENS, freshDraft } from "../screens";

function midWalkDraft() {
  const draft = freshDraft();
  draft.welcome = "start";
  draft.financial.choice = "cash_now";
  return draft;
}

describe("builderHistory", () => {
  it("is empty on a fresh draft at the first screen", () => {
    expect(builderHistory(freshDraft(), 0)).toEqual([]);
  });

  it("lists passed screens in walk order with title and summary", () => {
    const nodes = builderHistory(midWalkDraft(), 2);
    expect(nodes.map((n) => n.node_id)).toEqual(["welcome", "financial_objective"]);
    expect(nodes[0].title).toBe("Your destination");
    expect(nodes[1].value).toEqual({ single: "I want substantial cash at closing" });
  });

  it("keeps answered screens visible after a jump back past them", () => {
    // Jumped back to the welcome screen (position 0) — the financial answer
    // still exists in the draft and must stay clickable in the rail.
    const nodes = builderHistory(midWalkDraft(), 0);
    expect(nodes.map((n) => n.node_id)).toEqual(["welcome", "financial_objective"]);
  });

  it("never includes the review screen — it is a checkpoint, not a decision", () => {
    const draft = midWalkDraft();
    draft.role.choice = "retired";
    const nodes = builderHistory(draft, SCREENS.length - 1);
    expect(nodes.some((n) => n.node_id === "review")).toBe(false);
  });

  it("summarizes proceeds as band label, custom range, or not-yet", () => {
    const draft = freshDraft();
    draft.proceeds.mode = "band";
    draft.proceeds.band = "500k_to_1m";
    expect(builderHistory(draft, 3).find((n) => n.node_id === "closing_proceeds")?.value).toEqual({
      single: "$500K–$1M",
    });

    draft.proceeds.mode = "custom";
    draft.proceeds.band = null;
    expect(builderHistory(draft, 3).find((n) => n.node_id === "closing_proceeds")?.value).toEqual({
      single: "Custom range",
    });

    draft.proceeds.mode = null;
    expect(builderHistory(draft, 3).find((n) => n.node_id === "closing_proceeds")?.value).toEqual({
      single: "I don't know yet",
    });
  });

  it("labels an untouched optional screen passed by Continue as not sure", () => {
    const draft = freshDraft();
    const nodes = builderHistory(draft, 6); // passed shape (index 5) without answering
    expect(nodes.map((n) => n.node_id)).toContain("employee_ownership_shape");
    expect(nodes.find((n) => n.node_id === "employee_ownership_shape")?.value).toEqual({
      single: "I'm not sure",
    });
  });
});

describe("selectedScreenChoice", () => {
  it("returns the selected option with its tradeoff copy", () => {
    const draft = midWalkDraft();
    const screen = SCREENS.find((s) => s.id === "financial_objective")!;
    const choice = selectedScreenChoice(screen, draft);
    expect(choice?.value).toBe("cash_now");
    expect(choice?.pros?.length).toBeGreaterThan(0);
    expect(choice?.cons?.length).toBeGreaterThan(0);
  });

  it("is null when nothing is selected or the screen has no per-option tradeoffs", () => {
    const draft = freshDraft();
    const financial = SCREENS.find((s) => s.id === "financial_objective")!;
    expect(selectedScreenChoice(financial, draft)).toBeNull();

    // Proceeds bands are pure preference bands — no tradeoff column.
    draft.proceeds.mode = "band";
    draft.proceeds.band = "under_500k";
    const proceeds = SCREENS.find((s) => s.id === "closing_proceeds")!;
    expect(selectedScreenChoice(proceeds, draft)).toBeNull();
  });
});
