// Tiny shared presentation helpers for the components.

import type { AnswerValue } from "../types";

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

export function timeLabel(value: unknown): string {
  const parsed = wireTimestamp(value);
  if (parsed !== null) {
    return parsed.toLocaleString("en-US", {
      year: "numeric", month: "short", day: "numeric",
      hour: "2-digit", minute: "2-digit",
    });
  }
  // Unknown shape — show the raw value rather than "Invalid Date".
  return String(value ?? "—");
}

/**
 * Parses the backend's timestamp wire form into a Date. Rust `time::OffsetDateTime`
 * serializes as `[year, day_of_year, hour, minute, second, nanosecond,
 * offset_hour, offset_minute, offset_second]`; ISO strings are also accepted so
 * test fixtures keep working. Returns null for anything unparseable.
 */
export function wireTimestamp(value: unknown): Date | null {
  if (typeof value === "string") {
    const fromIso = new Date(value);
    return Number.isNaN(fromIso.getTime()) ? null : fromIso;
  }
  if (!Array.isArray(value) || value.length < 6) return null;
  const [year, ordinal, hour, minute, second, nanos] = value;
  if (![year, ordinal, hour, minute, second].every((n) => Number.isInteger(n))) return null;
  // Day-of-year arithmetic: month 0 + ordinal days lands on the right date.
  const millis = Math.floor((typeof nanos === "number" ? nanos : 0) / 1_000_000);
  const parsed = new Date(Date.UTC(year, 0, ordinal, hour, minute, second, millis));
  return Number.isNaN(parsed.getTime()) ? null : parsed;
}

/** One-line display of a wire AnswerValue for answer history lists. */
export function answerSummary(value: AnswerValue): string {
  if (typeof value === "string") return value === "confirmed" ? "Confirmed" : value;
  if ("single" in value) return String(value.single);
  if ("multi" in value) return value.multi.join(", ");
  if ("ranking" in value) return value.ranking.map((v, i) => `${i + 1}. ${v}`).join("; ");
  if ("amount" in value) return `${Number(value.amount).toLocaleString("en-US")}`;
  return "(complex value)";
}
