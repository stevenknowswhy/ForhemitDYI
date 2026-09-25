//! Self-contained export renderers: printable HTML and PDF.
//!
//! Both renderers are offline by construction — no network fetches, no
//! external fonts, no JavaScript. The HTML is a single document with
//! inline styles; the PDF is written directly (PDF 1.4, base-14
//! Helvetica, WinAnsi encoding), so the package renders identically
//! without bundling or downloading fonts. Both renderers are pure
//! functions of the assembled package: the same package renders to the
//! same bytes, which is what the golden-file tests pin.

use crate::package::ProfessionalReviewPackage;
use crate::sections::{Block, ExecutiveSummary, Section};
use serde::{Deserialize, Serialize};

/// The export formats the engine can produce.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    /// Self-contained printable HTML.
    Html,
    /// Self-contained printable PDF (base-14 fonts, no embedding).
    Pdf,
}

/// The rendered bytes of one exported package.
#[derive(Clone, Debug, PartialEq)]
pub enum ExportedContent {
    /// The HTML document.
    Html(
        /// The full HTML document text.
        String,
    ),
    /// The PDF document.
    Pdf(
        /// The full PDF bytes.
        Vec<u8>,
    ),
}

/// One exported package artifact: its format, the engine's suggested
/// file name, and the rendered bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct ExportedPackage {
    /// The format the content was rendered in.
    pub format: ExportFormat,
    /// The engine's suggested file name — the shell may rename on save.
    pub file_name: String,
    /// The rendered bytes.
    pub content: ExportedContent,
}

/// Renders the package as a self-contained, printable HTML document.
#[derive(Clone, Copy, Debug, Default)]
pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Renders the full HTML document.
    #[must_use]
    pub fn render(&self, package: &ProfessionalReviewPackage) -> String {
        let mut out = String::with_capacity(16_384);
        out.push_str(HTML_HEAD);
        out.push_str("<h1>Professional Review Package</h1>\n");
        out.push_str(&format!(
            "<p class=\"assembled\">Assembled {}</p>\n",
            html_escape(&iso_stamp(package.created_at))
        ));
        out.push_str(&format!(
            "<aside class=\"disclosure\"><strong>Important:</strong> {}</aside>\n",
            html_escape(ProfessionalReviewPackage::disclosure())
        ));
        out.push_str(&provenance_html(package));
        out.push_str(&summary_html(&package.summary));
        for section in &package.sections {
            out.push_str(&section_html(section));
        }
        out.push_str(&format!(
            "<footer><p>{}</p></footer>\n</body>\n</html>\n",
            html_escape(ProfessionalReviewPackage::disclosure())
        ));
        out
    }
}

/// Renders the package as a self-contained, printable PDF.
#[derive(Clone, Copy, Debug, Default)]
pub struct PdfRenderer;

impl PdfRenderer {
    /// Renders the full PDF document.
    #[must_use]
    pub fn render(&self, package: &ProfessionalReviewPackage) -> Vec<u8> {
        pdf_document(package)
    }
}

/// Formats the assembly time as a sortable ISO-8601 stamp.
fn iso_stamp(at: time::OffsetDateTime) -> String {
    let format = time::format_description::well_known::Rfc3339;
    // The format call can only fail on out-of-range years; a package
    // carrying such a timestamp would fail assembly everywhere else too.
    at.format(&format)
        .unwrap_or_else(|_| "unknown time".to_owned())
}

// ---------------------------------------------------------------------------
// HTML rendering
// ---------------------------------------------------------------------------

