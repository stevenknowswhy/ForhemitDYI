//! The input ports: what the package engine reads.
//!
//! The engine depends only on `contracts` + `enginekit`, so the state of
//! the sibling engines arrives through these snapshot types — plain,
//! contract-typed views the app shell (the composition root) fills from
//! the journey, destination, reality, and scenario engines. The ports
//! carry *rendered, owner-facing text* for values, because rendering a
//! `ProceedsChoice` or a `FactRange` is those engines' vocabulary, not
//! this crate's; what the package engine owns is assembly, gating, and
//! honest display (PRP doc §57: the engine owns "package assembly", not
//! the underlying information).
//!
//! The one exception is the readiness gate: scenario readiness arrives
//! typed ([`ExportReadiness`]), because gating on a string would let a
//! typo export a draft scenario.

use forhemit_contracts::{
    BusinessRealityVersionId, DestinationId, DestinationVersionId, JourneyId, NonnegotiableState,
    Provenance, ScenarioVersionId, Verification, WorkspaceId,
};
use serde::{Deserialize, Serialize};

/// The scenario readiness vocabulary, as the package gate sees it
/// (Scenario Engine Data Model doc §40: readiness is "can the package
/// include it?"). Mirrored from the scenario crate's `ReadinessStatus`
/// so the runtime boundary stays clean; the mapping is exhaustive, so a
/// new readiness state forces the shell's adapter to decide.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportReadiness {
    /// Preliminary — early, honest about it.
    Preliminary,
    /// A named gap exists.
    InformationNeeded,
    /// Structured enough to compute over.
    Modelable,
    /// Ready to appear in comparisons.
    ReadyForComparison,
    /// Ready for the professional review package — the only readiness
    /// the package includes.
    ReadyForProfessionalReview,
    /// Currently under professional review.
    UnderReview,
    /// Revised after review.
    Revised,
    /// Superseded by a successor version.
    Superseded,
    /// Archived with its family.
    Archived,
}

/// The journey half of the snapshot: the walk that produced the owner's
/// answers (Journey Builder doc §23 — a package records "Journey Version
/// Used").
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JourneySnapshot {
    /// The journey definition the walk follows (e.g. `employee_ownership`).
    pub journey_id: JourneyId,
    /// The definition version the instance was started under — recorded
    /// on the package and in every export event.
    pub journey_version_used: String,
    /// Whether the walk has completed. An in-progress walk may still be
    /// exported — the owner decides — but the package says so.
    pub completed: bool,
    /// The walk's answers, one line per question actually asked, in walk
    /// order. Values are rendered as the owner saw them.
    pub answers: Vec<AnswerLine>,
    /// Nodes the owner passed without answering — question exits (EOJ
    /// v0.2 §30), by node id.
    pub skipped_node_ids: Vec<String>,
    /// Requirements marked nonnegotiable through answer effects (EOJ
    /// v0.2 §11), as the walk recorded them.
    pub marked_nonnegotiables: Vec<String>,
}

/// One journey answer, rendered.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerLine {
    /// The question node's id — the stable key the audit stream uses.
    pub node_id: String,
    /// The node's title — the question as the owner saw it.
    pub title: String,
    /// The chosen values, rendered as labels. Empty when the answer is a
    /// skip or a confirmation.
    pub values: Vec<String>,
}

/// The destination half of the snapshot: the DesiredOutcome version the
/// package is assembled against (Destination First — the "persistent
/// North Star").
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DestinationSnapshot {
    /// The destination's identity.
    pub destination_id: DestinationId,
    /// The version the package records — the exact provenance the spec
    /// requires ("records Journey Version Used and destination version").
    pub version_id: DestinationVersionId,
    /// The version's 1-based number in the destination's history.
    pub version_number: u32,
    /// The version's objectives, rendered, with their preference
    /// strengths — the 🔴/🟠/🟢 states (NONNEGOTIABLE doc §12).
    pub objectives: Vec<ObjectiveLine>,
}

