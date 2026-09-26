// Component tests for the reworked journey walk anatomy: question-first
// screens, collapsed disclosures, single-select auto-advance, the tradeoff
// column, and the clickable decision history. The api layer is mocked —
// these tests assert what the walk calls, not what the engine does.

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { mount, unmount, flushSync } from "svelte";
import JourneyWalk from "./JourneyWalk.svelte";
import { api } from "../api";
import type { AnsweredView, JourneyView, QuestionView } from "../types";

vi.mock("../api", () => ({
  api: {
    journeyState: vi.fn(),
    journeyStart: vi.fn(),
    journeyRecord: vi.fn(),
    journeyRevise: vi.fn(),
    journeySkip: vi.fn(),
  },
}));

const mounted: { app: ReturnType<typeof mount>; container: HTMLElement }[] = [];

function makeQuestion(fields: Partial<QuestionView> & { node_id: string; title: string }): QuestionView {
  return {
    text: `The text of ${fields.title}`,
    why_we_ask: null,
    body: null,
    interaction: "single_select",
    choices: [],
    required: true,
    decision_layer: "owner_objective",
    stage: "primary_objective",
    is_current: true,
    current_value: null,
    ...fields,
  };
}

function makeAnswered(fields: Partial<AnsweredView> & { node_id: string; title: string }): AnsweredView {
  const value = fields.value ?? { single: "cash" };
  return {
    text: `The text of ${fields.title}`,
    interaction: "single_select",
    choices: [],
    decision_layer: "owner_objective",
    stage: "primary_objective",
    revisable: true,
    value,
    versions: [{ version: 1, value, change_reason: null, recorded_at: "2026-09-25T10:00:00Z" }],
    ...fields,
  };
}

function baseJourney(current: QuestionView | null, extra: Partial<JourneyView> = {}): JourneyView {
  return {
    instance_id: "inst_test",
    journey_title: "Employee Ownership Journey",
    journey_version: "0.2",
    status: "in_progress",
    current,
    screens_to_show: [],
    outro_screens: [],
    answered: [],
    skipped: [],
    skipped_nodes: [],
    nonnegotiables: [],
    progress: { done: 0, total: 24 },
    ...extra,
  };
}

function flushLoad(): Promise<void> {
  return Promise.resolve().then(async () => {
    await Promise.resolve();
    flushSync();
  });
}

function render(): HTMLElement {
  const container = document.createElement("div");
  document.body.appendChild(container);
  const app = mount(JourneyWalk, { target: container, props: { onSnapshot: () => {} } });
  mounted.push({ app, container });
  flushSync();
  return container;
}

function choiceButtons(container: HTMLElement): HTMLButtonElement[] {
  return [...container.querySelectorAll<HTMLButtonElement>("button.choice")];
}

function primaryButton(container: HTMLElement): HTMLButtonElement | undefined {
  return [...container.querySelectorAll<HTMLButtonElement>(".nav button.primary")]
    .find((button) => button.textContent?.includes("Record answer") || button.textContent?.includes("Save new answer version"));
}

const SINGLE_QUESTION = makeQuestion({
  node_id: "primary_objective",
  title: "What are you hoping to accomplish?",
  text: "Pick the outcome you want most.",
  why_we_ask: "Your selection establishes the initial Owner Objective.",
  body: "Longer explanation of the owner objective layer.",
  choices: [
    {
      value: "cash",
      label: "Get substantial cash at closing",
      pros: ["Cash now — simplest to reinvest"],
      cons: ["Future upside if the company keeps growing is given up"],
    },
    { value: "income", label: "Create income over time" },
  ],
});

const MULTI_QUESTION = makeQuestion({
  node_id: "secondary_objectives",
  title: "What else matters?",
  text: "Pick every objective that matters.",
  interaction: "multi_select",
  choices: [
    { value: "retire", label: "Retire completely", pros: ["Freedom"], cons: ["Less involvement"] },
    { value: "jobs", label: "Preserve jobs" },
  ],
});

const RANK_QUESTION = makeQuestion({
  node_id: "secondary_ranking",
  title: "Rank what matters most.",
  text: "Rank the objectives.",
  interaction: "ranking",
  choices: [
    { value: "retire", label: "Retire completely" },
    { value: "jobs", label: "Preserve jobs" },
  ],
});

const NUMERIC_QUESTION = makeQuestion({
  node_id: "proceeds_target",
  title: "What proceeds do you need?",
  text: "Enter an amount.",
  interaction: "numeric",
  choices: [],
});

