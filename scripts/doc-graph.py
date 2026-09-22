#!/usr/bin/env python3
"""
doc-graph.py - a dependency-cruiser for the Forhemit design bible.

The corpus is Markdown, so `import` statements do not exist and an
import-graph tool has nothing to traverse. This script supplies the missing
layer: it extracts the corpus's *real* edge syntax and runs the four checks
dependency-cruiser is known for, translated to documents.

Edge syntax it understands (in descending order of authority):

  1. boundary summary tables   | **Engine** | Owns | Does Not Own |
     Every row names a DIFFERENT engine. This is a mutual, typed declaration.
  2. the Complete Architecture flow diagram (section 20)
  3. the "around all of it" infrastructure ring (section 20)
  4. the README three-layer roster ("Architecture at a Glance")
  5. explicit markdown links between documents

Checks it runs:

The authoritative list is GUARDS below -- it is the single source of truth for
the report's summary table, the console summary and `--check`'s exit status. The
thirteen fall into four families:

  roster agreement     declaration-consistency, readme-roster, coverage
  declared-but-absent  no-dangling, stale-absence
  boundary quality     boundary-tier-none, boundary-tier-prose, undeclared-reference,
                       never-named, structural-ambiguity
  housekeeping         naming-drift, no-orphans, no-layer-violation

Not everything in the architecture is an engine. The section-20 ring names
"Security", which is a cross-cutting property rather than a component: see
CROSS_CUTTING below, which records the decision and keeps it out of the
dangling check on purpose.

The report is versioned, so it can drift from the corpus it describes. Each
run compares the new report against the committed one: if the content changed
the outgoing version is archived to analysis/archive/ under the timestamp it
carried, and the new one takes its place. If nothing changed, the working tree
is left completely alone.

Usage:
    python3 scripts/doc-graph.py                 # write reports into analysis/
    python3 scripts/doc-graph.py --stdout        # print the report, write nothing
    python3 scripts/doc-graph.py --check         # run checks, write nothing, gate
    python3 scripts/doc-graph.py --check-report  # is the committed report current?
    python3 scripts/doc-graph.py --staged-files  # print paths that changed

--staged-files is the interface the pre-commit hook uses: it regenerates as a
side effect and prints one path per line, empty when the report is current.

--check and --check-report are the two read-only gates, and the only modes with a
meaningful exit status (0 pass, 1 fail). Neither publishes anything.

    --check         every guard in GUARDS is zero -- the corpus is clean.
    --check-report  the committed report matches the corpus -- no drift.

They fail independently and both matter: a clean corpus with a stale report is
still a false statement sitting on disk. Use them to verify; do not use the
default mode for that, because publishing overwrites the very report you would
compare against.
"""

from __future__ import annotations

import os
import re
import sys
import glob
import json
import subprocess
import urllib.parse
from datetime import datetime
from collections import defaultdict, OrderedDict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTDIR = os.path.join(ROOT, "analysis")

README = "README.md"
INDEX = "DOCUMENT-INDEX.md"
ARCH = ("Complete Architecture - Decision Engines, Transaction Engines, "
        "Platform Infrastructure.md")

REPORT = "GAP-ANALYSIS.md"
ARCHIVE_DIR = "archive"

# The report carries the moment it was generated. Two runs seconds apart must
# still compare as "unchanged", so the whole line is replaced before
# comparison -- otherwise every commit would archive a version that differs
# only by clock. It must consume the full line: a regex that stops at the
# minutes leaves ":26 -0700" behind and never matches.
GENERATED_LINE_RE = re.compile(r"^\* generated: .*$", re.M)
STAMP_RE = re.compile(r"^\* generated: (\d{4}-\d{2}-\d{2}) (\d{2}):(\d{2})",
                      re.M)

# The checks that are GUARDS: a non-empty result is a defect, and the corpus is
# expected to score zero on all of them. This tuple is the single source of
# truth for three consumers -- the report's summary table, the console summary,
# and `--check`'s exit status. It used to be written out twice (once as the
# report's `order` list, once as main()'s literal tuple), which is precisely the
# hand-maintained-view bug these checks exist to catch: add a guard to one copy
# and the other keeps silently reporting on the old set.
#
# `boundary-tier-table` and `accepted-multi-engine` are deliberately NOT guards.
# They are REGISTERS, and both are non-empty on a healthy corpus:
# `boundary-tier-table` counts documents using the machine-readable form (more
# is better), and `accepted-multi-engine` lists decisions already taken. A
# verifier that flags any non-empty bucket calls those two failures and reports
# a clean corpus as broken -- a false alarm that trains you to ignore the real
# ones. Only `boundary-tier-none` (no boundary section at all) is a guard.
GUARDS = (
    ("1",  "declaration-consistency", "Roster views that disagree with each other",       "High"),
    ("2",  "readme-roster",           "README's prose roster disagreeing with the layers", "High"),
    ("3",  "no-dangling",             "Dangling engines — declared, no document",         "High"),
    ("4",  "stale-absence",           "Documents claiming a built component is absent",   "High"),
    ("5",  "boundary-tier-none",      "Engine docs with no boundary section",             "High"),
    ("6",  "structural-ambiguity",    "Structural ambiguity",                             "Medium"),
    ("7",  "undeclared-reference",    "Boundaries with undeclared engines",               "Medium"),
    ("8",  "boundary-tier-prose",     "Boundary in prose, not machine-readable",          "Medium"),
    ("9",  "never-named",             "Engines no boundary table mentions",               "Medium"),
    ("10", "naming-drift",            "Naming drift",                                     "Low"),
    ("11", "no-orphans",              "Orphan documents",                                  "Low"),
    ("12", "no-layer-violation",      "Layer violations",                                  "Low"),
    ("13", "coverage",                "Coverage and count mismatches",                     "Low"),
)


def normalize(text: str) -> str:
    """Report text with the volatile generation stamp replaced by a token."""
    return GENERATED_LINE_RE.sub("* generated: <stamp>", text)


def archive_stamp(text: str) -> str:
    """
    Filename-safe stamp for a report: YYYY-MM-DD-HHMM.

    Prefers the stamp the report itself carries, so an archived file is named
    for the moment it described. Falls back to the HEAD commit date for
    reports written before stamping existed.
    """
    m = STAMP_RE.search(text)
    if m:
        return f"{m.group(1)}-{m.group(2)}{m.group(3)}"
    try:
        out = subprocess.run(
            ["git", "log", "-1", "--format=%cd", "--date=format:%Y-%m-%d-%H%M"],
            cwd=ROOT, capture_output=True, text=True, timeout=10)
        if out.returncode == 0 and out.stdout.strip():
            return out.stdout.strip()
    except (OSError, subprocess.SubprocessError):
        pass
    return datetime.now().strftime("%Y-%m-%d-%H%M")


# --------------------------------------------------------------------------
# 1. Canonical engine names and where they live
# --------------------------------------------------------------------------
# The authoritative architecture names come from README "Architecture at a
# Glance" and the section-20 flow/ring diagram. Each must resolve to exactly
# one document -- or to None, which is a dangling reference.

