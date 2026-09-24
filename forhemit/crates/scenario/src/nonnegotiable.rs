//! The destination's nonnegotiables under test (Scenario Engine Data
//! Model doc §13): the scenario rows that carry the destination
//! back-reference into scenario land.
//!
//! A nonnegotiable conflict is a [`crate::NonnegotiableConflict`] — a
//! structured object the owner decides on (NONNEGOTIABLE doc §5), never
//! a boolean warning and never silently relaxed
//! ("A nonnegotiable is never silently relaxed, ignored, or overridden").

use forhemit_contracts::{DestinationVersionId, NonnegotiableId, ObjectiveId, ScenarioVersionId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::value::TypedValue;

/// The stored nonnegotiable row (schema doc §13).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioNonnegotiable {
    /// The row's id.
    pub nonnegotiable_id: NonnegotiableId,
    /// The draft version the nonnegotiable belongs to.
    pub scenario_version_id: ScenarioVersionId,
    /// The destination objective being tested — `ObjectiveId` is stable
    /// across destination versions.
    pub destination_objective_id: ObjectiveId,
    /// The destination version whose designation is being tested.
    pub destination_version_id: DestinationVersionId,
    /// What the requirement says, in the owner's words.
    pub description: String,
    /// The requirement's value, if it has one (e.g. the minimum proceeds
    /// figure).
    pub value: Option<TypedValue>,
    /// When the row was added.
    pub created_at: OffsetDateTime,
}

/// The request half of adding a nonnegotiable to a draft. The engine
/// fills the row's id, version scope, and timestamp.
#[derive(Clone, Debug)]
pub struct NewNonnegotiable {
    /// The destination objective being tested.
    pub destination_objective_id: ObjectiveId,
    /// The destination version whose designation is being tested — must
    /// match the scenario version's pinned destination version.
    pub destination_version_id: DestinationVersionId,
    /// What the requirement says.
    pub description: String,
    /// The requirement's value, if it has one.
    pub value: Option<TypedValue>,
}
