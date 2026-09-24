//! Relevance conditions — the "Ask Only When Relevant" machinery.
//!
//! EOJ v0.2 §28: "Each question can contain a rule … ONLY IF … This
//! prevents questionnaire bloat." Conditions are data on the question,
//! evaluated against the answers recorded so far; a question whose
//! condition does not fire is never shown and can never be answered.

use forhemit_contracts::JourneyNodeId;
use serde::{Deserialize, Serialize};

use crate::answer::AnswerValue;

/// A condition that decides whether a question is asked.
///
/// Conditions reference other questions by node id and read their
/// latest answers. Composite forms (`all` / `any` / `not`) keep the
/// data file flat — no question needs bespoke Rust to be conditional.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Condition {
    /// The question's latest answer selects exactly `value`.
    Equals {
        /// The question to read.
        question: JourneyNodeId,
        /// The value to match.
        value: String,
    },
    /// The question's latest answer selects one of `values`.
    InSet {
        /// The question to read.
        question: JourneyNodeId,
        /// The values to match, any of.
        values: Vec<String>,
    },
    /// The question's latest answer carries at least `min` values —
    /// the "Only if the owner selected multiple secondary objectives"
    /// rule (EOJ v0.2 §6).
    CountAtLeast {
        /// The question to read.
        question: JourneyNodeId,
        /// The inclusive minimum.
        min: usize,
    },
    /// Every sub-condition holds.
    All {
        /// The sub-conditions.
        all: Vec<Condition>,
    },
    /// At least one sub-condition holds.
    Any {
        /// The sub-conditions.
        any: Vec<Condition>,
    },
    /// The sub-condition does not hold.
    Not {
        /// The sub-condition.
        not: Box<Condition>,
    },
}

impl Condition {
    /// Evaluates the condition against the latest answers so far.
    ///
    /// `latest` resolves a node's current answer value; a question with
    /// no answer yet fails every condition that reads it (an unanswered
    /// question cannot make another question relevant).
    pub fn evaluate(&self, latest: &dyn Fn(&JourneyNodeId) -> Option<AnswerValue>) -> bool {
        match self {
            Self::Equals { question, value } => {
                latest(question).is_some_and(|answer| answer.contains(value))
            }
            Self::InSet { question, values } => latest(question)
                .is_some_and(|answer| values.iter().any(|value| answer.contains(value))),
            Self::CountAtLeast { question, min } => {
                latest(question).is_some_and(|answer| answer.count() >= *min)
            }
            Self::All { all } => all.iter().all(|condition| condition.evaluate(latest)),
            Self::Any { any } => any.iter().any(|condition| condition.evaluate(latest)),
            Self::Not { not } => !not.evaluate(latest),
        }
    }

    /// Every node id the condition reads, at any depth — definition
    /// validation walks these to reject dangling references.
    pub fn referenced_questions(&self) -> Vec<&JourneyNodeId> {
        match self {
            Self::Equals { question, .. }
            | Self::InSet { question, .. }
            | Self::CountAtLeast { question, .. } => vec![question],
            Self::All { all } => all.iter().flat_map(Self::referenced_questions).collect(),
            Self::Any { any } => any.iter().flat_map(Self::referenced_questions).collect(),
            Self::Not { not } => not.referenced_questions(),
        }
    }
}
