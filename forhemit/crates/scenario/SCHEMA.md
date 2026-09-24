# Scenario Engine — Relational Schema + Event Contract

This is the artifact the *Scenario Engine Data Model — Schema Specification* prescribes
but never wrote. Its final section names the next specification:

> The next specification I would create is **"Scenario Engine Relational Schema + Event
> Contract"**. That would take this conceptual model one level deeper and define the
> actual tables/entities, primary keys, foreign keys, cardinalities, indexes,
> immutable-version rules, and the exact payload structure for events such as
> `ScenarioCreated`, `ScenarioConflictDetected`, `ScenarioWhatIfCreated`, and
> `ScenarioAffectedByRealityChange`.

This document is that specification. It is the contract for the `scenario` crate
(`forhemit-scenario`): the Rust types, their in-memory/SQLite mapping, and the audit
events each mutation emits. The implementation-spec's Verification row pins the
acceptance: every scenario version references exactly one destination version + one
business-reality version; finalized versions are immutable; conflicts carry severity,
never scores; one test per data-integrity rule (all ten, schema doc §49).

Conventions:

- All primary keys are typed identifier newtypes (`forhemit_contracts::ids`), stored as
  TEXT. IDs are ULIDs minted by the engine except where noted.
- `workspace_id` scopes every row (v1 is single-device; the column is reserved for the
  multi-workspace design and is present everywhere from day one).
- Timestamps are UTC (`OffsetDateTime`, RFC 3339 in JSON).
- Enum values are stored as their snake_case serde names, matching the JSON Schema
  exports in `forhemit/schema/`.
