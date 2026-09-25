//! The canonical package structure: a one-page executive summary, then
//! the 13 sections EOJ v0.2 §23 names, in order.
//!
//! The section titles are the journey document's own — the package never
//! invents new ones (spec: pinned journey truth; PRP doc §44: the package
//! "should be useful to a busy professional", so the structure is fixed).
//! Every section is built from exactly the snapshot data that owns it,
//! and a section with nothing recorded says so in words — a missing
//! answer is honest absence, never a blank zero (schema doc §15; Business
//! Reality doc §7).

use crate::inputs::{
    ExportReadiness, JourneySnapshot, ObjectiveLine, ScenarioSnapshot, WorkspaceSnapshot,
};
use forhemit_contracts::NonnegotiableState;
use serde::{Deserialize, Serialize};

/// The not-advice disclosure, carried on every package (spec task: "this
/// package organizes your own information; it is not a recommendation or
/// valuation"; README liability framing; PRP doc §47).
pub const DISCLOSURE: &str =
    "This package organizes your own information; it is not a recommendation or valuation.";

/// The 13 section titles, verbatim from EOJ v0.2 §23.
pub const SECTION_TITLES: [&str; 13] = [
    "Owner Objective",
    "Priorities",
    "Nonnegotiables",
    "Things the Owner Wants to Avoid",
    "Business Snapshot",
    "Owner Transition Preferences",
    "Scenarios Explored",
    "Assumptions Used",
    "Trade-Offs Identified",
    "Questions for Professionals",
    "Selected Professional Team",
    "Professional Determinations",
    "Current Working Plan",
];

/// A section's 1-based position — the package always numbers all 13,
/// even when a section has nothing recorded.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionNumber {
    /// Section 1.
    One,
    /// Section 2.
    Two,
    /// Section 3.
    Three,
    /// Section 4.
    Four,
    /// Section 5.
    Five,
    /// Section 6.
    Six,
    /// Section 7.
    Seven,
    /// Section 8.
    Eight,
    /// Section 9.
    Nine,
    /// Section 10.
    Ten,
    /// Section 11.
    Eleven,
    /// Section 12.
    Twelve,
    /// Section 13.
    Thirteen,
}

impl SectionNumber {
    /// The enum's position, 1-based.
    #[must_use]
    pub fn index(self) -> usize {
        self as usize + 1
    }

    /// The section's canonical title.
    #[must_use]
    pub fn title(self) -> &'static str {
        SECTION_TITLES[self.index() - 1]
    }
}

/// One rendered building block of a section.
///
/// Adjacent tagging (`{"block": "text", "content": "…"}`) because
/// internally-tagged enums cannot hold newtype variants that wrap bare
/// strings — and `Block::Text` is exactly that. Round-trip serde on
/// every public type is a spec requirement (Contracts row).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "block", content = "content", rename_all = "snake_case")]
pub enum Block {
    /// A labeled value: "Revenue — $4M–$6M (owner reported, unverified)".
    Line {
        /// The line's label.
        label: String,
        /// The rendered value, including its attribution when it has one.
        value: String,
    },
    /// A free-standing line — prose, a disclaimer, a note.
    Text(
        /// The line's text.
        String,
    ),
    /// A titled entry with sub-lines — a scenario, an unknown, a
    /// question.
    Entry {
        /// The entry's title line.
        title: String,
        /// The entry's detail lines, in order.
        lines: Vec<String>,
    },
    /// A callout the package must not bury — the scenario disclaimer,
    /// the exclusion list.
    Callout {
        /// The callout's label (e.g. "Platform Scenario").
        label: String,
        /// The callout's text.
        text: String,
    },
    /// An honest-empty marker: nothing has been recorded yet, stated in
    /// words with its context.
    NothingRecorded {
        /// What is missing and why that is the current truth.
        context: String,
    },
}

