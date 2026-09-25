// Component tests for the decision history rail — fixture views, no API.
// Narrow-width collapse is exercised through window.innerWidth, which the
// component measures on mount and on resize.

import { describe, it, expect, vi, afterEach } from "vitest";
import { mount, unmount, flushSync } from "svelte";
import DecisionHistory from "./DecisionHistory.svelte";
import { groupDecisions, stageLabel } from "./decisionHistory";
import { CASH_CHOICES, makeAnswered, makeVersion } from "./testFixtures";

const mounted: { app: ReturnType<typeof mount>; container: HTMLElement }[] = [];

type Props = {
  answered: ReturnType<typeof makeAnswered>[];
  skipped?: (string | { node_id: string; title: string; stage: string })[];
  revisingNodeId?: string | null;
  onSelect: (nodeId: string) => void;
};

function render(props: Props): HTMLElement {
  const container = document.createElement("div");
  document.body.appendChild(container);
  const app = mount(DecisionHistory, { target: container, props });
  mounted.push({ app, container });
  flushSync();
  return container;
}

afterEach(() => {
  window.innerWidth = 1024;
  while (mounted.length) {
    const { app, container } = mounted.pop()!;
    unmount(app);
    container.remove();
  }
});

function standardHistory() {
  return [
    makeAnswered({
      node_id: "transition_timing",
      title: "When do you want to transition?",
      stage: "timing",
      choices: CASH_CHOICES,
      value: { single: "cash" },
      versions: [makeVersion(1, { single: "cash" }), makeVersion(2, { single: "income" })],
    }),
    makeAnswered({
      node_id: "team_ready",
      title: "Is your team ready?",
      stage: "team_readiness",
      choices: CASH_CHOICES,
      value: { single: "income" },
    }),
  ];
}

// The rail's pure grouping logic — lives in ./decisionHistory so it can be
// tested without mounting. (Kept in this file rather than a sibling test
// file: a `decisionHistory.test.ts` would collide by case alone with
// `DecisionHistory.test.ts`, which breaks checkouts on case-insensitive
// filesystems.)
describe("stageLabel", () => {
  it("humanizes stage keys", () => {
    expect(stageLabel("timing")).toBe("Timing");
    expect(stageLabel("team_readiness")).toBe("Team readiness");
  });

  it("labels the empty stage as the Skipped fallback group", () => {
    expect(stageLabel("")).toBe("Skipped");
  });
});

describe("groupDecisions", () => {
  it("groups answered nodes by stage in first-appearance order", () => {
    const answered = [
      makeAnswered({ node_id: "n1", title: "Timing", stage: "timing" }),
      makeAnswered({ node_id: "n2", title: "Objective", stage: "primary_objective" }),
      makeAnswered({ node_id: "n3", title: "Horizon", stage: "timing" }),
    ];

    const groups = groupDecisions(answered, []);
    expect(groups.map((g) => g.stage)).toEqual(["timing", "primary_objective"]);
    expect(groups[0].entries.map((e) => e.nodeId)).toEqual(["n1", "n3"]);
    expect(groups[1].entries.map((e) => e.nodeId)).toEqual(["n2"]);
  });

  it("summarizes answers as choice labels, not raw values, with version counts", () => {
    const answered = [
      makeAnswered({
        node_id: "objective",
        title: "What are you hoping to accomplish?",
        stage: "primary_objective",
        choices: CASH_CHOICES,
        value: { single: "cash" },
      }),
      makeAnswered({
        node_id: "timing",
        title: "Timing",
        stage: "timing",
        versions: [makeVersion(1, { single: "cash" }), makeVersion(2, { single: "income" })],
      }),
    ];

    const [objective, timing] = groupDecisions(answered, []);
    expect(objective.entries[0].summary).toBe("Get substantial cash at closing");
    expect(objective.entries[0].versions).toBe(1);
    expect(objective.entries[0].skipped).toBe(false);
    expect(timing.entries[0].versions).toBe(2);
  });

  it("groups titled skipped steps under their real stage, labeled skipped", () => {
    const groups = groupDecisions([], [
      { node_id: "team_readiness", title: "Is your team ready?", stage: "team_readiness" },
    ]);
    expect(groups.map((g) => g.stage)).toEqual(["team_readiness"]);
    expect(groups[0].label).toBe("Team readiness");
    expect(groups[0].entries[0]).toMatchObject({
      nodeId: "team_readiness",
      title: "Is your team ready?",
      skipped: true,
      summary: "",
      versions: 0,
    });
  });

  it("joins titled skipped steps into an existing stage group", () => {
    const answered = [makeAnswered({ node_id: "n1", title: "Timing", stage: "timing" })];
    const groups = groupDecisions(answered, [
      { node_id: "horizon", title: "Horizon", stage: "timing" },
    ]);
    expect(groups).toHaveLength(1);
    expect(groups[0].entries.map((e) => e.nodeId)).toEqual(["n1", "horizon"]);
    expect(groups[0].entries[1].skipped).toBe(true);
  });

  it("lands bare skipped ids in a final Skipped group showing the raw id", () => {
    const answered = [makeAnswered({ node_id: "n1", title: "Timing", stage: "timing" })];
    const groups = groupDecisions(answered, ["legacy_node_id"]);
    expect(groups.map((g) => g.stage)).toEqual(["timing", ""]);
    expect(groups[1].label).toBe("Skipped");
    expect(groups[1].entries[0]).toMatchObject({
      nodeId: "legacy_node_id",
      title: "legacy_node_id",
      skipped: true,
    });
  });
});