- **Immutable** below means: no UPDATE path exists in the engine; a correction creates a
  new row/version (schema doc §44: "Corrections create new records or versions. They do
  not rewrite history.").
- Tables marked **(reserved)** belong to engines deferred with doc receipts (research,
  evidence, financial modeling, financing, seller note, professional review, transaction).
  They are defined here so the boundary never drifts, but v1 does not create rows in
  them and the v1 Rust types do not model them.

---

## 1. Identity map (typed IDs)

| Newtype | Stored form | Minted by |
| --- | --- | --- |
| `ScenarioFamilyId` | `fam_<ulid>` | scenario engine |
| `ScenarioVersionId` | `sv_<ulid>` | scenario engine |
| `SnapshotId` | `snap_<ulid>` | scenario engine |
| `AssumptionId` | `asm_<ulid>` | scenario engine |
| `ConstraintId` | `con_<ulid>` | scenario engine |
| `UnknownId` | `unk_<ulid>` | scenario engine |
| `ConflictId` | `cnf_<ulid>` | scenario engine |
| `BranchId` | `brn_<ulid>` | scenario engine |
| `ComparisonId` | `cmp_<ulid>` | scenario engine |
| `BusinessRealityVersionId` | `brv_<sha256-64hex>` | **derived, not minted** — see §4 |
| `DestinationVersionId` | (destination crate) | destination engine |
| `ObjectiveId` | (destination crate) | destination engine |
| `FactVersionId` | (reality crate) | reality engine |
| `FinancialModelVersionId` | (reserved) | financial modeling engine |
| `ProfessionalReviewId` | (reserved) | professional review engine |

---

## 2. Tables

### 2.1 `scenario_family` — one conceptual path through its entire history (schema doc §3)

| Column | Type | Constraints |
| --- | --- | --- |
| `scenario_family_id` | TEXT | **PK** |
| `workspace_id` | TEXT | NOT NULL, FK → workspace |
| `name` | TEXT | NOT NULL, non-empty |
| `scenario_type` | TEXT | NOT NULL, enum §6.1 |
| `branched_from_version_id` | TEXT | NULL, FK → `scenario_version.scenario_version_id` — set iff this family was created by a what-if branch (schema doc §30: `child_scenario_family_id` on the branch row; the family carries the mirror link) |
| `created_at` | RFC 3339 | NOT NULL |
| `created_by` | actor JSON | NOT NULL |
| `archived_at` | RFC 3339 | NULL — set only by `archive_family` |
| `archive_reason` | TEXT | NOT NULL iff `archived_at` set |

Indexes: `idx_scenario_family_workspace` (`workspace_id`).
`branched_from_version_id` is indexed through §2.9's branch table; it is denormalized
here for single-query lineage reads.

A family holds **no** mutable scenario content (schema doc §3: "`ScenarioFamily` does
not contain mutable scenario assumptions. Those belong to versions.").

### 2.2 `scenario_version` — the primary historical object (schema doc §4)

| Column | Type | Constraints |
| --- | --- | --- |
| `scenario_version_id` | TEXT | **PK** |
| `scenario_family_id` | TEXT | NOT NULL, FK → `scenario_family` |
| `version_number` | INTEGER | NOT NULL, ≥ 1, unique per family (`UNIQUE(scenario_family_id, version_number)`), gapless |
| `parent_version_id` | TEXT | NULL, FK → `scenario_version` — the version this one was created from (lineage); NULL on a family's first version |
| `supersedes_version_id` | TEXT | NULL, FK → `scenario_version` — the version this one officially replaces; equal to `parent_version_id` for revisions, NULL on branches (a branch does not supersede what it branches from) |
| `name` | TEXT | NOT NULL, non-empty |
| `description` | TEXT | NULL |
| `scenario_type` | TEXT | NOT NULL, enum §6.1 |
| `destination_version_id` | TEXT | NOT NULL, FK → destination crate's version — **integrity rule 1: exactly one destination version**. Single-valued NOT NULL; there is no second column and no link table, so "exactly one" is enforced by schema, not by convention |
| `business_reality_version_id` | TEXT | NOT NULL — **integrity rule 2: the Business Reality version used to create it**. Derived content address (§4); single-valued NOT NULL |
| `readiness_status` | TEXT | NOT NULL, enum §6.2 |
| `lifecycle_status` | TEXT | NOT NULL, enum §6.3 |
| `change_reason` | TEXT | NULL — why this version exists; NOT NULL on successors (version-creation triggers, schema doc §47) |
| `snapshot_id` | TEXT | NOT NULL, UNIQUE, FK → `scenario_snapshot` — 1:1 (schema doc §42) |
| `created_at` | RFC 3339 | NOT NULL |
| `created_by` | actor JSON | NOT NULL |

Indexes: `idx_scenario_version_family` (`scenario_family_id`, `version_number`),
`idx_scenario_version_destination` (`destination_version_id`),
`idx_scenario_version_reality` (`business_reality_version_id`).

**Draft vs version** (schema doc §46): a row exists from the moment the owner starts a
scenario, with `lifecycle_status = draft` (or `idea`). While in a draft lifecycle state
the row may be edited (each edit audited, §3.2). **Finalize** is the transition that
makes the row historical: `lifecycle_status` leaves the draft set and the row becomes
immutable forever. A material change after that produces a **new** row
(`version_number + 1`, `parent_version_id` + `supersedes_version_id` = the frozen
version) — never a mutation.

**Immutability rules** (schema doc §44): `scenario_version` (once finalized),
`scenario_snapshot`, historical `scenario_comparison`, `scenario_transaction_handoff`,
`scenario_status_history`, professional-determination references, and historical branch
lineage are append-only. The engine exposes no mutation path for them.

### 2.3 `scenario_snapshot` — what we actually knew (schema doc §6, §51)

1:1 with `scenario_version` (the version is a coherent snapshot of references, not a
mutable container of links).

| Column | Type | Constraints |
| --- | --- | --- |
| `snapshot_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, UNIQUE, FK → `scenario_version` |
| `destination_version_id` | TEXT | NOT NULL — mirrors the version's rule-1 reference |
| `business_reality_version_id` | TEXT | NOT NULL — mirrors rule 2 |
| `reality_fact_version_ids` | JSON array of TEXT | NOT NULL — the exact fact versions pinned; the authoritative rows stay in Business Reality (schema doc §6: "reference authoritative versions rather than duplicate entire source systems") |
| `financial_model_version_id` | TEXT | NULL, FK (reserved) |
| `research_snapshot_id` | TEXT | NULL (reserved) |
| `evidence_snapshot_id` | TEXT | NULL (reserved) |
| `financing_reference_set` | JSON (reserved) | NULL |
| `seller_note_reference_set` | JSON (reserved) | NULL |
| `professional_review_reference_set` | JSON (reserved) | NULL |
| `created_at` | RFC 3339 | NOT NULL |

### 2.4 `scenario_assumption` — typed, provenance-first (schema doc §7–11)

| Column | Type | Constraints |
| --- | --- | --- |
| `assumption_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK → `scenario_version` |
| `category` | TEXT | NOT NULL, enum §6.4 — descriptive, never an authority claim (schema doc §10) |
| `name` | TEXT | NOT NULL, non-empty |
| `description` | TEXT | NULL |
| `value_type` | TEXT | NOT NULL, enum §6.5 |
| `value` | JSON | NOT NULL — a `TypedValue` whose variant matches `value_type` (schema doc §8: never a bare string) |
| `provenance` | TEXT | NOT NULL, enum §6.6 — **integrity rule 4: every material assumption has provenance**; column is NOT NULL and the serde type has no default, so a row without provenance cannot be constructed or deserialized |
| `verification` | TEXT | NOT NULL, enum §6.7 — a **separate axis** from provenance (schema doc §11) |
| `nonnegotiable_objective_id` | TEXT | NULL, FK → destination crate's objective — spec's `nonnegotiable_backref`: set iff this assumption exists to test a destination nonnegotiable |
| `source_reference` | TEXT | NULL |
| `created_at` | RFC 3339 | NOT NULL |

Indexes: `idx_assumption_version` (`scenario_version_id`).

The system can therefore say "the scenario assumes a $10M valuation based on
owner-provided information that has not yet been professionally validated" — value,
provenance, and verification travel together (schema doc §9).

### 2.5 `scenario_constraint` (schema doc §12)

| Column | Type | Constraints |
| --- | --- | --- |
| `constraint_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `constraint_type` | TEXT | NOT NULL, enum §6.8 |
| `name` | TEXT | NOT NULL |
| `description` | TEXT | NULL |
| `value` | JSON | NULL — optional `TypedValue` |
| `source_reference` | TEXT | NULL |
| `created_at` | RFC 3339 | NOT NULL |

### 2.6 `scenario_nonnegotiable` — the destination's must-haves under test (schema doc §13)

| Column | Type | Constraints |
| --- | --- | --- |
| `nonnegotiable_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `destination_objective_id` | TEXT | NOT NULL, FK → destination crate's objective — "maintains the authoritative relationship to the Destination Engine"; `ObjectiveId` is stable across destination versions |
| `destination_version_id` | TEXT | NOT NULL — the destination version whose designation is being tested |
| `description` | TEXT | NOT NULL |
| `value` | JSON | NULL — optional `TypedValue` (e.g. the minimum proceeds figure) |
| `created_at` | RFC 3339 | NOT NULL |

### 2.7 `scenario_unknown` — first-class missing information (schema doc §15)

| Column | Type | Constraints |
| --- | --- | --- |
| `unknown_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `category` | TEXT | NULL |
| `description` | TEXT | NOT NULL, non-empty — never a blank zero (implementation spec: unknowns "show as first-class ScenarioUnknown objects … never blank zeros") |
| `importance` | TEXT | NOT NULL, enum §6.9 (CRITICAL / IMPORTANT / HELPFUL) |
| `resolution_status` | TEXT | NOT NULL, enum §6.10, default `open` |
| `required_action` | TEXT | NULL |
| `source_dependency` | TEXT | NULL |
| `created_at` | RFC 3339 | NOT NULL |
| `resolved_at` | RFC 3339 | NULL — set when resolution_status reaches `resolved`/`waived` |
| `resolved_by` | actor JSON | NULL, same condition |
| `resolution_reference` | TEXT | NULL, same condition |

The schema doc also lists a `blocking_level` field but prescribes no enumerated
vocabulary for it; it is **omitted from v1** rather than invented as a free string.
If a blocking vocabulary is defined later it joins as a new contract version.

### 2.8 `scenario_conflict` — severity describes the condition, never a score (schema doc §16–17)

| Column | Type | Constraints |
| --- | --- | --- |
| `conflict_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `conflict_type` | TEXT | NOT NULL, enum §6.11 |
| `severity` | TEXT | NOT NULL, enum §6.12 — INFORMATIONAL / ATTENTION / MATERIAL / BLOCKING; "describes the condition, not produce a scenario score" |
| `description` | TEXT | NOT NULL |
| `source_reference` | TEXT | NULL |
| `affected_object_type` | TEXT | NULL |
| `affected_object_id` | TEXT | NULL |
| `resolution` | JSON | NULL — `{ decided_by, decided_at, resolution_reference }`; NULL = open |
| `created_at` | RFC 3339 | NOT NULL |

Indexes: `idx_conflict_version` (`scenario_version_id`), `idx_conflict_type`
(`conflict_type`).

**Integrity rule 8:** conflicts are rows in their own table, never children of a
version's mutable content. A newer scenario version cannot make a conflict disappear:
`conflicts_of(v1)` keeps returning v1's conflicts after v2 exists, and there is no
delete API. A conflict is closed by recording a resolution on the row (a new audit
event), not by deletion.

### 2.9 `scenario_nonnegotiable_conflict` — the structured conflict, not `warning = true` (schema doc §14; NONNEGOTIABLE doc §5)

Created when a scenario conflicts with a destination nonnegotiable. It extends the
generic conflict row (shared `conflict_id`) with the destination back-reference and the
owner's decision. The system **surfaces** it and **lets the owner decide** — it never
filters the scenario, scores the constraint, or silently relaxes it
(NONNEGOTIABLE doc §5: "Keep This Must-Have / Explore a Different Scenario / Change My
Requirement — the owner decides").

| Column | Type | Constraints |
| --- | --- | --- |
| `conflict_id` | TEXT | **PK**, FK → `scenario_conflict` (whose `conflict_type` = `nonnegotiable`) |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `nonnegotiable_id` | TEXT | NOT NULL, FK → `scenario_nonnegotiable` |
| `destination_objective_id` | TEXT | NOT NULL — the destination back-reference |
| `destination_version_id` | TEXT | NOT NULL |
| `description` | TEXT | NOT NULL |
| `owner_decision` | TEXT | NULL, enum §6.13 — NULL until the **owner** decides; the engine never sets it |
| `decided_at` | RFC 3339 | NULL |
| `created_at` | RFC 3339 | NOT NULL |

### 2.10 `scenario_branch` — explicit what-if lineage (schema doc §30–31)

| Column | Type | Constraints |
| --- | --- | --- |
| `branch_id` | TEXT | **PK** |
| `parent_scenario_version_id` | TEXT | NOT NULL, FK — **integrity rule 7: a branch must identify its parent**; NOT NULL by schema |
| `child_scenario_family_id` | TEXT | NOT NULL, FK → `scenario_family` |
| `branch_type` | TEXT | NOT NULL, enum §6.14 |
| `reason` | TEXT | NOT NULL |
| `changed_assumptions` | JSON array of TEXT | NOT NULL — assumption ids the branch overrides |
| `created_at` | RFC 3339 | NOT NULL |
| `created_by` | actor JSON | NOT NULL |

Creating a branch never modifies the parent version (schema doc §31: "The parent must
never be modified by creation of a branch").

### 2.11 `scenario_comparison` — durable, winner-free (schema doc §32–34)

| Column | Type | Constraints |
| --- | --- | --- |
| `comparison_id` | TEXT | **PK** |
| `name` | TEXT | NOT NULL |
| `scenario_version_ids` | JSON array of TEXT | NOT NULL, ≥ 2 — "captures what was compared" |
| `created_at` | RFC 3339 | NOT NULL |
| `created_by` | actor JSON | NOT NULL |

### 2.12 `scenario_comparison_dimension` (schema doc §33)

| Column | Type | Constraints |
| --- | --- | --- |
| `dimension_id` | TEXT | **PK** |
| `comparison_id` | TEXT | NOT NULL, FK |
| `dimension_type` | TEXT | NOT NULL, enum §6.15 |
| `label` | TEXT | NOT NULL |
| `display_order` | INTEGER | NOT NULL |

### 2.13 `scenario_comparison_result` — factual, not evaluative (schema doc §34)

| Column | Type | Constraints |
| --- | --- | --- |
| `result_id` | TEXT | **PK** |
| `comparison_id` | TEXT | NOT NULL, FK |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `dimension_id` | TEXT | NOT NULL, FK |
| `value` | JSON | NULL — optional `TypedValue` (the factual reading, e.g. "24 months") |
| `outcome` | TEXT | NOT NULL, enum §6.16 — factual status (`aligns`, `does_not_currently_align`, `insufficient_information`), no rank |
| `source_reference` | TEXT | NULL |
| `created_at` | RFC 3339 | NOT NULL |

`UNIQUE(comparison_id, scenario_version_id, dimension_id)`. There is **no** ordering
column, no `winner`, no `recommended` row, and no aggregate score —
**integrity rule 10**: "Scenario data cannot become an implicit recommendation."

### 2.14 `scenario_status_history` (schema doc §36) — immutable

| Column | Type | Constraints |
| --- | --- | --- |
| `status_history_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `previous_lifecycle` | TEXT | NULL (NULL = initial) |
| `new_lifecycle` | TEXT | NOT NULL |
| `previous_readiness` | TEXT | NULL |
| `new_readiness` | TEXT | NOT NULL |
| `reason` | TEXT | NULL |
| `changed_by` | actor JSON | NOT NULL |
| `changed_at` | RFC 3339 | NOT NULL |

### 2.15 `scenario_impact` — when upstream data changes (schema doc §35)

| Column | Type | Constraints |
| --- | --- | --- |
| `impact_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `source_change_type` | TEXT | NOT NULL (e.g. `business_reality`, `destination`) |
| `source_change_id` | TEXT | NOT NULL |
| `impact_type` | TEXT | NOT NULL — ASSUMPTION_AFFECTED / MODEL_AFFECTED / OBJECTIVE_ALIGNMENT_AFFECTED / NONNEGOTIABLE_AFFECTED / UNKNOWN_AFFECTED / CONFLICT_AFFECTED / RESEARCH_FRESHNESS_AFFECTED |
| `impact_description` | TEXT | NOT NULL |
| `severity` | TEXT | NOT NULL, enum §6.12 |
| `detected_at` | RFC 3339 | NOT NULL |
| `status` | TEXT | NOT NULL, default `open` |

### 2.16 `scenario_selection_record` (schema doc §37) — selection ≠ recommendation

| Column | Type | Constraints |
| --- | --- | --- |
| `selection_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `selection_type` | TEXT | NOT NULL — FURTHER_EXPLORATION / PROFESSIONAL_REVIEW / TRANSACTION_HANDOFF / ARCHIVE |
| `reason` | TEXT | NULL |
| `selected_by` | actor JSON | NOT NULL |
| `selected_at` | RFC 3339 | NOT NULL |
| `decision_record_reference` | TEXT | NULL (reserved: Decision Record engine) |

### 2.17 `scenario_transaction_handoff` (schema doc §38–39) — reserved (transaction layer is out of v1)

### 2.17a `scenario_professional_review_reference` — attribution boundary (schema doc §27, §29) — v1

The professional's actual determination remains owned by the professional review engine
(schema doc §43 authority map); the scenario stores only the **reference** — who
reviewed, in what role, and where their feedback lives. Two hard rules:

- The reference is always attributed: `professional_role` and
  `professional_identity_reference` are NOT NULL. An anonymous "a professional
  reviewed this" cannot be recorded (integrity rule 5).
- A reference never flips the scenario into an approved state (schema doc §29):
  `review_status` is process vocabulary (`not_reviewed` / `awaiting_review` /
  `reviewed`), and no status in this schema means "professionally approved".

| Column | Type | Constraints |
| --- | --- | --- |
| `reference_id` | TEXT | **PK** |
| `scenario_version_id` | TEXT | NOT NULL, FK |
| `professional_review_id` | TEXT | NOT NULL (reserved id — minted when the review engine exists; carried opaquely in v1) |
| `review_status` | TEXT | NOT NULL, default `not_reviewed` |
| `professional_role` | TEXT | NOT NULL |
| `professional_identity_reference` | TEXT | NOT NULL |
| `feedback_reference` | TEXT | NULL |
| `created_at` | RFC 3339 | NOT NULL |

### 2.18 Reference/link tables — reserved (schema doc §20–28)

`scenario_dependency` (§18–19), `scenario_evidence_link` (§20), `scenario_research_link`
(§21), `assumption_evidence_link` (§22), `scenario_financial_model_link` (§23),
`scenario_model_output_reference` (§24), `scenario_financing_link` (§25),
`scenario_seller_note_link` (§26), `professional_feedback_reference` (§28). The Scenario
Engine **references, never owns**, these domains (schema doc §43 external authority
map). They are defined in the doc entity model but v1 creates no rows: research,
evidence, financing, seller-note, financial modeling, and the professional-review flow
beyond the §2.17a attribution reference are deferred with doc receipts
(implementation-spec scope table). The one v1 exception is the model-output reference's
**type-level** rule 3 support: a reserved `FinancialModelVersionId` type exists in
contracts so a future row cannot exist without its model version.

### 2.19 Version-creation triggers (schema doc §47)

A new material `scenario_version` is created when: a material assumption changes; a
nonnegotiable changes; Destination changes; Business Reality changes and the scenario is
refreshed; a branch is created; professional feedback causes a change (reserved); the
owner requests a materially different path. Minor presentation changes do not create
versions.

---

## 3. Event contract

Every mutation emits exactly one audit event, **before** the mutation is applied
(audit-first, Implementation Roadmap Phase 1): a refused event refuses the mutation —
a scenario row never exists without its event. All events below carry the
`AuditEvent::V1` envelope (Audit doc §63 fields + `previous_event_hash` chain link)
with `source_engine: "scenario"`. The `payload` field is the typed payload below,
serialized to JSON and addressed by digest (`payload_reference`). `correlation_id`
groups the activity; `causation_id` chains follow-on events.

### 3.1 `ScenarioCreated` — a family is born

Emitted by `create_family`. Payload:

```json
{
  "scenario_family_id": "fam_…",
  "name": "Employee Ownership + Seller Financing",
  "scenario_type": "seller_financed_employee_acquisition",
  "branched_from_version_id": null,
  "branch_id": null
}
```

`branched_from_version_id` / `branch_id` are non-null when the family was created by a
what-if branch (`ScenarioWhatIfCreated` names the same ids from the branch side).

### 3.2 `ScenarioDraftUpdated` — a draft mutation (schema doc §46)

Emitted by every draft-scope mutation (metadata, assumptions, constraints,
nonnegotiables, unknowns). One event type with a typed `section` discriminator; the
payload preserves previous values or references (Audit integrity rule 4). Payload:

```json
{
  "scenario_version_id": "sv_…",
  "section": "assumptions | constraints | nonnegotiables | unknowns | metadata | started",
  "change": "added | removed | revised",
  "object_id": "asm_…",
  "previous_value": null,
  "new_value": { }
}
```

`section: "started"` marks the draft's creation (its first event).

### 3.3 `ScenarioVersionFinalized` — the draft becomes history

Emitted by `finalize_version`. Payload:

```json
{
  "scenario_version_id": "sv_…",
  "scenario_family_id": "fam_…",
  "version_number": 1,
  "lifecycle_status": "preliminary",
  "readiness_status": "preliminary",
  "snapshot_id": "snap_…",
  "destination_version_id": "…",
  "business_reality_version_id": "brv_…",
  "assumption_count": 4,
  "unknown_count": 1
}
```

After this event the version is immutable (rule 6); further material change creates a
successor draft (§2.19) whose events reference the frozen version.

### 3.4 `ScenarioConflictDetected` — including nonnegotiable conflicts

Emitted by conflict recording. Payload for a general conflict:

```json
{
  "conflict_id": "cnf_…",
  "scenario_version_id": "sv_…",
  "conflict_type": "data",
  "severity": "material",
  "description": "…",
  "nonnegotiable": null
}
```

For a nonnegotiable conflict, `conflict_type` is `"nonnegotiable"` and `nonnegotiable`
carries the structured extension instead of a `warning` flag (schema doc §14):

```json
{
  "conflict_id": "cnf_…",
  "scenario_version_id": "sv_…",
  "conflict_type": "nonnegotiable",
  "severity": "blocking",
  "description": "Modeled closing proceeds $2.4M are below the owner's $3M must-have.",
  "nonnegotiable": {
    "nonnegotiable_id": "nn_…",
    "destination_objective_id": "obj_…",
    "destination_version_id": "…"
  }
}
```

### 3.5 `ScenarioConflictResolved` — the owner decides

Emitted when a resolution is recorded on a conflict (NONNEGOTIABLE doc §5: keep /
explore another path / change the requirement — decided by the owner, never by the
engine). Payload:

```json
{
  "conflict_id": "cnf_…",
  "scenario_version_id": "sv_…",
  "owner_decision": "keep_requirement | explore_another_path | change_requirement",
  "decided_by": "owner"
}
```

`change_requirement` does not edit anything here: changing the requirement is a
destination-engine edit that creates a new destination version (integrity rule 9), and
the scenario responds through the successor-version flow (§3.7).

### 3.6 `ScenarioWhatIfCreated` — a branch (schema doc §30–31)

Emitted by `create_what_if`. Payload:

```json
{
  "branch_id": "brn_…",
  "branch_type": "what_if",
  "parent_scenario_version_id": "sv_…",
  "child_scenario_family_id": "fam_…",
  "reason": "What if you received less cash at closing?",
  "changed_assumptions": ["asm_…"]
}
```

The parent version is untouched; the branch's child family starts its own version
history at 1.

### 3.7 `ScenarioAffectedByRealityChange` — upstream moved (schema doc §35, §5)

Emitted by `detect_reality_change` when the current Business Reality fact-version set
digests to a different `business_reality_version_id` than the one a version pinned.
The version is **not** modified — the impact is recorded and surfaced. Payload:

```json
{
  "scenario_version_id": "sv_…",
  "impact_type": "nonnegotiable_affected | assumption_affected | unknown_affected",
  "pinned_business_reality_version_id": "brv_…",
  "current_business_reality_version_id": "brv_…",
  "severity": "attention"
}
```

The same detection covers destination drift (rule 9's scenario side): when the owner
changes a nonnegotiable, the destination engine creates a new destination version; the
scenario's pinned `destination_version_id` still points at the old version — the engine
never re-points a stored version — and a successor draft pinned to the new destination
version is how the scenario picks the change up.

### 3.8 `ScenarioComparisonRecorded` — a durable comparison (schema doc §32)

Emitted by `record_comparison`. Payload:

```json
{
  "comparison_id": "cmp_…",
  "name": "ESOP vs staged ownership",
  "scenario_version_ids": ["sv_…", "sv_…"],
  "dimension_count": 2,
  "result_count": 4
}
```

### 3.9 `ScenarioProfessionalReviewRecorded` — an attribution reference (schema doc §27, §29)

Emitted by `record_professional_review_reference` (works on drafts and finalized
versions — it is an append-only reference row, not version content). Payload:

```json
{
  "reference_id": "ref_…",
  "scenario_version_id": "sv_…",
  "professional_review_id": "…",
  "professional_role": "cpa",
  "professional_identity_reference": "Smith Transaction Advisory",
  "review_status": "awaiting_review"
}
```

No `approved` value exists in `review_status`: a reference records that a professional
was engaged, never that the platform approved anything (schema doc §29).

### 3.10 `ScenarioReadinessChanged` — the readiness axis moves (schema doc §36, §40)

Emitted by `set_readiness` on a finalized version (drafts are `preliminary` by
definition). Readiness and lifecycle are separate axes and stay separate: this event
never changes `lifecycle_status`. A status-history row is written with the same
prev/new pair (§2.14). Payload:

```json
{
  "scenario_version_id": "sv_…",
  "previous_readiness": "modelable",
  "new_readiness": "ready_for_professional_review",
  "reason": "Every critical unknown is resolved; the owner asked for the package."
}
```

### 3.11 `ScenarioFamilyArchived` — a family is retired (schema doc §3)

Emitted by `archive_family`. Retention, not deletion: every version and its history
remain readable. Payload:

```json
{
  "scenario_family_id": "fam_…",
  "archive_reason": "Owner chose to explore only the staged path."
}
```

---

## 4. The Business Reality version address (integrity rule 2)

The schema doc requires every scenario version to reference "the Business Reality
version used to create it". The v1 reality engine versions facts, not the aggregate
snapshot, so the scenario engine derives a **content address** for the fact-version set
it pins:

```
business_reality_version(fact_version_ids) =
    "brv_" ++ hex(sha256( sort(fact_version_ids) joined by "\n" ))
```

- Same facts → same address; any revision in the underlying facts → a different
  address. "What did we actually know when we made this scenario?" (schema doc §51) is
  answered by the pinned address plus the snapshot's fact-version list.
- The address is stored on both `scenario_version` and `scenario_snapshot`; the fact
  version ids themselves are stored in the snapshot (§2.3) so the digest is verifiable
  later.
- An empty fact set digests to the address of the empty set — a scenario created
  against an empty reality is honest about that (no invented facts).

---

## 5. The ten data-integrity rules → enforcement (schema doc §49)

| # | Rule | Enforcement |
| --- | --- | --- |
| 1 | Every Scenario Version references exactly one Destination version | `destination_version_id` is a single-valued NOT NULL column; the engine requires it at creation; test `rule_1_every_version_references_exactly_one_destination_version` |
| 2 | Every Scenario Version references the Business Reality version used to create it | `business_reality_version_id` NOT NULL, content-addressed from the pinned fact versions; test `rule_2_every_version_references_the_reality_version_used` |
| 3 | Every material financial output references a Financial Model version | `FinancialModelVersionId` is a required field of the model-output-reference type (type-level, reserved until the financial modeling engine exists); test `rule_3_model_outputs_reference_a_model_version` |
| 4 | Every material assumption has provenance | `provenance` NOT NULL, no serde default (a payload without it fails deserialization); test `rule_4_every_assumption_has_provenance` |
| 5 | Every professional determination remains attributed | professional references require role + identity; a reference never flips the scenario to an approved state (schema doc §29); test `rule_5_professional_determinations_remain_attributed` |
| 6 | Historical versions cannot be mutated | `finalize_version` freezes the row; draft mutations on a finalized version return `FinalizedVersionImmutable`; no API mutates stored content; test `rule_6_historical_versions_cannot_be_mutated` |
| 7 | A branch must identify its parent | `parent_scenario_version_id` NOT NULL; branch creation is derived from a parent version; test `rule_7_a_branch_identifies_its_parent` |
| 8 | A conflict cannot disappear merely because a new scenario version exists | conflicts live in their own table keyed by version; no delete API; test `rule_8_conflicts_outlive_new_versions` |
| 9 | Changing an owner nonnegotiable creates a new Destination version | scenario versions are pinned, never re-pointed; the change flows through a destination-engine version + a successor scenario version; test `rule_9_nonnegotiable_change_creates_a_new_destination_version` |
| 10 | Scenario data cannot become an implicit recommendation | comparison has no winner/ranking/score anywhere in the type or API; outcomes are factual statuses; test `rule_10_scenario_data_is_never_an_implicit_recommendation` |

---

## 6. Enumerations (canonical values)

Stored as snake_case serde names. Variant sets are the schema doc's lists, verbatim.

### 6.1 `scenario_type` (§41)
`esop` · `direct_employee_purchase` · `management_buyout` ·
`employee_owned_acquisition_entity` · `staged_ownership` ·
`seller_financed_employee_acquisition` · `hybrid` · `retain_and_transition` · `other`

### 6.2 `readiness_status` (§40) — *can the package include it?*
`preliminary` · `information_needed` · `modelable` · `ready_for_comparison` ·
`ready_for_professional_review` · `under_review` · `revised` · `superseded` ·
`archived`

### 6.3 `lifecycle_status` (§40) — *where is it in its life?*
`idea` · `draft` · `preliminary` · `modelable` · `comparable` ·
`under_professional_review` · `revised` · `selected_for_further_exploration` ·
`handed_off` · `historical` · `archived` · `superseded`

Draft lifecycles (mutable window): `idea`, `draft`. Everything else is immutable.

### 6.4 `assumption_category` (§10)
`business` · `financial` · `ownership` · `market` · `financing` · `tax` · `legal` ·
`operational` · `timing` · `employee` · `governance` · `seller` · `buyer` ·
`external_environment` · `other`

### 6.5 `value_type` (§8)
`number` · `currency` · `percentage` · `date` · `date_range` · `duration` · `boolean` ·
`text` · `enum` · `range` · `reference`

### 6.6 `provenance` (§41) — how it entered the system
`owner_reported` · `document_supported` · `research_supported` ·
`professionally_supplied` · `scenario_assumed` · `system_derived` · `model_derived`

### 6.7 `verification` (§11, §41) — how well it is verified (independent axis)
`unknown` · `unverified` · `partially_verified` · `verified` ·
`professionally_verified` · `contested` · `rejected`

### 6.8 `constraint_type` (§12)
`hard` · `strong_preference` · `preference` · `unknown` · `professional_requirement` ·
`external_constraint`

### 6.9 `unknown_importance` (§15)
`critical` · `important` · `helpful`

### 6.10 `unknown_resolution_status` (§15)
`open` · `in_progress` · `resolved` · `waived` · `superseded`

### 6.11 `conflict_type` (§16)
`owner_objective` · `nonnegotiable` · `data` · `research` · `professional` · `model` ·
`dependency` · `other`

### 6.12 `conflict_severity` (§17) — describes the condition, never a score
`informational` · `attention` · `material` · `blocking`

### 6.13 `owner_decision` (NONNEGOTIABLE doc §5)
`keep_requirement` · `explore_another_path` · `change_requirement`

### 6.14 `branch_type` (§30)
`what_if` · `alternative` · `stress_case` · `professional_request` · `owner_request`

### 6.15 `comparison_dimension_type` (§33)
`owner_objective` · `financial` · `ownership` · `personal` · `business` · `timing` ·
`financing` · `seller_note` · `uncertainty` · `information_completeness` · `evidence` ·
`nonnegotiable` · `professional_review` · `outstanding_questions`

### 6.16 `comparison_outcome` — factual, not evaluative (§34)
`aligns` · `does_not_currently_align` · `insufficient_information`

---

## 7. v0.2 structure templates — data, not code

The five ownership structures EOJ v0.2 Stage 14 names ship as data files in
`templates/` (`esop.json`, `direct_employee_purchase.json`,
`management_buyout.json`, `employee_owned_acquisition_entity.json`,
`staged_ownership.json`). Each file carries: `template_id`, `journey_version`
("eoj-0.2"), `scenario_type`, `name`, `what_it_is` (one plain-language sentence),
`potential_characteristics` and `trade_offs` (filled per scenario at runtime, empty in
the shipped seed), and the journey's standard `professional_questions`
(NONNEGOTIABLE doc §14, verbatim). No structure becomes selected merely because it was
displayed (EOJ v0.2 Stage 14) — a template is a seed for a draft, never a proposal.

---

## 8. Engine surface (v1)

The crate is in-memory in v1 (the audit stream is the durable record; SQLite mapping is
the app shell's composition concern, exactly like the reality and destination crates).
Public operations, each audit-first:

`create_family` · `start_draft` · `update_draft` (assumptions, constraints,
nonnegotiables, unknowns, metadata) · `record_conflict` · `record_nonnegotiable_conflict`
· `resolve_conflict` · `record_professional_review_reference` · `finalize_version` ·
`start_successor_draft` · `create_what_if` · `record_comparison` ·
`detect_reality_change` · `archive_family`
plus read accessors (`family`, `version`, `versions_of_family`, `conflicts_of`,
`branches_from`, `comparison`, …).

Deferred with doc receipts: dependencies and all external reference/link tables
(§2.18), selection records and handoff (§2.16–2.17), professional-review flow, and any
network behavior. Scenario owns "the representation of a possible path" — never the
truth of the inputs, the mathematics, the professional determination, the owner's
decision, or the transaction (schema doc §52).
