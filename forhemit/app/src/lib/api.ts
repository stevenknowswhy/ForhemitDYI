// The command client. Inside Tauri it invokes commands; in the dev-server
// harness (a plain browser) it POSTs to the localhost-only /command endpoint.
// Both paths hit the same adapter functions.

import type {
  AnswerValue,
  AssumptionWire,
  AuditLogLine,
  ChangeReason,
  ComparisonView,
  ComparisonWire,
  Completeness,
  ConflictWire,
  ConstraintWire,
  Destination,
  DestinationContent,
  ExportedFileView,
  FactKind,
  FactValue,
  FactVersion,
  FamilyAndVersionView,
  JourneyView,
  NonnegotiableWire,
  PackagePreviewView,
  ScenarioCreateWire,
  ScenarioFamilyView,
  ScenarioVersionView,
  UnknownWire,
  VaultDocumentHistoryView,
  VaultDocumentView,
  VaultSearchHitView,
  VaultStatusView,
  VaultVersionContentView,
  VerifyView,
  WhatIfWire,
} from "./types";

// Re-exported so components can name the wire types without reaching past
// the API layer.
export type {
  AssumptionWire,
  ComparisonWire,
  ConflictWire,
  ConstraintWire,
  NonnegotiableWire,
  ScenarioCreateWire,
  UnknownWire,
  WhatIfWire,
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

async function tauriInvoke<T>(name: string, args: Record<string, unknown>): Promise<T> {
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
  // Scenario explorer — every mutation returns the version view; the
  // engine records the audit event and enforces the integrity rules.
  scenarioCreate: (request: ScenarioCreateWire) =>
    call<FamilyAndVersionView>("scenario_create", { request }),
  scenarioFamilies: () => call<ScenarioFamilyView[]>("scenario_families"),
  scenarioFamilyView: (familyId: string) =>
    call<ScenarioFamilyView>("scenario_family_view", { family_id: familyId }),
  scenarioVersionView: (versionId: string) =>
    call<ScenarioVersionView>("scenario_version_view", { version_id: versionId }),
  scenarioAddAssumption: (versionId: string, request: AssumptionWire) =>
    call<ScenarioVersionView>("scenario_add_assumption", { version_id: versionId, request }),
  scenarioAddUnknown: (versionId: string, request: UnknownWire) =>
    call<ScenarioVersionView>("scenario_add_unknown", { version_id: versionId, request }),
  scenarioResolveUnknown: (
    versionId: string,
    unknownId: string,
    status: string,
    resolutionReference: string | null,
  ) =>
    call<ScenarioVersionView>("scenario_resolve_unknown", {
      version_id: versionId,
      unknown_id: unknownId,
      status,
      resolution_reference: resolutionReference,
    }),
  scenarioAddConstraint: (versionId: string, request: ConstraintWire) =>
    call<ScenarioVersionView>("scenario_add_constraint", { version_id: versionId, request }),
  scenarioAddNonnegotiable: (versionId: string, request: NonnegotiableWire) =>
    call<ScenarioVersionView>("scenario_add_nonnegotiable", { version_id: versionId, request }),
  scenarioRecordConflict: (versionId: string, request: ConflictWire) =>
    call<ScenarioVersionView>("scenario_record_conflict", { version_id: versionId, request }),
  scenarioResolveConflict: (
    versionId: string,
    conflictId: string,
    ownerDecision: string | null,
    resolutionReference: string | null,
  ) =>
    call<ScenarioVersionView>("scenario_resolve_conflict", {
      version_id: versionId,
      conflict_id: conflictId,
      owner_decision: ownerDecision,
      resolution_reference: resolutionReference,
    }),
  scenarioFinalize: (versionId: string) =>
    call<ScenarioVersionView>("scenario_finalize", { version_id: versionId }),
  scenarioSetReadiness: (versionId: string, readiness: string, reason: string | null) =>
    call<ScenarioVersionView>("scenario_set_readiness", {
      version_id: versionId,
      readiness,
      reason,
    }),
  scenarioWhatIf: (request: WhatIfWire) =>
    call<FamilyAndVersionView>("scenario_what_if", { request }),
  scenarioComparison: (request: ComparisonWire) =>
    call<ComparisonView>("scenario_comparison", { request }),
  // Vault — import takes base64 content; the engine encrypts at rest.
  vaultStatus: () => call<VaultStatusView>("vault_status"),
  vaultSetup: (recoveryPassphrase: string) =>
    call<VaultStatusView>("vault_setup", { recovery_passphrase: recoveryPassphrase }),
  vaultRecover: (recoveryPassphrase: string) =>
    call<VaultStatusView>("vault_recover", { recovery_passphrase: recoveryPassphrase }),
  vaultImport: (filename: string, contentBase64: string, note: string | null) =>
    call<VaultDocumentView>("vault_import", {
      filename,
      content_base64: contentBase64,
      note,
    }),
  vaultDocuments: () => call<VaultDocumentView[]>("vault_documents"),
  vaultDocumentHistory: (documentId: string) =>
    call<VaultDocumentHistoryView>("vault_document_history", { document_id: documentId }),
  vaultDocumentContent: (versionId: string) =>
    call<VaultVersionContentView>("vault_document_content", { version_id: versionId }),
  vaultSearch: (query: string) => call<VaultSearchHitView[]>("vault_search", { query }),
  vaultBackup: (recoveryPassphrase: string) =>
    call<ExportedFileView>("vault_backup", { recovery_passphrase: recoveryPassphrase }),
  // Package — preview exposes provenance and readiness gating first.
  packagePreview: () => call<PackagePreviewView>("package_preview"),
  packageExport: (format: "html" | "pdf") => call<ExportedFileView>("package_export", { format }),
};