/// One package section: its number, its canonical title, and its blocks.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// The section's position.
    pub number: SectionNumber,
    /// The canonical title (`SectionNumber::title`), owned so the
    /// package serializes and round-trips without borrows.
    pub title: String,
    /// The rendered content, in order.
    pub blocks: Vec<Block>,
}

/// The one-page executive summary — "the document the owner can hand
/// someone across a desk" (EOJ v0.2 §24).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutiveSummary {
    /// What the owner wants, from the destination and the journey's
    /// primary-goal answer.
    pub what_i_want: Vec<Block>,
    /// The owner's must-haves — 🔴 nonnegotiables.
    pub must_haves: Vec<String>,
    /// Strong preferences — 🟠.
    pub strong_preferences: Vec<String>,
    /// What the owner wants to avoid.
    pub wants_to_avoid: Vec<String>,
    /// The paths being evaluated — the ready scenarios' names.
    pub paths_evaluated: Vec<String>,
    /// The main questions for the owner's advisors.
    pub main_questions: Vec<String>,
}

/// Renders the preference strength as the doc's vocabulary, for lines
/// that carry an objective's designation.
fn preference_label(preference: &NonnegotiableState) -> Option<&'static str> {
    match preference {
        NonnegotiableState::Unspecified => None,
        NonnegotiableState::Preference => Some("preference"),
        NonnegotiableState::StrongPreference => Some("strong preference"),
        NonnegotiableState::Nonnegotiable => Some("must-have"),
    }
}

/// The journey's answer line for one node, if the walk recorded one.
fn journey_answer<'a>(
    journey: &'a JourneySnapshot,
    node_id: &str,
) -> Option<&'a crate::inputs::AnswerLine> {
    journey.answers.iter().find(|line| line.node_id == node_id)
}

/// Renders a journey answer as the chosen labels joined.
fn answer_value(line: &crate::inputs::AnswerLine) -> String {
    if line.values.is_empty() {
        "answered without a recorded value".to_owned()
    } else {
        line.values.join(", ")
    }
}

/// Section 1 — Owner Objective: the destination's rendered objectives
/// plus the journey's primary-goal answer (PRP doc §11: the package
/// "should start with the destination").
fn owner_objective(snapshot: &WorkspaceSnapshot) -> Section {
    let mut blocks = Vec::new();
    for objective in &snapshot.destination.objectives {
        let value = match preference_label(&objective.preference) {
            Some(label) => format!("{} ({})", objective.value, label),
            None => objective.value.clone(),
        };
        blocks.push(Block::Line {
            label: objective.label.clone(),
            value,
        });
    }
    if let Some(primary) = journey_answer(&snapshot.journey, "primary_objective") {
        blocks.push(Block::Line {
            label: "Primary goal (journey)".to_owned(),
            value: answer_value(primary),
        });
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no destination objectives have been recorded yet".to_owned(),
        });
    }
    section(SectionNumber::One, blocks)
}

/// Section 2 — Priorities: the secondary objectives in the owner's rank
/// order when one was recorded, else in selection order.
fn priorities(snapshot: &WorkspaceSnapshot) -> Section {
    let mut blocks = Vec::new();
    match journey_answer(&snapshot.journey, "secondary_ranking") {
        Some(ranked) if !ranked.values.is_empty() => {
            for (position, value) in ranked.values.iter().enumerate() {
                blocks.push(Block::Line {
                    label: format!("Priority {}", position + 1),
                    value: value.clone(),
                });
            }
        }
        _ => {
            if let Some(selected) = journey_answer(&snapshot.journey, "secondary_objectives") {
                if !selected.values.is_empty() {
                    blocks.push(Block::Text(
                        "The owner has not ranked these priorities; selection order only."
                            .to_owned(),
                    ));
                    for value in &selected.values {
                        blocks.push(Block::Line {
                            label: "Selected".to_owned(),
                            value: value.clone(),
                        });
                    }
                }
            }
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no secondary priorities have been recorded yet".to_owned(),
        });
    }
    section(SectionNumber::Two, blocks)
}

