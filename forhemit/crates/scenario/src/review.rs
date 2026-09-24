//! Professional review references (Scenario Engine Data Model doc §27,
//! §29): the attribution boundary. The professional's actual
//! determination is owned by the professional review engine; the scenario
//! stores only the reference — who reviewed, in what role, and where
//! their feedback lives. A reference never flips the scenario into an
//! approved state (§29).

use forhemit_contracts::{ProfessionalReviewId, ProfessionalReviewReferenceId, ScenarioVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Where the review process stands (schema doc §27). Process vocabulary
/// only: no status here means "professionally approved".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfessionalReviewStatus {
    /// No review has been requested.
    NotReviewed,
    /// A review is pending.
    AwaitingReview,
    /// A review has happened — see the feedback reference for what was
    /// said.
    Reviewed,
}

/// The stored reference row (schema doc §27). Role and identity are
/// required — an anonymous "a professional reviewed this" cannot be
/// recorded (integrity rule 5: "Every professional determination remains
/// attributed to its source").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionalReviewReference {
    /// The reference's id.
    pub reference_id: ProfessionalReviewReferenceId,
    /// The scenario version the review relates to.
    pub scenario_version_id: ScenarioVersionId,
    /// The review's id — reserved for the professional review engine and
    /// carried opaquely in v1.
    pub professional_review_id: ProfessionalReviewId,
    /// Where the review process stands.
    pub review_status: ProfessionalReviewStatus,
    /// The professional's role (e.g. "cpa", "esop_valuation_advisor").
    pub professional_role: String,
    /// Who the professional is — an identity reference, not a free-form
    /// opinion.
    pub professional_identity_reference: String,
    /// Where the feedback lives, if it exists yet.
    pub feedback_reference: Option<String>,
    /// When the reference was recorded.
    pub created_at: OffsetDateTime,
}

/// The request half of recording a review reference. Role and identity
/// are required fields, not options — the type makes an unattributed
/// reference unconstructible.
#[derive(Clone, Debug)]
pub struct NewReviewReference {
    /// The review's id (carried opaquely in v1).
    pub professional_review_id: ProfessionalReviewId,
    /// Where the review process stands.
    pub review_status: ProfessionalReviewStatus,
    /// The professional's role.
    pub professional_role: String,
    /// Who the professional is.
    pub professional_identity_reference: String,
    /// Where the feedback lives, if it exists yet.
    pub feedback_reference: Option<String>,
}
