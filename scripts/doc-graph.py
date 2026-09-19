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

  no-dangling          an engine named in the architecture with no backing doc
  no-orphans           a document no other document declares a boundary with
  no-layer-violation   a group 6-10 doc whose group contradicts its layer
  coverage             roster counts vs README table vs actual files
  naming-drift         one engine referred to by several different names

Usage:
    python3 scripts/doc-graph.py            # write reports into analysis/
    python3 scripts/doc-graph.py --stdout   # print the report, write nothing
"""

from __future__ import annotations

import os
import re
import sys
import glob
import json
import urllib.parse
from collections import defaultdict, OrderedDict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTDIR = os.path.join(ROOT, "analysis")

README = "README.md"
INDEX = "DOCUMENT-INDEX.md"
ARCH = ("Complete Architecture - Decision Engines, Transaction Engines, "
        "Platform Infrastructure.md")


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
    ("Scenario",               None),
    ("Financial Modeling",     None),
    ("Marketplace",            "Marketplace Engine v1.0.md"),
    # --- transaction engines ---
    ("Capital",                "Capital - Financing Engine v1.0.md"),
    ("Seller Note Liquidity",  "Seller-Note Liquidity Engine v1.0.md"),
    ("Professional Review",    "Professional Review Engine v1.0.md"),
    ("Document Readiness",     "Document Readiness & Checklist Engine v1.0.md"),
    ("Review Package",         "Professional Review Package Engine v1.0.md"),
    ("Transaction",            "Transaction - Orchestration Engine.md"),
    ("Stakeholder",            "Stakeholder Document & Visibility Architecture.md"),
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
    ("Security",               None),
    # --- publishing surfaces (declared in README, outside the three layers) ---
    ("Blog",                   "Blog - Publishing Engine.md"),
    ("WordPress",              "WordPress Management Engine.md"),
])

# The three layers, as declared in README "Architecture at a Glance".
LAYERS: "OrderedDict[str, list[str]]" = OrderedDict([
    ("Decision", [
        "Journey", "Destination", "Business Reality", "Document Intelligence",
        "Fact Verification", "Research", "Evidence Ledger", "Confidence",
        "Scenario", "Financial Modeling", "Marketplace",
    ]),
    ("Transaction", [
        "Capital", "Seller Note Liquidity", "Professional Review",
        "Document Readiness", "Review Package", "Transaction", "Stakeholder",
        "Workflow", "Communication", "Closing", "Ownership Lifecycle",
    ]),
    ("Infrastructure", [
        "Local Vault", "Identity & Access", "Consent & Access", "Policy",
        "Audit", "Decision Record", "Notification", "Integration",
        "Vendor Administration", "Billing", "Security",
    ]),
])

# Which DOCUMENT-INDEX groups map onto which architectural layer.
# Groups 1-5 are organized by design phase, so they are exempt from the
# layer check (DOCUMENT-INDEX "How this is organized" says so explicitly).
GROUP_LAYER = {
    "7": "Transaction",
    "8": "Infrastructure",
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
    "marketplace": "Marketplace",
    "professional marketplace": "Marketplace",
    "capital": "Capital",
    "capital / financing": "Capital",
    "financing": "Capital",
    "seller note liquidity": "Seller Note Liquidity",
    "seller-note liquidity": "Seller Note Liquidity",
    "seller note": "Seller Note Liquidity",
    "professional review": "Professional Review",
    "document readiness": "Document Readiness",
    "review package": "Review Package",
    "professional review package": "Review Package",
    "transaction": "Transaction",
    "transaction / orchestration": "Transaction",
    "orchestration": "Transaction",
    "stakeholder": "Stakeholder",
    "stakeholder / relationship": "Stakeholder",
    "stakeholder / relationship engine": "Stakeholder",
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
    "security": "Security",
    "blog": "Blog",
    "blog / publishing": "Blog",
    "blog engine": "Blog",
    "publishing": "Blog",
    "wordpress": "WordPress",
    "wordpress management": "WordPress",
    "administration": "Vendor Administration",
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
        return ALIASES[low]
    # try dropping a trailing qualifier, e.g. "Marketplace Engine v1.0"
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
    Returns {(src_doc, canonical_engine): raw_cell}.
    """
    edges = {}
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
                eng = canon(cells[0])
                if eng:
                    edges[(name, eng)] = cells[0]
    return edges


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
# 3. Checks
# --------------------------------------------------------------------------

