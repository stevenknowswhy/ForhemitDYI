//! The journey definition format — versioned data files, not code.
//!
//! "The application should not hard-code a single questionnaire" (Journey
//! Builder doc §1). A [`JourneyDefinition`] is the parsed form of one
//! definition file: questions with their full metadata (Journey Builder
//! doc §5), Storage Scope declarations (§6), follow-up effects, and
//! "Ask Only When Relevant" conditionals (EOJ v0.2 §28).
//!
//! Definitions are strict: unknown fields in the data file are rejected
//! at parse time, and `validate` rejects dangling node references and
//! other structural bugs before any owner walks the journey. The
//! Employee Ownership Journey v0.2 file lives in
//! `journeys/employee_ownership_v0.2.json` and is embedded at compile
//! time; nothing about that journey is written in Rust.

use forhemit_contracts::{DecisionLayer, JourneyId, JourneyNodeId};
use serde::{Deserialize, Serialize};

use crate::answer::AnswerValue;
use crate::condition::Condition;
use crate::error::JourneyError;
use crate::scope::StorageScope;

/// A definition's version — "Every published journey receives a version"
/// (Journey Builder doc §23). Recorded on every answer, audit payload,
/// and instance document so outputs can always say which journey version
/// produced them.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct JourneyVersion(String);

impl JourneyVersion {
    /// Creates a version, rejecting empty values.
    ///
    /// # Errors
    ///
    /// An empty or blank version string.
    pub fn new(value: impl Into<String>) -> Result<Self, JourneyError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(JourneyError::DefinitionInvalid {
                reason: "journey version must be a non-empty string".to_owned(),
            })
        } else {
            Ok(Self(value))
        }
    }

    /// Borrows the version string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for JourneyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One journey definition — the parsed, validated form of a data file.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyDefinition {
    /// The journey's stable identity (`"employee_ownership"`).
    pub journey_id: JourneyId,
    /// The name the UI shows.
    pub title: String,
    /// The definition's version (Journey Builder doc §23).
    pub version: JourneyVersion,
    /// One-paragraph description of the journey's purpose.
    pub description: String,
    /// The design document the definition transcribes — provenance for
    /// the data file itself.
    pub source: String,
    /// The node the walk starts on — the welcome screen.
    pub start_node: JourneyNodeId,
    /// The walk's nodes, in journey order (EOJ v0.2 §26's one view).
    pub nodes: Vec<NodeDef>,
}

/// One node — "one interaction or information step" (Journey Builder
/// doc §3).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeDef {
    /// The node's stable id, referenced by conditions and answers.
    pub node_id: JourneyNodeId,
    /// The stage the node belongs to — a logical section of the journey
    /// (Journey Builder doc §3), e.g. `"business_snapshot"`.
    pub stage: String,
    /// Screen or question.
    pub kind: NodeKind,
    /// The node's heading, verbatim from the journey document.
    pub title: String,
    /// The node's body copy, verbatim from the journey document, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Question metadata — present exactly when `kind` is
    /// [`NodeKind::Question`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<QuestionDef>,
}

impl NodeDef {
    /// The node's question metadata, if this is a question node.
    pub fn question(&self) -> Option<&QuestionDef> {
        self.question.as_ref()
    }
}

impl QuestionDef {
    /// The question's allowed choice values. Fixed choices contribute
    /// their values; a `choices_from` question contributes the values
    /// resolved from its source answers (passed in by the engine, which
    /// owns the answer state).
    pub fn allowed_values<'a>(&'a self, resolved: &'a [String]) -> Vec<&'a str> {
        if self.choices.is_empty() {
            resolved.iter().map(String::as_str).collect()
        } else {
            self.choices
                .iter()
                .map(|choice| choice.value.as_str())
                .collect()
        }
    }