beforeEach(() => {
  vi.useFakeTimers();
  vi.mocked(api.journeyState).mockResolvedValue(baseJourney(SINGLE_QUESTION));
  vi.mocked(api.journeyRecord).mockResolvedValue(baseJourney(null, { status: "completed" }));
  vi.mocked(api.journeyRevise).mockResolvedValue(baseJourney(null, { status: "completed" }));
  vi.mocked(api.journeySkip).mockResolvedValue(baseJourney(null, { status: "completed" }));
  window.innerWidth = 1024;
});

afterEach(() => {
  while (mounted.length) {
    const { app, container } = mounted.pop()!;
    unmount(app);
    container.remove();
  }
  vi.useRealTimers();
  vi.clearAllMocks();
});

describe("screen anatomy", () => {
  it("renders the question title as the leading element with disclosures closed by default", async () => {
    const container = render();
    await flushLoad();

    const why = container.querySelector<HTMLDetailsElement>("details.why-disclosure");
    expect(why).not.toBeNull();
    expect(why?.open).toBe(false);
    // Both explanation sources live behind the one disclosure.
    expect(container.querySelector("details.why-disclosure")?.textContent).toContain(
      "Your selection establishes the initial Owner Objective.",
    );
    expect(container.querySelector("details.why-disclosure")?.textContent).toContain(
      "Longer explanation of the owner objective layer.",
    );
  });

  it("collapses content screens to a closed one-line Read more disclosure", async () => {
    vi.mocked(api.journeyState).mockResolvedValue(
      baseJourney(SINGLE_QUESTION, {
        screens_to_show: [
          { node_id: "welcome", title: "Welcome to the walk", body: "A long welcome paragraph.", stage: "welcome" },
        ],
      }),
    );
    const container = render();
    await flushLoad();

    const screen = container.querySelector<HTMLDetailsElement>("details.content-screen");
    expect(screen).not.toBeNull();
    expect(screen?.open).toBe(false);
    expect(screen?.querySelector("summary")?.textContent).toContain("Welcome to the walk");
    expect(screen?.querySelector("summary")?.textContent).toContain("Read more");
  });
});

describe("single-select auto-advance", () => {
  it("advances exactly once, after the settle, when an option is selected", async () => {
    const container = render();
    await flushLoad();

    choiceButtons(container)[0].click();
    flushSync();

    // Within the settle window nothing has fired yet.
    vi.advanceTimersByTime(599);
    expect(api.journeyRecord).not.toHaveBeenCalled();

    vi.advanceTimersByTime(1);
    expect(api.journeyRecord).toHaveBeenCalledTimes(1);
    expect(api.journeyRecord).toHaveBeenCalledWith("primary_objective", { single: "cash" });

    // Exactly once — no further fires.
    vi.advanceTimersByTime(2000);
    expect(api.journeyRecord).toHaveBeenCalledTimes(1);
  });

  it("never auto-advances on multi-select, ranking, or numeric questions", async () => {
    // Multi: clicking options never fires — only the explicit Record does.
    vi.mocked(api.journeyState).mockResolvedValue(baseJourney(MULTI_QUESTION));
    let container = render();
    await flushLoad();
    for (const button of choiceButtons(container)) button.click();
    flushSync();
    vi.advanceTimersByTime(5000);
    expect(api.journeyRecord).not.toHaveBeenCalled();
    primaryButton(container)!.click();
    flushSync();
    await flushLoad();
    expect(api.journeyRecord).toHaveBeenCalledTimes(1);
    unmount(mounted.pop()!.app);
    container.remove();
    vi.mocked(api.journeyRecord).mockClear();

    // Ranking: same — explicit Record only.
    vi.mocked(api.journeyState).mockResolvedValue(baseJourney(RANK_QUESTION));
    container = render();
    await flushLoad();
    choiceButtons(container)[0].click();
    flushSync();
    vi.advanceTimersByTime(5000);
    expect(api.journeyRecord).not.toHaveBeenCalled();
    primaryButton(container)!.click();
    flushSync();
    await flushLoad();
    expect(api.journeyRecord).toHaveBeenCalledTimes(1);
    unmount(mounted.pop()!.app);
    container.remove();
    vi.mocked(api.journeyRecord).mockClear();

    // Numeric: the input plus its Record button; no timer-driven advance.
    vi.mocked(api.journeyState).mockResolvedValue(baseJourney(NUMERIC_QUESTION));
    container = render();
    await flushLoad();
    vi.advanceTimersByTime(5000);
    expect(api.journeyRecord).not.toHaveBeenCalled();
    const input = container.querySelector<HTMLInputElement>("input[type=number]");
    expect(input).not.toBeNull();
    input!.value = "500000";
    input!.dispatchEvent(new Event("input"));
    flushSync();
    primaryButton(container)!.click();
    flushSync();
    await flushLoad();
    expect(api.journeyRecord).toHaveBeenCalledTimes(1);
    expect(api.journeyRecord).toHaveBeenCalledWith("proceeds_target", { amount: 500000 });
  });
});