ENGINE_HOME: "OrderedDict[str, str | None]" = OrderedDict([
    # --- decision engines ---
    ("Journey",                "Journey Builder Architecture & Employee Ownership Journey.md"),
    ("Destination",            "Destination Engine v1.0.md"),
    ("Business Reality",       "Business Reality Engine v1.0 - Current State Business Assessment.md"),
    ("Document Intelligence",  "Expanded Reality Architecture - Document Intelligence and Fact Verification.md"),
    ("Fact Verification",      "Expanded Reality Architecture - Document Intelligence and Fact Verification.md"),
    ("Research",               "Goal-to-Reality Confidence Research Engine.md"),
    ("Evidence Ledger",        "Goal-to-Reality Confidence Research Engine.md"),
    ("Confidence",             "Goal-to-Reality Confidence Research Engine.md"),
    ("Scenario",               "Scenario Engine.md"),
    ("Financial Modeling",     "Financial Modeling Engine.md"),
    ("Valuation",             "Valuation Engine.md"),
    ("Marketplace",            "Marketplace Engine v1.0.md"),
    # --- transaction engines ---
    ("Capital",                "Capital - Financing Engine v1.0.md"),
    ("Underwriting",           "Underwriting Engine.md"),
    ("Seller Note Liquidity",  "Seller-Note Liquidity Engine v1.0.md"),
    ("Professional Review",    "Professional Review Engine v1.0.md"),
    ("Professional Determination", "Professional Determination Engine v1.0.md"),
    ("Document Readiness",     "Document Readiness & Checklist Engine v1.0.md"),
    ("Review Package",         "Professional Review Package Engine v1.0.md"),
    ("Transaction",            "Transaction - Orchestration Engine.md"),
    ("Stakeholder",            "Stakeholder - Relationship Engine.md"),
    ("Stakeholder Disclosure", "Stakeholder Document & Visibility Architecture.md"),
    ("Workflow",               "Workflow Engine.md"),
    ("Communication",          "Communication Engine.md"),
    ("Closing",                "Closing Engine v1.0.md"),
    ("Ownership Lifecycle",    "Ownership Lifecycle Engine v1.0.md"),
    # --- platform infrastructure ---
    ("Local Vault",            "Local Vault - Workspace Engine.md"),
    ("Identity & Access",      "Identity & Access Engine.md"),
    ("Consent & Access",       "Consent & Access Engine.md"),
    ("Policy",                 "Policy - Compliance Engine.md"),
    ("Audit",                  "Audit - Provenance Engine.md"),
    ("Decision Record",        "Decision Record Engine.md"),
    ("Notification",           "Notification Engine.md"),
    ("Integration",            "Integration Engine.md"),
    ("Vendor Administration",  "Vendor Administration - Vetting Engine v1.0.md"),
    ("Billing",                "Billing - Commercial Engine.md"),
    # "Security" is deliberately absent from this table. It is a cross-cutting
    # property, not an engine -- see CROSS_CUTTING below for the reasoning and
    # for where each security responsibility actually lives.
    # --- publishing surfaces (declared in README, outside the three layers) ---
    ("Blog",                   "Blog - Publishing Engine.md"),
    ("WordPress",              "WordPress Management Engine.md"),
])


# --------------------------------------------------------------------------
# 1b. Cross-cutting properties (deliberately NOT engines)
# --------------------------------------------------------------------------
# The section-20 "around all of it" ring lists nine entries. Eight name an
# engine that has its own document. The ninth, "Security", does not -- and the
# 2026-09-19 review concluded it should not.
#
# The reasoning: every concrete security responsibility already has an owner,
# and Identity & Access section 72 defines a CLOSED three-stage authorization
# chain (Identity & Access -> Policy / Compliance -> Consent & Access) that the
# corpus calls "one of the strongest architectural decisions in the entire
# platform". A Security Engine would be a fourth authority able to answer the
# same question differently, and would own "everything, partially" -- the
# inverse of the locked principle that each engine owns one thing.
#
# Recording it here keeps the decision explicit. Keeping it OUT of ENGINE_HOME
# is the decision, not an oversight, so it correctly produces no dangling
# finding. If a boundary table ever names one of these as an engine, canon()
# returns None and undeclared-reference raises it for a human decision.
#
# name -> (nature, where the responsibility actually lives, what is out of scope)

CROSS_CUTTING: "OrderedDict[str, tuple[str, str, str]]" = OrderedDict([
    ("Security", (
        "a property every engine must have, not a component with a boundary",
        "authentication, MFA, sessions, devices, reauthentication, high-risk "
        "changes, emergency suspension, service and AI identities, API "
        "credentials, separation of duties, tenant isolation -> Identity & "
        "Access; rules, gates, restrictions, credential expiry, fail-safe, "
        "emergency override -> Policy / Compliance; least privilege, purpose "
        "and time limitation, revocation, three-zone model -> Consent & "
        "Access; encryption at rest, integrity checks, privacy modes -> Local "
        "Vault; secrets management -> Integration section 73; forensics -> "
        "Audit / Provenance",
        "operational security -- vulnerability management, penetration "
        "testing, incident response, breach notification, security monitoring "
        "(SIEM), infrastructure hardening, threat modelling, SOC 2 / ISO "
        "27001 -- is deliberately outside the engine model. It is platform "
        "infrastructure, not a decision layer.",
    )),
])

# A note on a homonym, so nobody "fixes" it later: in Capital, Seller-Note
# Liquidity, Professional Review Package, Professional Marketplace, Stakeholder
# and the Brainstorming Brief, the word "security" usually means LOAN SECURITY
# (collateral pledged against a loan) -- not platform security. Capital carries
# both senses: its section 38 is platform security, its loan terms list
# "Security" as collateral. The two must not be conflated when reading a
# boundary row.

# The three layers, as declared in README "Architecture at a Glance".
LAYERS: "OrderedDict[str, list[str]]" = OrderedDict([
    ("Decision", [
        "Journey", "Destination", "Business Reality", "Document Intelligence",
        "Fact Verification", "Research", "Evidence Ledger", "Confidence",
        "Scenario", "Financial Modeling", "Valuation", "Marketplace",
    ]),
    ("Transaction", [
        "Capital", "Underwriting", "Seller Note Liquidity", "Professional Review",
        "Professional Determination", "Document Readiness", "Review Package", "Transaction", "Stakeholder",
        "Stakeholder Disclosure", "Workflow", "Communication", "Closing", "Ownership Lifecycle",
    ]),
    ("Infrastructure", [
        "Local Vault", "Identity & Access", "Consent & Access", "Policy",
        "Audit", "Decision Record", "Notification", "Integration",
        "Vendor Administration", "Billing",
        # "Security" is NOT listed here. It is a cross-cutting property, not a
        # layer member -- see CROSS_CUTTING above. The section-20 ring and the
        # README roster used to name it as infrastructure, which is what made
        # it look like a missing engine; both have been reconciled.
    ]),
])