    /// Validates that an answer's shape and content fit this question —
    /// the interaction type constrains the shape, the resolved choice
    /// set constrains the content. Rankings must be a full ordering of
    /// the resolved set (a partial ranking silently discards answers);
    /// "select up to 3" is capped at three (EOJ v0.2 §5).
    ///
    /// # Errors
    ///
    /// [`JourneyError::InvalidAnswer`] naming `node_id` and the violated
    /// rule.
    pub fn validate_answer(
        &self,
        node_id: &JourneyNodeId,
        value: &AnswerValue,
        resolved: &[String],
    ) -> Result<(), JourneyError> {
        let invalid = |reason: String| JourneyError::InvalidAnswer {
            node_id: node_id.clone(),
            reason,
        };
        let allowed = self.allowed_values(resolved);
        let all_known =
            |values: &[String]| values.iter().all(|value| allowed.contains(&value.as_str()));
        match (&self.interaction, value) {
            (
                InteractionType::SingleSelect | InteractionType::YesNo,
                AnswerValue::Single(value),
            ) => {
                if allowed.contains(&value.as_str()) {
                    Ok(())
                } else {
                    Err(invalid(format!(
                        "value {value:?} is not one of the offered choices"
                    )))
                }
            }
            (InteractionType::MultiSelect, AnswerValue::Multi(values)) => {
                if values.is_empty() {
                    return Err(invalid(
                        "a multi-select answer must select at least one choice".to_owned(),
                    ));
                }
                if all_known(values) {
                    Ok(())
                } else {
                    Err(invalid(
                        "a selected value is not one of the offered choices".to_owned(),
                    ))
                }
            }
            (InteractionType::SelectUpTo3, AnswerValue::Multi(values)) => {
                if values.is_empty() || values.len() > 3 {
                    return Err(invalid(
                        "a select-up-to-3 answer carries between one and three choices".to_owned(),
                    ));
                }
                if all_known(values) {
                    Ok(())
                } else {
                    Err(invalid(
                        "a selected value is not one of the offered choices".to_owned(),
                    ))
                }
            }
            (InteractionType::Ranking, AnswerValue::Ranking(values)) => {
                let mut remaining: Vec<&str> = allowed.to_vec();
                for value in values {
                    match remaining
                        .iter()
                        .position(|allowed| *allowed == value.as_str())
                    {
                        Some(index) => {
                            remaining.remove(index);
                        }
                        None => {
                            return Err(invalid(format!(
                                "ranking value {value:?} is not in the choice set or is ranked twice"
                            )));
                        }
                    }
                }
                if !remaining.is_empty() {
                    return Err(invalid(
                        "a ranking must order every offered choice — none may be dropped"
                            .to_owned(),
                    ));
                }
                Ok(())
            }
            (InteractionType::Numeric, AnswerValue::Amount(_)) => Ok(()),
            (InteractionType::Confirm, AnswerValue::Confirmed) => Ok(()),
            (interaction, value) => Err(invalid(format!(
                "answer shape {value:?} does not fit the {interaction:?} interaction"
            ))),
        }
    }
}

/// Screen or question — the two node kinds the runtime distinguishes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    /// Content only — the welcome screen. Screens never hold answers.
    Screen,
    /// One interaction that captures an answer.
    Question,
}

/// A question's full metadata — the Journey Builder doc §5 tree, as
/// data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionDef {
    /// The question text, verbatim from the journey document.
    pub text: String,
    /// Why the journey asks — "Show why it matters" (EOJ v0.2 §32).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why_we_ask: Option<String>,
    /// The interaction pattern the builder selected for this question
    /// (Journey-Builder Rules doc: "the builder should select the
    /// appropriate interaction").
    pub interaction: InteractionType,
    /// The fixed choice set, when the question's choices are its own.
    /// Questions whose choices come from earlier answers use
    /// [`Self::choices_from`] instead.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<ChoiceDef>,
    /// Earlier questions whose selected values form this question's
    /// choice set — e.g. the nonnegotiable sweep offers the objectives
    /// the owner actually selected (EOJ v0.2 §7).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choices_from: Option<Vec<JourneyNodeId>>,
    /// Whether the walk can complete while the question is unanswered.
    pub required: bool,
    /// Where the answer is stored (Journey Builder doc §6) — placement
    /// is declared here, never chosen by callers.
    pub storage_scope: StorageScope,
    /// Which of the three decision layers the answer belongs to, when it
    /// belongs to one (THREE DECISION LAYERS doc §7). Business-snapshot
    /// questions are facts, not decisions, and carry no layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_layer: Option<DecisionLayer>,
    /// The question's internal priority (EOJ v0.2 §29: "Essential
    /// information first").
    pub information_value: InformationValue,
    /// "Ask Only When Relevant" — the question is asked only when this
    /// condition fires on the answers so far (EOJ v0.2 §28).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_when: Option<Condition>,
    /// Follow-up effects — what recording an answer sets in motion.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

/// A selectable choice.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceDef {
    /// The stable value an answer records.
    pub value: String,
    /// The label the UI shows, verbatim from the journey document.
    pub label: String,
}

/// The interaction patterns the v0.2 journey uses — the subset of the
/// Journey Builder doc §4 native types EOJ v0.2 needs. New interaction
/// types join as additive variants.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    /// One choice from the set.
    SingleSelect,
    /// Any number of choices from the set.
    MultiSelect,
    /// At most three choices from the set (EOJ v0.2 §5: "Select up to 3").
    /// Pinned wire name — `snake_case` alone would contract to
    /// `select_up_to3`.
    #[serde(rename = "select_up_to_3")]
    SelectUpTo3,
    /// An ordering of the resolved choice set, most important first.
    Ranking,
    /// Yes or no.
    YesNo,
    /// A numeric amount, in whole currency units.
    Numeric,
    /// An explicit confirmation — review and commitment nodes.
    Confirm,
}

