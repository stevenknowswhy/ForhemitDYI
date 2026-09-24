//! # forhemit-journey
//!
//! The Journey Engine: the conversation that discovers the owner's intent,
//! run as a **stateful process, not a form** (Journey Builder Architecture
//! doc §8).
//!
//! The journey is data. [`definition::JourneyDefinition`] loads a versioned
//! JSON definition file — questions, choices, per-question metadata
//! (Storage Scope, information value, decision layer), follow-up rules, and
//! "Ask Only When Relevant" conditionals (EOJ v0.2 §28) — and the runtime
//! walks it: the Employee Ownership Journey v0.2 text lives in
//! `journeys/employee_ownership_v0.2.json`, never in Rust. A new journey
//! (Management Buyout, Direct Acquisition, …) is a new data file, not new
//! code (Journey Builder doc §1: "The application should not hard-code a
//! single questionnaire").
//!
//! ## The walk
//!
//! A [`JourneyInstance`] is one owner's walk of one definition: it holds
//! the current position, the versioned answers bucketed by their declared
//! [`StorageScope`], recorded skips, and derived nonnegotiables. Every
//! mutation recomputes the walk position — the first eligible question
//! that is neither answered nor skipped — so editing an earlier answer
//! recalculates which later questions are relevant (Journey Builder doc
//! §9: "dependent information should be recalculated" and "the system
//! must not erase the original decision"; answers are versioned, never
//! edited).
//!
//! ## Storage scopes
//!
//! Each answer is stored under the scope its question declares (Journey
//! Builder doc §6). In v1 every answer is physically local — nothing
//! leaves the machine — but the declaration is honored structurally, so a
//! future hybrid layer can move only the buckets the definition says it
//! may, without re-deriving placement from question text.
//!
//! ## Audit-first
//!
//! Every state transition — instance started, answer recorded, answer
//! revised, node skipped, instance completed — emits exactly one
//! [`AuditEventDraft`] **before** the mutation is applied: a walk state
//! never exists without its event (Implementation Roadmap Phase 1: "audit
//! infrastructure early"). The engine holds the sink through
//! [`AuditSink`]; the app shell wires the concrete audit store at the
//! composition root.
//!
//! ## Persistence
//!
//! Unlike the reality and scenario engines (whose durable record is the
//! audit stream), the journey runtime owns resumable state: a
//! [`JourneyInstance`] serializes to a versioned
//! [`JourneyInstanceDocument`] and persists through
//! [`store::JourneyStore`] (SQLite, WAL), so a walk survives an app
//! restart exactly as it left it — answers, versions, skips, and position.

mod answer;
mod condition;
mod definition;
mod engine;
mod error;
mod instance;
mod scope;
mod store;

pub use answer::{AnswerRecord, AnswerValue, AnswerVersion};
pub use condition::Condition;
pub use definition::{
    ChoiceDef, Effect, InformationValue, InteractionType, JourneyDefinition, JourneyVersion,
    NodeDef, NodeKind, QuestionDef,
};
pub use engine::{
    AnswerRecordedPayload, AnswerRevisedPayload, InstanceCompletedPayload, InstanceStartedPayload,
    JourneyEngine, NodeSkippedPayload, RecordAnswer, ReviseAnswer, SkipNode,
};
pub use error::JourneyError;
pub use instance::{InstanceStatus, JourneyInstance, JourneyInstanceDocument, MarkedNonnegotiable};
pub use scope::StorageScope;
pub use store::{JourneyStore, MemoryJourneyStore, SqliteJourneyStore};

/// The embedded Employee Ownership Journey v0.2 definition — the pinned
/// journey truth, transcribed as data.
pub const EMPLOYEE_OWNERSHIP_V0_2_JSON: &str =
    include_str!("../journeys/employee_ownership_v0.2.json");

/// Loads the built-in Employee Ownership Journey v0.2 definition.
///
/// # Errors
///
/// If the embedded data file no longer parses or fails structural
/// validation — a build-time data bug, not a runtime condition.
pub fn load_employee_ownership_v0_2() -> Result<JourneyDefinition, JourneyError> {
    JourneyDefinition::parse(EMPLOYEE_OWNERSHIP_V0_2_JSON)
}