describe("tradeoff column", () => {
  it("shows the selected option's pros and cons, updating with the selection", async () => {
    const container = render();
    await flushLoad();

    choiceButtons(container)[0].click();
    flushSync();
    expect(container.textContent).toContain("What this choice gains");
    expect(container.textContent).toContain("Cash now — simplest to reinvest");
    expect(container.textContent).toContain("What it gives up");
    expect(container.textContent).toContain("Future upside if the company keeps growing is given up");

    // An option without tradeoffs renders no column.
    choiceButtons(container)[1].click();
    flushSync();
    expect(container.querySelector(".pros-cons")).toBeNull();
  });
});

describe("decision history click-back", () => {
  const history = baseJourney(SINGLE_QUESTION, {
    answered: [
      // A real answered view carries the question's own choices — the
      // revise view re-asks THIS node, so its options must render.
      makeAnswered({
        node_id: "transition_timing",
        title: "When do you want to transition?",
        text: "Pick the horizon you want.",
        stage: "timing",
        choices: SINGLE_QUESTION.choices,
      }),
    ],
    progress: { done: 1, total: 24 },
  });

  function clickRailNode(container: HTMLElement) {
    const node = [...container.querySelectorAll<HTMLButtonElement>("button.node")]
      .find((button) => button.textContent?.includes("When do you want to transition?"));
    expect(node).toBeDefined();
    node!.click();
    flushSync();
  }

  it("reopens the decision through the revise path and saves with a reason", async () => {
    vi.mocked(api.journeyState).mockResolvedValue(history);
    const container = render();
    await flushLoad();

    clickRailNode(container);
    // The change-reason banner shows; the revise view re-asks the node.
    expect(container.querySelector(".revise-banner")).not.toBeNull();
    expect(container.querySelector(".revise-banner")?.textContent).toContain("When do you want to transition?");

    choiceButtons(container)[0].click();
    flushSync();
    const save = [...container.querySelectorAll<HTMLButtonElement>(".nav button.primary")]
      .find((button) => button.textContent?.includes("Save new answer version"));
    save!.click();
    flushSync();

    expect(api.journeyRevise).toHaveBeenCalledTimes(1);
    expect(api.journeyRevise).toHaveBeenCalledWith(
      "transition_timing",
      { single: "cash" },
      "priorities_changed",
    );
    expect(api.journeyRecord).not.toHaveBeenCalled();
  });

  it("cancels a revision writing nothing", async () => {
    vi.mocked(api.journeyState).mockResolvedValue(history);
    const container = render();
    await flushLoad();

    clickRailNode(container);
    expect(container.querySelector(".revise-banner")).not.toBeNull();

    const cancel = [...container.querySelectorAll<HTMLButtonElement>(".nav button.secondary")]
      .find((button) => button.textContent?.includes("Cancel revision"));
    expect(cancel).toBeDefined();
    cancel!.click();
    flushSync();

    expect(api.journeyRevise).not.toHaveBeenCalled();
    expect(api.journeyRecord).not.toHaveBeenCalled();
    expect(api.journeySkip).not.toHaveBeenCalled();
    expect(container.querySelector(".revise-banner")).toBeNull();
  });

  it("renders an answered node the engine can no longer revise read-only, never clickable", async () => {
    vi.mocked(api.journeyState).mockResolvedValue(
      baseJourney(SINGLE_QUESTION, {
        answered: [
          makeAnswered({
            node_id: "secondary_ranking",
            title: "Rank what matters most.",
            stage: "prioritize",
            revisable: false,
          }),
        ],
        progress: { done: 1, total: 24 },
      }),
    );
    const container = render();
    await flushLoad();

    expect(container.textContent).toContain("no longer applicable");
    const buttons = [...container.querySelectorAll("button.node")];
    expect(buttons.some((button) => button.textContent?.includes("Rank what matters most."))).toBe(false);
    // Clicking it is impossible — nothing reopened.
    expect(container.querySelector(".revise-banner")).toBeNull();
  });
});