/// A question's internal priority (EOJ v0.2 §29).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InformationValue {
    /// Shapes the next decision — asked first, never delayed.
    Essential,
    /// Materially useful when it fires.
    Useful,
    /// Nice to have; may be skipped freely.
    Optional,
}

/// A follow-up effect — what recording an answer sets in motion.
///
/// Effects are data on the question so the destination engine (and any
/// later consumer) can learn about derived nonnegotiables without the
/// runtime knowing journey specifics.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    /// When `when` fires on the answer being recorded, the named
    /// requirement is marked nonnegotiable — EOJ v0.2 §11: choosing
    /// "Essential" for employee ownership marks it 🔴 nonnegotiable, and
    /// "The system doesn't need to ask this again later."
    MarkNonnegotiable {
        /// The condition on the answer being recorded.
        when: Condition,
        /// The requirement to mark nonnegotiable.
        target: String,
    },
    /// When `when` fires, every value the answer selects is marked
    /// nonnegotiable by its choice label — the nonnegotiable sweep over
    /// the owner's objectives (EOJ v0.2 Stage 4): each selected
    /// objective becomes a must-have; unmarked ones stay preferences.
    MarkAnswerValuesNonnegotiable {
        /// The condition on the answer being recorded.
        when: Condition,
    },
}

impl JourneyDefinition {
    /// Parses and validates a definition file.
    ///
    /// # Errors
    ///
    /// [`JourneyError::DefinitionParse`] when the JSON does not match
    /// the format; [`JourneyError::DefinitionInvalid`] when it parses
    /// but is structurally unsound (dangling references, a non-screen
    /// start node, non-question nodes without question metadata, …).
    pub fn parse(json: &str) -> Result<Self, JourneyError> {
        let definition: Self = serde_json::from_str(json).map_err(JourneyError::DefinitionParse)?;
        definition.validate()?;
        Ok(definition)
    }

    /// The node with the given id, if the definition has one.
    pub fn node(&self, node_id: &JourneyNodeId) -> Option<&NodeDef> {
        self.nodes.iter().find(|node| &node.node_id == node_id)
    }

    /// The definition's question nodes, in walk order.
    pub fn question_nodes(&self) -> impl Iterator<Item = &NodeDef> {
        self.nodes
            .iter()
            .filter(|node| node.kind == NodeKind::Question)
    }

