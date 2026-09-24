//! Integration tests for the audit store — the spec's Verification row for
//! "Audit store": "append N events → verify chain; mutate one row →
//! verification returns a named tamper error; corrections add events,
//! never edit."
//!
//! Tamper tests go through rusqlite directly (as an attacker with file
//! access would), after dropping the guard triggers — the triggers
//! themselves are tested separately.

#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

use forhemit_audit::{new_correlation_id, AuditStore, AuditStoreError, TamperKind};
use forhemit_contracts::{
    ActorClassification, ActorId, ActorKind, ActorRecord, AuditEvent, AuditEventDraft,
    AuditEventType, CorrelationId, EngineId, EventId, ObjectId, Sha256Hex, WorkspaceId,
};
use forhemit_enginekit::{AuditSink, Clock};
use rusqlite::Connection;
use std::sync::Arc;
use std::thread;
use tempfile::TempDir;

/// A clock that advances one nanosecond per read, so timestamps never tie.
struct TickingClock {
    next: std::sync::Mutex<i128>,
}

impl Clock for TickingClock {
    fn now(&self) -> time::OffsetDateTime {
        let mut next = self.next.lock().unwrap();
        let value = *next;
        *next += 1;
        time::OffsetDateTime::from_unix_timestamp_nanos(value).unwrap()
    }
}

fn clock() -> Arc<dyn Clock> {
    Arc::new(TickingClock {
        next: std::sync::Mutex::new(1_700_000_000_000_000_000),
    })
}

fn actor() -> ActorRecord {
    ActorRecord {
        actor_id: ActorId::new("owner_stefano").unwrap(),
        classification: ActorClassification::Human,
        kind: Some(ActorKind::Owner),
        origination: None,
    }
}

fn draft(object: &str, correlation: &CorrelationId, note: &str) -> AuditEventDraft {
    AuditEventDraft {
        event_type: AuditEventType::OwnerDecisionRecorded,
        source_engine: EngineId::Destination,
        source_object: ObjectId::new(object).unwrap(),
        actor: actor(),
        workspace_id: WorkspaceId::new("ws_test").unwrap(),
        transaction_id: None,
        correlation_id: correlation.clone(),
        causation_id: None,
        payload: serde_json::json!({ "note": note }),
    }
}

/// Opens a file-backed store in a fresh temp directory, returning the
/// store and the database path (tamper tests open the file directly).
fn file_store() -> (AuditStore, TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let store = AuditStore::open(dir.path().join("audit.db"), clock()).unwrap();
    (store, dir)
}

/// The database file behind a store, opened for direct (attacker) access.
fn raw_connection(dir: &TempDir) -> Connection {
    Connection::open(dir.path().join("audit.db")).unwrap()
}

/// Re-creates any dropped guard triggers — a realistic tamperer covers
/// their tracks, and only then does row-level verification reach the
/// specific tamper kind under test.
fn restore_guard_triggers(conn: &Connection) {
    conn.execute_batch(
        "CREATE TRIGGER IF NOT EXISTS audit_events_no_update
         BEFORE UPDATE ON audit_events BEGIN SELECT RAISE(ABORT, 'restored'); END;
         CREATE TRIGGER IF NOT EXISTS audit_events_no_delete
         BEFORE DELETE ON audit_events BEGIN SELECT RAISE(ABORT, 'restored'); END;
         CREATE TRIGGER IF NOT EXISTS audit_event_payloads_no_update
         BEFORE UPDATE ON audit_event_payloads BEGIN SELECT RAISE(ABORT, 'restored'); END;
         CREATE TRIGGER IF NOT EXISTS audit_event_payloads_no_delete
         BEFORE DELETE ON audit_event_payloads BEGIN SELECT RAISE(ABORT, 'restored'); END;",
    )
    .unwrap();
}

fn tamper_kind(error: AuditStoreError) -> TamperKind {
    match error {
        AuditStoreError::TamperDetected(evidence) => evidence.kind,
        other => panic!("expected TamperDetected, got {other:?}"),
    }
}

