//! # forhemit-reality
//!
//! The Business Reality engine — the current-state fact base of the
//! Forhemit desktop app ("What does the business look like right now?",
//! Business Reality doc §1).
//!
//! ## What v1 owns
//!
//! The Level-1 Business Snapshot fact set (Business Reality doc §8):
//! industry, years operating, revenue, operating cash flow, debt,
//! employee count, and ownership structure — each stated by the owner as
//! one fact carrying field-level provenance (doc §31): value, period,
//! definition label, provenance, verification, and version lineage.
//!
//! ## The honesty invariants
//!
//! - Facts are stamped [`Provenance::OwnerReported`] and
//!   [`Verification::Unverified`]. The vocabularies carry the
//!   document-supported and professionally-verified levels for later
//!   activation (Expanded Reality §5: owner verified ≠ professionally
//!   verified), but no v1 construction path claims a verification the
//!   platform did not perform (Business Reality doc §29: never "treat
//!   estimates as verified information").
//! - A revision creates a new immutable fact version and retains the
//!   history (doc §25); every mutation emits exactly one audit event,
//!   and a fact version exists only if its event was accepted
//!   (audit-first, Implementation Roadmap Phase 1).
//! - "I don't know" is the absence of a fact — never a stored zero or a
//!   blank value.
//!
//! The engine describes reality; it does not judge the destination
//! (doc §41) — no scores, no feasibility, no recommendation.

pub mod engine;
pub mod error;
pub mod fact;

pub use engine::{FactRecordedPayload, FactRevisedPayload, RealityEngine, RecordFact, ReviseFact};
pub use error::RealityError;
pub use fact::{
    FactKind, FactPeriod, FactRange, FactUnit, FactValue, FactVersion, FactVersionInput,
};
