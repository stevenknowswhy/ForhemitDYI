// Component tests for the tradeoff column — fixture views, no API.

import { describe, it, expect, afterEach } from "vitest";
import { mount, unmount, flushSync } from "svelte";
import ProsConsColumn from "./ProsConsColumn.svelte";

const mounted: { app: ReturnType<typeof mount>; container: HTMLElement }[] = [];

function render(props: { pros?: string[]; cons?: string[] }): HTMLElement {
  const container = document.createElement("div");
  document.body.appendChild(container);
  const app = mount(ProsConsColumn, { target: container, props });
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
});

describe("ProsConsColumn", () => {
  it("renders gains and gives-up sections when both are present", () => {
    const container = render({
      pros: ["Cash now — simplest to reinvest or diversify", "A clean break at closing"],
      cons: ["Future upside if the company keeps growing is given up"],
    });

    const headings = [...container.querySelectorAll("h4")].map((h) => h.textContent);
    expect(headings).toEqual(["What this choice gains", "What it gives up"]);

    const lists = container.querySelectorAll("ul");
    expect(lists[0].querySelectorAll("li")).toHaveLength(2);
    expect(lists[0].textContent).toContain("Cash now — simplest to reinvest or diversify");
    expect(lists[1].querySelectorAll("li")).toHaveLength(1);
    expect(lists[1].textContent).toContain("Future upside if the company keeps growing is given up");
  });

  it("renders only the gains section when cons are absent", () => {
    const container = render({ pros: ["Cash now"] });
    expect(container.querySelector("h4")?.textContent).toBe("What this choice gains");
    expect(container.textContent).not.toContain("What it gives up");
  });

  it("renders only the gives-up section when pros are absent", () => {
    const container = render({ cons: ["Your role ends unless separately contracted"] });
    expect(container.querySelector("h4")?.textContent).toBe("What it gives up");
    expect(container.textContent).not.toContain("What this choice gains");
  });

  it("renders nothing when both are absent", () => {
    expect(render({}).textContent?.trim()).toBe("");
  });

  it("renders nothing when both are empty arrays", () => {
    expect(render({ pros: [], cons: [] }).textContent?.trim()).toBe("");
  });

  it("renders list items verbatim", () => {
    const container = render({
      pros: ["Predictable personal cash flow for years"],
      cons: ["Depends on the buyer's continued performance", "Inflation erodes fixed payments"],
    });
    const items = [...container.querySelectorAll("li")].map((li) => li.textContent);
    expect(items).toEqual([
      "Predictable personal cash flow for years",
      "Depends on the buyer's continued performance",
      "Inflation erodes fixed payments",
    ]);
  });
});