#[test]
fn append_n_events_then_verify_chain_passes() {
    let (store, _dir) = file_store();
    let correlation = new_correlation_id();
    let mut last_hash = None;
    for index in 0..5 {
        let stored = store
            .append(draft(&format!("obj_{index}"), &correlation, "step"))
            .unwrap();
        last_hash = Some(stored.event_hash.clone());
    }

    let status = store.verify_chain().unwrap();
    assert_eq!(status.event_count, 5);
    assert_eq!(status.head_sequence, Some(5));
    assert_eq!(
        status.head_event_hash.as_ref().map(Sha256Hex::as_str),
        last_hash.as_ref().map(Sha256Hex::as_str)
    );
}

#[test]
fn chain_walk_links_every_event_to_its_predecessor() {
    let (store, _dir) = file_store();
    let correlation = new_correlation_id();
    let mut previous: Option<Sha256Hex> = None;
    for index in 0..4 {
        let stored = store
            .append(draft(&format!("obj_{index}"), &correlation, "step"))
            .unwrap();
        let AuditEvent::V1 {
            previous_event_hash,
            ..
        } = &stored.event;
        match (&previous, previous_event_hash.as_str()) {
            (None, genesis) => assert_eq!(
                genesis,
                forhemit_audit::GENESIS_PREVIOUS_HASH_HEX,
                "first event chains to the genesis digest"
            ),
            (Some(expected), actual) => assert_eq!(actual, expected.as_str()),
        }
        previous = Some(stored.event_hash.clone());
    }
    assert!(store.verify_chain().is_ok(), "clean log verifies");
}

#[test]
fn direct_sql_update_is_blocked_by_guard_trigger() {
    let (store, dir) = file_store();
    store
        .append(draft("obj_1", &new_correlation_id(), "original"))
        .unwrap();

    let attacker = raw_connection(&dir);
    let error = attacker
        .execute(
            "UPDATE audit_events SET event_type = 'scenario_changed'",
            [],
        )
        .expect_err("guard trigger must abort the update");
    assert!(
        error.to_string().contains("append-only"),
        "unexpected error: {error}"
    );

    let error = attacker
        .execute("DELETE FROM audit_events", [])
        .expect_err("guard trigger must abort the delete");
    assert!(
        error.to_string().contains("append-only"),
        "unexpected error: {error}"
    );

    assert!(store.verify_chain().is_ok(), "log unchanged and intact");
}

#[test]
fn edited_row_content_is_detected_as_tamper() {
    let (store, dir) = file_store();
    for index in 0..3 {
        store
            .append(draft(
                &format!("obj_{index}"),
                &new_correlation_id(),
                "step",
            ))
            .unwrap();
    }

    // Attacker: remove the guards, then rewrite one row's event type.
    let attacker = raw_connection(&dir);
    attacker
        .execute_batch("DROP TRIGGER audit_events_no_update; DROP TRIGGER audit_events_no_delete;")
        .unwrap();
    attacker
        .execute(
            "UPDATE audit_events SET event_type = 'scenario_changed' WHERE sequence = 2",
            [],
        )
        .unwrap();
    restore_guard_triggers(&attacker);

    let error = store.verify_chain().expect_err("tampering must be caught");
    match tamper_kind(error) {
        TamperKind::ContentHashMismatch { .. } => {} // the named error
        other => panic!("expected ContentHashMismatch, got {other:?}"),
    }
}

#[test]
fn altered_payload_bytes_are_detected() {
    let (store, dir) = file_store();
    for index in 0..2 {
        store
            .append(draft(
                &format!("obj_{index}"),
                &new_correlation_id(),
                "step",
            ))
            .unwrap();
    }

    let attacker = raw_connection(&dir);
    attacker
        .execute_batch("DROP TRIGGER audit_event_payloads_no_update; DROP TRIGGER audit_event_payloads_no_delete;")
        .unwrap();
    attacker
        .execute(
            "UPDATE audit_event_payloads SET payload = ?1",
            [b"forged".to_vec()],
        )
        .unwrap();
    restore_guard_triggers(&attacker);

    let error = store
        .verify_chain()
        .expect_err("payload tampering must be caught");
    match tamper_kind(error) {
        TamperKind::PayloadDigestMismatch { .. } => {}
        other => panic!("expected PayloadDigestMismatch, got {other:?}"),
    }
}

