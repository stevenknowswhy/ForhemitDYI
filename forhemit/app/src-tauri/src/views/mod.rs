//! Display projections the UI consumes — pure functions over engine state,
//! no domain rules. The journey view shapes a [`JourneyInstance`] and its
//! definition into everything one screen render needs: the current
//! question with its resolved choices, the pending content screens the
//! walk has passed through, the answered and skipped questions with their
//! version history, and progress.

mod package;
mod scenario;
mod vault;

pub use package::{
    exported_file_view, objective_lines, package_preview_view, workspace_snapshot,
    ExportedFileView, PackagePreviewView,
};
pub use scenario::{
    comparison_view, conflict_decision_from_wire, family_view, version_view, AssumptionView,
    BranchView, ComparisonCellView, ComparisonDimensionView, ComparisonView, ConflictView,
    FamilyAndVersionView, NonnegotiableConflictView, NonnegotiableUnderTestView,
    ScenarioFamilyView, ScenarioVersionView, StatusEntryView, UnknownView, VALUE_TYPE_OPTIONS,
};
pub use vault::{
    document_view, locked_view, not_set_up_view, search_hit_view, status_view,
    version_content_view, version_views, VaultDocumentHistoryView, VaultDocumentView,
    VaultSearchHitView, VaultState, VaultStatusView, VaultVersionContentView, VaultVersionView,
    SHARING_WORDING,
};

use forhemit_journey::{
    AnswerRecord, AnswerValue, InstanceStatus, JourneyDefinition, JourneyInstance, NodeDef,
    NodeKind, QuestionDef,
};
use serde::Serialize;

use crate::state::AppEngines;

/// One selectable choice as the UI renders it.
#[derive(Clone, Debug, Serialize)]
pub struct ChoiceView {
    /// The stable value an answer records.
    pub value: String,
    /// The label the UI shows.
    pub label: String,
}

/// A question the walk is asking (or re-asking in revise mode).
#[derive(Clone, Debug, Serialize)]
pub struct QuestionView {
    /// The question's node id.
    pub node_id: String,
    /// The node's heading.
    pub title: String,
    /// The question text.
    pub text: String,
    /// Why the journey asks.
    pub why_we_ask: Option<String>,
    /// The interaction pattern (`single_select`, `ranking`, …).
    pub interaction: String,
    /// The choice set — resolved from earlier answers when the question
    /// draws its choices with `choices_from`.
    pub choices: Vec<ChoiceView>,
    /// Whether the walk can complete while the question is unanswered
    /// (an optional question offers "Skip this question").
    pub required: bool,
    /// The decision layer the answer belongs to, if any.
    pub decision_layer: Option<String>,
    /// The stage the question belongs to.
    pub stage: String,
    /// Whether the walk is currently positioned on this question.
    pub is_current: bool,
    /// The current latest answer value, when re-asking an answered
    /// question in revise mode.
    pub current_value: Option<AnswerValue>,
}

/// A content-only node the walk passes without answering.
#[derive(Clone, Debug, Serialize)]
pub struct ScreenView {
    /// The screen's node id.
    pub node_id: String,
    /// The screen's heading.
    pub title: String,
    /// The screen's body copy.
    pub body: Option<String>,
    /// The stage the screen belongs to.
    pub stage: String,
}

/// One answered question with its full version history.
#[derive(Clone, Debug, Serialize)]
pub struct AnsweredView {
    /// The question's node id.
    pub node_id: String,
    /// The node's heading.
    pub title: String,
    /// The question text.
    pub text: String,
    /// The interaction pattern.
    pub interaction: String,
    /// The resolved choice set (for re-asking in revise mode).
    pub choices: Vec<ChoiceView>,
    /// The decision layer the answer belongs to, if any.
    pub decision_layer: Option<String>,
    /// The latest answer value.
    pub value: AnswerValue,
    /// The answer's version history, oldest first.
    pub versions: Vec<AnswerVersionView>,
}

