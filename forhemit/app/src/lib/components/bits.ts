// Tiny shared presentation helpers for the components.

export const OWNER_CHIP = "You said this — owner-stated";
export const SYSTEM_CHIP = "Recorded by Forhemit — system";

export function statusLabel(status: string): string {
  switch (status) {
    case "draft": return "Draft";
    case "working": return "Working draft — not sure yet";
    case "confirmed": return "Confirmed";
    case "under_professional_review": return "Under professional review";
    case "revised": return "Revised";
    case "superseded": return "Superseded";
    case "archived": return "Archived";
    default: return status;
  }
}

export function statusClass(status: string): string {
  switch (status) {
    case "confirmed": return "status-confirmed";
    case "working": return "status-working";
    case "archived": return "status-archived";
    default: return "status-draft";
  }
}

export function reasonLabel(value: string): string {
  return (
    {
      learned_something_new: "I learned something new",
      priorities_changed: "My priorities changed",
      professional_suggested_another_approach: "My professional suggested another approach",
      business_changed: "The business changed",
      exploring_different_outcome: "I want to explore a different outcome",
      other: "Other",
    }[value] ?? value
  );
}

export function timeLabel(iso: string): string {
  try {
    return new Date(iso).toLocaleString("en-US", {
      year: "numeric", month: "short", day: "numeric",
      hour: "2-digit", minute: "2-digit",
    });
  } catch {
    return iso;
  }
}

/** One-line display of a wire AnswerValue for answer history lists. */
export function answerSummary(value: unknown): string {
  if (typeof value === "string") return value === "confirmed" ? "Confirmed" : value;
  if ("single" in value) return String(value.single);
  if ("multi" in value) return value.multi.join(", ");
  if ("ranking" in value) return value.ranking.map((v, i) => `${i + 1}. ${v}`).join("; ");
  if ("amount" in value) return `$${Number(value.amount).toLocaleString("en-US")}`;
  return "(complex value)";
}