#[test]
fn missing_payload_is_detected() {
    let (store, dir) = file_store();
    store
        .append(draft("obj_1", &new_correlation_id(), "step"))
        .unwrap();

    let attacker = raw_connection(&dir);
    attacker
        .execute_batch("DROP TRIGGER audit_event_payloads_no_delete;")
        .unwrap();
    attacker
        .execute("DELETE FROM audit_event_payloads", [])
        .unwrap();
    restore_guard_triggers(&attacker);

    let error = store
        .verify_chain()
        .expect_err("missing payload must be caught");
    match tamper_kind(error) {
        TamperKind::MissingPayload { .. } => {}
        other => panic!("expected MissingPayload, got {other:?}"),
    }
}

#[test]
fn deleted_middle_row_breaks_the_chain() {
    let (store, dir) = file_store();
    for index in 0..4 {
        store
            .append(draft(
                &format!("obj_{index}"),
                &new_correlation_id(),
                "step",
            ))
            .unwrap();
    }

    let attacker = raw_connection(&dir);
    attacker
        .execute_batch("DROP TRIGGER audit_events_no_update; DROP TRIGGER audit_events_no_delete;")
        .unwrap();
    attacker
        .execute("DELETE FROM audit_events WHERE sequence = 2", [])
        .unwrap();
    restore_guard_triggers(&attacker);

    let error = store.verify_chain().expect_err("deletion must be caught");
    match tamper_kind(error) {
        TamperKind::ChainLinkMismatch { .. } => {}
        other => panic!("expected ChainLinkMismatch, got {other:?}"),
    }
}

#[test]
fn forged_insert_that_breaks_the_link_is_detected() {
    let (store, dir) = file_store();
    store
        .append(draft("obj_1", &new_correlation_id(), "real"))
        .unwrap();

    // A forged INSERT needs no trigger bypass — no update/delete trigger
    // fires on insert. The forged row claims to precede the real one: its
    // content hashes consistently, but the real row no longer chains to it.
    let attacker = raw_connection(&dir);
    attacker
        .execute(
            "INSERT INTO audit_events (
                event_id, event_version, event_type, source_engine, source_object, actor,
                timestamp_unix_nanos, workspace_id, transaction_id, correlation_id,
                causation_id, payload_digest, previous_event_hash, event_hash
            ) VALUES ('01FORGEDFORGEDFORGEDFORGEDFORGE', 'v1', 'owner_decision_recorded',
                'destination', 'obj_forged', '{}', 0, 'ws_test', NULL, 'C', NULL,
                'deadbeef', ?, 'forgedhash')",
            [forhemit_audit::GENESIS_PREVIOUS_HASH_HEX],
        )
        .unwrap();

    let error = store.verify_chain().expect_err("forged row must be caught");
    match tamper_kind(error) {
        TamperKind::ContentHashMismatch { .. } | TamperKind::ChainLinkMismatch { .. } => {}
        other => panic!("expected a tamper error, got {other:?}"),
    }
}

#[test]
fn removed_guard_trigger_is_detected() {
    let (store, dir) = file_store();
    store
        .append(draft("obj_1", &new_correlation_id(), "step"))
        .unwrap();

    let attacker = raw_connection(&dir);
    attacker
        .execute_batch("DROP TRIGGER audit_event_payloads_no_update;")
        .unwrap();

    let error = store
        .verify_chain()
        .expect_err("missing guard trigger must be caught");
    match tamper_kind(error) {
        TamperKind::GuardTriggerMissing { name } => {
            assert_eq!(name, "audit_event_payloads_no_update");
        }
        other => panic!("expected GuardTriggerMissing, got {other:?}"),
    }
}

