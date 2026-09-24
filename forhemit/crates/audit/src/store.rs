//! The append-only, hash-chained audit event store.
//!
//! Implementation Roadmap Phase 1: "Build the immutable event model first…
//! Establish the append-only storage pattern and basic integrity
//! verification." Storage is SQLite; append-only behavior is enforced at
//! three layers:
//!
//! 1. **API** — the store offers appends and reads only; there is no
//!    update or delete method (Audit doc integrity rule 12).
//! 2. **Storage** — `BEFORE UPDATE`/`BEFORE DELETE` guard triggers on both
//!    tables abort direct SQL mutation (integrity rule 13: corrections
//!    create additional events rather than rewriting history).
//! 3. **Verification** — [`AuditStore::verify_chain`] recomputes every
//!    row's content hash and payload digest and walks the
//!    `previous_event_hash` links, so tampering that defeats the guard
//!    triggers (dropping them) is reported as a named
//!    [`AuditStoreError::TamperDetected`].

use crate::error::{AuditStoreError, TamperEvidence, TamperKind};
use forhemit_contracts::{
    ActorRecord, AuditEvent, AuditEventDraft, AuditEventType, CausationId, CorrelationId, EngineId,
    EventId, ObjectId, PayloadRef, Sha256Hex, TransactionId, WorkspaceId,
};
use forhemit_enginekit::{AuditError, AuditSink, Clock};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// SHA-256 of the empty byte string — the `previous_event_hash` of the
/// first event in a fresh log.
pub const GENESIS_PREVIOUS_HASH_HEX: &str =
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

const TRIG_EVENT_NO_UPDATE: &str = "audit_events_no_update";
const TRIG_EVENT_NO_DELETE: &str = "audit_events_no_delete";
const TRIG_PAYLOAD_NO_UPDATE: &str = "audit_event_payloads_no_update";
const TRIG_PAYLOAD_NO_DELETE: &str = "audit_event_payloads_no_delete";

/// Guard triggers verification requires to still be installed.
const GUARD_TRIGGERS: [&str; 4] = [
    TRIG_EVENT_NO_UPDATE,
    TRIG_EVENT_NO_DELETE,
    TRIG_PAYLOAD_NO_UPDATE,
    TRIG_PAYLOAD_NO_DELETE,
];

const EVENT_COLUMNS: &str = "sequence, event_id, event_version, event_type, source_engine, \
     source_object, actor, timestamp_unix_nanos, workspace_id, transaction_id, correlation_id, \
     causation_id, payload_digest, previous_event_hash, event_hash";

const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS audit_events (
    sequence             INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id             TEXT NOT NULL UNIQUE,
    event_version        TEXT NOT NULL,
    event_type           TEXT NOT NULL,
    source_engine        TEXT NOT NULL,
    source_object        TEXT NOT NULL,
    actor                TEXT NOT NULL,
    timestamp_unix_nanos INTEGER NOT NULL,
    workspace_id         TEXT NOT NULL,
    transaction_id       TEXT,
    correlation_id       TEXT NOT NULL,
    causation_id         TEXT,
    payload_digest       TEXT NOT NULL,
    previous_event_hash  TEXT NOT NULL,
    event_hash           TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_event_payloads (
    payload_digest TEXT PRIMARY KEY,
    payload        BLOB NOT NULL
);

CREATE TRIGGER IF NOT EXISTS audit_events_no_update
BEFORE UPDATE ON audit_events
BEGIN
    SELECT RAISE(ABORT, 'audit_events is append-only: corrections create additional events rather than rewriting history (Audit integrity rule 13)');
END;

CREATE TRIGGER IF NOT EXISTS audit_events_no_delete
BEFORE DELETE ON audit_events
BEGIN
    SELECT RAISE(ABORT, 'audit_events is append-only (Audit integrity rule 12)');
END;

CREATE TRIGGER IF NOT EXISTS audit_event_payloads_no_update
BEFORE UPDATE ON audit_event_payloads
BEGIN
    SELECT RAISE(ABORT, 'audit_event_payloads is content-addressed and immutable');
END;

CREATE TRIGGER IF NOT EXISTS audit_event_payloads_no_delete
BEFORE DELETE ON audit_event_payloads
BEGIN
    SELECT RAISE(ABORT, 'audit_event_payloads is content-addressed and immutable');
END;
";

/// One stored audit event: the versioned contract plus the store's
/// integrity metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct StoredAuditEvent {
    /// Position in the append-only log, starting at 1.
    pub sequence: i64,
    /// The stored event (contract version V1).
    pub event: AuditEvent,
    /// Hash of this event's content — the value the next event chains to.
    pub event_hash: Sha256Hex,
}