/// The owner's nonnegotiables and preferences, grouped by strength —
/// the 🔴/🟠/🟢 treatment the PRP doc §13 requires ("the package should
/// distinguish must-have from strong preference from flexible").
/// Returns (must-haves, strong preferences, preferences) as rendered
/// lines.
fn grouped_designations(snapshot: &WorkspaceSnapshot) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut must_haves = Vec::new();
    let mut strong = Vec::new();
    let mut preference = Vec::new();
    for objective in &snapshot.destination.objectives {
        match objective.preference {
            NonnegotiableState::Nonnegotiable => {
                must_haves.push(format!("{} — {}", objective.label, objective.value));
            }
            NonnegotiableState::StrongPreference => {
                strong.push(format!("{} — {}", objective.label, objective.value));
            }
            NonnegotiableState::Preference => {
                preference.push(format!("{} — {}", objective.label, objective.value));
            }
            NonnegotiableState::Unspecified => {}
        }
    }
    for marked in &snapshot.journey.marked_nonnegotiables {
        must_haves.push(marked.clone());
    }
    (must_haves, strong, preference)
}

/// Section 3 — Nonnegotiables.
fn nonnegotiables(snapshot: &WorkspaceSnapshot) -> Section {
    let (must_haves, strong, preference) = grouped_designations(snapshot);
    let mut blocks = Vec::new();
    for (label, items) in [
        ("Must-have (nonnegotiable)", must_haves),
        ("Strong preference", strong),
        ("Preference", preference),
    ] {
        for item in items {
            blocks.push(Block::Line {
                label: label.to_owned(),
                value: item,
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no objective has been designated a must-have or a preference yet".to_owned(),
        });
    }
    section(SectionNumber::Three, blocks)
}

/// Section 4 — Things the Owner Wants to Avoid: the journey's avoidance
/// answers plus the destination's avoidance objectives (the shell marks
/// those with an "Avoidance" label).
fn avoidances(snapshot: &WorkspaceSnapshot) -> Section {
    let mut blocks = Vec::new();
    if let Some(avoided) = journey_answer(&snapshot.journey, "avoid_outcomes") {
        for value in &avoided.values {
            blocks.push(Block::Line {
                label: "Journey".to_owned(),
                value: value.clone(),
            });
        }
    }
    for objective in &snapshot.destination.objectives {
        if objective.label.starts_with("Avoidance") {
            blocks.push(Block::Line {
                label: objective.label.clone(),
                value: objective.value.clone(),
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no avoidances have been recorded yet".to_owned(),
        });
    }
    section(SectionNumber::Four, blocks)
}

/// Section 5 — Business Snapshot: the owner-reported facts with their
/// provenance and verification stamps (PRP doc §15: only Business
/// Reality information, honestly labeled).
fn business_snapshot(snapshot: &WorkspaceSnapshot) -> Section {
    let mut blocks = Vec::new();
    for fact in &snapshot.reality.facts {
        let mut value = fact.value.clone();
        if let Some(definition) = &fact.definition {
            value.push_str(&format!(" ({definition})"));
        }
        value.push_str(&format!(
            " — owner reported, verification: {:?}, period: {}",
            fact.verification, fact.period
        ));
        blocks.push(Block::Line {
            label: fact.kind.clone(),
            value,
        });
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no business facts have been recorded yet".to_owned(),
        });
    }
    section(SectionNumber::Five, blocks)
}

