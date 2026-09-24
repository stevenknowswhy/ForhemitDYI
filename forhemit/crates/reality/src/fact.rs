//! The Business Snapshot fact model — Level 1 of the Business Reality
//! engine's progressive detail (Business Reality doc §8).
//!
//! Every fact is owner-stated: it is stamped
//! [`Provenance::OwnerReported`] and [`Verification::Unverified`] (THREE
//! DECISION LAYERS: what the owner reports is Owner Objective-layer
//! material, never a platform or professional conclusion). The
//! provenance and verification vocabularies carry the document-supported
//! and professionally-verified levels for later activation — Expanded
//! Reality §5: "owner verified ≠ professionally verified" — but there is
//! no v1 construction path that claims a verification the platform did
//! not perform (Business Reality doc §29: never "treat estimates as
//! verified information").
//!
//! Values follow doc §7, "Do not force precision too early": ranges with
//! open ends ("Under $1M", "$25M+") are first-class, and a metric's
//! definition label is preserved as the owner stated it (Expanded
//! Reality §12 — "Adjusted EBITDA" is never silently converted to
//! "EBITDA").

use forhemit_contracts::{FactId, FactVersionId, Provenance, Verification};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::RealityError;

/// Which field of the Business Snapshot a fact states — the Level-1
/// snapshot fields (Business Reality doc §8).
///
/// Variants are additive: a later level of the snapshot adds new kinds as
/// new cases, never by reinterpreting existing facts.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FactKind {
    /// What the business does (doc §3, Company Identity).
    Industry,
    /// How long the business has operated (doc §8).
    YearsOperating,
    /// Annual revenue — a range or, once the owner knows it, an exact
    /// figure in dollars (doc §7).
    Revenue,
    /// Annual operating cash flow / EBITDA range (doc §8, §9).
    OperatingCashFlow,
    /// Outstanding debt (doc §8; feeds the doc §16 debt flag later).
    Debt,
    /// Number of employees (doc §8).
    EmployeeCount,
    /// Ownership structure — e.g. founder-owned, two partners (doc §8).
    OwnershipStructure,
}

/// The measurement unit a fact kind is stated in (Business Reality doc
/// §31: a field carries Value and Unit).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FactUnit {
    /// Whole dollars.
    Dollars,
    /// Years (how long the business has operated).
    Years,
    /// A headcount.
    Count,
    /// A free-text description.
    Text,
}

impl FactKind {
    /// The unit this kind is stated in.
    #[must_use]
    pub fn unit(self) -> FactUnit {
        match self {
            Self::Revenue | Self::OperatingCashFlow | Self::Debt => FactUnit::Dollars,
            Self::YearsOperating => FactUnit::Years,
            Self::EmployeeCount => FactUnit::Count,
            Self::Industry | Self::OwnershipStructure => FactUnit::Text,
        }
    }

    /// Whether a value's shape fits this kind — enforced at fact
    /// construction so a revenue fact can never hold an industry string.
    #[must_use]
    pub fn accepts(self, value: &FactValue) -> bool {
        match self {
            Self::Industry | Self::OwnershipStructure => {
                matches!(value, FactValue::Text(_))
            }
            Self::YearsOperating
            | Self::EmployeeCount
            | Self::Revenue
            | Self::OperatingCashFlow
            | Self::Debt => !matches!(value, FactValue::Text(_)),
        }
    }

    /// The value shapes this kind accepts, for error messages.
    #[must_use]
    pub fn accepted_shapes(self) -> &'static str {
        match self {
            Self::Industry | Self::OwnershipStructure => "text",
            Self::YearsOperating
            | Self::EmployeeCount
            | Self::Revenue
            | Self::OperatingCashFlow
            | Self::Debt => "number or range",
        }
    }
}

/// The owner-stated value of a fact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FactValue {
    /// A descriptive answer (industry, ownership structure).
    Text(String),
    /// An exact whole-number figure (Business Reality doc §7: exact
    /// information comes later, but is accepted once the owner knows it).
    Number(u64),
    /// A range with optional bounds — "Under $1M", "$5M–$10M", "$25M+".
    Range(FactRange),
}

impl FactValue {
    /// The value's shape name, for error messages.
    #[must_use]
    pub fn shape_name(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Number(_) => "number",
            Self::Range(_) => "range",
        }
    }
}

/// A range with optional bounds (Business Reality doc §7). At least one
/// bound must be present, and a present lower bound never exceeds a
/// present upper bound.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactRange {
    /// The inclusive lower bound, or none for an open lower end
    /// ("$25M and up").
    pub lower: Option<u64>,
    /// The inclusive upper bound, or none for an open upper end
    /// ("under $1M").
    pub upper: Option<u64>,
}

impl FactRange {
    /// Builds a validated range.
    ///
    /// # Errors
    ///
    /// [`RealityError::InvalidFactRange`] when both bounds are missing or
    /// the lower bound exceeds the upper bound.
    pub fn new(lower: Option<u64>, upper: Option<u64>) -> Result<Self, RealityError> {
        let range = Self { lower, upper };
        range.validate()?;
        Ok(range)
    }

    /// Checks the range's invariants. Fact construction runs this even
    /// for values built directly as enum variants, so an inverted or
    /// empty range cannot become a fact.
    ///
    /// # Errors
    ///
    /// [`RealityError::InvalidFactRange`] as in [`FactRange::new`].
    pub fn validate(&self) -> Result<(), RealityError> {
        match (self.lower, self.upper) {
            (None, None) => Err(RealityError::InvalidFactRange {
                reason: "at least one bound must be present",
            }),
            (Some(lower), Some(upper)) if lower > upper => Err(RealityError::InvalidFactRange {
                reason: "the lower bound exceeds the upper bound",
            }),
            _ => Ok(()),
        }
    }
}