/// Outcome of a passing chain verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainStatus {
    /// Number of events in the log.
    pub event_count: u64,
    /// Sequence of the last event, if any.
    pub head_sequence: Option<i64>,
    /// Hash of the last event, if any.
    pub head_event_hash: Option<Sha256Hex>,
}

/// The append-only audit event store for one workspace database.
///
/// Cheap to clone: all handles share one connection guarded by a mutex —
/// the single-writer pattern that keeps chain appends serialized. The
/// store owns no update or delete path at all.
#[derive(Clone)]
pub struct AuditStore {
    conn: Arc<Mutex<Connection>>,
    clock: Arc<dyn Clock>,
}

/// The raw column values of one stored row.
///
/// The event-hash input is exactly the JSON serialization of these values
/// in this field order (serde serializes struct fields in declaration
/// order), so the appender and the verifier hash identical bytes. Changing
/// the field set or order changes every stored hash — a contract break
/// that must ship as a new `AuditEvent` version.
#[derive(Serialize)]
struct HashRow<'a> {
    event_id: &'a str,
    event_version: &'a str,
    event_type: &'a str,
    source_engine: &'a str,
    source_object: &'a str,
    actor: &'a str,
    timestamp_unix_nanos: i64,
    workspace_id: &'a str,
    transaction_id: Option<&'a str>,
    correlation_id: &'a str,
    causation_id: Option<&'a str>,
    payload_digest: &'a str,
    previous_event_hash: &'a str,
}

/// A stored row mapped out of SQLite, before contract reconstruction.
struct RawStoredRow {
    sequence: i64,
    event_id: String,
    event_version: String,
    event_type: String,
    source_engine: String,
    source_object: String,
    actor: String,
    timestamp_unix_nanos: i64,
    workspace_id: String,
    transaction_id: Option<String>,
    correlation_id: String,
    causation_id: Option<String>,
    payload_digest: String,
    previous_event_hash: String,
    event_hash: String,
}

fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawStoredRow> {
    Ok(RawStoredRow {
        sequence: row.get(0)?,
        event_id: row.get(1)?,
        event_version: row.get(2)?,
        event_type: row.get(3)?,
        source_engine: row.get(4)?,
        source_object: row.get(5)?,
        actor: row.get(6)?,
        timestamp_unix_nanos: row.get(7)?,
        workspace_id: row.get(8)?,
        transaction_id: row.get(9)?,
        correlation_id: row.get(10)?,
        causation_id: row.get(11)?,
        payload_digest: row.get(12)?,
        previous_event_hash: row.get(13)?,
        event_hash: row.get(14)?,
    })
}

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// Hashes the canonical stored representation of one event.
fn content_hash(row: &HashRow<'_>) -> Result<Sha256Hex, AuditStoreError> {
    let bytes = serde_json::to_vec(row)?;
    let digest = to_hex(&Sha256::digest(bytes));
    Sha256Hex::parse(&digest)
        .map_err(|_| AuditStoreError::Internal("recomputed digest failed validation".to_owned()))
}

fn event_type_key(event_type: &AuditEventType) -> Result<String, AuditStoreError> {
    match serde_json::to_value(event_type)? {
        serde_json::Value::String(key) => Ok(key),
        other => Err(AuditStoreError::Internal(format!(
            "AuditEventType serialized as {other:?}, expected a string key"
        ))),
    }
}

fn engine_key(engine: &EngineId) -> Result<String, AuditStoreError> {
    match serde_json::to_value(engine)? {
        serde_json::Value::String(key) => Ok(key),
        other => Err(AuditStoreError::Internal(format!(
            "EngineId serialized as {other:?}, expected a string key"
        ))),
    }
}