/// Section 6 — Owner Transition Preferences: the timing, involvement,
/// liquidity, and seller-financing answers.
fn transition_preferences(snapshot: &WorkspaceSnapshot) -> Section {
    let preference_nodes = [
        "preferred_timing",
        "post_transaction_involvement",
        "involvement_duration",
        "liquidity_preference",
        "explore_liquidity_target",
        "liquidity_target_amount",
        "seller_financing_interest",
        "seller_note_intent",
    ];
    let mut blocks = Vec::new();
    for node in preference_nodes {
        if let Some(line) = journey_answer(&snapshot.journey, node) {
            blocks.push(Block::Line {
                label: line.title.clone(),
                value: answer_value(line),
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no transition preferences have been recorded yet".to_owned(),
        });
    }
    section(SectionNumber::Six, blocks)
}

/// Section 7 — Scenarios Explored: the ready scenarios, each with its
/// open unknowns and conflicts, under the platform-scenario disclaimer;
/// excluded scenarios are named as excluded (PRP doc §52 Exclusion
/// Manifest).
fn scenarios_explored(
    included: &[&ScenarioSnapshot],
    excluded: &[(&ScenarioSnapshot, ExportReadiness)],
) -> Section {
    let mut blocks = Vec::new();
    blocks.push(Block::Callout {
        label: "Platform Scenario".to_owned(),
        text: "These scenarios reflect the owner's stated objectives and the assumptions \
               currently recorded in the platform. They are exploratory and are not a \
               professional determination."
            .to_owned(),
    });
    for scenario in included {
        let mut lines = vec![format!(
            "Type: {} · lifecycle: {} · readiness: ready for professional review",
            scenario.scenario_type, scenario.lifecycle_status
        )];
        if let Some(description) = &scenario.description {
            lines.push(description.clone());
        }
        if scenario.unknowns.is_empty() {
            lines.push("Open questions: none recorded.".to_owned());
        } else {
            lines.push("Open questions this scenario does not yet answer:".to_owned());
            for unknown in &scenario.unknowns {
                lines.push(format!(
                    "- {} (importance: {}, status: {})",
                    unknown.description, unknown.importance, unknown.resolution_status
                ));
            }
        }
        if !scenario.conflicts.is_empty() {
            lines.push("Recorded conflicts (see Trade-Offs):".to_owned());
            for conflict in &scenario.conflicts {
                lines.push(format!(
                    "- {} (severity: {})",
                    conflict.description, conflict.severity
                ));
            }
        }
        blocks.push(Block::Entry {
            title: scenario.name.clone(),
            lines,
        });
    }
    if included.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "no scenario has reached the ready-for-professional-review readiness yet"
                .to_owned(),
        });
    }
    if !excluded.is_empty() {
        let named: Vec<String> = excluded
            .iter()
            .map(|(scenario, readiness)| format!("{} (readiness: {readiness:?})", scenario.name))
            .collect();
        blocks.push(Block::Callout {
            label: "Not included".to_owned(),
            text: format!(
                "{} — below the ready-for-professional-review readiness \
                 required for package inclusion.",
                named.join("; ")
            ),
        });
    }
    section(SectionNumber::Seven, blocks)
}