# Engines that are declared in ENGINE_HOME but deliberately belong to no
# layer: the publishing surfaces sit outside the three-layer model. Naming
# them explicitly is what lets check 0 tell "deliberately layerless" apart
# from "someone added an engine to ENGINE_HOME and forgot to place it",
# which would otherwise let the layer check skip that engine in silence.
LAYERLESS = frozenset({"Blog", "WordPress"})

# Documents that deliberately hold more than one engine. Both carry N self-rows
# in their boundary table, exactly as the recipe prescribes, so the corpus has
# decided this -- but the decision had nowhere to live, and an unexplained
# multi-engine document is indistinguishable from two engines accidentally
# pointed at the same file. That is the identical shape LAYERLESS fixes for
# layers: without an explicit register, a settled question reads as an open one
# forever. Check 0 validates the register is not stale.
#
# Each entry records what would reverse it, so the exemption is a decision with
# a stated expiry condition rather than a permanent pass.
MULTI_ENGINE_DOCS: "OrderedDict[str, str]" = OrderedDict([
    ("Expanded Reality Architecture - Document Intelligence and Fact Verification.md",
     "one pipeline in two halves: Document Intelligence extracts the facts, "
     "Fact Verification checks them against each other and against the stated "
     "reality. Split only if either engine ever gains a home document of its "
     "own."),
    ("Goal-to-Reality Confidence Research Engine.md",
     "one specification in three parts: Research gathers, Evidence Ledger "
     "records provenance (section 12), Confidence scores. Split only if any of "
     "the three ever gains a home document of its own."),
])

# Which DOCUMENT-INDEX groups map onto which architectural layer.
# Groups 1-5 are organized by design phase, and groups 7, 10 and 11 hold
# principle/overview documents rather than engines, so all of those are
# exempt from the layer check (DOCUMENT-INDEX "How this is organized" says
# so explicitly). Groups 6, 8 and 9 are layer-organized and are checked.
GROUP_LAYER = {
    "6": "Decision",
    "8": "Transaction",
    "9": "Infrastructure",
}

# Free-text variants -> canonical key. Everything the corpus actually says.
ALIASES = {
    "destination": "Destination",
    "journey": "Journey",
    "business reality": "Business Reality",
    "document intelligence / extraction": "Document Intelligence",
    "document intelligence": "Document Intelligence",
    "fact verification & conflict": "Fact Verification",
    "fact verification": "Fact Verification",
    "research": "Research",
    "evidence ledger": "Evidence Ledger",
    "evidence": "Evidence Ledger",
    "confidence": "Confidence",
    "scenario": "Scenario",
    "financial modeling": "Financial Modeling",
    "financial modelling": "Financial Modeling",
    "valuation": "Valuation",
    "marketplace": "Marketplace",
    "professional marketplace": "Marketplace",
    "capital": "Capital",
    "capital / financing": "Capital",
    "financing": "Capital",
    "underwriting": "Underwriting",
    "loan underwriting": "Underwriting",
    "underwriting packet": "Underwriting",
    "seller note liquidity": "Seller Note Liquidity",
    "seller-note liquidity": "Seller Note Liquidity",
    "seller note": "Seller Note Liquidity",
    "professional review": "Professional Review",
    "document readiness": "Document Readiness",
    "review package": "Review Package",
    "professional review package": "Review Package",
    "professional determination": "Professional Determination",
    "professional determination engine": "Professional Determination",
    "transaction": "Transaction",
    "transaction / orchestration": "Transaction",
    "orchestration": "Transaction",
    "stakeholder": "Stakeholder",
    "stakeholder / relationship": "Stakeholder",
    "stakeholder / relationship engine": "Stakeholder",
    "stakeholder disclosure": "Stakeholder Disclosure",
    "stakeholder disclosure engine": "Stakeholder Disclosure",
    "workflow": "Workflow",
    "communication": "Communication",
    "closing": "Closing",
    "ownership lifecycle": "Ownership Lifecycle",
    "local vault": "Local Vault",
    "local vault / workspace": "Local Vault",
    "documents / vault": "Local Vault",
    "vault": "Local Vault",
    "identity & access": "Identity & Access",
    "identity": "Identity & Access",
    "identity / permissions": "Identity & Access",
    "identity & permissions": "Identity & Access",
    "consent & access": "Consent & Access",
    "consent": "Consent & Access",
    "consent & permissions": "Consent & Access",
    "policy": "Policy",
    "policy / compliance": "Policy",
    "compliance": "Policy",
    "audit": "Audit",
    "audit / provenance": "Audit",
    "provenance": "Audit",
    "decision record": "Decision Record",
    "notification": "Notification",
    "notifications": "Notification",
    "integration": "Integration",
    "integrations": "Integration",
    "vendor administration": "Vendor Administration",
    "vendor administration / vetting": "Vendor Administration",
    "vendor vetting": "Vendor Administration",
    "vendor": "Vendor Administration",
    "billing": "Billing",
    "billing / commercial": "Billing",
    "commercial": "Billing",
    # No "security" alias. Security is not an engine (see CROSS_CUTTING), so a
    # boundary row naming it must surface as an undeclared reference rather
    # than resolve to a phantom key.
    "blog": "Blog",
    "blog / publishing": "Blog",
    "blog engine": "Blog",
    "publishing": "Blog",
    "wordpress": "WordPress",
    "wordpress management": "WordPress",
    # No "administration" alias either. It previously mapped to Vendor
    # Administration, which is wrong: the section-20 ring's "Administration"
    # means platform and organization administration, owned by Identity &
    # Access section 57. Vendor Administration vets external professionals and
    # vendors, a different job. Mapping it here would have silently attributed
    # platform administration to vendor vetting.
}


def canon(raw: str) -> "str | None":
    """Normalize a free-text engine name to a canonical key, or None."""
    if not raw:
        return None
    s = raw.replace("**", "").replace("*", "").strip()
    s = re.sub(r"\s+", " ", s)
    s = re.sub(r"\bengines?\b", "", s, flags=re.I).strip(" -/\u2014")
    s = re.sub(r"\s+", " ", s).strip()
    low = s.lower()
    if low in ALIASES:
        key = ALIASES[low]
        # Guard: an alias must resolve to a real engine key. Without this, an
        # alias left pointing at a retired key still returns truthy from
        # canon(), so the row lands in `edges` -- where ENGINE_HOME.get()
        # returns None and the row is dropped silently. That is precisely the
        # bug undeclared-reference exists to catch, so fail closed here.
        return key if key in ENGINE_HOME else None
    # Fall back to an exact case-insensitive match on a canonical key. The
    # "Engine"/"Engines" suffix is already gone by this point (stripped above),
    # so bare names like "Valuation" resolve here. Versioned forms such as
    # "Marketplace Engine v1.0" deliberately do not: boundary tables name
    # engines bare, so a versioned reference is a signal worth surfacing
    # rather than smoothing over.
    for key in ENGINE_HOME:
        if low == key.lower():
            return key
    return None


# --------------------------------------------------------------------------
# 2. Corpus loading
# --------------------------------------------------------------------------