describe("DecisionHistory", () => {
  it("renders nothing when there is no history", () => {
    expect(render({ answered: [], skipped: [], onSelect: vi.fn() }).textContent?.trim()).toBe("");
  });

  it("groups entries by stage in first-appearance order with humanized labels", () => {
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });
    const stageLabels = [...container.querySelectorAll(".stage")].map((s) => s.textContent);
    expect(stageLabels).toEqual(["Timing", "Team readiness"]);
  });

  it("shows the answer as the choice label, not the raw value", () => {
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });
    expect(container.textContent).toContain("Get substantial cash at closing");
    expect(container.textContent).not.toContain("value_a");
  });

  it("shows a version count only when an answer has more than one version", () => {
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });
    expect(container.textContent).toContain("2 versions");
    expect(container.textContent).not.toContain("1 versions");
  });

  it("labels skipped nodes with their real titles under their stage", () => {
    const container = render({
      answered: standardHistory(),
      skipped: [{ node_id: "team_ready_followup", title: "Who leads after you?", stage: "team_readiness" }],
      onSelect: vi.fn(),
    });
    expect(container.textContent).toContain("Who leads after you?");
    expect(container.textContent).toContain("skipped");
    // Skipped steps are never clickable.
    const buttons = [...container.querySelectorAll("button.node")];
    expect(buttons.some((b) => b.textContent?.includes("Who leads after you?"))).toBe(false);
  });

  it("lands bare skipped ids in a final Skipped group showing the raw id", () => {
    const container = render({ answered: standardHistory(), skipped: ["legacy_node_id"], onSelect: vi.fn() });
    const stageLabels = [...container.querySelectorAll(".stage")].map((s) => s.textContent);
    expect(stageLabels[stageLabels.length - 1]).toBe("Skipped");
    expect(container.textContent).toContain("legacy_node_id");
  });

  it("emits the clicked node's id", () => {
    const onSelect = vi.fn();
    const container = render({ answered: standardHistory(), onSelect });
    const buttons = container.querySelectorAll<HTMLButtonElement>("button.node");
    expect(buttons).toHaveLength(2);
    buttons[1].click();
    flushSync();
    expect(onSelect).toHaveBeenCalledTimes(1);
    expect(onSelect).toHaveBeenCalledWith("team_ready");
  });

  it("renders the full rail at wide widths", () => {
    window.innerWidth = 1024;
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });
    expect(container.querySelector(".rail")).not.toBeNull();
    expect(container.querySelector(".pill")).toBeNull();
    expect(container.textContent).toContain("2 decisions made — click any to revisit");
  });

  it("collapses to a pill below the narrow width and re-expands on click", () => {
    window.innerWidth = 400;
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });

    const pill = container.querySelector<HTMLButtonElement>(".pill");
    expect(pill).not.toBeNull();
    expect(pill?.textContent).toContain("2 decisions made");
    expect(container.querySelector(".rail")).toBeNull();

    pill?.click();
    flushSync();
    expect(container.querySelector(".rail")).not.toBeNull();
    expect(container.textContent).toContain("Get substantial cash at closing");
  });

  it("responds to a resize across the breakpoint", () => {
    window.innerWidth = 1024;
    const container = render({ answered: standardHistory(), onSelect: vi.fn() });
    expect(container.querySelector(".rail")).not.toBeNull();

    window.innerWidth = 400;
    window.dispatchEvent(new Event("resize"));
    flushSync();
    expect(container.querySelector(".pill")).not.toBeNull();
    expect(container.querySelector(".rail")).toBeNull();
  });

  it("highlights the revising node and auto-expands the rail on narrow screens", () => {
    window.innerWidth = 400;
    const container = render({
      answered: standardHistory(),
      revisingNodeId: "transition_timing",
      onSelect: vi.fn(),
    });
    // The pill is bypassed so the highlight is visible without an extra click.
    expect(container.querySelector(".rail")).not.toBeNull();
    const revising = container.querySelector<HTMLButtonElement>("button.node.revising");
    expect(revising).not.toBeNull();
    expect(revising?.textContent).toContain("When do you want to transition?");
    expect(revising?.textContent).toContain("revising");
  });
});