fn event_type_from_key(key: &str) -> Result<AuditEventType, AuditStoreError> {
    serde_json::from_value(serde_json::Value::String(key.to_owned())).map_err(|error| {
        AuditStoreError::UnsupportedStoredEvent(format!("event_type {key:?}: {error}"))
    })
}

fn engine_from_key(key: &str) -> Result<EngineId, AuditStoreError> {
    serde_json::from_value(serde_json::Value::String(key.to_owned())).map_err(|error| {
        AuditStoreError::UnsupportedStoredEvent(format!("source_engine {key:?}: {error}"))
    })
}

fn genesis_previous_hash() -> Result<Sha256Hex, AuditStoreError> {
    Sha256Hex::parse(GENESIS_PREVIOUS_HASH_HEX).map_err(|_| {
        AuditStoreError::Internal("GENESIS_PREVIOUS_HASH_HEX is not a valid digest".to_owned())
    })
}

/// Store-written identifiers parse back through their constructors; a
/// failure means the stored value no longer satisfies the contract (a
/// downgrade reading a newer database, or tampering).
trait FromStored: Sized {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError>;
}

impl FromStored for EventId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        EventId::new(raw)
    }
}

impl FromStored for ObjectId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        ObjectId::new(raw)
    }
}

impl FromStored for WorkspaceId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        WorkspaceId::new(raw)
    }
}

impl FromStored for TransactionId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        TransactionId::new(raw)
    }
}

impl FromStored for CorrelationId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        CorrelationId::new(raw)
    }
}

impl FromStored for CausationId {
    fn from_stored(raw: &str) -> Result<Self, forhemit_contracts::ids::EmptyIdError> {
        CausationId::new(raw)
    }
}

fn parse_id<T: FromStored>(raw: &str, column: &'static str) -> Result<T, AuditStoreError> {
    T::from_stored(raw).map_err(|_| {
        AuditStoreError::UnsupportedStoredEvent(format!("{column}: identifier is empty"))
    })
}

fn tamper(sequence: i64, event_id: Option<EventId>, kind: TamperKind) -> AuditStoreError {
    AuditStoreError::TamperDetected(TamperEvidence {
        sequence,
        event_id,
        kind,
    })
}

impl RawStoredRow {
    /// Reconstructs the contract event from the stored columns.
    fn contract(&self) -> Result<AuditEvent, AuditStoreError> {
        let timestamp =
            time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(self.timestamp_unix_nanos))
                .map_err(|error| AuditStoreError::Time(error.to_string()))?;
        Ok(AuditEvent::V1 {
            event_id: parse_id(&self.event_id, "event_id")?,
            event_type: event_type_from_key(&self.event_type)?,
            source_engine: engine_from_key(&self.source_engine)?,
            source_object: parse_id(&self.source_object, "source_object")?,
            actor: serde_json::from_str(&self.actor).map_err(|error| {
                AuditStoreError::UnsupportedStoredEvent(format!("actor: {error}"))
            })?,
            timestamp,
            workspace_id: parse_id(&self.workspace_id, "workspace_id")?,
            transaction_id: self
                .transaction_id
                .as_deref()
                .map(|raw| parse_id(raw, "transaction_id"))
                .transpose()?,
            correlation_id: parse_id(&self.correlation_id, "correlation_id")?,
            causation_id: self
                .causation_id
                .as_deref()
                .map(|raw| parse_id(raw, "causation_id"))
                .transpose()?,
            payload_reference: PayloadRef {
                digest: Sha256Hex::parse(&self.payload_digest).map_err(|_| {
                    AuditStoreError::UnsupportedStoredEvent(
                        "payload_digest is malformed".to_owned(),
                    )
                })?,
            },
            previous_event_hash: Sha256Hex::parse(&self.previous_event_hash).map_err(|_| {
                AuditStoreError::UnsupportedStoredEvent(
                    "previous_event_hash is malformed".to_owned(),
                )
            })?,
        })
    }

    /// The canonical hash input for this row's content.
    fn hash_row(&self) -> HashRow<'_> {
        HashRow {
            event_id: &self.event_id,
            event_version: &self.event_version,
            event_type: &self.event_type,
            source_engine: &self.source_engine,
            source_object: &self.source_object,
            actor: &self.actor,
            timestamp_unix_nanos: self.timestamp_unix_nanos,
            workspace_id: &self.workspace_id,
            transaction_id: self.transaction_id.as_deref(),
            correlation_id: &self.correlation_id,
            causation_id: self.causation_id.as_deref(),
            payload_digest: &self.payload_digest,
            previous_event_hash: &self.previous_event_hash,
        }
    }
}