def load_docs():
    docs = {}
    for path in sorted(glob.glob(os.path.join(ROOT, "*.md"))):
        name = os.path.basename(path)
        with open(path, encoding="utf-8") as fh:
            docs[name] = fh.read()
    return docs


def parse_index_groups(docs):
    """DOCUMENT-INDEX.md -> {filename: (group_number, group_title)}."""
    text = docs.get(INDEX, "")
    out = {}
    group_no, group_title = None, None
    for line in text.splitlines():
        m = re.match(r"^##\s+(\d+)\.\s+(.*)$", line)
        if m:
            group_no, group_title = m.group(1), m.group(2).strip()
            continue
        if group_no and line.startswith("|"):
            link = re.search(r"\]\(([^)]+\.md)\)", line)
            if link:
                fn = urllib.parse.unquote(link.group(1))
                out[fn] = (group_no, group_title)
    return out


def parse_readme_groups(docs):
    """README group table -> OrderedDict{group_title: expected_count}."""
    text = docs.get(README, "")
    out = OrderedDict()
    for line in text.splitlines():
        m = re.match(r"^\|\s*([^|*][^|]*?)\s*\|\s*(\d+)\s*\|", line)
        if m:
            title, count = m.group(1).strip(), int(m.group(2))
            if title.lower() == "group":
                continue
            out[title] = count
    return out


def parse_boundary_tables(docs):
    """
    Find every | Engine | Owns | Does Not Own | table.

    The header must be exactly those three cells. Matching loosely (any row
    containing both phrases) produces false positives, because DOCUMENT-INDEX
    describes boundary tables in prose inside its own tables.

    Returns (edges, unresolved):
      edges      {(src_doc, canonical_engine): raw_cell}
      unresolved {(src_doc, raw_cell)} for rows naming something that is not a
                 declared engine -- a boundary with an engine that has no home
                 in the architecture, which is a finding rather than noise.
    """
    edges = {}
    unresolved = set()
    for name, text in docs.items():
        lines = text.splitlines()
        for i, line in enumerate(lines):
            if not line.startswith("|"):
                continue
            hdr = [c.strip().lower() for c in line.strip().strip("|").split("|")]
            if hdr != ["engine", "owns", "does not own"]:
                continue
            for row in lines[i + 2:]:
                if not row.startswith("|"):
                    break
                cells = [c.strip() for c in row.strip().strip("|").split("|")]
                if not cells or set(cells[0]) <= set("-: "):
                    continue
                raw = cells[0].replace("**", "").strip()
                eng = canon(cells[0])
                if eng:
                    edges[(name, eng)] = cells[0]
                elif raw:
                    unresolved.add((name, raw))
    return edges, unresolved


BOUNDARY_HEAD_RE = re.compile(
    r"^#{1,4}\s*[0-9.]*\s*(Boundary|Architectural Lock|What This Engine Owns"
    r"|What .*Owns|Explicit Exclusions|Out of Scope)", re.I | re.M)


def boundary_tier(text):
    """
    How well does this document declare its boundary?

      table  -- a machine-readable | Engine | Owns | Does Not Own | table
      prose  -- the boundary is described in headings and prose only
      none   -- no boundary declaration at all
    """
    lines = text.splitlines()
    for line in lines:
        if not line.startswith("|"):
            continue
        hdr = [c.strip().lower() for c in line.strip().strip("|").split("|")]
        if hdr == ["engine", "owns", "does not own"]:
            return "table"
    return "prose" if BOUNDARY_HEAD_RE.search(text) else "none"


def parse_arch_section20(docs):
    """Extract the flow-diagram engine set and the infrastructure ring."""
    text = docs.get(ARCH, "")
    flow, ring = [], []
    m = re.search(r"#\s*20\..*?(?=\n#\s*21\.)", text, re.S)
    if not m:
        return flow, ring
    body = m.group(0)
    fences = re.findall(r"```text\n(.*?)```", body, re.S)
    if fences:
        collapsed = re.sub(r"\s+", " ", fences[0])
        for key in ENGINE_HOME:
            probe = key.replace("&", r"&")
            if re.search(re.escape(probe) + r"( Engine)?\b", collapsed):
                flow.append(key)
        if "Financial Modeling" in collapsed:
            if "Financial Modeling" not in flow:
                flow.append("Financial Modeling")
        if "Professional Determination" in collapsed:
            flow.append("__ProfessionalDetermination__")
    if len(fences) > 1:
        for line in fences[1].splitlines():
            line = line.strip()
            if line:
                ring.append(line)
    return flow, ring


def parse_links(docs):
    """Explicit markdown links between docs -> set of (src, dst)."""
    names = set(docs)
    out = set()
    for name, text in docs.items():
        for m in re.finditer(r"\]\(([^)]+)\)", text):
            tgt = urllib.parse.unquote(m.group(1).split("#")[0].strip())
            base = os.path.basename(tgt)
            if base.endswith(".md") and base in names and base != name:
                out.add((name, base))
    return out


# --------------------------------------------------------------------------
# 2b. Staleness: a document asserting a component is absent when it exists
# --------------------------------------------------------------------------
# The corpus grew by appending documents, and several early documents were
# written as proposals -- "here is what we do not have yet". Nothing revisited
# them once the components were built, so their headings and claims still read
# as present tense. That is a real defect: it made the author believe eighteen
# built engines were missing.
#
# Both patterns are deliberately narrow. The corpus legitimately says "missing
# information", "missing documents", "proposed transaction" and "NOT YET
# REVIEWED" -- domain vocabulary about a transaction in progress, not claims
# about the architecture. So a hit only counts when it names a component that
# actually has a backing document.

ABSENCE_HEADING_RE = re.compile(
    r"^#{1,4}\s*[0-9.]*\s*(?:Missing|Not Yet Built|To Build|Proposed|Needed)"
    r"\s*[:\u2014-]?\s*(.+?)\s*$", re.I | re.M)

ABSENCE_CLAIM_RE = re.compile(
    r"(?:does not|doesn't|do not|don't)\s+(?:yet\s+)?exist"
    r"|has not been (?:built|specified|defined|written|created)"
    r"|not yet (?:built|specified|defined|implemented|created)"
    r"|needs? to be (?:built|specified|defined|written|created)"
    r"|still missing",
    re.I)


