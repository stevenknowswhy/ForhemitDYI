//! The two lifecycle axes (Scenario Engine Data Model doc §40) and the
//! scenario-type vocabulary (§41).
//!
//! **Readiness** ("can the package include it?") and **lifecycle** ("where
//! is it in its life?") are separate axes and must never be blended — the
//! separation is what keeps scenario data from becoming an implicit
//! recommendation (integrity rule 10).

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Where a scenario version is in its life (schema doc §40). Only the
/// draft-window states are mutable; everything else is history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStatus {
    /// A captured idea — mutable.
    Idea,
    /// A working draft — mutable.
    Draft,
    /// Finalized; first post-draft state.
    Preliminary,
    /// Structured enough to compute over.
    Modelable,
    /// Ready to appear in comparisons.
    Comparable,
    /// With a professional.
    UnderProfessionalReview,
    /// Revised after professional feedback.
    Revised,
    /// Marked for further exploration (a selection record, not a
    /// recommendation — schema doc §37).
    SelectedForFurtherExploration,
    /// Handed to the transaction layer (schema doc §38–39).
    HandedOff,
    /// Superseded in history; retained.
    Historical,
    /// Archived; retained.
    Archived,
    /// Superseded by a successor; retained.
    Superseded,
}

impl LifecycleStatus {
    /// True while the version may still be edited in place (the mutable
    /// draft window — schema doc §46).
    pub fn is_draft(self) -> bool {
        matches!(self, Self::Idea | Self::Draft)
    }

    /// The status's serde name — error messages name the same
    /// vocabulary the schema and events use.
    pub fn status_name(self) -> &'static str {
        match self {
            Self::Idea => "idea",
            Self::Draft => "draft",
            Self::Preliminary => "preliminary",
            Self::Modelable => "modelable",
            Self::Comparable => "comparable",
            Self::UnderProfessionalReview => "under_professional_review",
            Self::Revised => "revised",
            Self::SelectedForFurtherExploration => "selected_for_further_exploration",
            Self::HandedOff => "handed_off",
            Self::Historical => "historical",
            Self::Archived => "archived",
            Self::Superseded => "superseded",
        }
    }
}

/// Whether the package can include this version (schema doc §40) — the
/// export-gating axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStatus {
    /// Preliminary — early, honest about it.
    Preliminary,
    /// A named gap exists (`information_needed` rows in the journey).
    InformationNeeded,
    /// Structured enough to compute over.
    Modelable,
    /// Ready to appear in comparisons.
    ReadyForComparison,
    /// Ready for the professional review package.
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

impl ReadinessStatus {
    /// The status's serde name.
    pub fn status_name(self) -> &'static str {
        match self {
            Self::Preliminary => "preliminary",
            Self::InformationNeeded => "information_needed",
            Self::Modelable => "modelable",
            Self::ReadyForComparison => "ready_for_comparison",
            Self::ReadyForProfessionalReview => "ready_for_professional_review",
            Self::UnderReview => "under_review",
            Self::Revised => "revised",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }
}

/// The scenario-type vocabulary (schema doc §41).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioType {
    /// Employee stock ownership plan.
    Esop,
    /// Employees purchase the business directly.
    DirectEmployeePurchase,
    /// The existing management team buys the business.
    ManagementBuyout,
    /// A new entity owned by employees is formed to acquire the company.
    EmployeeOwnedAcquisitionEntity,
    /// Ownership transfers to employees in stages over time.
    StagedOwnership,
    /// Employee acquisition financed by a seller note.
    SellerFinancedEmployeeAcquisition,
    /// A blend of structures.
    Hybrid,
    /// The owner retains the company and transitions gradually.
    RetainAndTransition,
    /// None of the above — the owner is exploring something else.
    Other,
}

/// One row of the version's status history (schema doc §36) — immutable,
/// append-only. A single entry may record both axes at once (finalizing
/// changes lifecycle and pins readiness).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusHistoryEntry {
    /// Lifecycle before the transition; `None` on the initial entry.
    pub previous_lifecycle: Option<LifecycleStatus>,
    /// Lifecycle after the transition.
    pub new_lifecycle: LifecycleStatus,
    /// Readiness before the transition; `None` on the initial entry.
    pub previous_readiness: Option<ReadinessStatus>,
    /// Readiness after the transition.
    pub new_readiness: ReadinessStatus,
    /// Why the status moved, as stated by the actor.
    pub reason: Option<String>,
    /// Who made the transition.
    pub changed_by: forhemit_contracts::ActorRecord,
    /// When the transition happened.
    pub changed_at: OffsetDateTime,
}
