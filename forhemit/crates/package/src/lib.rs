//! # forhemit-package
//!
//! The Professional Review Package engine — the journey's final assembly
//! step (EOJ v0.2 stage 20; README MVP boundary: the first release stops
//! at *Professional Review*).
//!
//! ## What lives here
//!
//! - [`inputs`] — the input ports: contract-typed snapshots of the
//!   journey, destination, Business Reality, and scenario state, mapped
//!   into this crate by the app shell (the composition root). This crate
//!   depends only on `contracts` and `enginekit` — the package engine
//!   reads the other engines' *state*, never their internals (Standalone
//!   Engine Architecture).
//! - [`sections`] — the canonical section structure: a one-page executive
//!   summary followed by the 13 sections EOJ v0.2 §23 names, verbatim,
//!   each built from exactly the snapshot data that owns it.
//! - [`package`] — the `ProfessionalReviewPackage` object and the
//!   `PackageEngine`: assembly with readiness gating, and the export
//!   action that emits one audit event per exported format.
//! - [`export`] — self-contained, printable HTML and PDF renderers.
//!   No network, no fonts to fetch: the PDF writer uses the base-14
//!   Helvetica fonts and plain text, so the package renders identically
//!   offline.
//!
//! ## The rules that shape the crate
//!
//! - **Readiness gates the export.** Only scenario versions whose
//!   readiness is `ready_for_professional_review` (schema doc §40 —
//!   "can the package include it?") may appear in the package. Excluded
//!   scenarios are named as excluded, with the readiness that kept them
//!   out — never silently dropped (PRP doc §52 Exclusion Manifest).
//! - **Unknowns stay visible.** A scenario's `ScenarioUnknown` rows are
//!   first-class entries with their importance (schema doc §15) — never
//!   rendered as blank zeros, never omitted (implementation spec).
//! - **One source of truth, honestly labeled.** Every line carries its
//!   layer: owner-reported, platform scenario, or system-derived (PRP doc
//!   §27 "No unattributed AI claims"; THREE DECISION LAYERS §14–15
//!   "Whose information am I looking at?").
//! - **Not advice.** Every export carries the disclosure: this package
//!   organizes the owner's own information; it is not a recommendation
//!   or valuation (README: "Nothing here constitutes legal, tax,
//!   investment, valuation, or financing advice"; PRP doc §47).
//! - **Audit first.** The export action emits `PackageExported` before
//!   the caller sees the bytes (Implementation Roadmap Phase 1); the
//!   event payload records every source version the package was
//!   assembled from (Journey Builder doc §23: a package records "Journey
//!   Version Used").

pub mod error;
pub mod export;
pub mod inputs;
pub mod package;
pub mod sections;

pub use error::PackageError;
pub use export::{ExportFormat, ExportedContent, ExportedPackage, HtmlRenderer, PdfRenderer};
pub use inputs::{
    AnswerLine, AssumptionLine, ConflictLine, DestinationSnapshot, ExportReadiness, FactLine,
    JourneySnapshot, ObjectiveLine, RealitySnapshot, ScenarioSnapshot, UnknownLine,
    WorkspaceSnapshot,
};
pub use package::{
    gate_scenarios, ExcludedScenario, PackageEngine, ProfessionalReviewPackage, ProvenanceBlock,
};
pub use sections::{
    build_executive_summary, build_sections, Block, ExecutiveSummary, Section, SectionNumber,
    SECTION_TITLES,
};