def check_stale_absence(docs):
    """
    A document claiming a component is absent when that component exists.

    Catches two shapes:

      * an absence heading naming a built component
        ("# 3. Missing: Financial Modeling Engine")
      * an absence claim within a sentence of a built component's name

    Domain vocabulary is never a finding, because a hit only counts when it
    resolves to a component with a backing document.

    Self-references are skipped: a document necessarily names its own engine
    ("The Workflow Engine should know: ... a task needs to be created"), and
    that is domain language, not a claim about the architecture. The defect
    this looks for is a document talking about SOMEONE ELSE's component.
    """
    findings = []
    for name, text in sorted(docs.items()):
        if name in (README, INDEX):
            continue
        for m in ABSENCE_HEADING_RE.finditer(text):
            target = m.group(1).strip()
            eng = canon(target)
            home = ENGINE_HOME.get(eng) if eng else None
            if home and home != name:
                findings.append(
                    f"{name}: heading \u201c{m.group(0).strip()}\u201d presents "
                    f"{eng} as absent, but it has a document ({home})")
        for m in ABSENCE_CLAIM_RE.finditer(text):
            window = text[max(0, m.start() - 160):m.end() + 160]
            for eng, home in ENGINE_HOME.items():
                if not home or home == name:
                    continue
                if re.search(r"\b" + re.escape(eng) + r"\b", window, re.I):
                    findings.append(
                        f"{name}: \u201c{m.group(0).strip()}\u201d appears beside "
                        f"{eng}, which has a document ({home})")
                    break
    return findings


# --------------------------------------------------------------------------
# 2c. README's three-layer roster: the last hand-maintained view
# --------------------------------------------------------------------------
# README "Architecture at a Glance" declares the architecture a second time,
# in prose, as three dot-separated rosters. Nothing parsed it, so it could
# drift freely -- and it did: "Security" sat in the infrastructure roster
# while CROSS_CUTTING excluded it from the engine set, which is what made
# Security read as a missing engine in the first place. The group-count table
# beside it was checked (coverage); the roster was not.
#
# This is the same class of defect as declaration-consistency: a hand-kept
# view of the roster that no check reads. Verify it against LAYERS (the
# three-layer declaration) and CROSS_CUTTING (the deliberate non-engines).

ROSTER_LINE_RE = re.compile(
    r"^\*\*(Decision engines|Transaction engines|Platform infrastructure)"
    r"\*\*\s*\u2014\s*(.+?)\s*$", re.M)

ROSTER_LAYER = {
    "Decision engines": "Decision",
    "Transaction engines": "Transaction",
    "Platform infrastructure": "Infrastructure",
}

CROSS_ROSTER_RE = re.compile(
    r"^\*\*Cross-cutting, not an engine\*\*\s*\u2014\s*([^.,;]+)", re.M)


def check_readme_roster(docs):
    """README's prose roster must agree with LAYERS and CROSS_CUTTING."""
    findings = []
    text = docs.get(README, "")
    seen = defaultdict(set)
    for m in ROSTER_LINE_RE.finditer(text):
        layer = ROSTER_LAYER[m.group(1)]
        for raw in m.group(2).split("\u00b7"):
            raw = raw.strip()
            if not raw:
                continue
            key = canon(raw)
            if not key:
                findings.append(
                    f"README roster ({layer}) names \u201c{raw}\u201d, which is "
                    f"not a declared engine")
            else:
                seen[layer].add(key)
    for layer, keys in LAYERS.items():
        missing = sorted(set(keys) - seen.get(layer, set()))
        if missing:
            findings.append(
                f"README {layer} roster omits declared engines: "
                f"{', '.join(missing)}")
        extra = sorted(seen.get(layer, set()) - set(keys))
        if extra:
            findings.append(
                f"README {layer} roster lists engines not declared in that "
                f"layer: {', '.join(extra)}")
    for m in CROSS_ROSTER_RE.finditer(text):
        name = m.group(1).strip()
        if name not in CROSS_CUTTING:
            findings.append(
                f"README cross-cutting line names \u201c{name}\u201d, which is "
                f"not recorded in CROSS_CUTTING")
    if CROSS_CUTTING and not CROSS_ROSTER_RE.search(text):
        findings.append(
            "README does not name the cross-cutting properties: "
            + ", ".join(CROSS_CUTTING))
    return findings


# --------------------------------------------------------------------------
# 3. Checks
# --------------------------------------------------------------------------

