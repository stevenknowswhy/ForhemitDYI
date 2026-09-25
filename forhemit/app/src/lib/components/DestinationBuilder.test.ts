// Component tests for the reworked Destination Builder anatomy: collapsed
// disclosures, the tradeoff column on selection, the decision-history rail
// with draft-preserving jump-to-step, and the unchanged wholesale submit.
// The api module is mocked — the builder is UI-only until the final submit.

import { describe, it, expect, vi, afterEach } from "vitest";
import { mount, unmount, flushSync } from "svelte";
import type { Destination } from "../types";
import DestinationBuilder from "./DestinationBuilder.svelte";

const { destinationCreate, destinationConfirm, destinationCompleteness } = vi.hoisted(() => ({
  destinationCreate: vi.fn(),
  destinationConfirm: vi.fn(),
  destinationCompleteness: vi.fn(),
}));

vi.mock("../api", () => ({
  api: { destinationCreate, destinationConfirm, destinationCompleteness },
}));

function makeDestination(): Destination {
  return {
    destination_id: "dest_test",
    workspace_id: "ws_test",
    status: "draft",
    created_at: [0, 0, 0, 0, 0, 0, 0, 0, 0],
    created_by: { actor_id: "owner", display_name: "Owner" },
    versions: [],
  };
}

const mounted: { app: ReturnType<typeof mount>; container: HTMLElement }[] = [];

function render(seed?: Destination | null): HTMLElement {
  const container = document.createElement("div");
  document.body.appendChild(container);
  const app = mount(DestinationBuilder, {
    target: container,
    props: {
      seed: seed ?? null,
      onDone: (_: Destination) => {},
      onHome: () => {},
    },
  });
  mounted.push({ app, container });
  flushSync();
  return container;
}

afterEach(() => {
  while (mounted.length) {
    const { app, container } = mounted.pop()!;
    unmount(app);
    container.remove();
  }
  vi.clearAllMocks();
});

function buttonWithText(container: HTMLElement, text: string): HTMLButtonElement {
  const found = [...container.querySelectorAll("button")].find(
    (b) => b.textContent?.trim() === text,
  );
  if (!found) throw new Error(`button not found: ${text}`);
  return found;
}

function heading(container: HTMLElement): string {
  return container.querySelector("h2")?.textContent ?? "";
}

// The settle timer (~600ms) that lets the pros/cons column register before
// the walk auto-advances.
async function settle() {
  await new Promise((r) => setTimeout(r, 700));
  flushSync();
}

describe("DestinationBuilder anatomy", () => {
  it("renders the why disclosure closed by default", () => {
    const container = render();
    const details = container.querySelector("details.why-disclosure");
    expect(details).not.toBeNull();
    expect((details as HTMLDetailsElement).open).toBe(false);
  });

  it("collapses the welcome screen to essentials", () => {
    const container = render();
    expect(heading(container)).toContain("Your destination");
    // One short prompt line + the two start choices — no three-line prose.
    expect(container.querySelectorAll(".prompt").length).toBe(1);
    expect(container.querySelectorAll(".choices .choice").length).toBe(2);
  });

  it("shows the tradeoff column for the selected choice, then auto-advances", async () => {
    const container = render();
    buttonWithText(container, "Start building my destination").click();
    flushSync();
    await settle();
    expect(heading(container)).toContain("The main financial outcome");

    buttonWithText(container, "I want substantial cash at closing").click();
    flushSync();
    // The pros/cons moment — the column appears with the choice's copy.
    expect(container.textContent).toContain("What this choice gains");
    expect(container.textContent).toContain("What it gives up");
    expect(container.textContent).toContain("Future upside if the company keeps growing is given up");

    await settle();
    expect(heading(container)).toContain("Cash at closing");
  });

  it("keeps no rail before any decision is made", () => {
    const container = render();
    expect(container.querySelector("aside.rail")).toBeNull();
  });

  it("jumping to a past decision preserves the draft built so far", async () => {
    const container = render();
    buttonWithText(container, "Start building my destination").click();
    await settle();
    buttonWithText(container, "I want substantial cash at closing").click();
    await settle();
    expect(heading(container)).toContain("Cash at closing");

    // Click the welcome node in the history rail.
    const railNode = [...container.querySelectorAll("button.node")].find((n) =>
      n.textContent?.includes("Your destination"),
    );
    expect(railNode).toBeDefined();
    (railNode as HTMLButtonElement).click();
    flushSync();
    expect(heading(container)).toContain("Your destination");

    // Continue forward — the financial answer survived the jump.
    buttonWithText(container, "Continue →").click();
    flushSync();
    expect(heading(container)).toContain("The main financial outcome");
    const cashButton = buttonWithText(container, "I want substantial cash at closing");
    expect(cashButton.className).toContain("selected");
  });

  it("submits wholesale, once, at the end — no per-screen persists", async () => {
    destinationCreate.mockResolvedValue(makeDestination());
    destinationConfirm.mockResolvedValue(makeDestination());
    destinationCompleteness.mockResolvedValue({
      financial: "established",
      ownership: "established",
      personal: "established",
      timing: "established",
      preservation: "established",
      avoidances: "established",
    });

    const container = render();
    const click = (text: string) => {
      buttonWithText(container, text).click();
      flushSync();
    };
    const walk = async (clicks: string[]) => {
      for (const text of clicks) click(text);
      await Promise.resolve();
    };

    await walk(["Start building my destination"]);
    await settle();
    await walk(["I want substantial cash at closing"]);
    await settle();
    await walk(["$500K–$1M", "Continue →"]);
    await walk(["Yes", "Continue →"]);
    await walk(["All employees", "Continue →"]);
    await walk(["Broadly shared among employees", "Continue →"]);
    await walk(["I'm not sure", "Continue →"]);
    await walk(["I'm ready to step away completely", "Continue →"]);
    await walk(["1–3 years", "Continue →"]);
    await walk(["Employees remain with the company", "Continue →"]);
    await walk(["Selling to an outside buyer", "Continue →"]);
    await walk(["Continue →"]); // notes — optional, passed empty
    expect(heading(container)).toContain("Review your destination");

    await walk(["Confirm — this is the outcome I'm trying to create"]);
    await Promise.resolve();

    expect(destinationCreate).toHaveBeenCalledTimes(1);
    const content = destinationCreate.mock.calls[0][0];
    expect(Object.keys(content).sort()).toEqual(
      [
        "additional_context",
        "avoidances",
        "closing_proceeds",
        "employee_ownership_shape",
        "financial_objective",
        "future_income",
        "owner_role",
        "ownership_allocation",
        "ownership_participants",
        "preservation_goals",
        "transition_timing",
      ].sort(),
    );
    expect(content.financial_objective.value).toEqual({ answered: "cash_now" });
    expect(destinationConfirm).toHaveBeenCalledTimes(1);
  });
});
