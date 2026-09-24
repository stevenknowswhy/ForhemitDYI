//! Durable comparisons (Scenario Engine Data Model doc §32–34): a
//! comparison captures what was compared and the factual outcome per
//! dimension — never which scenario won (integrity rule 10: "Scenario
//! data cannot become an implicit recommendation").

use forhemit_contracts::{
    ActorRecord, ComparisonDimensionId, ComparisonId, ComparisonResultId, ScenarioVersionId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::value::TypedValue;

/// What a comparison dimension compares (schema doc §33).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonDimensionType {
    /// Against an owner objective.
    OwnerObjective,
    /// Financial figures.
    Financial,
    /// Ownership structure.
    Ownership,
    /// Personal circumstances.
    Personal,
    /// Business continuity.
    Business,
    /// Timing.
    Timing,
    /// Financing.
    Financing,
    /// Seller-note terms.
    SellerNote,
    /// Uncertainty.
    Uncertainty,
    /// How complete the information is.
    InformationCompleteness,
    /// Evidence quality.
    Evidence,
    /// Nonnegotiable alignment.
    Nonnegotiable,
    /// Professional review state.
    ProfessionalReview,
    /// Open questions.
    OutstandingQuestions,
}

/// The factual status of one version on one dimension (schema doc §34) —
/// no rank, no score.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOutcome {
    /// The scenario aligns with the dimension's reference point.
    Aligns,
    /// The scenario does not currently align.
    DoesNotCurrentlyAlign,
    /// Not enough information to say.
    InsufficientInformation,
}

/// One comparison dimension (schema doc §33).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonDimension {
    /// The dimension's id.
    pub dimension_id: ComparisonDimensionId,
    /// The comparison this dimension belongs to.
    pub comparison_id: ComparisonId,
    /// What the dimension compares.
    pub dimension_type: ComparisonDimensionType,
    /// The dimension's label.
    pub label: String,
    /// Display order — presentation, never ranking.
    pub display_order: u32,
}

/// One result cell (schema doc §34).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonResult {
    /// The result's id.
    pub result_id: ComparisonResultId,
    /// The comparison this result belongs to.
    pub comparison_id: ComparisonId,
    /// The scenario version the result is about.
    pub scenario_version_id: ScenarioVersionId,
    /// The dimension the result is about.
    pub dimension_id: ComparisonDimensionId,
    /// The factual reading, if one exists (e.g. "24 months").
    pub value: Option<TypedValue>,
    /// The factual status.
    pub outcome: ComparisonOutcome,
    /// Where the reading came from.
    pub source_reference: Option<String>,
    /// When the result was recorded.
    pub created_at: OffsetDateTime,
}

/// The stored comparison (schema doc §32): name, what was compared, and
/// the dimensions/results tables. No winner column exists anywhere.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioComparison {
    /// The comparison's id.
    pub comparison_id: ComparisonId,
    /// What the owner called the comparison.
    pub name: String,
    /// The scenario versions compared — at least two.
    pub scenario_version_ids: Vec<ScenarioVersionId>,
    /// The dimensions compared on.
    pub dimensions: Vec<ComparisonDimension>,
    /// The recorded results.
    pub results: Vec<ComparisonResult>,
    /// When the comparison was recorded.
    pub created_at: OffsetDateTime,
    /// Who recorded it.
    pub created_by: ActorRecord,
}

/// The request half of `ScenarioEngine::record_comparison`.
#[derive(Clone, Debug)]
pub struct NewComparison {
    /// What the owner calls the comparison.
    pub name: String,
    /// The scenario versions compared — at least two.
    pub scenario_version_ids: Vec<ScenarioVersionId>,
    /// The dimensions compared on.
    pub dimensions: Vec<NewComparisonDimension>,
    /// The results; `dimension_index` refers to the `dimensions` vec.
    pub results: Vec<NewComparisonResult>,
}

/// A new dimension definition.
#[derive(Clone, Debug)]
pub struct NewComparisonDimension {
    /// What the dimension compares.
    pub dimension_type: ComparisonDimensionType,
    /// The dimension's label.
    pub label: String,
    /// Display order — presentation, never ranking.
    pub display_order: u32,
}

/// A new result cell; `dimension_index` refers to the position of the
/// dimension in the `NewComparison::dimensions` vec.
#[derive(Clone, Debug)]
pub struct NewComparisonResult {
    /// The scenario version the result is about — must be in the
    /// comparison's version set.
    pub scenario_version_id: ScenarioVersionId,
    /// The dimension's index in the comparison being recorded.
    pub dimension_index: usize,
    /// The factual reading, if one exists.
    pub value: Option<TypedValue>,
    /// The factual status.
    pub outcome: ComparisonOutcome,
    /// Where the reading came from.
    pub source_reference: Option<String>,
}