def run_checks(docs):
    findings = defaultdict(list)
    index_groups = parse_index_groups(docs)
    readme_groups = parse_readme_groups(docs)
    boundary, unresolved = parse_boundary_tables(docs)
    flow, ring = parse_arch_section20(docs)
    links = parse_links(docs)

    all_docs = set(docs)
    content_docs = all_docs - {README, INDEX}

    # ---- check 1: no-dangling ------------------------------------------
    dangling = []
    for key, home in ENGINE_HOME.items():
        if home is None:
            dangling.append(key)
        elif home not in all_docs:
            dangling.append(f"{key} -> {home} (file missing)")
    findings["no-dangling"] = dangling

    # ---- check 2: no-orphans -------------------------------------------
    # A boundary table's own row describes what THIS engine owns. It is a
    # self-loop, not a relationship, so it must not count as an inbound edge.
    # Twelve engine docs open their boundary table with such a row; counting
    # them would let a document satisfy its own no-orphans and never-named
    # checks, which is the same self-reference bug fixed in stale-absence.
    inbound_boundary = defaultdict(set)
    outbound_boundary = defaultdict(set)
    for (src, eng) in boundary:
        home = ENGINE_HOME.get(eng)
        if home and home in all_docs:
            outbound_boundary[src].add(home)
            if home != src:
                inbound_boundary[home].add(src)

    inbound_links = defaultdict(set)
    for (src, dst) in links:
        inbound_links[dst].add(src)

    orphans = []
    for d in sorted(content_docs):
        if not inbound_boundary[d] and not inbound_links[d]:
            orphans.append(d)
    findings["no-orphans"] = orphans

    # boundary-lock coverage, in three tiers
    # Skip a home that is not on disk. no-dangling has already reported it as a
    # High-severity finding (check 1, above); without this guard `docs[d]`
    # raises KeyError and the traceback discards that finding -- so the one
    # input this check exists to catch was the one input that crashed the run.
    engine_docs = {v for v in ENGINE_HOME.values() if v and v in docs}
    tiers = defaultdict(list)
    for d in sorted(engine_docs):
        tiers[boundary_tier(docs[d])].append(d)
    findings["boundary-tier-table"] = tiers["table"]
    findings["boundary-tier-prose"] = tiers["prose"]
    findings["boundary-tier-none"] = tiers["none"]

    # engines that no boundary table anywhere names
    never_named = []
    for key, home in ENGINE_HOME.items():
        if home is None:
            continue
        if not inbound_boundary.get(home):
            never_named.append(f"{key} ({home})")
    findings["never-named"] = sorted(never_named)

    # structural ambiguity: several architecture names sharing one document,
    # and names whose home document is a questionable match
    home_to_keys = defaultdict(list)
    for k, v in ENGINE_HOME.items():
        if v:
            home_to_keys[v].append(k)
    # Only UNREGISTERED multi-engine documents are ambiguous. A registered one
    # is a decision the corpus already made and recorded in MULTI_ENGINE_DOCS,
    # not a question still waiting for an answer -- reporting it forever would
    # make this check a standing complaint rather than a detector.
    collapsed = [
        f"{', '.join(sorted(ks))} -> one document ({h})"
        for h, ks in sorted(home_to_keys.items())
        if len(ks) > 1 and h not in MULTI_ENGINE_DOCS
    ]
    if "__ProfessionalDetermination__" in flow and "Professional Determination" not in ENGINE_HOME:
        collapsed.append(
            "Professional Determination is a node in the section-20 flow "
            "diagram, between Professional Review and Document Readiness, but "
            "it is not listed as an engine in any layer roster")
    findings["structural-ambiguity"] = collapsed
    # Supersession questions (which journey document is current; whether the
    # Stakeholder relationship layer gets its own document) used to be hardcoded
    # here. No document edit could ever clear them, so they were a to-do list
    # living inside a detector. They are decisions for a human, and now live in
    # analysis/REMAINING-WORK.md under Track C3.
    findings["accepted-multi-engine"] = [
        f"{h} — {', '.join(sorted(ks))}: {why}"
        for h, ks in sorted(home_to_keys.items())
        if len(ks) > 1 and h in MULTI_ENGINE_DOCS
        for why in [MULTI_ENGINE_DOCS[h]]
    ]

    # a boundary table naming something that is not a declared engine: the
    # document claims a relationship with an engine that has no home in the
    # architecture. Silent until now, because unresolvable rows were dropped.
    findings["undeclared-reference"] = sorted(
        f"{src} declares a boundary with “{raw}”, which is not a declared engine"
        for src, raw in unresolved)

    # a document still telling the reader that a built component is absent
    findings["stale-absence"] = check_stale_absence(docs)

    # ---- check 0: declaration-consistency -------------------------------
    # ENGINE_HOME, LAYERS and LAYERLESS are three hand-maintained views of the
    # same roster. If they disagree, the layer check below fails OPEN: an
    # engine with no layer entry yields declared=None and is skipped, so a
    # forgotten placement looks exactly like a passing check. Assert the
    # invariant here so the roster cannot drift quietly.
    decl = []
    placed = defaultdict(list)
    for layer, keys in LAYERS.items():
        for k in keys:
            placed[k].append(layer)
    for k in sorted(placed):
        if k not in ENGINE_HOME:
            decl.append(f"LAYERS declares “{k}” in {placed[k][0]}, but "
                        f"ENGINE_HOME has no such engine")
        elif len(placed[k]) > 1:
            decl.append(f"“{k}” is declared in {len(placed[k])} layers: "
                        f"{', '.join(placed[k])}")
    for k in ENGINE_HOME:
        if not placed.get(k) and k not in LAYERLESS:
            decl.append(f"“{k}” is in ENGINE_HOME but in no layer, and is not "
                        f"listed in LAYERLESS -- either place it or exempt it")
    for k in sorted(LAYERLESS):
        if k not in ENGINE_HOME:
            decl.append(f"LAYERLESS names “{k}”, which is not an engine")
        elif placed.get(k):
            decl.append(f"“{k}” is in LAYERLESS but also declared in "
                        f"{placed[k][0]}")
    # A register entry that no longer describes reality would suppress a real
    # finding, so validate it the same way: an exemption set nobody checks is
    # just a way to hide things.
    for doc in sorted(MULTI_ENGINE_DOCS):
        keys = sorted(k for k, v in ENGINE_HOME.items() if v == doc)
        if not keys:
            decl.append(f"MULTI_ENGINE_DOCS names “{doc}”, which is no "
                        f"engine's home document")
        elif len(keys) < 2:
            decl.append(f"MULTI_ENGINE_DOCS names “{doc}”, but only one engine "
                        f"({keys[0]}) has it as home — no longer multi-engine")
    findings["declaration-consistency"] = decl

    # README's prose roster is the fourth view of the same roster, and the
    # last one nothing read.
    findings["readme-roster"] = check_readme_roster(docs)

    # ---- check 3: no-layer-violation -----------------------------------
    violations = []
    for doc, (gno, gtitle) in index_groups.items():
        expected_layer = GROUP_LAYER.get(gno)
        if not expected_layer:
            continue
        actual = None
        for key, home in ENGINE_HOME.items():
            if home == doc:
                actual = key
                break
        if actual is None:
            continue
        declared = None
        for layer, keys in LAYERS.items():
            if actual in keys:
                declared = layer
        if declared and declared != expected_layer:
            violations.append(
                f"{doc}: group {gno} ({gtitle}) implies {expected_layer}, "
                f"but the engine is declared {declared}")
    findings["no-layer-violation"] = violations

    # ---- check 4: coverage ---------------------------------------------
    cov = []
    idx_total = len(index_groups)
    readme_total = sum(readme_groups.values())
    if idx_total != len(content_docs):
        cov.append(f"DOCUMENT-INDEX lists {idx_total} docs; repo has "
                   f"{len(content_docs)} content docs")
    if readme_total != idx_total:
        cov.append(f"README group table totals {readme_total}; "
                   f"DOCUMENT-INDEX lists {idx_total}")
    unlisted = sorted(content_docs - set(index_groups))
    if unlisted:
        cov.append("Not in DOCUMENT-INDEX: " + ", ".join(unlisted))
    ghost = sorted(set(index_groups) - all_docs)
    if ghost:
        cov.append("In DOCUMENT-INDEX but missing on disk: " + ", ".join(ghost))

    # per-group counts vs README
    per_group = defaultdict(int)
    for _doc, (gno, _t) in index_groups.items():
        per_group[gno] += 1
    idx_titles = OrderedDict()
    for _doc, (gno, t) in index_groups.items():
        idx_titles[gno] = t
    for gno in sorted(idx_titles, key=int):
        title = idx_titles[gno]
        actual_n = per_group[gno]
        expected_n = readme_groups.get(title)
        if expected_n is not None and expected_n != actual_n:
            cov.append(f"Group {gno} ({title}): README says {expected_n}, "
                       f"index lists {actual_n}")
    findings["coverage"] = cov

    # ---- check 5: naming-drift -----------------------------------------
    variants = defaultdict(set)
    for (_src, eng), raw in boundary.items():
        variants[eng].add(raw.strip())
    drift = []
    for eng, raws in sorted(variants.items()):
        if len(raws) > 1:
            drift.append(f"{eng}: " + " | ".join(sorted(raws)))
    # architecture vs boundary naming
    arch_names = {k for k in ENGINE_HOME}
    findings["naming-drift"] = drift

    stats = {
        "docs_total": len(all_docs),
        "docs_content": len(content_docs),
        "index_groups": idx_total,
        "readme_total": readme_total,
        "boundary_tables": len({s for (s, _e) in boundary}),
        "boundary_edges": len(boundary),
        "link_edges": len(links),
        "engine_keys": len(ENGINE_HOME),
        "engines_with_home": sum(1 for v in ENGINE_HOME.values() if v),
        "flow_nodes": [f for f in flow],
        "ring": ring,
        "group_counts": dict(per_group),
        "group_titles": dict(idx_titles),
        "readme_groups": dict(readme_groups),
    }
    return findings, stats, boundary, flow, ring, inbound_boundary


# --------------------------------------------------------------------------
# 4. Rendering
# --------------------------------------------------------------------------

