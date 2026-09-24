//! Answers and their version history.
//!
//! "Every answer is versioned; back/edit/change create new versions"
//! (implementation spec, Journey-runtime row) — the original is retained,
//! never edited (Journey Builder doc §9: "The system must not erase the
//! original decision").

use forhemit_contracts::{AnswerId, AnswerVersionId, JourneyNodeId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::scope::StorageScope;

/// The owner's response to one question.
///
/// The shape is validated against the question's interaction type at
/// answer time — a single-select answer is exactly one known choice
/// value; a ranking is a permutation of the resolved choice set.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerValue {
    /// One selected choice value (single-select, yes/no).
    Single(String),
    /// Selected choice values (multi-select, select-up-to-3).
    Multi(Vec<String>),
    /// Ordered choice values, most important first (ranking).
    Ranking(Vec<String>),
    /// A numeric amount in whole currency units (the liquidity target).
    Amount(u64),
    /// An explicit confirmation (review and commitment nodes).
    Confirmed,
}

impl AnswerValue {
    /// The selected values as scalars — a single's one value, a multi's
    /// members, a ranking's ordered values; amount/confirm have none.
    pub fn scalars(&self) -> Vec<&str> {
        match self {
            Self::Single(value) => vec![value.as_str()],
            Self::Multi(values) | Self::Ranking(values) => {
                values.iter().map(String::as_str).collect()
            }
            Self::Amount(_) | Self::Confirmed => Vec::new(),
        }
    }

    /// Whether the answer selects `value` — how `equals` / `in_set`
    /// conditions read an answer.
    pub fn contains(&self, value: &str) -> bool {
        self.scalars().into_iter().any(|v| v == value)
    }

    /// How many values the answer carries — how `count_at_least`
    /// conditions read an answer.
    pub fn count(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multi(values) | Self::Ranking(values) => values.len(),
            Self::Amount(_) | Self::Confirmed => 1,
        }
    }
}

/// One immutable answer version. A revision appends a new version and
/// points it at the one it supersedes; the superseded version is
/// retained verbatim.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerVersion {
    /// The version's identity.
    pub version_id: AnswerVersionId,
    /// One-based version number within the answer's history.
    pub version: u32,
    /// The owner's response.
    pub value: AnswerValue,
    /// The Storage Scope the question declared — recorded on the version
    /// itself, so placement survives even if the definition later
    /// revises the question (Journey Builder doc §6).
    pub storage_scope: StorageScope,
    /// When the version was recorded.
    pub recorded_at: OffsetDateTime,
    /// The version this one supersedes — None for a first version.
    pub supersedes: Option<AnswerVersionId>,
    /// Why the answer changed — always present on a revision, never on a
    /// first version.
    pub change_reason: Option<String>,
}

/// One question's full versioned answer — the stable identity (per
/// node) that survives revisions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerRecord {
    /// The answer's stable identity across all versions.
    pub answer_id: AnswerId,
    /// The question the answer belongs to.
    pub node_id: JourneyNodeId,
    /// All versions, oldest first. The latest is the answer's current
    /// value; the rest are history the walk must not erase.
    pub versions: Vec<AnswerVersion>,
}

impl AnswerRecord {
    /// The answer's current value — its latest version.
    pub fn latest(&self) -> &AnswerVersion {
        self.versions
            .last()
            .unwrap_or_else(|| unreachable!("an answer record always has at least one version"))
    }
}
