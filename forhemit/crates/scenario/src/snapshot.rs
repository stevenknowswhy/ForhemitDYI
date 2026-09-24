//! The scenario snapshot (Scenario Engine Data Model doc §6, §51): the
//! pinned reference set that answers "what did we actually know when we
//! made this scenario?". 1:1 with a scenario version.

use forhemit_contracts::{
    BusinessRealityVersionId, DestinationVersionId, FactVersionId, FinancialModelVersionId,
    ScenarioVersionId, SnapshotId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The stored snapshot row (schema doc §6). The snapshot **references**
/// authoritative versions rather than duplicating them (schema doc §6);
/// the fact-version ids are pinned so the derived Business Reality
/// version address stays verifiable later.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioSnapshot {
    /// The snapshot's id.
    pub snapshot_id: SnapshotId,
    /// The version this snapshot belongs to.
    pub scenario_version_id: ScenarioVersionId,
    /// Mirrors the version's rule-1 destination reference.
    pub destination_version_id: DestinationVersionId,
    /// Mirrors the version's rule-2 reality reference.
    pub business_reality_version_id: BusinessRealityVersionId,
    /// The exact fact versions pinned at creation.
    pub reality_fact_version_ids: Vec<FactVersionId>,
    /// Reserved for the financial modeling engine (integrity rule 3
    /// keeps the type required on any future model-output reference).
    pub financial_model_version_id: Option<FinancialModelVersionId>,
    /// Reserved for the research engine.
    pub research_snapshot_id: Option<String>,
    /// Reserved for the evidence engine.
    pub evidence_snapshot_id: Option<String>,
    /// Reserved reference set (schema doc §25).
    pub financing_reference_set: Option<String>,
    /// Reserved reference set (schema doc §26).
    pub seller_note_reference_set: Option<String>,
    /// Reserved reference set (schema doc §27).
    pub professional_review_reference_set: Option<String>,
    /// When the snapshot was taken.
    pub created_at: OffsetDateTime,
}