/// The document head with the inline print stylesheet.
const HTML_HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Professional Review Package</title>
<style>
  :root { color-scheme: light; }
  body { font-family: -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
         line-height: 1.45; color: #1a1a1a; max-width: 46rem; margin: 2rem auto; padding: 0 1rem; }
  h1 { font-size: 1.6rem; margin-bottom: 0; }
  .assembled { color: #555; font-size: 0.85rem; margin-top: 0.2rem; }
  .disclosure { border: 2px solid #8a6d3b; background: #fdf8ec; padding: 0.6rem 0.9rem;
                border-radius: 6px; font-size: 0.9rem; }
  .provenance { background: #f4f4f5; border-radius: 6px; padding: 0.8rem 1rem; margin: 1rem 0; }
  .provenance dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.2rem 1rem; margin: 0; }
  .provenance dt { font-weight: 600; }
  .provenance dd { margin: 0; word-break: break-all; }
  h2 { font-size: 1.1rem; border-bottom: 1px solid #ddd; padding-bottom: 0.2rem;
       margin-top: 1.8rem; }
  .line { margin: 0.3rem 0; }
  .line .label { font-weight: 600; }
  .line .label::after { content: ": "; }
  .entry { border-left: 3px solid #cbd5e1; padding-left: 0.8rem; margin: 0.7rem 0; }
  .entry h3 { font-size: 0.95rem; margin: 0.2rem 0; }
  .entry ul { margin: 0.2rem 0 0.2rem 1rem; padding: 0; }
  .callout { background: #f0f7ff; border-radius: 6px; padding: 0.5rem 0.8rem; margin: 0.6rem 0; }
  .callout .label { font-weight: 600; }
  .callout .label::after { content: ": "; }
  .nothing-recorded { color: #666; font-style: italic; }
  footer { margin-top: 2rem; border-top: 1px solid #ddd; padding-top: 0.6rem;
           color: #555; font-size: 0.85rem; }
  @media print {
    body { max-width: none; margin: 0; font-size: 10.5pt; }
    section { page-break-inside: avoid; }
    h2 { page-break-after: avoid; }
  }
</style>
</head>
<body>
"#;

/// Escapes text for safe inclusion in HTML content and attributes.
fn html_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(character),
        }
    }
    out
}

/// Renders the provenance block as a definition list.
fn provenance_html(package: &ProfessionalReviewPackage) -> String {
    let provenance = &package.provenance;
    let mut out = String::from("<section class=\"provenance\">\n<h2>Provenance</h2>\n<dl>\n");
    let included = if provenance.included_scenario_version_ids.is_empty() {
        "none — no scenario has reached the ready-for-professional-review readiness".to_owned()
    } else {
        provenance
            .included_scenario_version_ids
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    };
    let excluded = if provenance.excluded_scenarios.is_empty() {
        "none".to_owned()
    } else {
        provenance
            .excluded_scenarios
            .iter()
            .map(|excluded| format!("{} (readiness: {:?})", excluded.name, excluded.readiness))
            .collect::<Vec<_>>()
            .join("; ")
    };
    for (label, value) in [
        ("Journey", provenance.journey_id.as_str().to_owned()),
        (
            "Journey version used",
            provenance.journey_version_used.clone(),
        ),
        ("Destination", provenance.destination_id.as_str().to_owned()),
        (
            "Destination version",
            provenance.destination_version_id.as_str().to_owned(),
        ),
        (
            "Destination version number",
            provenance.destination_version_number.to_string(),
        ),
        (
            "Business reality version",
            provenance.business_reality_version_id.as_str().to_owned(),
        ),
        ("Scenario versions included", included),
        ("Scenarios excluded", excluded),
        ("Assembled", iso_stamp(package.created_at)),
    ] {
        out.push_str(&format!(
            "<dt>{}:</dt><dd>{}</dd>\n",
            html_escape(label),
            html_escape(&value)
        ));
    }
    out.push_str("</dl>\n</section>\n");
    out
}

/// Renders the one-page executive summary.
fn summary_html(summary: &ExecutiveSummary) -> String {
    let mut out =
        String::from("<section class=\"executive-summary\">\n<h2>Executive Summary</h2>\n");
    out.push_str("<h3>What I want</h3>\n");
    out.push_str(&blocks_html(&summary.what_i_want));
    out.push_str("<h3>My must-haves</h3>\n<ul>\n");
    for item in &summary.must_haves {
        out.push_str(&format!("<li>{}</li>\n", html_escape(item)));
    }
    out.push_str("</ul>\n<h3>Strong preferences</h3>\n<ul>\n");
    for item in &summary.strong_preferences {
        out.push_str(&format!("<li>{}</li>\n", html_escape(item)));
    }
    out.push_str("</ul>\n<h3>Things I want to avoid</h3>\n<ul>\n");
    for item in &summary.wants_to_avoid {
        out.push_str(&format!("<li>{}</li>\n", html_escape(item)));
    }
    out.push_str("</ul>\n<h3>Paths being evaluated</h3>\n<ul>\n");
    if summary.paths_evaluated.is_empty() {
        out.push_str(
            "<li class=\"nothing-recorded\">No scenario has reached the \
             ready-for-professional-review readiness yet</li>\n",
        );
    }
    for item in &summary.paths_evaluated {
        out.push_str(&format!("<li>{}</li>\n", html_escape(item)));
    }
    out.push_str("</ul>\n<h3>Main questions for my advisors</h3>\n<ul>\n");
    if summary.main_questions.is_empty() {
        out.push_str("<li class=\"nothing-recorded\">None recorded yet</li>\n");
    }
    for item in &summary.main_questions {
        out.push_str(&format!("<li>{}</li>\n", html_escape(item)));
    }
    out.push_str("</ul>\n</section>\n");
    out
}

/// Renders one section with its canonical number and title.
fn section_html(section: &Section) -> String {
    let mut out = format!(
        "<section>\n<h2>{}. {}</h2>\n",
        section.number.index(),
        html_escape(&section.title)
    );
    out.push_str(&blocks_html(&section.blocks));
    out.push_str("</section>\n");
    out
}

/// Renders a block list — shared by sections and the summary.
fn blocks_html(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            Block::Line { label, value } => out.push_str(&format!(
                "<div class=\"line\"><span class=\"label\">{}</span><span class=\"value\">{}</span></div>\n",
                html_escape(label),
                html_escape(value)
            )),
            Block::Text(text) => out.push_str(&format!("<p>{}</p>\n", html_escape(text))),
            Block::Entry { title, lines } => {
                out.push_str(&format!("<div class=\"entry\">\n<h3>{}</h3>\n<ul>\n", html_escape(title)));
                for line in lines {
                    out.push_str(&format!("<li>{}</li>\n", html_escape(line)));
                }
                out.push_str("</ul>\n</div>\n");
            }
            Block::Callout { label, text } => out.push_str(&format!(
                "<aside class=\"callout\"><span class=\"label\">{}</span>{}</aside>\n",
                html_escape(label),
                html_escape(text)
            )),
            Block::NothingRecorded { context } => out.push_str(&format!(
                "<p class=\"nothing-recorded\">Nothing recorded yet — {}.</p>\n",
                html_escape(context)
            )),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// PDF rendering — a minimal, dependency-free PDF 1.4 writer
// ---------------------------------------------------------------------------

/// Letter page width in points.
const PAGE_WIDTH: f64 = 612.0;
/// Letter page height in points.
const PAGE_HEIGHT: f64 = 792.0;
/// Page margin in points.
const MARGIN: f64 = 72.0;
/// Body font size in points.
const BODY_SIZE: f64 = 10.0;

/// One laid-out text line, waiting to be flowed onto pages.
struct PdfLine {
    text: String,
    bold: bool,
    size: f64,
    /// Left indent in points.
    indent: f64,
    /// Extra vertical space before the line, in points.
    space_before: f64,
}

/// The line-push callback shared by the layout pass and block renderer.
type PushLine<'a> = &'a mut dyn FnMut(&str, bool, f64, f64, f64, &mut Vec<PdfLine>);

/// Approximates a character's advance width in em units (Helvetica).
/// Deliberately conservative — over-estimating keeps wrapped text inside
/// the margins.
fn char_em(character: char) -> f64 {
    match character {
        ' ' | '\u{00A0}' => 0.30,
        'i' | 'j' | 'l' | '.' | ',' | ';' | ':' | '!' | '|' | '\'' => 0.30,
        'f' | 't' | 'r' | '(' | ')' | '[' | ']' | '-' | '/' => 0.37,
        'm' | 'w' | 'M' | 'W' | '@' => 0.85,
        'A'..='Z' => 0.68,
        '0'..='9' => 0.56,
        _ => 0.53,
    }
}

/// A character's advance width in points at `size`.
fn char_width(character: char, size: f64) -> f64 {
    char_em(character) * size
}

/// A string's advance width in points at `size`.
fn text_width(text: &str, size: f64) -> f64 {
    text.chars()
        .map(|character| char_width(character, size))
        .sum()
}

/// Wraps text to `max_width` points at `size`, greedy word wrap with a
/// hard break for over-long words.
fn wrap_text(text: &str, max_width: f64, size: f64, out: &mut Vec<String>) {
    for paragraph in text.split('\n') {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if text_width(&candidate, size) <= max_width {
                current = candidate;
            } else {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
                // A single word longer than the line: hard-break it.
                let mut piece = String::new();
                for character in word.chars() {
                    piece.push(character);
                    if text_width(&piece, size) > max_width {
                        piece.pop();
                        out.push(piece.clone());
                        piece.clear();
                        piece.push(character);
                    }
                }
                current = piece;
            }
        }
        out.push(current);
    }
}

/// Maps a character to its WinAnsiEncoding byte, if representable.
fn winansi(character: char) -> Option<u8> {
    match character {
        '\u{0020}'..='\u{007E}' => Some(character as u8),
        '\u{00A0}' => Some(0x20),
        '\u{00A1}'..='\u{00FF}' => Some(character as u8),
        '\u{2010}' | '\u{2011}' | '\u{2012}' | '\u{2013}' => Some(0x96), // en dash
        '\u{2014}' => Some(0x97),                                        // em dash
        '\u{2018}' => Some(0x91),
        '\u{2019}' => Some(0x92),
        '\u{201C}' => Some(0x93),
        '\u{201D}' => Some(0x94),
        '\u{2022}' => Some(0x95), // bullet
        '\u{2026}' => Some(0x85), // ellipsis
        '\u{20AC}' => Some(0x80), // euro
        '\u{0152}' => Some(0x8C), // OE ligature
        '\u{0153}' => Some(0x9C), // oe ligature
        _ => None,
    }
}

/// Encodes a string as a PDF literal string: WinAnsi bytes with the
/// three PDF escapes. Unrepresentable characters become `?` — honest
/// degradation, never silent data invention.
fn pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 8);
    out.push('(');
    for character in text.chars() {
        let byte = winansi(character).unwrap_or(b'?');
        match byte {
            b'(' | b')' | b'\\' => {
                out.push('\\');
                out.push(byte as char);
            }
            0x20..=0x7E => out.push(byte as char),
            other => {
                out.push_str(&format!("\\{other:03o}"));
            }
        }
    }
    out.push(')');
    out
}

/// Escapes a raw (ASCII) byte string the same way — used for any
/// assembled text.
fn pdf_text_line(text: &str, font: &str, size: f64, x: f64, y: f64) -> String {
    format!(
        "BT /{font} {size:.1} Tf 1 0 0 1 {x:.2} {y:.2} Tm {} Tj ET\n",
        pdf_string(text)
    )
}

/// Lays the package out as a flat list of positioned lines.
fn layout_lines(package: &ProfessionalReviewPackage) -> Vec<PdfLine> {
    const CONTENT_WIDTH: f64 = PAGE_WIDTH - 2.0 * MARGIN;
    let mut lines = Vec::new();
    let mut push = |text: &str,
                    bold: bool,
                    size: f64,
                    indent: f64,
                    space_before: f64,
                    lines: &mut Vec<PdfLine>| {
        let mut wrapped = Vec::new();
        wrap_text(text, CONTENT_WIDTH - indent, size, &mut wrapped);
        for (position, line) in wrapped.into_iter().enumerate() {
            lines.push(PdfLine {
                text: line,
                bold,
                size,
                indent,
                space_before: if position == 0 { space_before } else { 2.0 },
            });
        }
    };

    push(
        "Professional Review Package",
        true,
        18.0,
        0.0,
        0.0,
        &mut lines,
    );
    push(
        &format!("Assembled {}", iso_stamp(package.created_at)),
        false,
        9.0,
        0.0,
        6.0,
        &mut lines,
    );
    push(
        &format!("Important: {}", ProfessionalReviewPackage::disclosure()),
        false,
        10.0,
        0.0,
        14.0,
        &mut lines,
    );

    // Provenance block — required provenance, never omitted.
    push("Provenance", true, 13.0, 0.0, 20.0, &mut lines);
    let provenance = &package.provenance;
    for line in [
        format!("Journey: {}", provenance.journey_id),
        format!("Journey version used: {}", provenance.journey_version_used),
        format!("Destination: {}", provenance.destination_id),
        format!(
            "Destination version: {} (number {})",
            provenance.destination_version_id, provenance.destination_version_number
        ),
        format!(
            "Business reality version: {}",
            provenance.business_reality_version_id
        ),
        format!(
            "Scenario versions included: {}",
            if provenance.included_scenario_version_ids.is_empty() {
                "none — no scenario has reached the ready-for-professional-review readiness"
                    .to_owned()
            } else {
                provenance
                    .included_scenario_version_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ),
        format!(
            "Scenarios excluded: {}",
            if provenance.excluded_scenarios.is_empty() {
                "none".to_owned()
            } else {
                provenance
                    .excluded_scenarios
                    .iter()
                    .map(|excluded| {
                        format!("{} (readiness: {:?})", excluded.name, excluded.readiness)
                    })
                    .collect::<Vec<_>>()
                    .join("; ")
            }
        ),
        format!("Assembled: {}", iso_stamp(package.created_at)),
    ] {
        push(&line, false, BODY_SIZE, 0.0, 3.0, &mut lines);
    }

    // Executive summary.
    push("Executive Summary", true, 13.0, 0.0, 20.0, &mut lines);
    let summary = &package.summary;
    push("What I want", true, BODY_SIZE, 0.0, 8.0, &mut lines);
    for block in &summary.what_i_want {
        match block {
            Block::Line { label, value } => {
                push(
                    &format!("{label}: {value}"),
                    false,
                    BODY_SIZE,
                    12.0,
                    2.0,
                    &mut lines,
                );
            }
            Block::Text(text) => push(text, false, BODY_SIZE, 12.0, 2.0, &mut lines),
            _ => {}
        }
    }
    push("My must-haves", true, BODY_SIZE, 0.0, 8.0, &mut lines);
    if summary.must_haves.is_empty() {
        push(
            "None recorded yet.",
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    for item in &summary.must_haves {
        push(
            &format!("- {item}"),
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    push("Strong preferences", true, BODY_SIZE, 0.0, 8.0, &mut lines);
    if summary.strong_preferences.is_empty() {
        push(
            "None recorded yet.",
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    for item in &summary.strong_preferences {
        push(
            &format!("- {item}"),
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    push(
        "Things I want to avoid",
        true,
        BODY_SIZE,
        0.0,
        8.0,
        &mut lines,
    );
    if summary.wants_to_avoid.is_empty() {
        push(
            "None recorded yet.",
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    for item in &summary.wants_to_avoid {
        push(
            &format!("- {item}"),
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    push(
        "Paths being evaluated",
        true,
        BODY_SIZE,
        0.0,
        8.0,
        &mut lines,
    );
    if summary.paths_evaluated.is_empty() {
        push(
            "No scenario has reached the ready-for-professional-review readiness yet.",
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    for item in &summary.paths_evaluated {
        push(
            &format!("- {item}"),
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    push(
        "Main questions for my advisors",
        true,
        BODY_SIZE,
        0.0,
        8.0,
        &mut lines,
    );
    if summary.main_questions.is_empty() {
        push(
            "None recorded yet.",
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }
    for item in &summary.main_questions {
        push(
            &format!("- {item}"),
            false,
            BODY_SIZE,
            12.0,
            2.0,
            &mut lines,
        );
    }

    // The 13 canonical sections.
    for section in &package.sections {
        push(
            &format!("{}. {}", section.number.index(), section.title),
            true,
            13.0,
            0.0,
            22.0,
            &mut lines,
        );
        for block in &section.blocks {
            render_block(block, &mut push, &mut lines);
        }
    }
    lines
}

/// Renders one content block into laid-out lines.
fn render_block(block: &Block, push: PushLine<'_>, lines: &mut Vec<PdfLine>) {
    match block {
        Block::Line { label, value } => {
            push(
                &format!("{label}: {value}"),
                false,
                BODY_SIZE,
                0.0,
                3.0,
                lines,
            );
        }
        Block::Text(text) => {
            push(text, false, BODY_SIZE, 0.0, 3.0, lines);
        }
        Block::Entry {
            title,
            lines: sub_lines,
        } => {
            push(title, true, BODY_SIZE, 0.0, 8.0, lines);
            for sub in sub_lines {
                push(&format!("- {sub}"), false, BODY_SIZE, 14.0, 2.0, lines);
            }
        }
        Block::Callout { label, text } => {
            push(
                &format!("{label}: {text}"),
                false,
                BODY_SIZE,
                12.0,
                6.0,
                lines,
            );
        }
        Block::NothingRecorded { context } => {
            push(
                &format!("Nothing recorded yet — {context}."),
                false,
                BODY_SIZE,
                0.0,
                3.0,
                lines,
            );
        }
    }
}

/// Flows laid-out lines into page content streams, emitting the footer
/// (page number and the short disclosure) on every page.
fn pdf_document(package: &ProfessionalReviewPackage) -> Vec<u8> {
    let all_lines = layout_lines(package);
    let top = PAGE_HEIGHT - MARGIN;
    let bottom = MARGIN;
    let mut pages: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut y = top;
    for line in &all_lines {
        let leading = line.size * 1.3;
        let needed = leading + line.space_before;
        if y - needed < bottom && !current.is_empty() {
            pages.push(std::mem::take(&mut current));
            y = top;
        }
        y -= needed;
        let font = if line.bold { "F2" } else { "F1" };
        current.push_str(&pdf_text_line(
            &line.text,
            font,
            line.size,
            MARGIN + line.indent,
            y,
        ));
    }
    if !current.is_empty() {
        pages.push(current);
    }
    if pages.is_empty() {
        pages.push(String::new());
    }

    assemble_pdf(&pages)
}

/// Writes one numbered object into the buffer, tracking its byte offset.
fn push_object(out: &mut Vec<u8>, offsets: &mut Vec<usize>, number: usize, body: &str) {
    offsets.push(out.len());
    out.extend_from_slice(format!("{number} 0 obj\n{body}\nendobj\n").as_bytes());
}

/// Serializes pages into a valid PDF file: catalog, page tree, the two
/// base-14 fonts, one content stream per page, xref, trailer.
fn assemble_pdf(pages: &[String]) -> Vec<u8> {
    let page_count = pages.len();
    // Object numbering: 1 catalog, 2 pages tree, 3 F1, 4 F2, then per
    // page: content stream then page object.
    let first_content = 5;
    let mut offsets: Vec<usize> = Vec::with_capacity(4 + page_count * 2);
    let mut out: Vec<u8> = Vec::with_capacity(16 * 1024);

    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    push_object(
        &mut out,
        &mut offsets,
        1,
        "<< /Type /Catalog /Pages 2 0 R >>",
    );
    let kids: Vec<String> = (0..page_count)
        .map(|page| format!("{} 0 R", first_content + page * 2 + 1))
        .collect();
    push_object(
        &mut out,
        &mut offsets,
        2,
        &format!(
            "<< /Type /Pages /Count {page_count} /Kids [{}] >>",
            kids.join(" ")
        ),
    );
    push_object(
        &mut out,
        &mut offsets,
        3,
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
    );
    push_object(
        &mut out,
        &mut offsets,
        4,
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>",
    );

    for (page, content) in pages.iter().enumerate() {
        let content_number = first_content + page * 2;
        let page_number = content_number + 1;
        let mut footer = pdf_text_line(
            &format!("Page {} of {}", page + 1, page_count),
            "F1",
            8.0,
            MARGIN,
            40.0,
        );
        footer.push_str(&pdf_text_line(
            "Not a recommendation or valuation.",
            "F1",
            8.0,
            PAGE_WIDTH - MARGIN - 150.0,
            40.0,
        ));
        let stream = format!("{content}{footer}");
        let stream_body = format!("<< /Length {} >>\nstream\n{stream}endstream", stream.len());
        push_object(&mut out, &mut offsets, content_number, &stream_body);
        push_object(
            &mut out,
            &mut offsets,
            page_number,
            &format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {PAGE_WIDTH} {PAGE_HEIGHT}] \
                  /Resources << /Font << /F1 3 0 R /F2 4 0 R >> >> /Contents {content_number} 0 R >>"
            ),
        );
    }

    let xref_offset = out.len();
    let object_count = offsets.len() + 1;
    out.extend_from_slice(format!("xref\n0 {object_count}\n").as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in &offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {object_count} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n"
        )
        .as_bytes(),
    );
    out
}