impl AuditStore {
    /// Opens (creating if needed) a store at `path`.
    pub fn open(path: impl AsRef<Path>, clock: Arc<dyn Clock>) -> Result<Self, AuditStoreError> {
        let conn = Connection::open(path)?;
        Self::init(conn, clock)
    }

    /// Opens a transient in-memory store — for tests and scratch use.
    pub fn open_in_memory(clock: Arc<dyn Clock>) -> Result<Self, AuditStoreError> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn, clock)
    }

    fn init(conn: Connection, clock: Arc<dyn Clock>) -> Result<Self, AuditStoreError> {
        conn.busy_timeout(Duration::from_millis(5_000))?;
        // WAL: writers never block readers. FULL: a committed append
        // survives power loss — an audit log may not lose its tail.
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            clock,
        })
    }

    /// Appends one event to the log.
    ///
    /// The store derives `event_id` (fresh ULID), `timestamp` (injected
    /// clock), and `previous_event_hash` (current chain head); the payload
    /// is serialized and addressed by its SHA-256 digest. The head read
    /// and the insert run inside one `BEGIN IMMEDIATE` transaction under
    /// the store's write lock — the single-writer pattern that keeps
    /// concurrent appends chain-consistent.
    pub fn append(&self, draft: AuditEventDraft) -> Result<StoredAuditEvent, AuditStoreError> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|_| AuditStoreError::LockPoisoned)?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

        let head: Option<String> = tx
            .query_row(
                "SELECT event_hash FROM audit_events ORDER BY sequence DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let previous_event_hash = match head.as_deref() {
            Some(stored) => Sha256Hex::parse(stored).map_err(|_| {
                AuditStoreError::Internal(
                    "stored head event_hash failed digest validation".to_owned(),
                )
            })?,
            None => genesis_previous_hash()?,
        };

        let event_id = crate::new_event_id();
        let timestamp = self.clock.now();
        let timestamp_unix_nanos = i64::try_from(timestamp.unix_timestamp_nanos())
            .map_err(|_| AuditStoreError::Time("clock time out of storable range".to_owned()))?;

        let payload_bytes = serde_json::to_vec(&draft.payload)?;
        let payload_digest = to_hex(&Sha256::digest(&payload_bytes));
        tx.execute(
            "INSERT INTO audit_event_payloads (payload_digest, payload) VALUES (?1, ?2)
             ON CONFLICT (payload_digest) DO NOTHING",
            params![payload_digest, payload_bytes],
        )?;

        let event_version = "v1";
        let event_type_key = event_type_key(&draft.event_type)?;
        let engine_key = engine_key(&draft.source_engine)?;
        let actor_json = serde_json::to_string(&draft.actor)?;

        let event_hash = content_hash(&HashRow {
            event_id: event_id.as_str(),
            event_version,
            event_type: &event_type_key,
            source_engine: &engine_key,
            source_object: draft.source_object.as_str(),
            actor: &actor_json,
            timestamp_unix_nanos,
            workspace_id: draft.workspace_id.as_str(),
            transaction_id: draft.transaction_id.as_ref().map(TransactionId::as_str),
            correlation_id: draft.correlation_id.as_str(),
            causation_id: draft.causation_id.as_ref().map(CausationId::as_str),
            payload_digest: &payload_digest,
            previous_event_hash: previous_event_hash.as_str(),
        })?;

        tx.execute(
            "INSERT INTO audit_events (
                event_id, event_version, event_type, source_engine, source_object, actor,
                timestamp_unix_nanos, workspace_id, transaction_id, correlation_id,
                causation_id, payload_digest, previous_event_hash, event_hash
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                event_id.as_str(),
                event_version,
                event_type_key,
                engine_key,
                draft.source_object.as_str(),
                actor_json,
                timestamp_unix_nanos,
                draft.workspace_id.as_str(),
                draft.transaction_id.as_ref().map(TransactionId::as_str),
                draft.correlation_id.as_str(),
                draft.causation_id.as_ref().map(CausationId::as_str),
                payload_digest,
                previous_event_hash.as_str(),
                event_hash.as_str(),
            ],
        )?;
        let sequence = tx.last_insert_rowid();

        tx.commit()?;

        Ok(StoredAuditEvent {
            sequence,
            event: AuditEvent::V1 {
                event_id,
                event_type: draft.event_type,
                source_engine: draft.source_engine,
                source_object: draft.source_object,
                actor: draft.actor,
                timestamp,
                workspace_id: draft.workspace_id,
                transaction_id: draft.transaction_id,
                correlation_id: draft.correlation_id,
                causation_id: draft.causation_id,
                payload_reference: PayloadRef {
                    digest: Sha256Hex::parse(&payload_digest).map_err(|_| {
                        AuditStoreError::Internal("payload digest failed validation".to_owned())
                    })?,
                },
                previous_event_hash,
            },
            event_hash,
        })
    }

    /// Appends a correction of an original event.
    ///
    /// Integrity rule 13: "Corrections create additional events rather
    /// than rewriting history." The correction inherits the original's
    /// source engine, object, workspace, and correlation id (Audit doc
    /// §18 — related activity shares a correlation identifier), and its
    /// `causation_id` names the original event. The original row is never
    /// touched.
    pub fn append_correction(
        &self,
        original_event_id: &EventId,
        event_type: AuditEventType,
        actor: ActorRecord,
        payload: serde_json::Value,
    ) -> Result<StoredAuditEvent, AuditStoreError> {
        let original = self.get_event(original_event_id)?.ok_or_else(|| {
            AuditStoreError::CorrectionTargetNotFound {
                event_id: original_event_id.clone(),
            }
        })?;
        let AuditEvent::V1 {
            source_engine,
            source_object,
            workspace_id,
            correlation_id,
            ..
        } = original.event;
        self.append(AuditEventDraft {
            event_type,
            source_engine,
            source_object,
            actor,
            workspace_id,
            // A correction is its own unit of work; linkage rides on
            // correlation and causation.
            transaction_id: None,
            correlation_id,
            causation_id: Some(parse_id(original_event_id.as_str(), "causation_id")?),
            payload,
        })
    }

    /// Fetches one stored event by id.
    pub fn get_event(
        &self,
        event_id: &EventId,
    ) -> Result<Option<StoredAuditEvent>, AuditStoreError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AuditStoreError::LockPoisoned)?;
        let raw = conn
            .query_row(
                &format!("SELECT {EVENT_COLUMNS} FROM audit_events WHERE event_id = ?1"),
                [event_id.as_str()],
                map_row,
            )
            .optional()?;
        raw.map(self_row_to_stored).transpose()
    }

    /// Every event about `object_id`, oldest first — the Roadmap Phase 1
    /// outcome question, "Who changed what, when, and which engine
    /// produced the change?"
    pub fn history_for_object(
        &self,
        object_id: &ObjectId,
    ) -> Result<Vec<StoredAuditEvent>, AuditStoreError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AuditStoreError::LockPoisoned)?;
        let mut stmt = conn.prepare(&format!(
            "SELECT {EVENT_COLUMNS} FROM audit_events WHERE source_object = ?1 ORDER BY sequence"
        ))?;
        let mut rows = stmt.query([object_id.as_str()])?;
        let mut history = Vec::new();
        while let Some(row) = rows.next()? {
            let raw = map_row(row)?;
            history.push(self_row_to_stored(raw)?);
        }
        Ok(history)
    }

    /// Verifies the log's integrity (Audit doc §16 — "The system must be
    /// able to detect tampering with historical audit records").
    ///
    /// Recomputes every row's content hash and payload digest, walks the
    /// `previous_event_hash` links back to the genesis digest, and requires
    /// the append-only guard triggers to still be installed. A clean log
    /// passes; any alteration returns [`AuditStoreError::TamperDetected`]
    /// naming the offending row.
    pub fn verify_chain(&self) -> Result<ChainStatus, AuditStoreError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AuditStoreError::LockPoisoned)?;

        for trigger in GUARD_TRIGGERS {
            let installed: i64 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'trigger' AND name = ?1",
                [trigger],
                |row| row.get(0),
            )?;
            if installed == 0 {
                return Err(tamper(
                    0,
                    None,
                    TamperKind::GuardTriggerMissing {
                        name: trigger.to_owned(),
                    },
                ));
            }
        }

        let mut stmt = conn.prepare(&format!(
            "SELECT {EVENT_COLUMNS} FROM audit_events ORDER BY sequence"
        ))?;
        let mut rows = stmt.query([])?;

        let mut event_count: u64 = 0;
        let mut expected_previous = genesis_previous_hash()?;
        let mut head: Option<(i64, Sha256Hex)> = None;

        while let Some(row) = rows.next()? {
            let raw = map_row(row)?;
            let sequence = raw.sequence;
            let event_id = EventId::new(&raw.event_id).ok();

            // 1. The row's content must hash to its stored event hash.
            let recomputed_hash = content_hash(&raw.hash_row())?;
            if recomputed_hash.as_str() != raw.event_hash {
                return Err(tamper(
                    sequence,
                    event_id,
                    TamperKind::ContentHashMismatch {
                        stored: raw.event_hash.clone(),
                        recomputed: recomputed_hash.to_string(),
                    },
                ));
            }

            // 2. The row must chain to the event before it.
            if raw.previous_event_hash != expected_previous.as_str() {
                return Err(tamper(
                    sequence,
                    event_id,
                    TamperKind::ChainLinkMismatch {
                        stored_previous: raw.previous_event_hash.clone(),
                        expected_previous: expected_previous.to_string(),
                    },
                ));
            }

            // 3. The referenced payload must exist and match its digest.
            let payload: Option<Vec<u8>> = conn
                .query_row(
                    "SELECT payload FROM audit_event_payloads WHERE payload_digest = ?1",
                    [raw.payload_digest.as_str()],
                    |row| row.get(0),
                )
                .optional()?;
            let Some(payload_bytes) = payload else {
                return Err(tamper(
                    sequence,
                    event_id,
                    TamperKind::MissingPayload {
                        digest: raw.payload_digest.clone(),
                    },
                ));
            };
            let recomputed_digest = to_hex(&Sha256::digest(&payload_bytes));
            if recomputed_digest != raw.payload_digest {
                return Err(tamper(
                    sequence,
                    event_id,
                    TamperKind::PayloadDigestMismatch {
                        digest: raw.payload_digest.clone(),
                        recomputed: recomputed_digest,
                    },
                ));
            }

            expected_previous = recomputed_hash;
            event_count += 1;
            head = Some((sequence, expected_previous.clone()));
        }

        Ok(ChainStatus {
            event_count,
            head_sequence: head.as_ref().map(|(sequence, _)| *sequence),
            head_event_hash: head.map(|(_, hash)| hash),
        })
    }
}