/// One version of one answer.
#[derive(Clone, Debug, Serialize)]
pub struct AnswerVersionView {
    /// One-based version number.
    pub version: u32,
    /// The value this version recorded.
    pub value: AnswerValue,
    /// Why the answer changed — set on revisions only.
    pub change_reason: Option<String>,
    /// When this version was recorded.
    pub recorded_at: String,
}

/// A nonnegotiable derived from an answer through a definition effect.
#[derive(Clone, Debug, Serialize)]
pub struct MarkedNonnegotiableView {
    /// The requirement marked nonnegotiable.
    pub target: String,
    /// The answer node that marked it.
    pub node_id: String,
}

/// Walk progress: questions addressed vs the definition's question count.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct ProgressView {
    /// Questions answered or skipped.
    pub done: usize,
    /// Questions in the definition.
    pub total: usize,
}

/// The walk state one render needs.
#[derive(Clone, Debug, Serialize)]
pub struct JourneyView {
    /// The instance's identity.
    pub instance_id: String,
    /// The journey's title.
    pub journey_title: String,
    /// The definition version the walk follows.
    pub journey_version: String,
    /// `in_progress` or `completed`.
    pub status: String,
    /// The current question, if the walk is still going.
    pub current: Option<QuestionView>,
    /// Content screens between the last visited node and the current
    /// question — shown with a local Continue before the question.
    pub screens_to_show: Vec<ScreenView>,
    /// Content screens after the walk's final question — the outro shown
    /// once the walk completes.
    pub outro_screens: Vec<ScreenView>,
    /// Every answered question, in visit order.
    pub answered: Vec<AnsweredView>,
    /// Questions passed without an answer.
    pub skipped: Vec<String>,
    /// Nonnegotiables derived from answers.
    pub nonnegotiables: Vec<MarkedNonnegotiableView>,
    /// Questions addressed vs definition total.
    pub progress: ProgressView,
}

/// The walk's view for the UI.
///
/// # Errors
///
/// The session lock is poisoned.
pub fn journey_view(
    engines: &AppEngines,
    instance: &JourneyInstance,
) -> Result<JourneyView, String> {
    let definition = &engines.definition;
    let answered_total = instance
        .answers_by_scope
        .values()
        .map(|records| records.len())
        .sum::<usize>();
    let total = definition.question_nodes().count();
    let current = instance
        .current_node
        .as_ref()
        .and_then(|node_id| question_view(definition, instance, node_id, true));
    Ok(JourneyView {
        instance_id: instance.instance_id.as_str().to_owned(),
        journey_title: definition.title.clone(),
        journey_version: definition.version.as_str().to_owned(),
        status: match instance.status {
            InstanceStatus::InProgress => "in_progress",
            InstanceStatus::Completed => "completed",
        }
        .to_owned(),
        current,
        screens_to_show: pending_screens(definition, instance),
        outro_screens: outro_screens(definition, instance),
        answered: answered_views(definition, instance),
        skipped: instance
            .skipped
            .iter()
            .map(|node_id| node_id.as_str().to_owned())
            .collect(),
        nonnegotiables: instance
            .nonnegotiables
            .iter()
            .map(|mark| MarkedNonnegotiableView {
                target: mark.target.clone(),
                node_id: mark.node_id.as_str().to_owned(),
            })
            .collect(),
        progress: ProgressView {
            done: answered_total + instance.skipped.len(),
            total,
        },
    })
}

/// The question view for one node, with its resolved choices and, in
/// revise mode, the latest answer value.
fn question_view(
    definition: &JourneyDefinition,
    instance: &JourneyInstance,
    node_id: &forhemit_contracts::JourneyNodeId,
    is_current: bool,
) -> Option<QuestionView> {
    let node = definition.node(node_id)?;
    let question = node.question()?;
    let current_value = instance
        .answer(node_id)
        .map(|record| record.latest().value.clone());
    Some(QuestionView {
        node_id: node_id.as_str().to_owned(),
        title: node.title.clone(),
        text: question.text.clone(),
        why_we_ask: question.why_we_ask.clone(),
        interaction: interaction_name(question),
        choices: resolved_choices(definition, instance, question),
        required: question.required,
        decision_layer: question
            .decision_layer
            .as_ref()
            .map(|layer| snake_case(&format!("{layer:?}"))),
        stage: node.stage.clone(),
        is_current,
        current_value,
    })
}