    fn validate(&self) -> Result<(), JourneyError> {
        // The version tag is load-bearing: answers, audit payloads, and
        // the review package all record which journey produced them.
        if self.version.as_str().trim().is_empty() {
            return Err(JourneyError::DefinitionInvalid {
                reason: "journey version must be a non-empty string".to_owned(),
            });
        }
        if self.nodes.is_empty() {
            return Err(JourneyError::DefinitionInvalid {
                reason: "a journey needs at least one node".to_owned(),
            });
        }

        let mut seen = std::collections::HashSet::new();
        for node in &self.nodes {
            if !seen.insert(node.node_id.as_str()) {
                return Err(JourneyError::DefinitionInvalid {
                    reason: format!("duplicate node id {}", node.node_id),
                });
            }
            match (&node.kind, &node.question) {
                (NodeKind::Screen, Some(_)) => {
                    return Err(JourneyError::DefinitionInvalid {
                        reason: format!("screen node {} carries question metadata", node.node_id),
                    });
                }
                (NodeKind::Question, None) => {
                    return Err(JourneyError::DefinitionInvalid {
                        reason: format!("question node {} has no question metadata", node.node_id),
                    });
                }
                _ => {}
            }
        }

        let start = self
            .node(&self.start_node)
            .ok_or_else(|| JourneyError::DefinitionInvalid {
                reason: format!("start_node {} does not exist", self.start_node),
            })?;
        if start.kind != NodeKind::Screen {
            return Err(JourneyError::DefinitionInvalid {
                reason: format!("start_node {} must be a screen", self.start_node),
            });
        }

        // The walk's first question must be unconditional, or a definition
        // could open on nothing to ask.
        let first_question = self
            .nodes
            .iter()
            .find(|node| node.kind == NodeKind::Question);
        if let Some(node) = first_question {
            if node
                .question
                .as_ref()
                .and_then(|question| question.show_when.as_ref())
                .is_some()
            {
                return Err(JourneyError::DefinitionInvalid {
                    reason: format!("the first question {} must be unconditional", node.node_id),
                });
            }
        }

        for node in &self.nodes {
            let Some(question) = node.question.as_ref() else {
                continue;
            };
            // Only choice-based interactions need a choice set; numeric
            // and confirm questions answer by shape alone.
            let needs_choices = matches!(
                question.interaction,
                InteractionType::SingleSelect
                    | InteractionType::MultiSelect
                    | InteractionType::SelectUpTo3
                    | InteractionType::Ranking
                    | InteractionType::YesNo
            );
            if needs_choices && question.choices.is_empty() && question.choices_from.is_none() {
                return Err(JourneyError::DefinitionInvalid {
                    reason: format!(
                        "question {} has neither choices nor choices_from",
                        node.node_id
                    ),
                });
            }
            if let Some(sources) = &question.choices_from {
                for source in sources {
                    let source_node =
                        self.node(source)
                            .ok_or_else(|| JourneyError::DefinitionInvalid {
                                reason: format!(
                                    "question {} draws choices from unknown node {source}",
                                    node.node_id
                                ),
                            })?;
                    if source_node.kind != NodeKind::Question {
                        return Err(JourneyError::DefinitionInvalid {
                            reason: format!(
                                "question {} draws choices from {source}, which is not a question",
                                node.node_id
                            ),
                        });
                    }
                }
            }
            for condition in question.show_when.iter().chain(question.effects.iter().map(
                |effect| match effect {
                    Effect::MarkNonnegotiable { when, .. }
                    | Effect::MarkAnswerValuesNonnegotiable { when } => when,
                },
            )) {
                for referenced in condition.referenced_questions() {
                    let referenced_node =
                        self.node(referenced)
                            .ok_or_else(|| JourneyError::DefinitionInvalid {
                                reason: format!(
                                    "condition on node {} references unknown node {referenced}",
                                    node.node_id
                                ),
                            })?;
                    if referenced_node.kind != NodeKind::Question {
                        return Err(JourneyError::DefinitionInvalid {
                            reason: format!(
                                "condition on node {} reads {referenced}, which is not a question",
                                node.node_id
                            ),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    fn minimal_valid_json() -> String {
        // Welcome screen + one unconditional question.
        r#"{
            "journey_id": "test_journey",
            "title": "Test",
            "version": "0.1",
            "description": "A minimal definition for tests.",
            "source": "nowhere",
            "start_node": "welcome",
            "nodes": [
                {"node_id": "welcome", "stage": "welcome", "kind": "screen", "title": "Welcome"},
                {"node_id": "only", "stage": "only", "kind": "question", "title": "Only",
                 "question": {
                    "text": "Pick one.",
                    "interaction": "single_select",
                    "choices": [{"value": "a", "label": "A"}, {"value": "b", "label": "B"}],
                    "required": true,
                    "storage_scope": "local_private",
                    "information_value": "essential"
                 }}
            ]
        }"#
        .to_owned()
    }

    #[test]
    fn parses_a_minimal_definition() {
        let definition = JourneyDefinition::parse(&minimal_valid_json()).unwrap();
        assert_eq!(definition.journey_id.as_str(), "test_journey");
        assert_eq!(definition.version.as_str(), "0.1");
        assert_eq!(definition.question_nodes().count(), 1);
    }

    #[test]
    fn rejects_unknown_fields() {
        let json = minimal_valid_json()
            .replace("\"title\": \"Test\"", "\"title\": \"Test\", \"extra\": 1");
        assert!(matches!(
            JourneyDefinition::parse(&json),
            Err(JourneyError::DefinitionParse(_))
        ));
    }

    #[test]
    fn rejects_dangling_start_node() {
        let json =
            minimal_valid_json().replace("\"start_node\": \"welcome\"", "\"start_node\": \"nope\"");
        assert!(matches!(
            JourneyDefinition::parse(&json),
            Err(JourneyError::DefinitionInvalid { .. })
        ));
    }

    #[test]
    fn rejects_conditional_first_question() {
        let json = minimal_valid_json().replace(
            "\"information_value\": \"essential\"",
            "\"information_value\": \"essential\", \"show_when\": {\"op\": \"equals\", \"question\": \"only\", \"value\": \"a\"}",
        );
        assert!(matches!(
            JourneyDefinition::parse(&json),
            Err(JourneyError::DefinitionInvalid { .. })
        ));
    }

    #[test]
    fn rejects_dangling_condition_reference() {
        let json = minimal_valid_json().replace(
            "\"information_value\": \"essential\"",
            "\"information_value\": \"essential\", \"show_when\": {\"op\": \"equals\", \"question\": \"ghost\", \"value\": \"a\"}",
        );
        assert!(matches!(
            JourneyDefinition::parse(&json),
            Err(JourneyError::DefinitionInvalid { .. })
        ));
    }

    #[test]
    fn rejects_empty_version() {
        let json = minimal_valid_json().replace("\"version\": \"0.1\"", "\"version\": \" \"");
        assert!(matches!(
            JourneyDefinition::parse(&json),
            Err(JourneyError::DefinitionInvalid { .. })
        ));
    }
}