/// Section 8 — Assumptions Used: every included scenario's assumptions
/// with their provenance and verification stamps (PRP doc §16–17, §27).
fn assumptions_used(included: &[&ScenarioSnapshot]) -> Section {
    let mut blocks = Vec::new();
    for scenario in included {
        if scenario.assumptions.is_empty() {
            continue;
        }
        blocks.push(Block::Text(format!("Scenario: {}", scenario.name)));
        for assumption in &scenario.assumptions {
            let mut line = format!(
                "{} — {} (provenance: {:?}, verification: {:?})",
                assumption.name, assumption.value, assumption.provenance, assumption.verification
            );
            if let Some(description) = &assumption.description {
                line.push_str(&format!(". {description}"));
            }
            blocks.push(Block::Line {
                label: assumption.category.clone(),
                value: line,
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "the included scenarios record no assumptions yet".to_owned(),
        });
    }
    section(SectionNumber::Eight, blocks)
}

/// Section 9 — Trade-Offs Identified: the included scenarios' conflicts,
/// rendered as conditions with severity in words — never scores, never a
/// ranking (schema doc §16; comparison-without-winner, integrity rule 10).
fn trade_offs(included: &[&ScenarioSnapshot]) -> Section {
    let mut blocks = Vec::new();
    for scenario in included {
        for conflict in &scenario.conflicts {
            blocks.push(Block::Line {
                label: scenario.name.clone(),
                value: format!("{} (severity: {})", conflict.description, conflict.severity),
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "the included scenarios record no conflicts yet".to_owned(),
        });
    }
    section(SectionNumber::Nine, blocks)
}

/// Section 10 — Questions for Professionals: derived from the owner's
/// open unknowns and unverified assumptions. Every question here is
/// platform-derived and labeled as such (PRP doc §20, §27) — the
/// professional, not the platform, decides what the answers mean.
fn questions_for_professionals(included: &[&ScenarioSnapshot]) -> Section {
    let mut blocks = Vec::new();
    let mut count = 0usize;
    for scenario in included {
        for unknown in &scenario.unknowns {
            if unknown.resolution_status != "open" {
                continue;
            }
            count += 1;
            blocks.push(Block::Entry {
                title: format!("Question {count} (platform-derived)"),
                lines: vec![format!(
                    "Please help resolve: {} (importance: {}, scenario: {})",
                    unknown.description, unknown.importance, scenario.name
                )],
            });
        }
        for assumption in &scenario.assumptions {
            if assumption.verification == forhemit_contracts::Verification::Unverified {
                count += 1;
                blocks.push(Block::Entry {
                    title: format!("Question {count} (platform-derived)"),
                    lines: vec![format!(
                        "Please validate: {} — currently {} and unverified (scenario: {})",
                        assumption.name, assumption.value, scenario.name
                    )],
                });
            }
        }
    }
    blocks.push(Block::Text(
        "These questions were generated from the owner's recorded gaps and unverified \
         assumptions. They are not advice, and the platform does not know what the \
         answers should be."
            .to_owned(),
    ));
    section(SectionNumber::Ten, blocks)
}

/// Section 11 — Selected Professional Team. The v0.2 journey records no
/// professional selections as answers (the stage-21/22 nodes are
/// screens), so v1 states the absence honestly.
fn professional_team(snapshot: &WorkspaceSnapshot) -> Section {
    let mut blocks = Vec::new();
    for node in ["professional_team", "role_fit_check"] {
        if let Some(line) = journey_answer(&snapshot.journey, node) {
            blocks.push(Block::Line {
                label: line.title.clone(),
                value: answer_value(line),
            });
        }
    }
    if blocks.is_empty() {
        blocks.push(Block::NothingRecorded {
            context: "the journey has not recorded professional team selections yet".to_owned(),
        });
    }
    section(SectionNumber::Eleven, blocks)
}

/// Section 12 — Professional Determinations. v1 has no professional
/// review engine (deferred with a doc receipt — implementation spec's
/// out-of-scope table), so the section names the included scenarios and
/// says exactly what has and has not happened, rather than manufacturing
/// content.
fn professional_determinations(included: &[&ScenarioSnapshot]) -> Section {
    let mut blocks = Vec::new();
    for scenario in included {
        blocks.push(Block::Text(format!(
            "Scenario included for review: {} — not yet reviewed by a professional.",
            scenario.name
        )));
    }
    blocks.push(Block::Text(
        "No professional has reviewed this package yet. v1 has no professional review \
         engine; any future determination will be attributed to the professional who \
         made it, never presented as a platform recommendation."
            .to_owned(),
    ));
    section(SectionNumber::Twelve, blocks)
}

/// Section 13 — Current Working Plan: the recorded state in one place —
/// destination version, journey version and status, liquidity target,
/// timing, and the scenario set.
fn working_plan(snapshot: &WorkspaceSnapshot, included: &[&ScenarioSnapshot]) -> Section {
    let mut blocks = vec![
        Block::Line {
            label: "Destination version".to_owned(),
            value: format!(
                "{} (version {})",
                snapshot.destination.destination_id, snapshot.destination.version_number
            ),
        },
        Block::Line {
            label: "Journey version used".to_owned(),
            value: format!(
                "{} v{} — {}",
                snapshot.journey.journey_id,
                snapshot.journey.journey_version_used,
                if snapshot.journey.completed {
                    "walk completed"
                } else {
                    "walk still in progress"
                }
            ),
        },
        Block::Line {
            label: "Business reality version".to_owned(),
            value: snapshot.reality.business_reality_version_id.to_string(),
        },
    ];
    if let Some(target) = journey_answer(&snapshot.journey, "liquidity_target_amount") {
        blocks.push(Block::Line {
            label: "Liquidity target at closing".to_owned(),
            value: answer_value(target),
        });
    }
    if let Some(timing) = journey_answer(&snapshot.journey, "preferred_timing") {
        blocks.push(Block::Line {
            label: "Preferred timing".to_owned(),
            value: answer_value(timing),
        });
    }
    if included.is_empty() {
        blocks.push(Block::Text(
            "No scenario has reached the ready-for-professional-review readiness, so the \
             plan currently has no scenario set attached."
                .to_owned(),
        ));
    } else {
        let names: Vec<String> = included
            .iter()
            .map(|scenario| scenario.name.clone())
            .collect();
        blocks.push(Block::Line {
            label: "Scenarios in the plan".to_owned(),
            value: names.join(", "),
        });
    }
    section(SectionNumber::Thirteen, blocks)
}

/// Builds the executive summary from the same data the sections use —
/// one source of truth (EOJ v0.2 §24's shape).
pub fn build_executive_summary(
    snapshot: &WorkspaceSnapshot,
    included: &[&ScenarioSnapshot],
) -> ExecutiveSummary {
    let (must_haves, strong_preferences, _preference) = grouped_designations(snapshot);
    let wanted_labels = [
        "Main financial objective",
        "Desired cash at closing",
        "Desired future income",
        "Who should own the company",
    ];
    let mut what_i_want = Vec::new();
    for label in wanted_labels {
        if let Some(objective) = snapshot
            .destination
            .objectives
            .iter()
            .find(|objective: &&ObjectiveLine| objective.label == label)
        {
            what_i_want.push(Block::Line {
                label: objective.label.clone(),
                value: objective.value.clone(),
            });
        }
    }
    if let Some(primary) = journey_answer(&snapshot.journey, "primary_objective") {
        what_i_want.push(Block::Line {
            label: "Primary goal (journey)".to_owned(),
            value: answer_value(primary),
        });
    }
    let wants_to_avoid = journey_answer(&snapshot.journey, "avoid_outcomes")
        .map(|line| line.values.clone())
        .unwrap_or_default();
    let mut main_questions: Vec<String> = Vec::new();
    for scenario in included {
        for unknown in &scenario.unknowns {
            if unknown.resolution_status == "open" {
                main_questions.push(unknown.description.clone());
            }
        }
    }
    ExecutiveSummary {
        what_i_want,
        must_haves,
        strong_preferences,
        wants_to_avoid,
        paths_evaluated: included
            .iter()
            .map(|scenario| scenario.name.clone())
            .collect(),
        main_questions,
    }
}

/// Assembles all 13 sections from the snapshot, with the gate already
/// applied: `included` are the ready scenarios, `excluded` the ones the
/// readiness kept out.
pub fn build_sections(
    snapshot: &WorkspaceSnapshot,
    included: &[&ScenarioSnapshot],
    excluded: &[(&ScenarioSnapshot, ExportReadiness)],
) -> Vec<Section> {
    vec![
        owner_objective(snapshot),
        priorities(snapshot),
        nonnegotiables(snapshot),
        avoidances(snapshot),
        business_snapshot(snapshot),
        transition_preferences(snapshot),
        scenarios_explored(included, excluded),
        assumptions_used(included),
        trade_offs(included),
        questions_for_professionals(included),
        professional_team(snapshot),
        professional_determinations(included),
        working_plan(snapshot, included),
    ]
}

/// Fills in the canonical title from the section number.
fn section(number: SectionNumber, blocks: Vec<Block>) -> Section {
    Section {
        number,
        title: number.title().to_owned(),
        blocks,
    }
}
