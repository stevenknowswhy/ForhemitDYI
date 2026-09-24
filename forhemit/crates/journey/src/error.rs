//! Error surface of the journey runtime.
//!
//! Every variant names the condition it reports; nothing is swallowed.
//! The engine is audit-first: [`JourneyError::Audit`] means the
//! transition did **not** happen — a walk state never exists without its
//! audit event.

use forhemit_contracts::{JourneyId, JourneyNodeId};
use forhemit_enginekit::AuditError;

/// What can go wrong while loading a definition or walking a journey.
#[derive(Debug)]
pub enum JourneyError {
    /// The audit store rejected the event. The mutation did not happen —
    /// no answer version, skip, or completion exists without its audit
    /// event.
    Audit(AuditError),
    /// Serializing an event payload or instance document failed.
    Serialization(serde_json::Error),
    /// The definition file did not parse against the journey-definition
    /// contract.
    DefinitionParse(serde_json::Error),
    /// The definition parsed but is structurally invalid — a dangling
    /// node reference, an unknown condition field, a start node that is
    /// not a screen, or similar data bug.
    DefinitionInvalid {
        /// What is wrong, naming the offending node or field.
        reason: String,
    },
    /// The instance's journey identity does not match the definition it
    /// is being walked with — an instance may only be driven by the
    /// definition (and version) it started under.
    DefinitionMismatch {
        /// The journey the instance was started under.
        instance_journey: JourneyId,
        /// The journey of the definition passed to the engine.
        definition_journey: JourneyId,
    },
    /// No node exists under the given id.
    UnknownNode {
        /// The unknown node id.
        node_id: JourneyNodeId,
    },
    /// A screen node was treated as a question — screens hold content,
    /// never answers.
    NodeIsNotAQuestion {
        /// The node id.
        node_id: JourneyNodeId,
    },
    /// An answer (or skip) was offered to a node other than the walk's
    /// current position — the journey is a guided conversation, and the
    /// smallest useful next question is the one being asked.
    NotTheCurrentPosition {
        /// The node the walk is actually on (None when complete).
        current: Option<JourneyNodeId>,
        /// The node the caller tried to answer or skip.
        requested: JourneyNodeId,
    },
    /// The node's "Ask Only When Relevant" condition does not fire on
    /// the answers recorded so far (EOJ v0.2 §28) — the question is not
    /// being asked, so it cannot be answered.
    NodeNotEligible {
        /// The node id.
        node_id: JourneyNodeId,
    },
    /// The answer's shape or content does not fit the question's
    /// interaction type or choice set.
    InvalidAnswer {
        /// The node id.
        node_id: JourneyNodeId,
        /// What is wrong with the value.
        reason: String,
    },
    /// A revision was filed without a reason; "Why did it change?" is
    /// the first question the answer history must answer.
    EmptyChangeReason,
    /// No answer exists under the given node — a revision needs a
    /// previous version to supersede.
    AnswerNotFound {
        /// The node id.
        node_id: JourneyNodeId,
    },
    /// A required question cannot be skipped — only optional nodes offer
    /// a question exit (EOJ v0.2 §30).
    SkipOfRequiredNode {
        /// The node id.
        node_id: JourneyNodeId,
    },
    /// The instance was already completed and nothing eligible remains
    /// unanswered.
    InstanceAlreadyCompleted,
    /// The instance document could not be parsed from storage.
    StoreState(serde_json::Error),
    /// The instance id was not found in the store.
    InstanceNotFound {
        /// The unknown instance id, as stored text.
        instance_id: String,
    },
    /// SQLite reported an error.
    Store(rusqlite::Error),
    /// The store's write lock was poisoned by a panic in another thread.
    LockPoisoned,
}

impl std::fmt::Display for JourneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audit(source) => write!(f, "audit store rejected the event: {source}"),
            Self::Serialization(source) => write!(f, "payload serialization failed: {source}"),
            Self::DefinitionParse(source) => {
                write!(f, "journey definition did not parse: {source}")
            }
            Self::DefinitionInvalid { reason } => write!(f, "journey definition invalid: {reason}"),
            Self::DefinitionMismatch {
                instance_journey,
                definition_journey,
            } => write!(
                f,
                "instance belongs to journey {instance_journey}, not {definition_journey}"
            ),
            Self::UnknownNode { node_id } => write!(f, "unknown node {node_id}"),
            Self::NodeIsNotAQuestion { node_id } => {
                write!(f, "node {node_id} is a screen, not a question")
            }
            Self::NotTheCurrentPosition { current, requested } => write!(
                f,
                "node {requested} is not the walk's current position (at {current:?})"
            ),
            Self::NodeNotEligible { node_id } => {
                write!(
                    f,
                    "node {node_id} is not currently relevant — its condition does not fire"
                )
            }
            Self::InvalidAnswer { node_id, reason } => {
                write!(f, "invalid answer for node {node_id}: {reason}")
            }
            Self::EmptyChangeReason => write!(f, "a revision needs a change reason"),
            Self::AnswerNotFound { node_id } => write!(f, "no answer exists for node {node_id}"),
            Self::SkipOfRequiredNode { node_id } => {
                write!(f, "node {node_id} is required and cannot be skipped")
            }
            Self::InstanceAlreadyCompleted => {
                write!(f, "the journey instance is already completed")
            }
            Self::StoreState(source) => {
                write!(f, "stored instance document did not parse: {source}")
            }
            Self::InstanceNotFound { instance_id } => {
                write!(f, "no journey instance {instance_id} in the store")
            }
            Self::Store(source) => write!(f, "journey store error: {source}"),
            Self::LockPoisoned => write!(f, "journey store lock poisoned"),
        }
    }
}

impl std::error::Error for JourneyError {}

impl From<rusqlite::Error> for JourneyError {
    fn from(source: rusqlite::Error) -> Self {
        Self::Store(source)
    }
}