/// Reconstructs a stored event, verifying the row's integrity metadata
/// parses back into the contract.
fn self_row_to_stored(raw: RawStoredRow) -> Result<StoredAuditEvent, AuditStoreError> {
    let event = raw.contract()?;
    let event_hash = Sha256Hex::parse(&raw.event_hash).map_err(|_| {
        AuditStoreError::UnsupportedStoredEvent("event_hash is malformed".to_owned())
    })?;
    let sequence = raw.sequence;
    Ok(StoredAuditEvent {
        sequence,
        event,
        event_hash,
    })
}

/// The emission boundary engines use: they append drafts and can never
/// read, edit, or delete history. Errors are flattened into the sink's
/// stringly surface — the typed detail lives on [`AuditStoreError`].
impl AuditSink<AuditEventDraft> for AuditStore {
    fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
        self.append(event)
            .map(|_| ())
            .map_err(|error| AuditError(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use forhemit_contracts::{ActorClassification, ActorId, ActorKind};

    /// SHA-256 of the empty input, asserted against the well-known digest.
    #[test]
    fn genesis_is_sha256_of_empty_input() {
        let computed = to_hex(&Sha256::digest([]));
        assert_eq!(computed, GENESIS_PREVIOUS_HASH_HEX);
    }

    fn actor() -> ActorRecord {
        ActorRecord {
            actor_id: ActorId::new("owner_stefano").unwrap(),
            classification: ActorClassification::Human,
            kind: Some(ActorKind::Owner),
            origination: None,
        }
    }

    fn draft(event_type: AuditEventType, object: &str, correlation: &str) -> AuditEventDraft {
        AuditEventDraft {
            event_type,
            source_engine: EngineId::Audit,
            source_object: ObjectId::new(object).unwrap(),
            actor: actor(),
            workspace_id: WorkspaceId::new("ws_test").unwrap(),
            transaction_id: None,
            correlation_id: CorrelationId::new(correlation).unwrap(),
            causation_id: None,
            payload: serde_json::json!({ "note": "test" }),
        }
    }

    fn fixed_clock() -> Arc<dyn Clock> {
        struct Fixed;
        impl Clock for Fixed {
            fn now(&self) -> time::OffsetDateTime {
                time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap()
            }
        }
        Arc::new(Fixed)
    }

    #[test]
    fn appends_and_reads_back_the_full_contract() {
        let store = AuditStore::open_in_memory(fixed_clock()).unwrap();
        let stored = store
            .append(draft(
                AuditEventType::OwnerDecisionRecorded,
                "obj_1",
                "COR-1",
            ))
            .unwrap();

        assert_eq!(stored.sequence, 1);
        let AuditEvent::V1 {
            event_id,
            event_type,
            source_engine,
            source_object,
            actor: event_actor,
            timestamp,
            workspace_id,
            transaction_id,
            correlation_id,
            causation_id,
            payload_reference,
            previous_event_hash,
        } = &stored.event;
        assert_eq!(event_type, &AuditEventType::OwnerDecisionRecorded);
        assert_eq!(source_engine, &EngineId::Audit);
        assert_eq!(source_object.as_str(), "obj_1");
        assert_eq!(event_actor.actor_id.as_str(), "owner_stefano");
        assert_eq!(timestamp.unix_timestamp(), 1_700_000_000);
        assert_eq!(workspace_id.as_str(), "ws_test");
        assert!(transaction_id.is_none());
        assert_eq!(correlation_id.as_str(), "COR-1");
        assert!(causation_id.is_none());
        assert_eq!(previous_event_hash.as_str(), GENESIS_PREVIOUS_HASH_HEX);
        // The payload reference addresses the serialized payload bytes.
        let payload_bytes = serde_json::to_vec(&serde_json::json!({ "note": "test" })).unwrap();
        assert_eq!(
            payload_reference.digest.as_str(),
            to_hex(&Sha256::digest(&payload_bytes))
        );
        assert_eq!(event_id.as_str().len(), 26, "ULID length");

        let read_back = store.get_event(event_id).unwrap().unwrap();
        assert_eq!(read_back, stored);
        assert!(store.get_event(&crate::new_event_id()).unwrap().is_none());
    }

    #[test]
    fn history_for_object_filters_and_orders() {
        let store = AuditStore::open_in_memory(fixed_clock()).unwrap();
        store
            .append(draft(AuditEventType::OwnerDecisionRecorded, "obj_a", "C1"))
            .unwrap();
        store
            .append(draft(AuditEventType::ScenarioChanged, "obj_b", "C2"))
            .unwrap();
        store
            .append(draft(AuditEventType::OwnerDecisionRecorded, "obj_a", "C3"))
            .unwrap();

        let history = store
            .history_for_object(&ObjectId::new("obj_a").unwrap())
            .unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].sequence, 1);
        assert_eq!(history[1].sequence, 3);
    }
}