def mermaid_topology():
    """The declared end-to-end pipeline from Complete Architecture section 20."""
    return """```mermaid
flowchart TD
    OWNER(["Owner experience"])
    JOURNEY["Journey Engine"]
    DEST["Destination Engine"]
    REALITY["Business Reality Engine"]
    RESEARCH["Research Engine"]
    LEDGER["Evidence Ledger"]
    CONF["Confidence Engine"]
    SCEN{{"Scenario Engine<br/>NO DOCUMENT"}}
    FIN{{"Financial Modeling<br/>NO DOCUMENT"}}
    CAP["Capital Engine"]
    MKT["Marketplace Engine"]
    PREV["Professional Review Engine"]
    DET{{"Professional Determination<br/>not an engine?"}}
    DR["Document Readiness Engine"]
    PKG["Review Package Engine"]
    TXN["Transaction Engine"]
    WF["Workflow Engine"]
    COMM["Communication Engine"]
    VAULT["Documents / Vault"]
    CLOSE["Closing"]
    LIFE["Ownership Lifecycle Engine"]

    OWNER --> JOURNEY --> DEST
    DEST --> REALITY
    DEST --> RESEARCH --> LEDGER
    REALITY --> CONF
    LEDGER --> CONF
    CONF --> SCEN
    SCEN --> FIN
    SCEN --> CAP
    SCEN --> MKT
    FIN --> PREV
    CAP --> PREV
    MKT --> PREV
    PREV --> DET
    DET --> DR --> PKG --> TXN
    TXN --> WF
    TXN --> COMM
    TXN --> VAULT
    WF --> CLOSE
    COMM --> CLOSE
    VAULT --> CLOSE
    CLOSE --> LIFE

    classDef missing fill:#FCEBEB,stroke:#E24B4A,stroke-width:1px,color:#791F1F
    classDef ambiguous fill:#FAEEDA,stroke:#BA7517,stroke-width:1px,color:#633806
    class SCEN,FIN missing
    class DET ambiguous
```

Infrastructure ring (depends on by every engine above, declared separately in
section 20):

```mermaid
flowchart LR
    subgraph RING["Around all of it"]
        direction TB
        IA["Identity & Access"]
        CA["Consent & Permissions"]
        POL["Policy / Compliance"]
        AUD["Audit / Provenance"]
        NOT["Notifications"]
        INT["Integrations"]
        ADM["Administration"]
        BIL["Billing"]
        SEC{{"Security<br/>NO DOCUMENT"}}
    end
    classDef missing fill:#FCEBEB,stroke:#E24B4A,stroke-width:1px,color:#791F1F
    class SEC missing
```
"""


def mermaid_boundaries(boundary):
    """Every declared boundary relationship, as a graph."""
    lines = ["```mermaid", "flowchart LR"]
    ids = {}
    for i, key in enumerate(ENGINE_HOME):
        ids[key] = f"E{i}"
    for key, nid in ids.items():
        home = ENGINE_HOME[key]
        if home is None:
            lines.append(f'    {nid}{{"{key}<br/>NO DOCUMENT"}}')
        else:
            lines.append(f'    {nid}["{key}"]')
    seen = set()
    for (src, eng) in sorted(boundary):
        for key, home in ENGINE_HOME.items():
            if home == src:
                a = ids[key]
                break
        else:
            continue
        b = ids.get(eng)
        if not b or a == b:
            continue
        pair = tuple(sorted((a, b)))
        if pair in seen:
            continue
        seen.add(pair)
        lines.append(f"    {a} --- {b}")
    lines.append("    classDef missing fill:#FCEBEB,stroke:#E24B4A,"
                 "stroke-width:1px,color:#791F1F")
    missing = ",".join(ids[k] for k, v in ENGINE_HOME.items() if v is None)
    if missing:
        lines.append(f"    class {missing} missing")
    lines.append("```")
    return "\n".join(lines)


def render_report(findings, stats, boundary, flow, ring):
    L = []
    A = L.append
    A("# End-to-end gap analysis")
    A("")
    A(f"* generated: {datetime.now().astimezone().strftime('%Y-%m-%d %H:%M:%S %z')}")
    A("")
    A("Generated by `scripts/doc-graph.py` — a dependency graph for the design")
    A("bible. Every `.md` file is a node; the edges come from the corpus's real")
    A("edge syntax (boundary tables, the section-20 flow diagram, the")
    A("infrastructure ring, and explicit links).")
    A("")
    A("Superseded versions are kept in `analysis/archive/`. A `pre-commit`")
    A("hook regenerates this report, so it always describes the tree you are")
    A("committing.")
    A("")
    A("## Corpus at a glance")
    A("")
    A(f"* documents on disk: **{stats['docs_total']}** "
      f"({stats['docs_content']} content + README + index)")
    A(f"* DOCUMENT-INDEX entries: **{stats['index_groups']}**")
    A(f"* README group-table total: **{stats['readme_total']}**")
    A(f"* engine keys in the declared architecture: **{stats['engine_keys']}**")
    A(f"* engine keys with a backing document: **{stats['engines_with_home']}**")
    A(f"* cross-cutting properties, not engines: **{len(CROSS_CUTTING)}**"
      + (f" — {', '.join(CROSS_CUTTING)}" if CROSS_CUTTING else ""))
    A(f"* docs carrying a boundary table: **{stats['boundary_tables']}**")
    A(f"* boundary edges extracted: **{stats['boundary_edges']}**")
    A(f"* explicit markdown links between docs: **{stats['link_edges']}**")
    A("")
    A("---")
    A("")

    A("## Findings")
    A("")
    A("| # | Check | Count | Severity |")
    A("| --- | --- | --- | --- |")
    for num, key, label, sev in GUARDS:
        n = len(findings.get(key, []))
        A(f"| {num} | {label} | {n} | {sev} |")
    A("")

    def section(num, title, key, empty_msg):
        A(f"## {num}. {title}")
        A("")
        items = findings.get(key, [])
        if not items:
            A(empty_msg)
        else:
            for it in items:
                A(f"* {it}")
        A("")

    section("1", "Roster views that disagree with each other",
            "declaration-consistency",
            "None — ENGINE_HOME, LAYERS and LAYERLESS agree.")

    section("2", "README's prose roster disagreeing with the declared layers",
            "readme-roster",
            "None — the README roster matches LAYERS and CROSS_CUTTING.")

    section("3", "Dangling engines — declared in the architecture, no document",
            "no-dangling",
            "None — every declared engine resolves to a document.")

    section("4", "Documents claiming a built component is absent",
            "stale-absence",
            "None — no document still presents a built component as missing.")

    section("5", "Engine docs with no boundary section at all",
            "boundary-tier-none",
            "None.")

    section("6", "Structural ambiguity",
            "structural-ambiguity",
            "None.")

    accepted = findings.get("accepted-multi-engine", [])
    if accepted:
        A("Multi-engine documents on record — decided, so not counted above:")
        A("")
        for it in accepted:
            A(f"* {it}")
        A("")

    section("7", "Boundaries with engines that are not declared anywhere",
            "undeclared-reference",
            "None — every boundary names a declared engine.")

    section("8", "Boundary declared in prose only (not machine-readable)",
            "boundary-tier-prose",
            "None — every boundary is expressed as a table.")

    section("9", "Engines that no boundary table anywhere mentions",
            "never-named",
            "None — every engine is named by at least one boundary table.")

    section("10", "Naming drift — one engine, several names",
            "naming-drift",
            "None — engine names are used consistently.")

    section("11", "Orphan documents", "no-orphans",
            "None — every document is referenced by at least one other.")

    section("12", "Layer violations", "no-layer-violation",
            "None — every layer-organized group agrees with its declared layer.")

    section("13", "Coverage and count mismatches", "coverage",
            "None — roster, README table, and disk all agree.")

    A("## 14. Cross-cutting properties — deliberately not engines")
    A("")
    A("These names appear in the architecture but are properties every engine")
    A("must have, rather than components with their own boundary. They are")
    A("excluded from check 3 on purpose: the finding \"this has no document\"")
    A("was correct, and the answer was \"it should not have one\".")
    A("")
    if not CROSS_CUTTING:
        A("None.")
    else:
        for name, (nature, lives, out_of_scope) in CROSS_CUTTING.items():
            A(f"* **{name}** — {nature}.")
            A(f"  * Where the responsibility actually lives: {lives}.")
            A(f"  * Out of scope: {out_of_scope}")
    A("")

    A("## 15. Declared pipeline topology")
    A("")
    A(mermaid_topology())
    A("")
    A("## 16. Declared boundary graph")
    A("")
    A(mermaid_boundaries(boundary))
    A("")
    return "\n".join(L)