#[test]
fn correction_appends_and_never_edits() {
    let (store, _dir) = file_store();
    let correlation = new_correlation_id();
    let original = store
        .append(draft("obj_1", &correlation, "mistaken"))
        .unwrap();

    let correction = store
        .append_correction(
            original.event.event_id(),
            AuditEventType::CorrectionRecorded,
            actor(),
            serde_json::json!({ "note": "corrected", "replaces": original.event.event_id().as_str() }),
        )
        .unwrap();

    // The original event is untouched and still readable.
    let original_read = store.get_event(original.event.event_id()).unwrap().unwrap();
    assert_eq!(original_read, original);

    // The correction chains causation to the original and shares its
    // correlation id (Audit doc §18).
    let AuditEvent::V1 {
        causation_id,
        correlation_id,
        ..
    } = &correction.event;
    assert_eq!(
        causation_id.as_ref().unwrap().as_str(),
        original.event.event_id().as_str()
    );
    assert_eq!(correlation_id, &correlation);

    // Two events about the object, in order.
    let history = store
        .history_for_object(&ObjectId::new("obj_1").unwrap())
        .unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].event_hash, original.event_hash);
    assert_eq!(history[1].event_hash, correction.event_hash);

    assert!(store.verify_chain().is_ok(), "corrected log still verifies");
}

#[test]
fn correction_of_missing_event_is_a_named_error() {
    let (store, _dir) = file_store();
    let error = store
        .append_correction(
            &EventId::new("01NOPE").unwrap(),
            AuditEventType::CorrectionRecorded,
            actor(),
            serde_json::json!({}),
        )
        .expect_err("correction needs an existing target");
    assert!(matches!(
        error,
        AuditStoreError::CorrectionTargetNotFound { .. }
    ));
}

#[test]
fn concurrent_emitters_keep_the_chain_consistent() {
    let (store, dir) = file_store();
    let handles: Vec<_> = (0..8)
        .map(|worker| {
            let store = store.clone();
            let correlation = new_correlation_id();
            thread::spawn(move || {
                for index in 0..4 {
                    store
                        .append(draft(
                            &format!("obj_{worker}_{index}"),
                            &correlation,
                            "parallel",
                        ))
                        .unwrap();
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }

    let status = store.verify_chain().unwrap();
    assert_eq!(status.event_count, 32, "every concurrent append landed");
    assert_eq!(status.head_sequence, Some(32));

    // Sequences are dense from 1: the single-writer transaction never
    // handed two writers the same chain position.
    let probe = raw_connection(&dir);
    let (min_sequence, max_sequence, count): (i64, i64, i64) = probe
        .query_row(
            "SELECT MIN(sequence), MAX(sequence), COUNT(*) FROM audit_events",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!((min_sequence, max_sequence, count), (1, 32, 32));
}

#[test]
fn store_persists_across_reopen() {
    let (store, dir) = file_store();
    let correlation = new_correlation_id();
    let first = store.append(draft("obj_1", &correlation, "one")).unwrap();
    let second = store.append(draft("obj_2", &correlation, "two")).unwrap();
    drop(store);

    let reopened = AuditStore::open(dir.path().join("audit.db"), clock()).unwrap();
    assert_eq!(
        reopened.get_event(first.event.event_id()).unwrap().unwrap(),
        first
    );
    assert_eq!(
        reopened
            .get_event(second.event.event_id())
            .unwrap()
            .unwrap(),
        second
    );
    assert_eq!(reopened.verify_chain().unwrap().event_count, 2);

    // Appends continue the existing chain after reopen.
    let third = reopened
        .append(draft("obj_3", &correlation, "three"))
        .unwrap();
    let AuditEvent::V1 {
        previous_event_hash,
        ..
    } = &third.event;
    assert_eq!(previous_event_hash.as_str(), second.event_hash.as_str());
}

#[test]
fn store_is_usable_as_the_enginekit_audit_sink() {
    let (store, _dir) = file_store();
    let sink: &dyn AuditSink<AuditEventDraft> = &store;
    sink.emit(draft("obj_1", &new_correlation_id(), "via sink"))
        .unwrap();
    assert_eq!(store.verify_chain().unwrap().event_count, 1);
}