/// The serde name of the interaction pattern, as the data file spells it.
fn interaction_name(question: &QuestionDef) -> String {
    let name = match question.interaction {
        forhemit_journey::InteractionType::SingleSelect => "single_select",
        forhemit_journey::InteractionType::MultiSelect => "multi_select",
        forhemit_journey::InteractionType::SelectUpTo3 => "select_up_to_3",
        forhemit_journey::InteractionType::Ranking => "ranking",
        forhemit_journey::InteractionType::YesNo => "yes_no",
        forhemit_journey::InteractionType::Numeric => "numeric",
        forhemit_journey::InteractionType::Confirm => "confirm",
    };
    name.to_owned()
}

/// The choice set a `choices_from` question resolves from the answers so
/// far — value and label pairs, in source-question order. Mirrors the
/// engine's resolution exactly: only values the owner actually selected
/// in a source question appear.
fn resolved_choices(
    definition: &JourneyDefinition,
    instance: &JourneyInstance,
    question: &QuestionDef,
) -> Vec<ChoiceView> {
    if !question.choices.is_empty() {
        return question
            .choices
            .iter()
            .map(|choice| ChoiceView {
                value: choice.value.clone(),
                label: choice.label.clone(),
            })
            .collect();
    }
    let mut choices = Vec::new();
    for source in question.choices_from.iter().flatten() {
        let Some(source_question) = definition.node(source).and_then(|node| node.question()) else {
            continue;
        };
        for choice in &source_question.choices {
            if instance
                .latest_value(source)
                .is_some_and(|value| value.contains(&choice.value))
            {
                choices.push(ChoiceView {
                    value: choice.value.clone(),
                    label: choice.label.clone(),
                });
            }
        }
    }
    choices
}

/// Content screens between the last visited node (or the start) and the
/// current question — the screens the walk has passed since the last
/// answer and must display before asking the next one. Empty once the
/// walk completes (the outro covers those screens).
fn pending_screens(definition: &JourneyDefinition, instance: &JourneyInstance) -> Vec<ScreenView> {
    if instance.status == InstanceStatus::Completed {
        return Vec::new();
    }
    let last_visited = instance.visited.last();
    let mut pending = Vec::new();
    let mut after_last = last_visited.is_none();
    for node in &definition.nodes {
        if let Some(last) = last_visited {
            if &node.node_id == last {
                after_last = true;
                continue;
            }
        }
        if instance
            .current_node
            .as_ref()
            .is_some_and(|current| &node.node_id == current)
        {
            break;
        }
        if after_last && node.kind == NodeKind::Screen {
            pending.push(screen_view(node));
        }
    }
    pending
}

/// Content screens after the walk's final question — the outro shown once
/// the walk completes (scenario, professional-team, and review-package
/// screens).
fn outro_screens(definition: &JourneyDefinition, instance: &JourneyInstance) -> Vec<ScreenView> {
    if instance.status != InstanceStatus::Completed {
        return Vec::new();
    }
    let last_visited = instance.visited.last();
    let mut after_last = last_visited.is_none();
    definition
        .nodes
        .iter()
        .filter_map(|node| {
            if let Some(last) = last_visited {
                if &node.node_id == last {
                    after_last = true;
                    return None;
                }
            }
            if after_last && node.kind == NodeKind::Screen {
                Some(screen_view(node))
            } else {
                None
            }
        })
        .collect()
}

/// The display view of one screen node.
fn screen_view(node: &NodeDef) -> ScreenView {
    ScreenView {
        node_id: node.node_id.as_str().to_owned(),
        title: node.title.clone(),
        body: node.body.clone(),
        stage: node.stage.clone(),
    }
}

