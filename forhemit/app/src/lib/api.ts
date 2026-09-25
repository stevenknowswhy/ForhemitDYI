// The command client. Inside Tauri it invokes commands; in the dev-server
// harness (a plain browser) it POSTs to the localhost-only /command endpoint.
// Both paths hit the same adapter functions.

import type {
  AnswerValue,
  AuditLogLine,
  ChangeReason,
  Completeness,
  Destination,
  DestinationContent,
  FactKind,
  FactValue,
  FactVersion,
  JourneyView,
  VerifyView,
} from "./types";

// Tauri injects this global in the desktop shell; the dev harness does not.
declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

function inTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function postCommand<T>(name: string, args: object): Promise<T> {
  const response = await fetch(`/command/${name}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(args),
  });
  if (!response.ok) {
    // Engine refusals arrive as 400 + a quoted error string — displayed,
    // never swallowed.
    throw new Error(`${name}: ${(await response.text()) || response.status}`);
  }
  return (await response.json()) as T;
}

async function tauriInvoke<T>(name: string, args: object): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(name, args);
}

export async function call<T>(
  name: string,
  args: Record<string, unknown> = {},
): Promise<T> {
  return inTauri() ? tauriInvoke<T>(name, args) : postCommand<T>(name, args);
}

export const api = {
  destinationGet: () => call<Destination | null>("destination_get"),
  destinationCreate: (content: DestinationContent) =>
    call<Destination>("destination_create", { content }),
  destinationEdit: (
    content: DestinationContent,
    reason: ChangeReason,
    explanation: string | null,
  ) =>
    call<Destination>("destination_edit", { content, reason, explanation }),
  destinationConfirm: () => call<Destination>("destination_confirm"),
  destinationMarkWorking: () => call<Destination>("destination_mark_working"),
  destinationArchive: () => call<Destination>("destination_archive"),
  destinationCompleteness: (content: DestinationContent) =>
    call<Completeness>("destination_completeness", { content }),
  journeyState: () => call<JourneyView | null>("journey_state"),
  journeyStart: () => call<JourneyView>("journey_start"),
  journeyRecord: (nodeId: string, value: AnswerValue) =>
    call<JourneyView>("journey_record", { node_id: nodeId, value }),
  journeyRevise: (nodeId: string, value: AnswerValue, changeReason: string) =>
    call<JourneyView>("journey_revise", {
      node_id: nodeId,
      value,
      change_reason: changeReason,
    }),
  journeySkip: (nodeId: string) =>
    call<JourneyView>("journey_skip", { node_id: nodeId }),
  snapshotCurrent: () => call<FactVersion[]>("snapshot_current"),
  snapshotRecord: (
    kind: FactKind,
    value: FactValue,
    definition: string | null,
  ) => call<FactVersion[]>("snapshot_record", { kind, value, definition }),
  snapshotRevise: (
    factId: string,
    value: FactValue,
    definition: string | null,
    changeReason: string,
  ) =>
    call<FactVersion[]>("snapshot_revise", {
      fact_id: factId,
      value,
      definition,
      change_reason: changeReason,
    }),
  auditRecent: () => call<AuditLogLine[]>("audit_recent"),
  auditVerify: () => call<VerifyView>("audit_verify"),
};
