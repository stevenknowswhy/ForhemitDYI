//! Schema-export coverage: every public contract type exports a
//! self-contained draft-07 schema. CI additionally regenerates
//! `forhemit/schema/` and fails on drift.
#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: malformed schema must fail the test loudly

use forhemit_contracts::schema::{export_index, export_per_type, CONTRACT_TYPES};

#[test]
fn export_covers_exactly_the_contract_types() {
    let exported: Vec<&str> = export_per_type().iter().map(|(n, _)| *n).collect();
    assert_eq!(
        exported, CONTRACT_TYPES,
        "export list must match CONTRACT_TYPES exactly"
    );
    let mut sorted = exported.clone();
    sorted.sort_unstable();
    assert_eq!(exported, sorted, "CONTRACT_TYPES must stay sorted");
}

#[test]
fn every_exported_schema_is_self_contained_draft07() {
    for (name, json) in export_per_type() {
        let doc: serde_json::Value =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(
            doc.get("$schema").and_then(|v| v.as_str()),
            Some("http://json-schema.org/draft-07/schema#"),
            "{name}: wrong dialect"
        );
        assert_eq!(
            doc.get("title").and_then(|v| v.as_str()),
            Some(name),
            "{name}: title must name the type"
        );
    }
}

#[test]
fn index_lists_the_types() {
    let index: serde_json::Value = serde_json::from_str(&export_index()).unwrap();
    let types = index
        .get("types")
        .and_then(|v| v.as_array())
        .expect("types array");
    assert_eq!(types.len(), CONTRACT_TYPES.len());
}