/// Every answered question in visit order with its version history.
fn answered_views(definition: &JourneyDefinition, instance: &JourneyInstance) -> Vec<AnsweredView> {
    instance
        .visited
        .iter()
        .filter_map(|node_id| {
            let record = instance.answer(node_id)?;
            Some(answered_view(definition, record))
        })
        .collect()
}

/// The display view of one answered question.
fn answered_view(definition: &JourneyDefinition, record: &AnswerRecord) -> AnsweredView {
    let node = definition.node(&record.node_id);
    let question = node.and_then(|node| node.question());
    let versions = record
        .versions
        .iter()
        .map(|version| AnswerVersionView {
            version: version.version,
            value: version.value.clone(),
            change_reason: version.change_reason.clone(),
            recorded_at: version
                .recorded_at
                .format(&time::format_description::well_known::Rfc3339)
                .map(|formatted| formatted.to_string())
                .unwrap_or_else(|_| version.recorded_at.to_string()),
        })
        .collect();
    AnsweredView {
        node_id: record.node_id.as_str().to_owned(),
        title: node.map_or_else(String::new, |node| node.title.clone()),
        text: question.map_or_else(String::new, |question| question.text.clone()),
        interaction: question.map_or_else(String::new, interaction_name),
        choices: question
            .map(|question| {
                question
                    .choices
                    .iter()
                    .map(|choice| ChoiceView {
                        value: choice.value.clone(),
                        label: choice.label.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        decision_layer: question
            .as_ref()
            .and_then(|question| question.decision_layer.as_ref())
            .map(|layer| snake_case(&format!("{layer:?}"))),
        value: record.latest().value.clone(),
        versions,
    }
}

/// CamelCase enum Debug names to the data file's snake_case form, for
/// decision-layer labels.
fn snake_case(display: &str) -> String {
    let mut out = String::with_capacity(display.len() + 4);
    for (index, character) in display.char_indices() {
        if character.is_ascii_uppercase() {
            if index != 0 {
                out.push('_');
            }
            out.extend(character.to_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// The audit chain verification result, as the UI displays it.
#[derive(Clone, Debug, Serialize)]
pub struct VerifyView {
    /// Whether the hash chain verified end to end.
    pub verified: bool,
    /// The number of events in the store.
    pub event_count: u64,
    /// The human-readable result — success wording or the failure detail.
    pub detail: String,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // tests: failures must panic the test

    use super::*;
    use forhemit_contracts::JourneyInstanceId;
    use forhemit_journey::{JourneyEngine, MemoryJourneyStore};

    /// Starts a real walk through the engine over the embedded definition.
    fn started_instance(engines: &AppEngines, instance_id: &str) -> JourneyInstance {
        let store = MemoryJourneyStore::new();
        let engine = JourneyEngine::new(
            engines.clock.as_ref(),
            engines.tee.as_ref(),
            &store,
            engines.workspace_id.clone(),
        );
        engine
            .start(
                &engines.definition,
                JourneyInstanceId::new(instance_id).unwrap(),
                &engines.actor,
            )
            .unwrap()
    }

    #[test]
    fn a_fresh_walk_shows_the_welcome_screen_first() {
        let engines = crate::test_support::opened_engines();
        let instance = started_instance(&engines, "inst_test");
        let view = journey_view(&engines, &instance).unwrap();
        assert_eq!(view.screens_to_show.len(), 1);
        assert_eq!(view.screens_to_show[0].node_id, "welcome");
        assert_eq!(view.current.as_ref().unwrap().node_id, "primary_objective");
        assert_eq!(view.progress.total, 24);
        assert_eq!(view.progress.done, 0);
        assert_eq!(view.status, "in_progress");
    }

    #[test]
    fn layer_labels_use_the_data_files_snake_case() {
        assert_eq!(snake_case("OwnerObjective"), "owner_objective");
        assert_eq!(snake_case("PlatformScenario"), "platform_scenario");
    }
}