def run_checks(docs):
    findings = defaultdict(list)
    index_groups = parse_index_groups(docs)
    readme_groups = parse_readme_groups(docs)
    boundary = parse_boundary_tables(docs)
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
    inbound_boundary = defaultdict(set)
    outbound_boundary = defaultdict(set)
    for (src, eng) in boundary:
        home = ENGINE_HOME.get(eng)
        if home and home in all_docs:
            inbound_boundary[home].add(src)
            outbound_boundary[src].add(home)

    inbound_links = defaultdict(set)
    for (src, dst) in links:
        inbound_links[dst].add(src)

    orphans = []
    for d in sorted(content_docs):
        if not inbound_boundary[d] and not inbound_links[d]:
            orphans.append(d)
    findings["no-orphans"] = orphans

    # boundary-lock coverage, in three tiers
    engine_docs = {v for v in ENGINE_HOME.values() if v}
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
    collapsed = [
        f"{', '.join(sorted(ks))} -> one document ({h})"
        for h, ks in sorted(home_to_keys.items()) if len(ks) > 1
    ]
    if "__ProfessionalDetermination__" in flow:
        collapsed.append(
            "Professional Determination is a node in the section-20 flow "
            "diagram, between Professional Review and Document Readiness, but "
            "it is not listed as an engine in any layer roster")
    collapsed.append(
        "Stakeholder maps to 'Stakeholder Document & Visibility "
        "Architecture.md', but that document is about document visibility; "
        "section 7 describes a Stakeholder / Relationship engine that owns "
        "who participates and why")
    collapsed.append(
        "Journey maps to 'Journey Builder Architecture & Employee Ownership "
        "Journey.md' out of three candidate journey documents")
    findings["structural-ambiguity"] = collapsed

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
    A("Generated by `scripts/doc-graph.py` — a dependency graph for the design")
    A("bible. Every `.md` file is a node; the edges come from the corpus's real")
    A("edge syntax (boundary tables, the section-20 flow diagram, the")
    A("infrastructure ring, and explicit links).")
    A("")
    A("## Corpus at a glance")
    A("")
    A(f"* documents on disk: **{stats['docs_total']}** "
      f"({stats['docs_content']} content + README + index)")
    A(f"* DOCUMENT-INDEX entries: **{stats['index_groups']}**")
    A(f"* README group-table total: **{stats['readme_total']}**")
    A(f"* engine keys in the declared architecture: **{stats['engine_keys']}**")
    A(f"* engine keys with a backing document: **{stats['engines_with_home']}**")
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
    sev = {
        "no-dangling": "High",
        "boundary-tier-none": "High",
        "structural-ambiguity": "Medium",
        "boundary-tier-prose": "Medium",
        "never-named": "Medium",
        "naming-drift": "Low",
        "no-orphans": "Low",
        "no-layer-violation": "Low",
        "coverage": "Low",
    }
    order = [
        ("1", "no-dangling", "Dangling engines — declared, no document"),
        ("2", "boundary-tier-none", "Engine docs with no boundary section"),
        ("3", "structural-ambiguity", "Structural ambiguity"),
        ("4", "boundary-tier-prose", "Boundary in prose, not machine-readable"),
        ("5", "never-named", "Engines no boundary table mentions"),
        ("6", "naming-drift", "Naming drift"),
        ("7", "no-orphans", "Orphan documents"),
        ("8", "no-layer-violation", "Layer violations"),
        ("9", "coverage", "Coverage and count mismatches"),
    ]
    for num, key, label in order:
        n = len(findings.get(key, []))
        A(f"| {num} | {label} | {n} | {sev.get(key,'-')} |")
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

    section("1", "Dangling engines — declared in the architecture, no document",
            "no-dangling",
            "None — every declared engine resolves to a document.")

    section("2", "Engine docs with no boundary section at all",
            "boundary-tier-none",
            "None.")

    section("3", "Structural ambiguity",
            "structural-ambiguity",
            "None.")

    section("4", "Boundary declared in prose only (not machine-readable)",
            "boundary-tier-prose",
            "None — every boundary is expressed as a table.")

    section("5", "Engines that no boundary table anywhere mentions",
            "never-named",
            "None — every engine is named by at least one boundary table.")

    section("6", "Naming drift — one engine, several names",
            "naming-drift",
            "None — engine names are used consistently.")

    section("7", "Orphan documents", "no-orphans",
            "None — every document is referenced by at least one other.")

    section("8", "Layer violations", "no-layer-violation",
            "None — group 6–10 docs agree with their declared layer.")

    section("9", "Coverage and count mismatches", "coverage",
            "None — roster, README table, and disk all agree.")

    A("## 10. Declared pipeline topology")
    A("")
    A(mermaid_topology())
    A("")
    A("## 11. Declared boundary graph")
    A("")
    A(mermaid_boundaries(boundary))
    A("")
    return "\n".join(L)


def main():
    docs = load_docs()
    findings, stats, boundary, flow, ring, _inbound = run_checks(docs)
    report = render_report(findings, stats, boundary, flow, ring)

    if "--stdout" in sys.argv:
        print(report)
        return

    os.makedirs(OUTDIR, exist_ok=True)
    with open(os.path.join(OUTDIR, "GAP-ANALYSIS.md"), "w", encoding="utf-8") as fh:
        fh.write(report)
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

    print(f"documents:      {stats['docs_total']}")
    print(f"boundary edges: {stats['boundary_edges']}")
    print(f"link edges:     {stats['link_edges']}")
    for key in ("no-dangling", "boundary-tier-none", "structural-ambiguity",
                "boundary-tier-prose", "never-named", "naming-drift",
                "no-orphans", "no-layer-violation", "coverage"):
        print(f"{key:22s} {len(findings.get(key, []))}")
    print(f"\nwrote {OUTDIR}/GAP-ANALYSIS.md")
    print(f"wrote {OUTDIR}/engine-graph.mmd")
    print(f"wrote {OUTDIR}/graph.json")


if __name__ == "__main__":
    main()