def publish_report(report):
    """
    Write the report, archiving the outgoing version when the content changed.

    Returns the paths that were created or modified, so the caller knows
    exactly what to stage. Returns an empty list when nothing changed -- in
    that case the working tree is left completely alone, which is what makes
    this safe to run on every commit.
    """
    path = os.path.join(OUTDIR, REPORT)
    archive_dir = os.path.join(OUTDIR, ARCHIVE_DIR)
    changed = []

    if os.path.exists(path):
        with open(path, encoding="utf-8") as fh:
            old = fh.read()
        if normalize(old) == normalize(report):
            return changed
        os.makedirs(archive_dir, exist_ok=True)
        stem = archive_stamp(old)
        dest = os.path.join(archive_dir, f"GAP-ANALYSIS-{stem}.md")
        n = 2
        while os.path.exists(dest):
            dest = os.path.join(archive_dir, f"GAP-ANALYSIS-{stem}-{n}.md")
            n += 1
        os.replace(path, dest)
        changed.append(dest)

    with open(path, "w", encoding="utf-8") as fh:
        fh.write(report)
    changed.append(path)
    return changed


def main():
    docs = load_docs()
    findings, stats, boundary, flow, ring, _inbound = run_checks(docs)
    report = render_report(findings, stats, boundary, flow, ring)

    if "--stdout" in sys.argv:
        print(report)
        return

    if "--check-report" in sys.argv:
        # Staleness gate for the *committed* report -- the counterpart to
        # --check, and a different question. --check asks "is the corpus clean?"
        # This asks "does the versioned report still describe it?" A clean corpus
        # with a stale report is still a false statement sitting on disk, so both
        # gates have to pass for the tree to be internally consistent.
        #
        # This is the half the pre-commit hook structurally cannot provide. The
        # hook REGENERATES the report, so it can never have cause to fail: it
        # silently repairs drift rather than reporting it. That is the right
        # behaviour at commit time and useless in CI, where the question is
        # whether the state you pushed was self-consistent. Nothing else in the
        # repo turns report drift into a non-zero exit.
        #
        # Ported from the Jev stack, which already had both flags; main had only
        # --check. Comparison is normalised, so a difference of clock alone is
        # not drift (see normalize()).
        report_path = os.path.join(OUTDIR, REPORT)
        rel = os.path.relpath(report_path, ROOT)
        try:
            with open(report_path, encoding="utf-8") as fh:
                committed = fh.read()
        except OSError as exc:
            print(f"versioned report is unavailable: {exc}", file=sys.stderr)
            return 1
        if normalize(committed) != normalize(report):
            print(
                f"{rel} is stale; run python3 scripts/doc-graph.py and commit "
                f"the result",
                file=sys.stderr,
            )
            return 1
        print(f"{rel} is current")
        return 0

    if "--check" in sys.argv:
        # Read-only pre-flight: run the checks, report, publish NOTHING. The exit
        # status is the point. The default mode's job is to *publish* the report
        # -- it rewrites analysis/GAP-ANALYSIS.md and archives the outgoing copy
        # -- so it is the wrong thing to reach for when all you want to know is
        # whether the corpus is clean. Verifying by publishing also destroys the
        # evidence you were verifying against, since the previous report is what
        # you would diff against.
        #
        # Exit 0 = every guard zero. Exit 1 = at least one guard reported
        # findings. This is the primitive a CI job or a pre-commit gate needs;
        # refresh-analysis.sh refreshes the report but deliberately never gates
        # on it, so nothing else in the repo turns a finding into a failure.
        print(f"documents:      {stats['docs_total']}")
        print(f"boundary edges: {stats['boundary_edges']}")
        print(f"link edges:     {stats['link_edges']}")
        print()
        failed = 0
        for _num, key, label, _sev in GUARDS:
            n = len(findings.get(key, []))
            failed += 1 if n else 0
            print(f"{'FAIL' if n else 'ok  '}  {key:22s} {n:4d}  {label}")
        print()
        if failed:
            print(f"{failed} of {len(GUARDS)} checks reported findings.")
            return 1
        print(f"all {len(GUARDS)} checks clean.")
        return 0

    os.makedirs(OUTDIR, exist_ok=True)
    changed = publish_report(report)
    with open(os.path.join(OUTDIR, "engine-graph.mmd"), "w", encoding="utf-8") as fh:
        fh.write(mermaid_topology() + "\n" + mermaid_boundaries(boundary))
    with open(os.path.join(OUTDIR, "graph.json"), "w", encoding="utf-8") as fh:
        json.dump({
            "stats": stats,
            "findings": {k: list(v) for k, v in findings.items()},
            "boundary_edges": sorted(
                [{"from": s, "to": e} for (s, e) in boundary],
                key=lambda x: (x["from"], x["to"])),
        }, fh, indent=2)

    # Machine-readable handoff for the pre-commit hook: the exact paths that
    # need staging. Empty output means the report did not change.
    if "--staged-files" in sys.argv:
        for p in changed:
            print(os.path.relpath(p, ROOT))
        return

    print(f"documents:      {stats['docs_total']}")
    print(f"boundary edges: {stats['boundary_edges']}")
    print(f"link edges:     {stats['link_edges']}")
    for _num, key, _label, _sev in GUARDS:
        print(f"{key:22s} {len(findings.get(key, []))}")
    if changed:
        for p in changed:
            print(f"updated {os.path.relpath(p, ROOT)}")
    else:
        print("\nreport unchanged")


if __name__ == "__main__":
    sys.exit(main())