/// Which period a fact's figure describes (Expanded Reality §13: "Date
/// and Period Are Mandatory" — old information must never be mistaken
/// for current reality).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FactPeriod {
    /// The fact describes the business right now.
    Current,
    /// A fiscal-year figure, e.g. FY2025.
    FiscalYear(u16),
    /// A calendar-year figure, e.g. 2025.
    CalendarYear(u16),
}

/// Everything a new fact version carries apart from its own version id
/// and timestamp — the input to [`FactVersion::owner_reported`]. No
/// provenance or verification field exists here: the constructor stamps
/// those, and it stamps them honestly.
#[derive(Clone, Debug)]
pub struct FactVersionInput {
    /// The fact's stable identity, shared by every version.
    pub fact_id: FactId,
    /// Which Business Snapshot kind the fact records.
    pub kind: FactKind,
    /// The stated value.
    pub value: FactValue,
    /// Which period the figure describes.
    pub period: FactPeriod,
    /// The owner's own definition label, if any.
    pub definition: Option<String>,
    /// The version this one supersedes, for revisions.
    pub supersedes: Option<FactVersionId>,
}

/// One immutable version of one Business Snapshot fact (Business Reality
/// doc §25: "Historical snapshots remain available" — a revision creates
/// a new version and never edits the one it supersedes).
///
/// All fields are read through getters; the constructors are the only
/// way in. [`FactVersion::owner_reported`] is v1's sole constructor and
/// stamps [`Provenance::OwnerReported`] / [`Verification::Unverified`] —
/// the v1 platform performs no verification, so no fact it produces can
/// claim one. Deserialization preserves whatever a stored document says
/// (doc §6: preserve the stated/supported/verified difference), which
/// keeps the data layer forward-compatible with the later activation of
/// the verification levels.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactVersion {
    fact_id: FactId,
    fact_version_id: FactVersionId,
    kind: FactKind,
    value: FactValue,
    period: FactPeriod,
    definition: Option<String>,
    provenance: Provenance,
    verification: Verification,
    supersedes: Option<FactVersionId>,
    recorded_at: OffsetDateTime,
}

impl FactVersion {
    /// Records the first or a revised version of an owner-stated fact.
    ///
    /// The fact's stable identity lives in `input`; `fact_version_id`
    /// names this immutable version and `recorded_at` stamps it. A
    /// revision sets `input.supersedes` to the version it replaces and
    /// keeps the same fact id.
    ///
    /// # Errors
    ///
    /// [`RealityError::KindValueMismatch`] when the value's shape does
    /// not fit the kind; [`RealityError::InvalidFactRange`] for a
    /// malformed range; [`RealityError::EmptyTextValue`] /
    /// [`RealityError::EmptyDefinition`] for blank text.
    pub fn owner_reported(
        input: FactVersionInput,
        fact_version_id: FactVersionId,
        recorded_at: OffsetDateTime,
    ) -> Result<Self, RealityError> {
        let FactVersionInput {
            fact_id,
            kind,
            value,
            period,
            definition,
            supersedes,
        } = input;
        if !kind.accepts(&value) {
            return Err(RealityError::KindValueMismatch {
                kind,
                actual: value.shape_name(),
                accepted: kind.accepted_shapes(),
            });
        }
        if let FactValue::Text(text) = &value {
            if text.trim().is_empty() {
                return Err(RealityError::EmptyTextValue);
            }
        }
        if let FactValue::Range(range) = &value {
            range.validate()?;
        }
        if definition
            .as_ref()
            .is_some_and(|text| text.trim().is_empty())
        {
            return Err(RealityError::EmptyDefinition);
        }
        Ok(Self {
            fact_id,
            fact_version_id,
            kind,
            value,
            period,
            definition,
            // The v1 honesty stamps: the owner states this fact and the
            // platform has verified nothing about it (doc §29).
            provenance: Provenance::OwnerReported,
            verification: Verification::Unverified,
            supersedes,
            recorded_at,
        })
    }

    /// The fact's stable identity — survives revisions, referenced by
    /// scenario assumptions.
    #[must_use]
    pub fn fact_id(&self) -> &FactId {
        &self.fact_id
    }

    /// This immutable version's identifier.
    #[must_use]
    pub fn fact_version_id(&self) -> &FactVersionId {
        &self.fact_version_id
    }

    /// Which Business Snapshot field this fact states.
    #[must_use]
    pub fn kind(&self) -> FactKind {
        self.kind
    }

    /// The owner-stated value.
    #[must_use]
    pub fn value(&self) -> &FactValue {
        &self.value
    }

    /// The period the figure describes.
    #[must_use]
    pub fn period(&self) -> FactPeriod {
        self.period
    }

    /// The metric label or qualifier as the owner stated it
    /// (Expanded Reality §12), when given.
    #[must_use]
    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    /// How this fact entered the system — always
    /// [`Provenance::OwnerReported`] in v1.
    #[must_use]
    pub fn provenance(&self) -> Provenance {
        self.provenance.clone()
    }

    /// How well the fact is verified — always
    /// [`Verification::Unverified`] in v1.
    #[must_use]
    pub fn verification(&self) -> Verification {
        self.verification.clone()
    }

    /// The version this one supersedes, for revisions.
    #[must_use]
    pub fn supersedes(&self) -> Option<&FactVersionId> {
        self.supersedes.as_ref()
    }

    /// When the owner stated this version — the "current as of" of the
    /// fact (Business Reality doc §26).
    #[must_use]
    pub fn recorded_at(&self) -> OffsetDateTime {
        self.recorded_at
    }
}
