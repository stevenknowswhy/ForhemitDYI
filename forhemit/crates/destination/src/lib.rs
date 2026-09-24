//! # forhemit-destination
//!
//! The Destination engine — system of record for the owner's desired
//! future state, the journey's persistent North Star (Destination First -
//! Locked Core Architecture).
//!
//! ## What lives here
//!
//! - [`content`] — the DesiredOutcome object tree covering all 13
//!   Destination Builder screens, with allocation validation and content
//!   diffing.
//! - [`objective`] — the objective wrapper: stable reference IDs, answers,
//!   and the 🔴/🟠/🟢 preference states ([`forhemit_contracts::NonnegotiableState`]).
//! - [`answer`] — the answer vocabulary: "I'm not sure" is always a
//!   legitimate answer.
//! - [`version`] — immutable versions linked into a retained chain.
//! - [`destination`] — the aggregate: append-only version history,
//!   process status, and the designated-objectives query downstream
//!   engines back-reference.
//! - [`engine`] — the mutation rules: validate, diff, append, emit one
//!   audit event per material change.
//! - [`completeness`] — the score-free completeness indicator.
//! - [`status`] — the process-state vocabulary.
//!
//! ## The one rule that shapes the crate
//!
//! Destination Engine doc §46 rule 8: "Every material change is versioned."
//! Stored versions are immutable and never edited in place; an edit is a
//! new version appended to the chain with the original retained and
//! queryable. The engine refuses mutations whose audit event cannot be
//! recorded — audit first, per Implementation Roadmap Phase 1.

pub mod answer;
pub mod completeness;
pub mod content;
pub mod destination;
pub mod engine;
pub mod error;
pub mod objective;
pub mod status;
pub mod version;

pub use answer::Answer;
pub use completeness::{AreaState, Completeness, CompletenessLabel};
pub use content::DestinationContent;
pub use destination::Destination;
pub use engine::{DestinationEngine, Edited};
pub use error::DestinationError;
pub use objective::Objective;
pub use status::DestinationStatus;
pub use version::{ChangeReason, DestinationVersion};