/// One destination objective, rendered with its preference strength.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectiveLine {
    /// What the objective is about (e.g. "Desired cash at closing").
    pub label: String,
    /// The owner's answer, rendered — including an explicit "I'm not
    /// sure", which is a first-class answer (Destination Builder doc §1).
    pub value: String,
    /// How strongly the objective is held.
    pub preference: NonnegotiableState,
}

/// The Business Reality half of the snapshot: the owner-reported fact
/// set (Business Reality doc §8's Level-1 Business Snapshot).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RealitySnapshot {
    /// The Business Reality version the scenarios were built against
    /// (Scenario Engine Data Model doc §4, integrity rule 2) — recorded
    /// in the package's provenance block.
    pub business_reality_version_id: BusinessRealityVersionId,
    /// The current facts, rendered, with their provenance and
    /// verification stamps.
    pub facts: Vec<FactLine>,
}

/// One Business Reality fact, rendered with its provenance stamps
/// (PRP doc §27: provenance is never blurred).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactLine {
    /// What the fact states (e.g. "Revenue").
    pub kind: String,
    /// The owner-stated value, rendered.
    pub value: String,
    /// The period the figure describes (e.g. "current", "FY2025").
    pub period: String,
    /// The metric label as the owner stated it, if any.
    pub definition: Option<String>,
    /// How the value entered the system.
    pub provenance: Provenance,
    /// How well the value is verified.
    pub verification: Verification,
}

/// One scenario version's package-facing state.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioSnapshot {
    /// The version's id — recorded in the provenance block when included.
    pub scenario_version_id: ScenarioVersionId,
    /// The family (conceptual path) the version belongs to.
    pub family_name: String,
    /// The version's name.
    pub name: String,
    /// The version's description, if any.
    pub description: Option<String>,
    /// The path type (e.g. "ESOP"), rendered.
    pub scenario_type: String,
    /// The version's lifecycle state, rendered (schema doc §40).
    pub lifecycle_status: String,
    /// The readiness state — the export gate reads this.
    pub readiness: ExportReadiness,
    /// The version's assumptions, rendered with provenance stamps.
    pub assumptions: Vec<AssumptionLine>,
    /// The version's named unknowns — first-class, never blank zeros
    /// (schema doc §15).
    pub unknowns: Vec<UnknownLine>,
    /// The version's recorded conflicts, as conditions with severity in
    /// words (schema doc §16: severity describes the condition, never a
    /// score).
    pub conflicts: Vec<ConflictLine>,
}

/// One scenario assumption, rendered with its provenance and verification
/// stamps (schema doc §9, §11).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssumptionLine {
    /// The assumption's name — the label the owner sees.
    pub name: String,
    /// The longer description, if any.
    pub description: Option<String>,
    /// The typed value, rendered.
    pub value: String,
    /// What the assumption is about, rendered (e.g. "financing").
    pub category: String,
    /// How the value entered the system.
    pub provenance: Provenance,
    /// How well the value is verified.
    pub verification: Verification,
}

/// One named unknown, rendered (schema doc §15). The package shows the
/// gap — never a zero standing in for it.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnknownLine {
    /// What is missing, in plain language.
    pub description: String,
    /// How much it matters, rendered ("critical", "important", "helpful").
    pub importance: String,
    /// Where the unknown stands, rendered ("open", "in_progress", …).
    pub resolution_status: String,
}

/// One recorded conflict, rendered as a condition (schema doc §16).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConflictLine {
    /// What the conflict is.
    pub description: String,
    /// The condition's weight, in words ("informational", "attention",
    /// "material", "blocking") — never a number.
    pub severity: String,
}

/// The whole workspace state a package is assembled from — the shell's
/// one mapping target.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceSnapshot {
    /// The workspace the state belongs to.
    pub workspace_id: WorkspaceId,
    /// The journey walk.
    pub journey: JourneySnapshot,
    /// The destination version.
    pub destination: DestinationSnapshot,
    /// The Business Reality fact set.
    pub reality: RealitySnapshot,
    /// Every scenario version the owner has, ready or not — the gate
    /// decides inclusion and names the exclusions.
    pub scenarios: Vec<ScenarioSnapshot>,
}
